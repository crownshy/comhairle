pub mod bot_service;
pub mod bulk_storage;
pub mod config;
pub mod db;
mod docs;
pub mod error;
pub mod mailer;
pub mod models;
mod routes;
pub mod schema_helpers;
mod tools;
pub mod translation_service;
pub mod websockets;
pub mod workers;

use bot_service::ComhairleBotService;
use clap::Parser;
use docs::docs_routes;
use mailer::ComhairleMailer;
pub use routes::auth::hash_pw;
use routes::auth::AUTH_KEY;
use tokio::fs;
use translation_service::TranslationService;
use websockets::WebSocketService;

#[cfg(test)]
mod test_helpers;

use std::sync::Arc;

use axum::{
    extract::DefaultBodyLimit,
    http::{header, HeaderValue, Method},
    Extension, Router,
};

use aide::{axum::ApiRouter, openapi::OpenApi, transform::TransformOpenApi};

use config::ComhairleConfig;
use db::run_migrations;
use error::ComhairleError;
use sqlx_postgres::PgPool;
use tower_http::cors::CorsLayer;

use crate::{
    bulk_storage::BulkStorageService, routes::workflows::WorkflowRouterContext, workers::JobQueues,
};

#[derive(Clone)]
pub struct ComhairleState {
    pub db: PgPool,
    pub config: ComhairleConfig,
    pub mailer: Arc<dyn ComhairleMailer>,
    pub websockets: Arc<dyn WebSocketService>,
    pub translation_service: Option<Arc<dyn TranslationService>>,
    pub bot_service: Option<Arc<dyn ComhairleBotService>>,
    pub bulk_storage_service: Arc<dyn BulkStorageService>,
    pub jobs: Arc<JobQueues>,
}

impl ComhairleState {
    fn required_bot_service(&self) -> Result<&Arc<dyn ComhairleBotService>, ComhairleError> {
        self.bot_service
            .as_ref()
            .ok_or(ComhairleError::NoBotServiceConfigured)
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

pub async fn setup_server(state: Arc<ComhairleState>) -> Result<Router<()>, ComhairleError> {
    let args = Args::try_parse().unwrap_or_default();

    tracing::info!("Running with config {:#?}", state.config);

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

    // Add whitelisted domains from config
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

    // Run migrations
    run_migrations(&state.db).await?;

    let auth_router = routes::auth::router(state.clone()).await;

    // build our application with a route
    let app = ApiRouter::new()
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
        .nest_api_service("/jobs", routes::jobs::router(state.clone()))
        .nest_api_service("/services", routes::services::router(state.clone()))
        .nest_api_service("/docs", docs_routes(state.clone()))
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api.clone()))) // Arc is very important here or you will face massive memory and performance issues
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
        .layer(cors);

    if args.export_api_spec {
        let json = serde_json::to_string_pretty(&api).unwrap();
        fs::write("open-api-spec.json", json.as_bytes()).await?;
    }

    println!("Config ${:#?}", state.config);

    Ok(app)
}
