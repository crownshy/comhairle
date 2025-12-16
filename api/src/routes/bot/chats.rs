use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with, put_with},
    ApiRouter,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use crate::{
    bot_service::{ComhairleChat, ComhairleLlm, ComhairlePrompt},
    error::ComhairleError,
    routes::bot::GetQueryParams,
    ComhairleState,
};

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path(chat_id): Path<String>,
) -> Result<(StatusCode, Json<ComhairleChat>), ComhairleError> {
    let (_, chat) = state.bot_service.get_chat(&chat_id).await?;

    Ok((StatusCode::OK, Json(chat)))
}

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    Query(params): Query<GetQueryParams>,
) -> Result<(StatusCode, Json<Vec<ComhairleChat>>), ComhairleError> {
    let (_, chats) = state.bot_service.list_chats(Some(params)).await?;

    Ok((StatusCode::OK, Json(chats)))
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct CreateChatRequest {
    pub name: String,
    pub knowledge_base_ids: Option<Vec<String>>,
    pub llm_model: Option<ComhairleLlm>,
    pub prompt: Option<ComhairlePrompt>,
}

#[instrument(err(Debug), skip(state))]
async fn create(
    State(state): State<Arc<ComhairleState>>,
    Json(payload): Json<CreateChatRequest>,
) -> Result<(StatusCode, Json<ComhairleChat>), ComhairleError> {
    let (_, chat) = state.bot_service.create_chat(payload).await?;

    Ok((StatusCode::CREATED, Json(chat)))
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct UpdateChatRequest {
    pub name: Option<String>,
    pub knowledge_base_ids: Option<Vec<String>>,
    pub llm_model: Option<ComhairleLlm>,
    pub prompt: Option<ComhairlePrompt>,
}

#[instrument(err(Debug), skip(state))]
async fn update(
    State(state): State<Arc<ComhairleState>>,
    Path(chat_id): Path<String>,
    Json(payload): Json<UpdateChatRequest>,
) -> Result<(StatusCode, Json<ComhairleChat>), ComhairleError> {
    let (_, chat) = state.bot_service.update_chat(&chat_id, payload).await?;

    Ok((StatusCode::OK, Json(chat)))
}

#[instrument(err(Debug), skip(state))]
async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path(chat_id): Path<String>,
) -> Result<StatusCode, ComhairleError> {
    let _ = state.bot_service.delete_chat(&chat_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListChats")
                    .summary("Get a list of chat bots")
                    .response::<200, Json<Vec<ComhairleChat>>>()
            }),
        )
        .api_route(
            "/{chat_id}",
            get_with(get, |op| {
                op.id("GetChat")
                    .summary("Get a chat bot by id")
                    .response::<200, Json<ComhairleChat>>()
            }),
        )
        .api_route(
            "/",
            post_with(create, |op| {
                op.id("CreateChat")
                    .summary("Create a new chat bot")
                    .response::<201, Json<ComhairleChat>>()
            }),
        )
        .api_route(
            "/{chat_id}",
            put_with(update, |op| {
                op.id("UpdateChat")
                    .summary("Update a chat bot")
                    .response::<200, Json<ComhairleChat>>()
            }),
        )
        .api_route(
            "/{chat_id}",
            delete_with(delete, |op| {
                op.id("DeleteChat")
                    .summary("Delete a chat bot")
                    .response::<204, ()>()
            }),
        )
        .with_state(state)
}
