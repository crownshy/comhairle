use std::sync::Arc;

use aide::axum::{routing::post_with, ApiRouter};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use schemars::JsonSchema;
use serde::Serialize;

use crate::{
    error::ComhairleError,
    models::api_key::{self, CreateRequest},
    routes::auth::RequiredAdminUser,
    ComhairleState,
};

#[derive(Serialize, Debug, JsonSchema)]
pub struct CreateResponse {
    key: String,
}

// No tracing to avoid keys being exposed in logs
async fn create(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<CreateRequest>,
) -> Result<(StatusCode, Json<CreateResponse>), ComhairleError> {
    let key = api_key::create(&state.db, payload).await?;

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
