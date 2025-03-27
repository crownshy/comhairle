pub mod config;
pub mod db;
pub mod error;
pub mod models;
mod routes;
mod tools;

pub use routes::auth::hash_pw;

#[cfg(test)]
mod test_helpers;

use std::sync::Arc;

use axum::{http::Method, Router};

use config::ComhairleConfig;
use db::run_migrations;
use error::ComhairleError;
use sqlx_postgres::PgPool;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct ComhairleState {
    pub db: PgPool,
    pub config: ComhairleConfig,
}

pub async fn setup_server(
    config: ComhairleConfig,
    db: PgPool,
) -> Result<Router<()>, ComhairleError> {
    tracing::info!("Running with config {config:#?}");

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_origin([
            "http://localhost".parse().unwrap(),
            "https://stage.comhairle.scot".parse().unwrap(),
        ]);

    // Run migrations
    run_migrations(&db).await?;

    // Construct shared state
    let state = ComhairleState {
        db,
        config: config.clone(),
    };
    let auth_router = routes::auth::router(&config).await;

    // build our application with a route
    let app = Router::new()
        .nest("/auth", auth_router)
        .nest("/conversation", routes::conversations::router())
        .nest(
            "/conversation/{conversation_id}/workflow",
            routes::workflows::router(),
        )
        .nest(
            "/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step",
            routes::workflow_steps::router(),
        )
        .nest(
            "/conversation/{conversation_id}/workflow/{workflow_id}/participation",
            routes::user_participation::router(),
        )
        .nest(
            "/conversation/{conversation_id}/workflow/{workflow_id}",
            routes::user_progress::router(),
        )
        .with_state(Arc::new(state))
        .layer(CookieManagerLayer::new())
        .layer(cors);

    Ok(app)
}
