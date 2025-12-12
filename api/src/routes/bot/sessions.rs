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
use axum_extra::extract::CookieJar;
use ragflow::chat::session::ConvoQuestion;
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use crate::{error::ComhairleError, routes::bot::GetQueryParams, ComhairleState};

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    jar: CookieJar,
    params: Query<GetQueryParams>,
) -> Result<(CookieJar, StatusCode), ComhairleError> {
    todo!();
}

#[derive(Deserialize, Debug, JsonSchema)]
struct CreateChatSessionRequest {
    name: Option<String>,
    user_id: Option<String>,
}

async fn create(
    State(state): State<Arc<ComhairleState>>,
    jar: CookieJar,
    Json(payload): Json<CreateChatSessionRequest>,
) -> Result<(CookieJar, StatusCode), ComhairleError> {
    todo!();
}

#[derive(Deserialize, Debug, JsonSchema)]
struct UpdateChatSessionRequest {
    name: Option<String>,
    user_id: Option<String>,
}

async fn update(
    State(state): State<Arc<ComhairleState>>,
    jar: CookieJar,
    Path(session_id): Path<String>,
    Json(payload): Json<UpdateChatSessionRequest>,
) -> Result<(CookieJar, StatusCode), ComhairleError> {
    todo!();
}

async fn delete(
    State(state): State<Arc<ComhairleState>>,
    jar: CookieJar,
    Path(session_id): Path<String>,
) -> Result<(CookieJar, StatusCode), ComhairleError> {
    todo!();
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
