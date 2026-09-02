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

use aide::{axum::ApiRouter, openapi::OpenApi, transform::TransformOpenApi};
use axum::{
    Extension, Router,
    extract::DefaultBodyLimit,
    http::{HeaderValue, Method, header},
};
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
use std::sync::Arc;
use tokio::fs;
use tower_http::cors::CorsLayer;
use translation_service::TranslationService;
use websockets::WebSocketService;
use websockets::handlers::video_call::VideoCallMessageHandler;

use crate::bulk_storage_service::BulkStorageService;
use crate::categorization_service::CategorizationService;
use crate::redis_connection::RedisConnection;
use crate::routes::workflows::WorkflowRouterContext;
use crate::transcription_service::Transcriber;
use crate::wiki_poll_service::WikiPollService;
use crate::worker_service::WorkerService;

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
/// Constructs the ApiRouter and extracts the OpenAPI spec.
/// Note that sub-routers like `routes::auth::router` are async and must be `.await`ed.
pub async fn build_app_and_spec(state: Arc<ComhairleState>) -> (Router, OpenApi) {
    aide::generate::on_error(|error| {
        println!("{error}");
    });

    aide::generate::extract_schemas(true);
    let mut api = OpenApi::default();

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

    // Rate limiting is applied per route inside the auth router: the strict
    // limiter belongs on the credential endpoints, not on session reads.
    // Added `.await` here to resolve the opaque Future issue
    let auth_router = routes::auth::router(state.clone()).await;

    let router = ApiRouter::new()
        .route("/health", axum::routing::get(health_check))
        .nest_api_service("/auth", auth_router)
        .nest_api_service(
            "/user",
            routes::user::router(state.clone())
                .nest_api_service(
                    "/preferences",
                    routes::user_conversation_preferences::router(state.clone()),
                )
                .nest_api_service("/profile", routes::user_profile::router(state.clone())),
        )
        .nest_api_service(
            "/notifications",
            routes::notifications::router(state.clone()),
        )
        .nest_api_service("/translations", routes::translations::router(state.clone()))
        .nest_api_service("/tools", tools::router(state.clone()))
        .nest_api_service(
            "/conversation",
            routes::conversations::router(state.clone())
                .nest_api_service(
                    "/{conversation_id}/workflow",
                    routes::workflows::router(state.clone(), WorkflowRouterContext::Conversation)
                        .nest_api_service(
                            "/{workflow_id}/workflow_step",
                            routes::workflow_steps::router(
                                state.clone(),
                                WorkflowRouterContext::Conversation,
                            ),
                        )
                        .nest_api_service(
                            "/{workflow_id}/progress",
                            routes::user_progress::router(state.clone()),
                        )
                        .nest_api_service(
                            "/{workflow_id}/recruitment_targets",
                            routes::recruitment_targets::router(state.clone()),
                        ),
                )
                .nest_api_service(
                    "/{conversation_id}/invite",
                    routes::invites::router(state.clone()),
                )
                .nest_api_service(
                    "/{conversation_id}/report",
                    routes::reports::router(state.clone()).nest_api_service(
                        "/{report_id}/impacts",
                        routes::report_impacts::router(state.clone()),
                    ),
                )
                .nest_api_service(
                    "/{conversation_id}/feedback",
                    routes::feedback::router(state.clone()),
                )
                .nest_api_service(
                    "/{conversation_id}/chats",
                    routes::chats::router(state.clone()),
                )
                .nest_api_service(
                    "/{conversation_id}/chat_sessions",
                    routes::chat_sessions::router(state.clone()),
                )
                .nest_api_service(
                    "/{conversation_id}/documents",
                    routes::documents::router(state.clone()),
                )
                .nest_api_service(
                    "/{conversation_id}/events",
                    routes::events::router(state.clone())
                        .nest_api_service(
                            "/{event_id}/attendances",
                            routes::event_attendances::router(state.clone()),
                        )
                        .nest_api_service(
                            "/{event_id}/workflows",
                            routes::workflows::router(state.clone(), WorkflowRouterContext::Event)
                                .nest(
                                    "/{workflow_id}/workflow_steps",
                                    routes::workflow_steps::router(
                                        state.clone(),
                                        WorkflowRouterContext::Event,
                                    ),
                                ),
                        )
                        .nest_api_service(
                            "/{event_id}/audio_recordings",
                            routes::audio_recordings::router(state.clone()),
                        ),
                ),
        )
        .nest_api_service(
            "/ws",
            websockets::routes::websocket_routes().with_state(state.clone()),
        )
        .nest_api_service(
            "/organizations",
            routes::organizations::router(state.clone()),
        )
        .nest_api_service("/regions", routes::regions::router(state.clone()))
        .nest_api_service("/region_areas", routes::region_areas::router(state.clone()))
        .nest_api_service("/media", routes::media::router(state.clone()))
        .nest_api_service("/jobs", routes::jobs::router(state.clone()))
        .nest_api_service("/services", routes::services::router(state.clone()))
        .nest_api_service("/api_keys", routes::api_keys::router(state.clone()))
        .nest_api_service(
            "/email_template_configs",
            routes::email_template_configs::router(state.clone()),
        )
        .nest_api_service("/permissions", routes::permissions::router(state.clone()))
        .nest_api_service("/docs", docs_routes(state.clone()))
        .finish_api_with(&mut api, api_docs)
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::request_logging::log_requests,
        ))
        .layer(Extension(Arc::new(api.clone()))) // Arc is very important here or you will face massive memory and performance issues
        .layer(DefaultBodyLimit::max(500 * 1024 * 1024))
        .layer(cors);

    (router, api)
}

pub async fn setup_server(state: Arc<ComhairleState>) -> Result<Router<()>, ComhairleError> {
    let args = Args::try_parse().unwrap_or_default();

    tracing::info!("Running with config {:#?}", state.config);

    run_migrations(&state.db).await?;

    let (app, api) = build_app_and_spec(state).await;

    if args.export_api_spec {
        let json = serde_json::to_string_pretty(&api).unwrap();
        fs::write("open-api-spec.json", json.as_bytes()).await?;
    }

    Ok(app)
}
