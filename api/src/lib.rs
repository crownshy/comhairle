pub mod bot_service;
pub mod bulk_storage_service;
pub mod categorization_service;
pub mod config;
pub mod db;
mod docs;
pub mod error;
pub mod mailer;
mod middleware;
pub mod models;
pub mod redis_connection;
mod routes;
pub mod schema_helpers;
#[cfg(test)]
mod test_helpers;
mod tools;
pub mod transcription_service;
pub mod translation_service;
pub mod websockets;
pub mod wiki_poll_service;
pub mod worker_service;

use std::sync::Arc;

use aide::{axum::ApiRouter, openapi::OpenApi, transform::TransformOpenApi};
use axum::extract::DefaultBodyLimit;
use axum::http::{HeaderValue, Method, header};
use axum::{Extension, Router};
use bot_service::ComhairleBotService;
use clap::Parser;
use config::ComhairleConfig;
use db::run_migrations;
use docs::docs_routes;
use error::ComhairleError;
use mailer::ComhairleMailer;
use routes::auth::AUTH_KEY;
pub use routes::auth::hash_pw;
use sqlx_postgres::PgPool;
use tower::Layer;
use tower::util::option_layer;
use tower_http::cors::CorsLayer;
use tower_http::normalize_path::{NormalizePath, NormalizePathLayer};
use translation_service::TranslationService;
use websockets::WebSocketService;
use websockets::handlers::video_call::VideoCallMessageHandler;

use crate::categorization_service::CategorizationService;
use crate::redis_connection::RedisConnection;
use crate::routes::workflows::WorkflowRouterContext;
use crate::transcription_service::Transcriber;
use crate::wiki_poll_service::WikiPollService;
use crate::worker_service::WorkerService;
use crate::{
    bulk_storage_service::BulkStorageService, middleware::rate_limit::auth_rate_limiter_if_enabled,
};

pub type App = NormalizePath<Router<()>>;

#[cfg(test)]
// sqlx::test expands every migration into the test binary for every invocation.
// So, it massively bloats both the binary size and compile time.
// Using a common migrator for all tests avoids this issue.
const SQLX_MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

#[derive(Clone)]
pub struct ComhairleState {
    pub db: PgPool,
    pub config: ComhairleConfig,
    pub mailer: Arc<dyn ComhairleMailer>,
    pub websockets: Arc<dyn WebSocketService>,
    /// Video call handler, held here (as well as registered on `websockets`) so HTTP
    /// routes can push updates to participants currently on a call (e.g. agenda changes).
    pub video_call_handler: Arc<VideoCallMessageHandler>,
    pub translation_service: Option<Arc<dyn TranslationService>>,
    pub bot_service: Option<Arc<dyn ComhairleBotService>>,
    pub wiki_poll_service: Arc<dyn WikiPollService>,
    pub bulk_storage_service: Option<Arc<dyn BulkStorageService>>,
    pub transcription_service: Option<Arc<dyn Transcriber>>,
    pub worker_service: Option<Arc<dyn WorkerService>>,
    pub categorization_service: Option<Arc<dyn CategorizationService>>,
    pub redis_conn: Option<Arc<dyn RedisConnection>>,
}

impl ComhairleState {
    fn required_bot_service(&self) -> Result<&Arc<dyn ComhairleBotService>, ComhairleError> {
        self.bot_service
            .as_ref()
            .ok_or(ComhairleError::NoBotServiceConfigured)
    }

    fn required_transcription_service(&self) -> Result<&Arc<dyn Transcriber>, ComhairleError> {
        self.transcription_service
            .as_ref()
            .ok_or(ComhairleError::NoTranscriptionServiceConfigured)
    }

    fn required_worker_service(&self) -> Result<&Arc<dyn WorkerService>, ComhairleError> {
        self.worker_service
            .as_ref()
            .ok_or(ComhairleError::NoWorkerServiceConfigured)
    }

    fn required_categorization_service(
        &self,
    ) -> Result<&Arc<dyn CategorizationService>, ComhairleError> {
        self.categorization_service
            .as_ref()
            .ok_or(ComhairleError::NoCategorizationServiceConfigured)
    }

    fn required_bulk_storage_service(
        &self,
    ) -> Result<&Arc<dyn BulkStorageService>, ComhairleError> {
        self.bulk_storage_service
            .as_ref()
            .ok_or(ComhairleError::NoBulkStorageServiceConfigured)
    }
}

fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Comhairle API")
        .summary("The API for the comhairle system")
        .description("An api for governence")
        .security_scheme(
            "JWT",
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Cookie,
                name: AUTH_KEY.into(),
                description: Some("A JWT for the current user".into()),
                extensions: Default::default(),
            },
        )
}

#[derive(Parser, Debug, Default)]
pub struct Args {
    #[arg(
        long,
        short = 'x',
        help = "Export open api spec json to a file to allow generation of the api client"
    )]
    export_api_spec: bool,
}

async fn health_check() -> &'static str {
    "OK"
}

pub async fn build_router_and_spec(
    enable_auth_rate_limiting: bool,
) -> (Router<Arc<ComhairleState>>, OpenApi) {
    aide::generate::on_error(|error| {
        tracing::error!("{error}");
    });

    aide::generate::extract_schemas(true);
    let mut api = OpenApi::default();

    let credential_limit_layer =
        option_layer(auth_rate_limiter_if_enabled(enable_auth_rate_limiting));

    let router = ApiRouter::<Arc<ComhairleState>>::new()
        .route("/health", axum::routing::get(health_check))
        .nest("/auth", routes::auth::router(credential_limit_layer).await)
        .nest(
            "/user",
            routes::user::router()
                .nest(
                    "/preferences",
                    routes::user_conversation_preferences::router(),
                )
                .nest("/profile", routes::user_profile::router()),
        )
        .nest("/notifications", routes::notifications::router())
        .nest("/translations", routes::translations::router())
        .nest("/tools", tools::router())
        .nest(
            "/conversation",
            routes::conversations::router()
                .nest(
                    "/{conversation_id}/workflow",
                    routes::workflows::router(WorkflowRouterContext::Conversation)
                        .nest(
                            "/{workflow_id}/workflow_step",
                            routes::workflow_steps::router(WorkflowRouterContext::Conversation),
                        )
                        .nest("/{workflow_id}/progress", routes::user_progress::router())
                        .nest(
                            "/{workflow_id}/recruitment_targets",
                            routes::recruitment_targets::router(),
                        ),
                )
                .nest("/{conversation_id}/invite", routes::invites::router())
                .nest(
                    "/{conversation_id}/report",
                    routes::reports::router()
                        .nest("/{report_id}/impacts", routes::report_impacts::router()),
                )
                .nest("/{conversation_id}/feedback", routes::feedback::router())
                .nest("/{conversation_id}/chats", routes::chats::router())
                .nest(
                    "/{conversation_id}/chat_instructions",
                    routes::chat_instructions::router(),
                )
                .nest(
                    "/{conversation_id}/moderation_policies",
                    routes::moderation_policies::router(),
                )
                .nest(
                    "/{conversation_id}/chat_sessions",
                    routes::chat_sessions::router(),
                )
                .nest("/{conversation_id}/documents", routes::documents::router())
                .nest(
                    "/{conversation_id}/events",
                    routes::events::router()
                        .nest(
                            "/{event_id}/attendances",
                            routes::event_attendances::router(),
                        )
                        .nest(
                            "/{event_id}/workflows",
                            routes::workflows::router(WorkflowRouterContext::Event).nest(
                                "/{workflow_id}/workflow_steps",
                                routes::workflow_steps::router(WorkflowRouterContext::Event),
                            ),
                        )
                        .nest(
                            "/{event_id}/audio_recordings",
                            routes::audio_recordings::router(),
                        ),
                ),
        )
        .nest("/ws", websockets::routes::websocket_routes())
        .nest("/organizations", routes::organizations::router())
        .nest("/regions", routes::regions::router())
        .nest("/region_areas", routes::region_areas::router())
        .nest("/media", routes::media::router())
        .nest("/jobs", routes::jobs::router())
        .nest("/services", routes::services::router())
        .nest("/api_keys", routes::api_keys::router())
        .nest(
            "/email_template_configs",
            routes::email_template_configs::router(),
        )
        .nest("/permissions", routes::permissions::router())
        .nest("/docs", docs_routes())
        .nest("/demographics", routes::demographics::router())
        .finish_api_with(&mut api, api_docs);

    (router, api)
}

/// Constructs the ApiRouter and extracts the OpenAPI spec.
/// Note that sub-routers like `routes::auth::router` are async and must be `.await`ed.
pub async fn build_app(state: Arc<ComhairleState>, export_spec: bool) -> App {
    // Setup CORS
    let mut allowed_origins = vec![
        "http://localhost".parse::<HeaderValue>().unwrap(),
        "http://localhost:3000".parse::<HeaderValue>().unwrap(),
        "http://localhost:5173".parse::<HeaderValue>().unwrap(),
        "https://stage.comhairle.scot"
            .parse::<HeaderValue>()
            .unwrap(),
    ];

    if let Some(whitelisted_domains) = &state.config.whitelisted_domains {
        for domain in whitelisted_domains {
            if let Ok(header_value) = domain.parse::<HeaderValue>() {
                allowed_origins.push(header_value);
                tracing::info!("Adding whitelisted domain to CORS: {}", domain);
            } else {
                tracing::warn!("Invalid domain format, skipping: {}", domain);
            }
        }
    }

    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::OPTIONS,
            Method::DELETE,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::ACCEPT_LANGUAGE,
            header::ACCEPT_ENCODING,
        ])
        .allow_origin(allowed_origins);

    let (router, api) = build_router_and_spec(state.config.enable_rate_limiting).await;

    if export_spec {
        let json = serde_json::to_string_pretty(&api).unwrap();
        tokio::fs::write("open-api-spec.json", json.as_bytes())
            .await
            .unwrap();
    }

    NormalizePathLayer::trim_trailing_slash().layer(
        router
            .with_state(state.clone())
            .layer(axum::middleware::from_fn_with_state(
                state,
                middleware::request_logging::log_requests,
            ))
            .layer(Extension(Arc::new(api.clone()))) // Arc is very important here or you will face massive memory and performance issues
            .layer(DefaultBodyLimit::max(500 * 1024 * 1024))
            .layer(cors),
    )
}

pub async fn setup_server(state: Arc<ComhairleState>) -> Result<App, ComhairleError> {
    let args = Args::try_parse().unwrap_or_default();

    tracing::info!("Running with config {:#?}", state.config);

    run_migrations(&state.db).await?;

    Ok(build_app(state, args.export_api_spec).await)
}
