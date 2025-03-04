mod auth;
pub mod config;
pub mod db;
mod error;
mod users;

#[cfg(test)]
mod test_helpers;

use std::sync::Arc;

use auth::setup_auth;
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
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    // Run migrations
    run_migrations(&db).await?;

    // Construct shared state
    let state = ComhairleState {
        db,
        config: config.clone(),
    };
    let auth_router = setup_auth(&config).await;

    // build our application with a route
    let app = Router::new()
        .nest("/auth", auth_router)
        .with_state(Arc::new(state))
        .layer(CookieManagerLayer::new())
        .layer(cors);

    Ok(app)
}
