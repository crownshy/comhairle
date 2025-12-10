pub mod config;
pub mod db;
mod docs;
pub mod error;
pub mod mailer;
pub mod models;
mod routes;
mod tools;
pub mod translation_service;
pub mod websockets;

use docs::docs_routes;
use mailer::ComhairleMailer;
pub use routes::auth::hash_pw;
use routes::auth::AUTH_KEY;
use translation_service::TranslationService;
use websockets::WebSocketService;

#[cfg(test)]
mod test_helpers;

use std::sync::Arc;

use axum::{http::Method, Extension, Router};

use aide::{axum::ApiRouter, openapi::OpenApi, transform::TransformOpenApi};

use config::ComhairleConfig;
use db::run_migrations;
use error::ComhairleError;
use sqlx_postgres::PgPool;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct ComhairleState {
    pub db: PgPool,
    pub config: ComhairleConfig,
    pub mailer: Arc<dyn ComhairleMailer>,
    pub websockets: Arc<dyn WebSocketService>,
    pub translation_service: Arc<dyn TranslationService>,
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

pub async fn setup_server(state: Arc<ComhairleState>) -> Result<Router<()>, ComhairleError> {
    tracing::info!("Running with config {:#?}", state.config);

    aide::generate::on_error(|error| {
        println!("{error}");
    });

    aide::generate::extract_schemas(true);
    let mut api = OpenApi::default();

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_origin([
            "http://localhost".parse().unwrap(),
            "http://localhost:3000".parse().unwrap(),
            "http://localhost:5173".parse().unwrap(),
            "https://stage.comhairle.scot".parse().unwrap(),
        ]);

    // Run migrations
    run_migrations(&state.db).await?;

    let auth_router = routes::auth::router(state.clone()).await;

    // build our application with a route
    let app = ApiRouter::new()
        .nest_api_service("/auth", auth_router)
        .nest_api_service(
            "/user",
            routes::user::router(state.clone()).nest_api_service(
                "/preferences",
                routes::user_conversation_preferences::router(state.clone()),
            ),
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
                    routes::workflows::router(state.clone())
                        .nest_api_service(
                            "/{workflow_id}/workflow_step",
                            routes::workflow_steps::router(state.clone()),
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
                ),
        )
        .nest_api_service(
            "/ws",
            websockets::routes::websocket_routes().with_state(state.clone()),
        )
        .nest_api_service("/docs", docs_routes(state.clone()))
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api.clone()))) // Arc is very important here or you will face massive memory and performance issues
        .layer(cors);

    Ok(app)
}
