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
    Json,
};
use ragflow::{
    chat::session::{
        ChatSession, ConvoQuestion, CreateChatSession as CreateRagflowSession,
        UpdateChatSession as UpdateRagflowSession,
    },
    DeleteResources as DeleteRagflowResources, GetQueryParams as RagflowQueryParams,
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use crate::{error::ComhairleError, routes::bot::GetQueryParams, ComhairleState};

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path(chat_id): Path<String>,
    Query(params): Query<GetQueryParams>,
) -> Result<(StatusCode, Json<Vec<ChatSession>>), ComhairleError> {
    let ragflow_params: RagflowQueryParams = params.into();

    let (_, sessions) = state
        .bot_service
        .get_chat_sessions(&chat_id, Some(ragflow_params))
        .await?;

    Ok((StatusCode::OK, Json(sessions)))
}

#[derive(Deserialize, Debug, JsonSchema)]
struct CreateChatSessionRequest {
    name: String,
    user_id: Option<String>,
}

async fn create(
    State(state): State<Arc<ComhairleState>>,
    Path(chat_id): Path<String>,
    Json(payload): Json<CreateChatSessionRequest>,
) -> Result<(StatusCode, Json<ChatSession>), ComhairleError> {
    let body: CreateRagflowSession = payload.into();

    let (_, session) = state
        .bot_service
        .create_chat_session(&chat_id, body)
        .await?;

    Ok((StatusCode::CREATED, Json(session)))
}

#[derive(Deserialize, Debug, JsonSchema)]
struct UpdateChatSessionRequest {
    name: Option<String>,
    user_id: Option<String>,
}

async fn update(
    State(state): State<Arc<ComhairleState>>,
    Path((chat_id, session_id)): Path<(String, String)>,
    Json(payload): Json<UpdateChatSessionRequest>,
) -> Result<StatusCode, ComhairleError> {
    let body: UpdateRagflowSession = payload.into();

    let _ = state
        .bot_service
        .update_chat_session(&session_id, &chat_id, body)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path((chat_id, session_id)): Path<(String, String)>,
) -> Result<StatusCode, ComhairleError> {
    let body = DeleteRagflowResources {
        ids: vec![&session_id],
    };

    let _ = state
        .bot_service
        .delete_chat_sessions(&chat_id, body)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize, Debug, JsonSchema)]
struct ChatConversationRequest {
    question: String,
    session_id: Option<String>,
    user_id: Option<String>,
}

#[instrument(err(Debug), skip(state))]
async fn converse_with_chat(
    State(state): State<Arc<ComhairleState>>,
    Path(chat_id): Path<String>,
    Json(payload): Json<ChatConversationRequest>,
) -> Result<impl IntoResponse, ComhairleError> {
    let body = ConvoQuestion {
        question: payload.question,
        stream: Some(true),
        session_id: payload.session_id,
        user_id: payload.user_id,
    };
    let stream = state.bot_service.converse_with_chat(&chat_id, body).await?;

    Ok(Body::from_stream(stream))
}

impl From<CreateChatSessionRequest> for CreateRagflowSession {
    fn from(input: CreateChatSessionRequest) -> Self {
        Self {
            name: input.name,
            user_id: input.user_id,
        }
    }
}

impl From<UpdateChatSessionRequest> for UpdateRagflowSession {
    fn from(input: UpdateChatSessionRequest) -> Self {
        Self {
            name: input.name,
            user_id: input.user_id,
        }
    }
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(get, |op| {
                op.id("GetSessions")
                    .summary("Get a list of chat sessions")
                    .response::<200, ()>()
            }),
        )
        .api_route(
            "/",
            post_with(create, |op| {
                op.id("CreateChatSession")
                    .summary("Create a new session for a chat bot")
                    .response::<201, ()>()
            }),
        )
        .api_route(
            "/{session_id}",
            put_with(update, |op| {
                op.id("UpdateChatSession")
                    .summary("Update a chat bot session")
                    .response::<200, ()>()
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
        .with_state(state)
}
