use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with, put_with},
    ApiRouter,
};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use crate::{
    bot_service::ComhairleKnowledgeBase, error::ComhairleError, routes::bot::GetQueryParams,
    ComhairleState,
};

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    Query(params): Query<GetQueryParams>,
) -> Result<(StatusCode, Json<Vec<ComhairleKnowledgeBase>>), ComhairleError> {
    let (_, knowledge_bases) = state.bot_service.list_knowledge_bases(Some(params)).await?;

    Ok((StatusCode::OK, Json(knowledge_bases)))
}

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path(knowledge_base_id): Path<String>,
) -> Result<(StatusCode, Json<ComhairleKnowledgeBase>), ComhairleError> {
    let (_, knowledge_base) = state
        .bot_service
        .get_knowledge_base(&knowledge_base_id)
        .await?;

    Ok((StatusCode::OK, Json(knowledge_base)))
}

#[derive(Deserialize, Debug, JsonSchema)]
struct CreateKnowledgeBaseRequest {
    name: String,
}

#[instrument(err(Debug), skip(state))]
async fn create(
    State(state): State<Arc<ComhairleState>>,
    Json(payload): Json<CreateKnowledgeBaseRequest>,
) -> Result<(StatusCode, Json<ComhairleKnowledgeBase>), ComhairleError> {
    let (_, knowledge_base) = state
        .bot_service
        .create_knowledge_base(payload.name, None)
        .await?;

    Ok((StatusCode::CREATED, Json(knowledge_base)))
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct UpdateKnowledgeBaseRequest {
    pub name: Option<String>,
}

#[instrument(err(Debug), skip(state))]
async fn update(
    State(state): State<Arc<ComhairleState>>,
    Path(knowledge_base_id): Path<String>,
    Json(payload): Json<UpdateKnowledgeBaseRequest>,
) -> Result<(StatusCode, Json<ComhairleKnowledgeBase>), ComhairleError> {
    let (_, knowledge_base) = state
        .bot_service
        .update_knowledge_base(&knowledge_base_id, payload)
        .await?;

    Ok((StatusCode::OK, Json(knowledge_base)))
}

#[instrument(err(Debug), skip(state))]
async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path(knowledge_base_id): Path<String>,
) -> Result<StatusCode, ComhairleError> {
    let _ = state
        .bot_service
        .delete_knowledge_base(knowledge_base_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListKnowledgeBases")
                    .summary("Get a list of knowledge bases from RAG system")
                    .response::<200, Json<Vec<ComhairleKnowledgeBase>>>()
            }),
        )
        .api_route(
            "/{knowledge_base_id}",
            get_with(get, |op| {
                op.id("GetKnowledgeBase")
                    .summary("Get a knowledge base from RAG system")
                    .response::<200, Json<ComhairleKnowledgeBase>>()
            }),
        )
        .api_route(
            "/",
            post_with(create, |op| {
                op.id("CreateKnowledgeBase")
                    .summary("Create a knowledge base in RAG system")
                    .response::<201, Json<ComhairleKnowledgeBase>>()
            }),
        )
        .api_route(
            "/{knowledge_base_id}",
            put_with(update, |op| {
                op.id("UpdateKnowledgeBase")
                    .summary("Update a knowledge base")
                    .response::<200, Json<ComhairleKnowledgeBase>>()
            }),
        )
        .api_route(
            "/{knowledge_base_id}",
            delete_with(delete, |op| {
                op.id("DeleteKnowledgeBase")
                    .summary("Delete a knowledge from from RAG system")
                    .response::<204, ()>()
            }),
        )
        .with_state(state)
}
