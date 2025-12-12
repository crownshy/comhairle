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
use ragflow::{
    chat::{Chat, CreateChat as CreateRagflowChat, Llm, Prompt, UpdateChat as UpdateRagflowChat},
    DeleteResources as DeleteRagflowResources,
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use crate::{error::ComhairleError, routes::bot::GetQueryParams, ComhairleState};

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    Query(params): Query<GetQueryParams>,
) -> Result<(StatusCode, Json<Vec<Chat>>), ComhairleError> {
    let (_, chats) = state.bot_service.get_chats(Some(params)).await?;

    Ok((StatusCode::OK, Json(chats)))
}

#[derive(Deserialize, Debug, JsonSchema)]
struct CreateChatRequest {
    name: String,
    knowledge_base_ids: Option<Vec<String>>,
    llm_model: Option<String>,
    prompt: Option<String>, // TODO:
}

#[instrument(err(Debug), skip(state))]
async fn create(
    State(state): State<Arc<ComhairleState>>,
    Json(payload): Json<CreateChatRequest>,
) -> Result<(StatusCode, Json<Chat>), ComhairleError> {
    let body: CreateRagflowChat = payload.into();

    let (_, chat) = state.bot_service.create_chat(body).await?;

    Ok((StatusCode::CREATED, Json(chat)))
}

#[derive(Deserialize, Debug, JsonSchema)]
struct UpdateChatRequest {
    name: Option<String>,
    knowledge_base_ids: Option<Vec<String>>,
    llm_model: Option<String>,
    prompt: Option<String>, // TODO:
}

#[instrument(err(Debug), skip(state))]
async fn update(
    State(state): State<Arc<ComhairleState>>,
    Path(chat_id): Path<String>,
    Json(payload): Json<UpdateChatRequest>,
) -> Result<StatusCode, ComhairleError> {
    let body: UpdateRagflowChat = payload.into();

    let _ = state.bot_service.update_chat(&chat_id, body).await?;

    Ok(StatusCode::OK)
}

#[instrument(err(Debug), skip(state))]
async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path(chat_id): Path<String>,
) -> Result<StatusCode, ComhairleError> {
    let body = DeleteRagflowResources {
        ids: vec![&chat_id],
    };

    let _ = state.bot_service.delete_chats(body).await?;

    Ok(StatusCode::NO_CONTENT)
}

impl From<CreateChatRequest> for CreateRagflowChat {
    fn from(input: CreateChatRequest) -> Self {
        Self {
            name: input.name,
            avatar: None,
            dataset_ids: input.knowledge_base_ids.unwrap_or_default(),
            llm: input.llm_model.map(|model| Llm {
                model_name: Some(model),
            }),
            prompt: input.prompt.map(|prompt| Prompt {
                prompt: Some(prompt),
                ..Default::default()
            }),
        }
    }
}

impl From<UpdateChatRequest> for UpdateRagflowChat {
    fn from(input: UpdateChatRequest) -> Self {
        Self {
            name: input.name,
            dataset_ids: input.knowledge_base_ids,
            llm: input.llm_model.map(|model| Llm {
                model_name: Some(model),
            }),
            prompt: input.prompt.map(|prompt| Prompt {
                prompt: Some(prompt),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(get, |op| {
                op.id("GetChats")
                    .summary("Get a list of chat bots")
                    .response::<200, ()>() // TODO:
            }),
        )
        .api_route(
            "/",
            post_with(create, |op| {
                op.id("CreateChat")
                    .summary("Create a new chat bot")
                    .response::<201, ()>() // TODO:
            }),
        )
        .api_route(
            "/{chat_id}",
            put_with(update, |op| {
                op.id("UpdateChat")
                    .summary("Update a chat bot")
                    .response::<200, ()>() // TODO:
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
