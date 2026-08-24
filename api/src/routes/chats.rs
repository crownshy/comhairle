use std::sync::Arc;

use aide::axum::{
    ApiRouter,
    routing::{get_with, put_with},
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use tracing::instrument;
use uuid::Uuid;

use crate::bot_service::{ComhairleChat, UpdateChatRequest};
use crate::models::conversation;
use crate::routes::auth::RequiredAdminUser;
use crate::{ComhairleError, ComhairleState};

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(user): RequiredAdminUser,
) -> Result<(StatusCode, Json<ComhairleChat>), ComhairleError> {
    let bot_service = state.required_bot_service()?;

    let conversation = conversation::get_by_id(&state.db, &conversation_id).await?;
    let chat_bot_id = match conversation.chat_bot_id {
        Some(id) => id,
        None => {
            return Err(ComhairleError::CorruptedData(
                "Missing chat_bot_id on conversation: {conversation_id}".to_string(),
            ));
        }
    };

    let (_, chat) = bot_service.get_chat(&chat_bot_id).await?;

    Ok((StatusCode::OK, Json(chat)))
}

#[instrument(err(Debug), skip(state))]
async fn update(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(user): RequiredAdminUser,
    Json(payload): Json<UpdateChatRequest>,
) -> Result<(StatusCode, Json<ComhairleChat>), ComhairleError> {
    let bot_service = state.required_bot_service()?;

    let conversation = conversation::get_by_id(&state.db, &conversation_id).await?;
    let chat_bot_id = match conversation.chat_bot_id {
        Some(id) => id,
        None => {
            return Err(ComhairleError::CorruptedData(
                "Missing chat_bot_id on conversation: {conversation_id}".to_string(),
            ));
        }
    };

    let (_, chat) = bot_service.update_chat(&chat_bot_id, payload).await?;

    Ok((StatusCode::OK, Json(chat)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(get, |op| {
                op.id("GetChat")
                    .tag("Chats")
                    .summary("Get chat bot")
                    .description("Get a conversation's bot service chat")
                    .security_requirement("JWT")
                    .response::<200, Json<ComhairleChat>>()
            }),
        )
        .api_route(
            "/",
            put_with(update, |op| {
                op.id("UpdateChat")
                    .tag("Chats")
                    .summary("Update chat bot")
                    .description("Update a conversation's bot service chat")
                    .security_requirement("JWT")
                    .response::<200, Json<ComhairleChat>>()
            }),
        )
        .with_state(state)
}
