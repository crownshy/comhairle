use aide::openapi::OpenApi;
use comhairle::error::ComhairleError;

/// Generates the OpenAPI spec without connecting to a live database or instantiating real services.
pub async fn generate_api_spec() -> Result<OpenApi, ComhairleError> {
    let (_, api_spec) = comhairle::build_router_and_spec(false).await;

    let json = serde_json::to_string_pretty(&api_spec).unwrap();
    tokio::fs::write("open-api-spec.json", json.as_bytes()).await?;

    Ok(api_spec)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate_api_spec().await?;
    Ok(())
}
