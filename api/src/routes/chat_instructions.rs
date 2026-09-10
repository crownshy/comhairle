use std::sync::Arc;

use aide::axum::{
    ApiRouter,
    routing::{get_with, post_with},
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use tracing::instrument;
use uuid::Uuid;

use crate::models::chat_instructions::{self, UpsertChatInstructions};
use crate::{ComhairleError, ComhairleState};
use dto::ChatInstructionsDto;

pub mod dto;

#[instrument(err(Debug), skip(state))]
async fn get_by_conversation(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ChatInstructionsDto>), ComhairleError> {
    let instructions =
        chat_instructions::get_by_conversation_id(&state.db, conversation_id).await?;

    Ok((StatusCode::OK, Json(instructions.into())))
}

#[instrument(err(Debug), skip(state))]
async fn upsert_for_conversation(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<UpsertChatInstructions>,
) -> Result<(StatusCode, Json<ChatInstructionsDto>), ComhairleError> {
    let instructions =
        chat_instructions::upsert_for_conversation(&state.db, conversation_id, payload).await?;

    Ok((StatusCode::OK, Json(instructions.into())))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(get_by_conversation, |op| {
                op.id("GetConversationChatInstructions")
                    .tag("ChatInstructions")
                    .summary("Get chat instructions by conversation_id")
                    .description("Get chat instructions by conversation_id")
                    .security_requirement("JWT")
                    .response::<200, Json<ChatInstructionsDto>>()
            }),
        )
        .api_route(
            "/",
            post_with(upsert_for_conversation, |op| {
                op.id("UpsertConversationChatInstructions")
                    .tag("ChatInstructions")
                    .summary("Upsert chat instructions ")
                    .description(
                        "Creates a new chat instructions record for \
                        a conversation or updates and existing record",
                    )
                    .security_requirement("JWT")
                    .response::<200, Json<ChatInstructionsDto>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use sqlx::PgPool;

    use super::*;

    use std::error::Error;

    use crate::models::model_test_helpers::{
        get_random_conversation_id, setup_default_app_and_session,
    };

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_new_instructions_for_conversation(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let (_, res, _) = session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/chat_instructions"),
                json!({
                    "target_reading_age": 10,
                })
                .to_string()
                .into(),
            )
            .await?;

        let instructions: ChatInstructionsDto = serde_json::from_value(res)?;

        assert_eq!(
            instructions.conversation_id, conversation_id,
            "incorrect conversation_id"
        );
        assert_eq!(
            instructions.target_reading_age,
            Some(10),
            "incorrect target_reading_age"
        );
        assert!(instructions.max_length.is_none(), "incorrect max_length");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_existing_instructions_for_conversation(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let (_, res, _) = session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/chat_instructions"),
                json!({
                    "target_reading_age": 10,
                })
                .to_string()
                .into(),
            )
            .await?;
        let new_instructions: ChatInstructionsDto = serde_json::from_value(res)?;

        let (_, res, _) = session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/chat_instructions"),
                json!({
                    "target_reading_age": 15,
                    "max_length": 1000
                })
                .to_string()
                .into(),
            )
            .await?;
        let updated_instructions: ChatInstructionsDto = serde_json::from_value(res)?;

        assert_eq!(
            updated_instructions.conversation_id, conversation_id,
            "incorrect conversation_id"
        );
        assert_eq!(
            updated_instructions.target_reading_age,
            Some(15),
            "incorrect target_reading_age"
        );
        assert_eq!(
            updated_instructions.max_length,
            Some(1000),
            "incorrect max_length"
        );
        assert_eq!(
            new_instructions.id, updated_instructions.id,
            "ids don't match"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_instructions_for_conversation(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let (_, res, _) = session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/chat_instructions"),
                json!({
                    "target_reading_age": 10,
                })
                .to_string()
                .into(),
            )
            .await?;
        let new_instructions: ChatInstructionsDto = serde_json::from_value(res)?;

        let (_, res, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/chat_instructions"),
            )
            .await?;
        let instructions: ChatInstructionsDto = serde_json::from_value(res)?;

        assert_eq!(new_instructions.id, instructions.id, "ids don't match");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_404_if_no_instructions_for_conversation(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let (status, res, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/chat_instructions"),
            )
            .await?;

        assert_eq!(status, StatusCode::NOT_FOUND, "incorrect status code");
        assert_eq!(
            res.get("err").unwrap(),
            "Chat Instructions not found",
            "incorrect error message"
        );

        Ok(())
    }
}
