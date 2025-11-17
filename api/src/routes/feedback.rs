use std::sync::Arc;

use aide::axum::{
    routing::{get_with, post_with, put_with},
    ApiRouter,
};
use axum::{
    extract::{Path, State},
    Json,
};
use hyper::StatusCode;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        self,
        feedback::{CreateFeedbackDTO, Feedback, PartialFeedback},
    },
    ComhairleState,
};

use super::auth::{RequiredAdminUser, RequiredUser};

async fn create_feedback(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredUser(user): RequiredUser,
    Json(create_request): Json<CreateFeedbackDTO>,
) -> Result<(StatusCode, Json<Feedback>), ComhairleError> {
    let feedback =
        models::feedback::create(&state.db, create_request, &conversation_id, &user.id).await?;

    Ok((StatusCode::CREATED, Json(feedback)))
}

async fn update_feedback(
    State(state): State<Arc<ComhairleState>>,
    Path((_, feedback_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(user): RequiredAdminUser,
    Json(update_request): Json<PartialFeedback>,
) -> Result<(StatusCode, Json<Feedback>), ComhairleError> {
    let feedback =
        models::feedback::update(&state.db, update_request, &feedback_id, &user.id).await?;
    Ok((StatusCode::OK, Json(feedback)))
}

async fn list_feedback_for_conversation(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<Feedback>>), ComhairleError> {
    let conversation = models::conversation::get_by_id(&state.db, &conversation_id).await?;
    if conversation.owner_id != user.id {
        return Err(ComhairleError::UserIsNotConversationOwner);
    }

    let feedback = models::feedback::list_for_conversation(&state.db, &conversation_id).await?;
    Ok((StatusCode::OK, Json(feedback)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_feedback, |op| {
                op.id("CreateFeedback")
                    .summary("Create a feedback statement on the conversation")
                    .response::<201, Json<Feedback>>()
            }),
        )
        .api_route(
            "/{feedback_id}",
            put_with(update_feedback, |op| {
                op.id("UpdateFeedback")
                    .summary("Update an ")
                    .response::<201, Json<Feedback>>()
            }),
        )
        .api_route(
            "/",
            get_with(list_feedback_for_conversation, |op| {
                op.id("ListFeedbackForConversation".into())
                    .summary("Return a list of feedback statements for a conversation")
                    .response::<200, Json<Feedback>>()
            }),
        )
        .with_state(state)
}
