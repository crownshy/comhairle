use std::sync::Arc;

use comhairle::error::ComhairleError;
use aide::openapi::OpenApi;

mod dummy;

/// Generates the OpenAPI spec without connecting to a live database or instantiating real services.
pub async fn generate_api_spec() -> Result<OpenApi, ComhairleError> {
    // Construct a state populated with dummy zeroed values for schema inspection
    // FIXME: GH 2026-08-19 - The API spec should not depend on runtime state.
    let dummy_state = Arc::new(unsafe { dummy::create_dummy_state() });

    let (_, api_spec) = comhairle::build_app_and_spec(dummy_state.clone()).await;

    let json = serde_json::to_string_pretty(&api_spec).unwrap();
    tokio::fs::write("open-api-spec.json", json.as_bytes()).await?;

    // Prevent dropping dangling pointers from memory
    std::mem::forget(dummy_state);

    Ok(api_spec)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate_api_spec().await?;
    Ok(())
}
