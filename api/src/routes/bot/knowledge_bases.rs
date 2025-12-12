use std::sync::Arc;

use aide::axum::{routing::post_with, ApiRouter};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use axum_extra::extract::CookieJar;
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use crate::{error::ComhairleError, ComhairleState};

#[derive(Deserialize, Debug, JsonSchema)]
struct CreateKnowledgeBaseRequest {
    name: String,
    description: Option<String>,
    permission: Option<String>,
}

#[instrument(err(Debug), skip(state))]
async fn create(
    State(state): State<Arc<ComhairleState>>,
    jar: CookieJar,
    Json(payload): Json<CreateKnowledgeBaseRequest>,
) -> Result<(CookieJar, StatusCode), ComhairleError> {
    // todo!();

    Ok((jar, StatusCode::CREATED))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/knowledge_bases",
            post_with(create, |op| {
                op.id("CreateKnowledgeBase")
                    .summary("Create a knowledge base in RAG system")
                    .response::<201, ()>()
            }),
        )
        .with_state(state)
}
