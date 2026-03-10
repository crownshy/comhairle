use std::sync::Arc;

use aide::axum::{routing::get_with, ApiRouter};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use schemars::JsonSchema;
use serde::Serialize;
use tracing::instrument;

use crate::{error::ComhairleError, routes::auth::RequiredUser, ComhairleState};

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

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListSupportedServices")
                    .summary("List of supported services")
                    .description(
                        "List of services supported (configured) by current Comhairle server",
                    )
                    .security_requirement("JWT")
                    .response::<200, Json<ComhairleServices>>()
            }),
        )
        .with_state(state)
}
