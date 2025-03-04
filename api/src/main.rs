use comhairle::{db::setup_db, setup_server};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // Load Config
    let config = comhairle::config::load()?;

    // Setup DB
    let db = setup_db(&config.database_url).await?;

    let app = setup_server(config, db).await?;

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
