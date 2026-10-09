mod error;
mod routes;

use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use rig::{client::CompletionClient, providers::ollama};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let base_url = std::env::var("SENSEMAKAR_OLLAMA_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:11434".to_string());
    let model_name = std::env::var("SENSEMAKAR_OLLAMA_MODEL").unwrap_or_else(|_| "qwen3-long".to_string());
    let api_key = std::env::var("SENSEMAKAR_OLLAMA_API_KEY").unwrap_or_default();
    let port: u16 = std::env::var("SENSEMAKAR_API_PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(8089);

    let client = ollama::Client::builder()
        .base_url(&base_url)
        .api_key(api_key.as_str())
        .build()?;
    let model = Arc::new(client.completion_model(&model_name));

    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/polis-report", post(routes::polis_report))
        .route("/generate-themes", post(routes::generate_themes))
        .route("/assign-themes", post(routes::assign_themes))
        .layer(TraceLayer::new_for_http())
        .with_state(model);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await?;
    tracing::info!("sensemakar_api listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
