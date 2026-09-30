//! Tests for [`crate::models::chat_instructions`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use crate::models::chat_instructions::*;
    #[allow(unused_imports)]
    use crate::models::error::{
        AuthError, ConversationError, DataError, EventError, InviteError, ModelError,
        PermissionError, ReportError, UserError, ValidationError, WorkflowError,
    };
    #[allow(unused_imports)]
    use crate::models::moderation_status::ModerationStatus;
    #[allow(unused_imports)]
    use crate::models::pagination::{Order, PageOptions, PaginatedResults};
    #[allow(unused_imports)]
    use crate::models::request_context::{ClientIp, ClientUserAgent};
    #[allow(unused_imports)]
    use crate::models::user_progress::ProgressStatus;
    #[allow(unused_imports)]
    use crate::models::users::User;
    #[allow(unused_imports)]
    use crate::models::{proposal_section, user_progress, users};
    #[allow(unused_imports)]
    use ::std::collections::{HashMap, HashSet};
    #[allow(unused_imports)]
    use chrono::{DateTime, Utc};
    #[allow(unused_imports)]
    use schemars::JsonSchema;
    #[allow(unused_imports)]
    use sqlx::PgPool;
    #[allow(unused_imports)]
    use uuid::Uuid;

    use std::error::Error;

    use crate::models::model_test_helpers::{
        get_random_conversation_id, setup_default_app_and_session,
    };

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_instructions_for_conversation(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let payload = UpsertChatInstructions {
            target_reading_age: Some(17),
            max_length: Some(500),
        };

        let instructions = upsert_for_conversation(&pool, conversation_id, &payload).await?;

        assert_eq!(
            instructions.conversation_id, conversation_id,
            "incorrect conversation_id"
        );
        assert_eq!(
            instructions.target_reading_age,
            Some(17),
            "incorrect reading age"
        );
        assert_eq!(instructions.max_length, Some(500), "incorrect max_length");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_instructions_if_already_exists(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let payload = UpsertChatInstructions {
            target_reading_age: Some(10),
            max_length: Some(100),
        };

        let new_instructions = upsert_for_conversation(&pool, conversation_id, &payload).await?;

        assert_eq!(
            new_instructions.conversation_id, conversation_id,
            "incorrect conversation_id before update"
        );
        assert_eq!(
            new_instructions.target_reading_age,
            Some(10),
            "incorrect reading age before update"
        );
        assert_eq!(
            new_instructions.max_length,
            Some(100),
            "incorrect max_length before update"
        );

        let payload = UpsertChatInstructions {
            target_reading_age: Some(15),
            max_length: Some(200),
        };

        let updated_instructions =
            upsert_for_conversation(&pool, conversation_id, &payload).await?;

        assert_eq!(
            updated_instructions.conversation_id, conversation_id,
            "incorrect conversation_id after update"
        );
        assert_eq!(
            updated_instructions.target_reading_age,
            Some(15),
            "incorrect reading age after update"
        );
        assert_eq!(
            updated_instructions.max_length,
            Some(200),
            "incorrect max_length after update"
        );
        assert_eq!(
            new_instructions.id, updated_instructions.id,
            "ids don't match for instructions"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_instructions_for_conversation(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let payload = UpsertChatInstructions {
            target_reading_age: Some(17),
            max_length: Some(500),
        };

        let new_instructions = upsert_for_conversation(&pool, conversation_id, &payload).await?;

        let fetched_instructions = get_by_conversation_id(&pool, conversation_id).await?;

        assert_eq!(
            new_instructions.id, fetched_instructions.id,
            "ids don't match for instructions"
        );

        Ok(())
    }
}
