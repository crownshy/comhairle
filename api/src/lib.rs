pub mod config;
pub mod db;
mod docs;
pub mod error;
pub mod models;
mod routes;
mod tools;

use docs::docs_routes;
pub use routes::auth::hash_pw;
use routes::auth::AUTH_KEY;

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

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::config::Region;

#[derive(Clone)]
pub struct ComhairleState {
    pub db: PgPool,
    pub config: ComhairleConfig,
    pub s3_client: aws_sdk_s3::Client,
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

pub async fn setup_server(
    config: ComhairleConfig,
    db: PgPool,
) -> Result<Router<()>, ComhairleError> {
    tracing::info!("Running with config {config:#?}");

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
            "https://stage.comhairle.scot".parse().unwrap(),
        ]);

    // Run migrations
    run_migrations(&db).await?;

    let region_provider =
        RegionProviderChain::first_try(Region::new("eu-west-2")).or_default_provider();

    let aws_config = aws_config::from_env().region(region_provider).load().await;

    let s3_client = aws_sdk_s3::Client::new(&aws_config);
    // Construct shared state
    let state = Arc::new(ComhairleState {
        db,
        config: config.clone(),
        s3_client,
    });
    let auth_router = routes::auth::router(&config, state.clone()).await;

    // build our application with a route
    let app = ApiRouter::new()
        .nest_api_service("/auth", auth_router)
        .nest_api_service("/user", routes::user::router(state.clone()))
        .nest_api_service("/tools", tools::router(state.clone()))
        .nest_api_service(
            "/conversation",
            routes::conversations::router(state.clone()).nest_api_service(
                "/{conversation_id}/workflow",
                routes::workflows::router(state.clone())
                    .nest_api_service(
                        "/{workflow_id}/workflow_step",
                        routes::workflow_steps::router(state.clone()),
                    )
                    .nest_api_service(
                        "/{workflow_id}/participation",
                        routes::user_participation::router(state.clone()),
                    )
                    .nest_api_service(
                        "/{workflow_id}/progress",
                        routes::user_progress::router(state.clone()),
                    )
                    .nest_api_service("/resource", routes::resources::router(state.clone())),
            ),
        )
        .nest_api_service("/docs", docs_routes(state.clone()))
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api))) // Arc is very important here or you will face massive memory and performance issues
        .layer(cors);

    Ok(app)
}
