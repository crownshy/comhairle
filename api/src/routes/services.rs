use std::sync::Arc;

use aide::axum::{routing::get_with, ApiRouter};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use tracing::instrument;

use crate::{error::ComhairleError, routes::auth::RequiredUser, ComhairleState};

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(_user): RequiredUser,
) -> Result<(StatusCode, Json<Vec<String>>), ComhairleError> {
    let mut services = vec![];

    if state.bot_service.is_some() {
        services.push("bot_service".to_string());
    }

    if state.translation_service.is_some() {
        services.push("translation_service".to_string());
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
                    .response::<200, Json<Vec<String>>>()
            }),
        )
        .with_state(state)
}
