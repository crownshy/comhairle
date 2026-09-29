use std::sync::Arc;

use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use axum_keycloak_auth::instance::KeycloakAuthInstance;
use schemars::JsonSchema;
use serde::Serialize;
use tracing::instrument;

use crate::{
    ComhairleState, error::ComhairleError, required_auth, routes::auth::extract::RequiredUser,
};

#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
struct ComhairleServices {
    bot_service: bool,
    translation_service: bool,
}

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(_user): RequiredUser,
) -> Result<(StatusCode, Json<ComhairleServices>), ComhairleError> {
    let mut services = ComhairleServices {
        bot_service: false,
        translation_service: false,
    };

    if state.bot_service.is_some() {
        services.bot_service = true;
    }

    if state.translation_service.is_some() {
        services.translation_service = true;
    }

    Ok((StatusCode::OK, Json(services)))
}

pub fn router(keycloak_auth_instance: Arc<KeycloakAuthInstance>) -> ApiRouter<Arc<ComhairleState>> {
    ApiRouter::new().api_route(
        "/",
        required_auth(
            get_with(list, |op| {
                op.id("ListSupportedServices")
                    .summary("List of supported services")
                    .description(
                        "List of services supported (configured) by current Comhairle server",
                    )
                    .security_requirement("JWT")
                    .response::<200, Json<ComhairleServices>>()
            }),
            None,
            keycloak_auth_instance.clone(),
        ),
    )
}
