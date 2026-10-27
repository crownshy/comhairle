use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::post_with;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use schemars::JsonSchema;
use serde::Serialize;

use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::models::api_key::{self, CreateApiKeyRequest};
use crate::routes::auth::{RequiredAdminUser, is_user_admin};

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
    if !is_user_admin(&state, &user).await {
        return Err(ComhairleError::UserNotAuthorized);
    }

    let key = api_key::create(&state.db, user.id, payload).await?;

    Ok((StatusCode::CREATED, Json(CreateResponse { key })))
}

pub fn router() -> ApiRouter<Arc<ComhairleState>> {
    ApiRouter::new().api_route(
        "/",
        post_with(create, |op| {
            op.summary("Generate api key")
                .response::<201, Json<CreateResponse>>()
        }),
    )
}
