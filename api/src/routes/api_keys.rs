use std::sync::Arc;

use aide::axum::{ApiRouter, routing::post_with};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use schemars::JsonSchema;
use serde::Serialize;

use crate::{
    ComhairleState,
    error::ComhairleError,
    models::api_key::{self, CreateApiKeyRequest},
    routes::auth::{RequiredAdminUser, is_user_admin},
};

#[derive(Serialize, Debug, JsonSchema)]
pub struct CreateResponse {
    key: String,
}

// No tracing to avoid keys being exposed in logs
async fn create(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Json(payload): Json<CreateApiKeyRequest>,
) -> Result<(StatusCode, Json<CreateResponse>), ComhairleError> {
    if !is_user_admin(&user, &state.config) {
        return Err(ComhairleError::UserNotAuthorized);
    }

    let key = api_key::create(&state.db, user.id, payload).await?;

    Ok((StatusCode::CREATED, Json(CreateResponse { key })))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create, |op| {
                op.summary("Generate api key")
                    .response::<201, Json<CreateResponse>>()
            }),
        )
        .with_state(state)
}
