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

use crate::{ComhairleError, ComhairleState};
use crate::{
    bot_service::{ComhairlePrompt, UpdateChatRequest, Variable},
    models::{
        chat_instructions::{self, UpsertChatInstructions},
        conversation,
    },
};
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
    let bot_service = state.required_bot_service()?;

    let conversation = conversation::get_by_id(&state.db, &conversation_id).await?;
    let chat_bot_id = conversation.chat_bot_id.ok_or_else(|| {
        ComhairleError::CorruptedData(format!(
            "Missing chat_bot_id on conversation {conversation_id}"
        ))
    })?;
    // Retrieve chat data first as some data needs to be added to update request
    // or it will be removed
    let (_, chat) = bot_service.get_chat(&chat_bot_id).await?;

    let instructions =
        chat_instructions::upsert_for_conversation(&state.db, conversation_id, &payload).await?;

    let mut variables = chat
        .prompt
        .and_then(|prompt| prompt.variables)
        .unwrap_or(vec![Variable {
            key: "knowledge".to_string(),
            optional: Some(true),
        }]);
    let original_len = variables.len();

    let payload_variables = [
        (payload.target_reading_age.is_some(), "target_reading_age"),
        (payload.max_length.is_some(), "max_length"),
    ];

    for (is_present, key) in payload_variables {
        if is_present && !variables.iter().any(|var| var.key == key) {
            variables.push(Variable {
                key: key.to_string(),
                optional: Some(true),
            });
        }
    }

    // Only update chat if variables has changed
    if variables.len() != original_len {
        let update_chat_payload = UpdateChatRequest {
            // Include knowledge_base_ids or they will be stripped (Ragflow specific)
            knowledge_base_ids: Some(chat.knowledge_base_ids),
            prompt: Some(ComhairlePrompt {
                variables: Some(variables),
                ..Default::default()
            }),
            ..Default::default()
        };

        bot_service
            .update_chat(&chat_bot_id, update_chat_payload)
            .await?;
    }

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
