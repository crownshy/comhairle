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
async fn create_knowledge_base(
    State(state): State<Arc<ComhairleState>>,
    jar: CookieJar,
    Json(payload): Json<CreateKnowledgeBaseRequest>,
) -> Result<(CookieJar, StatusCode), ComhairleError> {
    let _result = state
        .bot_service
        .create_knowledge_base(payload.name, payload.description)
        .await?;

    Ok((jar, StatusCode::CREATED))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/create_knowledge_base",
            post_with(create_knowledge_base, |op| {
                op.id("CreateKnowledgeBase")
                    .summary("Create a knowledge base in RAG system")
                    .response::<204, ()>()
            }),
        )
        .with_state(state)
}
