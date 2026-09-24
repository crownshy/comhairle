pub mod auth_service;
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
pub mod routes;
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

use aide::axum::ApiRouter;
use aide::axum::routing::ApiMethodRouter;
use aide::openapi::OpenApi;
use aide::transform::TransformOpenApi;
use axum::extract::DefaultBodyLimit;
use axum::http::{HeaderValue, Method, header};
use axum::{Extension, Router};
use axum_keycloak_auth::extract::TokenExtractor;
use axum_keycloak_auth::instance::{KeycloakAuthInstance, KeycloakConfig};
use axum_keycloak_auth::layer::KeycloakAuthLayer;
use axum_keycloak_auth::{NonEmpty, PassthroughMode, Url};
use bot_service::ComhairleBotService;
use clap::Parser;
use config::ComhairleConfig;
use db::run_migrations;
use docs::docs_routes;
use error::ComhairleError;
use mailer::ComhairleMailer;
use routes::auth::AUTH_KEY;
use sqlx_postgres::PgPool;
use tower::Layer;
use tower::util::option_layer;
use tower_http::cors::CorsLayer;
use tower_http::normalize_path::{NormalizePath, NormalizePathLayer};
use translation_service::TranslationService;
use websockets::WebSocketService;
use websockets::handlers::video_call::VideoCallMessageHandler;

use crate::auth_service::AuthService;
use crate::bulk_storage_service::BulkStorageService;
use crate::categorization_service::CategorizationService;
use crate::middleware::rate_limit::auth_rate_limiter_if_enabled;
use crate::redis_connection::RedisConnection;
use crate::routes::auth::extract::{ComhairleExtAttrs, KcAccessTokenCookieExtractor};
use crate::routes::workflows::WorkflowRouterContext;
#[cfg(test)]
use crate::test_helpers::test_auth_layer;
use crate::transcription_service::Transcriber;
use crate::wiki_poll_service::WikiPollService;
use crate::worker_service::WorkerService;

pub use crate::routes::auth::hash_pw;

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
    pub auth_service: Arc<dyn AuthService>,
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

/// Applies Keycloak authentication to a route without requiring a logged-in user.
///
/// Wraps `method_router` with [`PassthroughMode::Pass`]: requests without a valid access token
/// cookie still reach the handler, but with no `KeycloakAuthStatus`/`KeycloakToken`
/// extension inserted (or a `KeycloakAuthStatus::Failure` if using [`KeycloakAuthStatus`]).
/// Pair this with the [`OptionalUser`] extractor, not [`RequiredUser`], in the handler.
#[cfg_attr(test, allow(unused_variables))]
fn optional_auth(
    method_router: ApiMethodRouter<Arc<ComhairleState>>,
    instance: Arc<KeycloakAuthInstance>,
) -> ApiMethodRouter<Arc<ComhairleState>> {
    #[cfg(not(test))]
    {
        method_router.layer(keycloak_layer(
            instance.clone(),
            PassthroughMode::Pass,
            None,
        ))
    }
    #[cfg(test)]
    {
        test_auth_layer(method_router)
    }
}

/// Applies Keycloak authentication to a route, rejecting unauthenticated requests.
///
/// Wraps `method_router` with [`PassthroughMode::Block`]: requests without a valid access token
/// cookie are rejected before reaching the handler. On success, a `KeycloakToken`
/// extension is guaranteed to be present, so pair this with the [`RequiredUser`]
/// extractor in the handler.
///
/// `audiences` are appended to the default `"account"` audience; pass `None` if the
/// route has no additional audience requirements beyond the default.
#[cfg_attr(test, allow(unused_variables))]
pub fn required_auth(
    method_router: ApiMethodRouter<Arc<ComhairleState>>,
    audiences: Option<Vec<String>>,
    instance: Arc<KeycloakAuthInstance>,
) -> ApiMethodRouter<Arc<ComhairleState>> {
    #[cfg(not(test))]
    {
        method_router.layer(keycloak_layer(
            instance.clone(),
            PassthroughMode::Block,
            audiences,
        ))
    }
    #[cfg(test)]
    {
        test_auth_layer(method_router)
    }
}

/// Builds a [`KeycloakAuthLayer`] configured for this application: access tokens are
/// read from the Keycloak access-token cookie (see [`KcAccessTokenCookieExtractor`]),
/// decoded into `KeycloakToken<String, ComhairleExtAttrs>`.
fn keycloak_layer(
    instance: Arc<KeycloakAuthInstance>,
    mode: PassthroughMode,
    audiences: Option<Vec<String>>,
) -> KeycloakAuthLayer<String, ComhairleExtAttrs> {
    let mut expected_audiences = vec!["account".to_string()];
    if let Some(audiences) = audiences {
        for audience in audiences {
            expected_audiences.push(audience);
        }
    }

    KeycloakAuthLayer::<String, ComhairleExtAttrs>::builder()
        .instance(instance)
        .passthrough_mode(mode)
        .persist_raw_claims(false)
        .token_extractors(NonEmpty::<Arc<dyn TokenExtractor>> {
            head: Arc::new(KcAccessTokenCookieExtractor::default()),
            tail: vec![],
        })
        .expected_audiences(expected_audiences)
        .build()
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
    keycloak_auth_instance: Arc<KeycloakAuthInstance>,
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
        .nest(
            "/auth",
            routes::auth::router(keycloak_auth_instance.clone(), credential_limit_layer).await,
        )
        .nest(
            "/user",
            routes::user::router(keycloak_auth_instance.clone())
                .nest(
                    "/preferences",
                    routes::user_conversation_preferences::router(keycloak_auth_instance.clone()),
                )
                .nest(
                    "/profile",
                    routes::user_profile::router(keycloak_auth_instance.clone()),
                ),
        )
        .nest(
            "/notifications",
            routes::notifications::router(keycloak_auth_instance.clone()),
        )
        .nest(
            "/translations",
            routes::translations::router(keycloak_auth_instance.clone()),
        )
        .nest("/tools", tools::router(keycloak_auth_instance.clone()))
        .nest(
            "/conversation",
            routes::conversations::router(keycloak_auth_instance.clone())
                .nest(
                    "/{conversation_id}/workflow",
                    routes::workflows::router(
                        keycloak_auth_instance.clone(),
                        WorkflowRouterContext::Conversation,
                    )
                    .nest(
                        "/{workflow_id}/workflow_step",
                        routes::workflow_steps::router(
                            keycloak_auth_instance.clone(),
                            WorkflowRouterContext::Conversation,
                        ),
                    )
                    .nest(
                        "/{workflow_id}/progress",
                        routes::user_progress::router(keycloak_auth_instance.clone()),
                    )
                    .nest(
                        "/{workflow_id}/recruitment_targets",
                        routes::recruitment_targets::router(keycloak_auth_instance.clone()),
                    ),
                )
                .nest(
                    "/{conversation_id}/invite",
                    routes::invites::router(keycloak_auth_instance.clone()),
                )
                .nest(
                    "/{conversation_id}/report",
                    routes::reports::router(keycloak_auth_instance.clone()).nest(
                        "/{report_id}/impacts",
                        routes::report_impacts::router(keycloak_auth_instance.clone()),
                    ),
                )
                .nest(
                    "/{conversation_id}/feedback",
                    routes::feedback::router(keycloak_auth_instance.clone()),
                )
                .nest(
                    "/{conversation_id}/chats",
                    routes::chats::router(keycloak_auth_instance.clone()),
                )
                .nest(
                    "/{conversation_id}/chat_instructions",
                    routes::chat_instructions::router(keycloak_auth_instance.clone()),
                )
                .nest(
                    "/{conversation_id}/moderation_policies",
                    routes::moderation_policies::router(keycloak_auth_instance.clone()),
                )
                .nest(
                    "/{conversation_id}/chat_sessions",
                    routes::chat_sessions::router(keycloak_auth_instance.clone()),
                )
                .nest(
                    "/{conversation_id}/documents",
                    routes::documents::router(keycloak_auth_instance.clone()),
                )
                .nest(
                    "/{conversation_id}/events",
                    routes::events::router(keycloak_auth_instance.clone())
                        .nest(
                            "/{event_id}/attendances",
                            routes::event_attendances::router(keycloak_auth_instance.clone()),
                        )
                        .nest(
                            "/{event_id}/workflows",
                            routes::workflows::router(
                                keycloak_auth_instance.clone(),
                                WorkflowRouterContext::Event,
                            )
                            .nest(
                                "/{workflow_id}/workflow_steps",
                                routes::workflow_steps::router(
                                    keycloak_auth_instance.clone(),
                                    WorkflowRouterContext::Event,
                                ),
                            ),
                        )
                        .nest(
                            "/{event_id}/audio_recordings",
                            routes::audio_recordings::router(keycloak_auth_instance.clone()),
                        ),
                ),
        )
        .nest(
            "/ws",
            websockets::routes::websocket_routes(keycloak_auth_instance.clone()),
        )
        .nest(
            "/organizations",
            routes::organizations::router(keycloak_auth_instance.clone()),
        )
        .nest(
            "/regions",
            routes::regions::router(keycloak_auth_instance.clone()),
        )
        .nest(
            "/region_areas",
            routes::region_areas::router(keycloak_auth_instance.clone()),
        )
        .nest(
            "/media",
            routes::media::router(keycloak_auth_instance.clone()),
        )
        .nest(
            "/jobs",
            routes::jobs::router(keycloak_auth_instance.clone()),
        )
        .nest(
            "/services",
            routes::services::router(keycloak_auth_instance.clone()),
        )
        .nest(
            "/api_keys",
            routes::api_keys::router(keycloak_auth_instance.clone()),
        )
        .nest(
            "/email_template_configs",
            routes::email_template_configs::router(keycloak_auth_instance.clone()),
        )
        .nest(
            "/permissions",
            routes::permissions::router(keycloak_auth_instance.clone()),
        )
        .nest("/docs", docs_routes())
        .nest(
            "/demographics",
            routes::demographics::router(keycloak_auth_instance.clone()),
        )
        .finish_api_with(&mut api, api_docs);

    (router, api)
}

/// Constructs the ApiRouter and extracts the OpenAPI spec.
/// Note that sub-routers like `routes::auth::router` are async and must be `.await`ed.
pub async fn build_app(
    state: Arc<ComhairleState>,
    export_spec: bool,
) -> Result<App, ComhairleError> {
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

    let keycloak_auth_instance = Arc::new(KeycloakAuthInstance::new(
        KeycloakConfig::builder()
            .server(Url::parse(&state.config.auth_service.clone().url).unwrap())
            .realm(state.config.auth_service.clone().realm)
            .build(),
    ));

    let (router, api) =
        build_router_and_spec(keycloak_auth_instance, state.config.enable_rate_limiting).await;

    if export_spec {
        let json = serde_json::to_string_pretty(&api).unwrap();
        tokio::fs::write("open-api-spec.json", json.as_bytes()).await?;
    }

    Ok(NormalizePathLayer::trim_trailing_slash().layer(
        router
            .with_state(state.clone())
            .layer(axum::middleware::from_fn_with_state(
                state,
                middleware::request_logging::log_requests,
            ))
            .layer(Extension(Arc::new(api.clone()))) // Arc is very important here or you will face massive memory and performance issues
            .layer(DefaultBodyLimit::max(500 * 1024 * 1024))
            .layer(cors),
    ))
}

pub async fn setup_server(state: Arc<ComhairleState>) -> Result<App, ComhairleError> {
    let args = Args::try_parse().unwrap_or_default();

    tracing::info!("Running with config {:#?}", state.config);

    run_migrations(&state.db).await?;

    build_app(state, args.export_api_spec).await
}
