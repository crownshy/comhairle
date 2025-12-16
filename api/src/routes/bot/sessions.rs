use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with, put_with},
    ApiRouter,
};
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json,
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use crate::{
    bot_service::ComhairleChatSession, error::ComhairleError, routes::bot::GetQueryParams,
    ComhairleState,
};

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    Path(chat_id): Path<String>,
    Query(params): Query<GetQueryParams>,
) -> Result<(StatusCode, Json<Vec<ComhairleChatSession>>), ComhairleError> {
    let (_, sessions) = state
        .bot_service
        .list_chat_sessions(&chat_id, Some(params))
        .await?;

    Ok((StatusCode::OK, Json(sessions)))
}

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path((chat_id, session_id)): Path<(String, String)>,
) -> Result<(StatusCode, Json<ComhairleChatSession>), ComhairleError> {
    let (_, session) = state
        .bot_service
        .get_chat_session(&session_id, &chat_id)
        .await?;

    Ok((StatusCode::OK, Json(session)))
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct CreateChatSessionRequest {
    pub name: String,
    pub user_id: Option<String>,
}

async fn create(
    State(state): State<Arc<ComhairleState>>,
    Path(chat_id): Path<String>,
    Json(payload): Json<CreateChatSessionRequest>,
) -> Result<(StatusCode, Json<ComhairleChatSession>), ComhairleError> {
    let (_, session) = state
        .bot_service
        .create_chat_session(&chat_id, payload)
        .await?;

    Ok((StatusCode::CREATED, Json(session)))
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct UpdateChatSessionRequest {
    pub name: Option<String>,
    pub user_id: Option<String>,
}

async fn update(
    State(state): State<Arc<ComhairleState>>,
    Path((chat_id, session_id)): Path<(String, String)>,
    Json(payload): Json<UpdateChatSessionRequest>,
) -> Result<(StatusCode, Json<ComhairleChatSession>), ComhairleError> {
    let (_, session) = state
        .bot_service
        .update_chat_session(&session_id, &chat_id, payload)
        .await?;

    Ok((StatusCode::OK, Json(session)))
}

async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path((chat_id, session_id)): Path<(String, String)>,
) -> Result<StatusCode, ComhairleError> {
    let _ = state
        .bot_service
        .delete_chat_sessions(&session_id, &chat_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct ChatConversationRequest {
    pub question: String,
    pub user_id: Option<String>,
}

#[instrument(err(Debug), skip(state))]
async fn converse_with_chat(
    State(state): State<Arc<ComhairleState>>,
    Path((chat_id, session_id)): Path<(String, String)>,
    Json(payload): Json<ChatConversationRequest>,
) -> Result<impl IntoResponse, ComhairleError> {
    let stream = state
        .bot_service
        .converse_with_chat(&session_id, &chat_id, payload)
        .await?;

    Ok(Body::from_stream(stream))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("GetChatSessions")
                    .summary("Get a list of chat sessions")
                    .response::<200, Json<Vec<ComhairleChatSession>>>()
            }),
        )
        .api_route(
            "/{session_id}",
            get_with(get, |op| {
                op.id("GetChatSession")
                    .summary("Get a chat session by id")
                    .response::<200, Json<ComhairleChatSession>>()
            }),
        )
        .api_route(
            "/",
            post_with(create, |op| {
                op.id("CreateChatSession")
                    .summary("Create a new session for a chat bot")
                    .response::<201, Json<ComhairleChatSession>>()
            }),
        )
        .api_route(
            "/{session_id}",
            put_with(update, |op| {
                op.id("UpdateChatSession")
                    .summary("Update a chat bot session")
                    .response::<200, Json<ComhairleChatSession>>()
            }),
        )
        .api_route(
            "/{session_id}",
            delete_with(delete, |op| {
                op.id("DeleteChatSession")
                    .summary("Delete a session from a chat bot")
                    .response::<204, ()>()
            }),
        )
        .route("/{session_id}", post(converse_with_chat))
        .with_state(state)
}
