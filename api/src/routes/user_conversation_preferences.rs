use std::sync::Arc;

use aide::axum::{
    routing::{get_with, put_with},
    ApiRouter,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use schemars::JsonSchema;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    error::ComhairleError, models::user_conversation_preferences::UserConversationPreferences,
    ComhairleState,
};

use super::auth::RequiredUser;

#[derive(Deserialize, JsonSchema)]
pub struct UpdateUserConversationPreferences {
    pub receive_updates_by_notification: Option<bool>,
    pub receive_updates_by_email: Option<bool>,
    pub receive_similar_conversation_updates_by_email: Option<bool>,
    pub receive_similar_conversation_updates_by_notification: Option<bool>,
}

pub async fn get_user_conversation_preferences(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(conversation_id): Path<Uuid>,
) -> Result<(StatusCode, Json<UserConversationPreferences>), ComhairleError> {
    let preferences = crate::models::user_conversation_preferences::get_by_user_and_conversation(
        &state.db,
        &user.id,
        &conversation_id,
    )
    .await?;

    Ok((StatusCode::OK, Json(preferences)))
}

pub async fn get_all_user_conversation_preferences(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
) -> Result<(StatusCode, Json<Vec<UserConversationPreferences>>), ComhairleError> {
    let preferences =
        crate::models::user_conversation_preferences::get_by_user(&state.db, &user.id).await?;

    Ok((StatusCode::OK, Json(preferences)))
}

pub async fn update_user_conversation_preferences(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<UpdateUserConversationPreferences>,
) -> Result<(StatusCode, Json<UserConversationPreferences>), ComhairleError> {
    let preferences = crate::models::user_conversation_preferences::update(
        &state.db,
        &user.id,
        &conversation_id,
        payload.receive_updates_by_notification,
        payload.receive_updates_by_email,
        payload.receive_similar_conversation_updates_by_email,
        payload.receive_similar_conversation_updates_by_notification,
    )
    .await?;

    Ok((StatusCode::OK, Json(preferences)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(get_all_user_conversation_preferences, |op| {
                op.id("GetAllUserConversationPreferences")
                    .summary("Get all user conversation preferences")
                    .description("Returns all conversation notification preferences for the authenticated user")
                    .tag("User Preferences")
                    .response::<200, Json<Vec<UserConversationPreferences>>>()
            }),
        )
        .api_route(
            "/conversation/{conversation_id}",
            get_with(get_user_conversation_preferences, |op| {
                op.id("GetUserPreferenceForConversation")
                    .summary("Get user preferences for a conversation")
                    .description("Returns the notification preferences for a specific conversation")
                    .tag("User Preferences")
                    .response::<200, Json<UserConversationPreferences>>()
            }),
        )
        .api_route(
            "/conversation/{conversation_id}",
            put_with(update_user_conversation_preferences, |op| {
                op.id("UpdateUserPreferenceForConversation")
                    .summary("Update user preferences for a conversation")
                    .description("Updates notification preferences for a specific conversation")
                    .tag("User Preferences")
                    .response::<200, Json<UserConversationPreferences>>()
            }),
        )
        .with_state(state)
}
