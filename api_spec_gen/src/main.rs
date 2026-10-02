use std::sync::Arc;

use aide::openapi::OpenApi;
use axum_keycloak_auth::Url;
use axum_keycloak_auth::instance::{KeycloakAuthInstance, KeycloakConfig};
use comhairle::error::ComhairleError;

/// Generates the OpenAPI spec without connecting to a live database or instantiating real services.
pub async fn generate_api_spec() -> Result<OpenApi, ComhairleError> {
    // Fake KeycloakAuthInstance
    let dummy_keycloak_auth_instance = Arc::new(
        KeycloakAuthInstance::new(
            KeycloakConfig::builder()
                .server(Url::parse("http://fake.localhost:65535").unwrap())
                .realm("dummy".to_string())
                .build()
        )
    );

    let (_, api_spec) = comhairle::build_router_and_spec(dummy_keycloak_auth_instance.clone(), false).await;

    let json = serde_json::to_string_pretty(&api_spec).unwrap();
    tokio::fs::write("open-api-spec.json", json.as_bytes()).await?;

    Ok(api_spec)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate_api_spec().await?;
    Ok(())
}
