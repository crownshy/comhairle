//! Tests for [`crate::models::thinking_space_follow_up_question`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
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
    use crate::models::thinking_space_follow_up_question::*;
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

    use crate::{
        models::model_test_helpers::{
            get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
        },
        routes::workflow_steps::dto::WorkflowStepDto,
        test_helpers::thinking_space_tool_config,
        tools::{ToolConfig, thinking_space::ThinkingSpaceToolConfig},
    };

    use serde_json::json;
    use std::error::Error;

    async fn create_thinking_space_resources(
        pool: &PgPool,
    ) -> Result<(Uuid, Uuid, ThinkingSpaceToolConfig), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;

        let (_, value, _) = session
            .create_workflow_step(
                &app,
                &conversation_id.to_string(),
                &workflow_id.to_string(),
                json!({
                    "name": "test_workflow_step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "A test workflow_step with prioritization",
                    "is_offline": false,
                    "required": false,
                    "tool_setup": thinking_space_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        let user = users::create_guest_user(pool).await?;

        let tool_config = match workflow_step.preview_tool_config {
            ToolConfig::ThinkingSpace(config) => config,
            _ => panic!("Wrong tool config type"),
        };

        Ok((user.id, workflow_step.id, tool_config))
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_follow_up_questions(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (user_id, workflow_step_id, tool_config) =
            create_thinking_space_resources(&pool).await?;

        let create_follow_ups = CreateFollowUpQuestions {
            root_question_id: tool_config.root_questions.first().unwrap().id,
            follow_up_questions: vec![
                "A follow up question".to_string(),
                "Another follow up question".to_string(),
            ],
            workflow_step_id,
        };

        let follow_ups = create(&pool, user_id, &create_follow_ups).await?;

        assert_eq!(
            follow_ups.root_question_id,
            tool_config.root_questions.first().unwrap().id,
            "incorrect root_question_id"
        );
        assert_eq!(
            follow_ups.follow_up_questions.len(),
            2,
            "incorrect number of follow ups"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_follow_up_questions(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (user_id, workflow_step_id, tool_config) =
            create_thinking_space_resources(&pool).await?;

        let create_follow_ups = CreateFollowUpQuestions {
            root_question_id: tool_config.root_questions.first().unwrap().id,
            follow_up_questions: vec![
                "A follow up question".to_string(),
                "Another follow up question".to_string(),
            ],
            workflow_step_id,
        };

        let follow_ups = create(&pool, user_id, &create_follow_ups).await?;

        let update_follow_ups = UpdateFollowUpQuestions {
            follow_up_questions: vec![
                "An updated question".to_string(),
                "Another updated question".to_string(),
            ],
        };

        let follow_ups = update(&pool, follow_ups.id, &update_follow_ups).await?;

        assert_eq!(
            follow_ups.follow_up_questions,
            vec![
                "An updated question".to_string(),
                "Another updated question".to_string(),
            ],
            "follow up questions incorrect after update"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_follow_up_questions_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (user_id, workflow_step_id, tool_config) =
            create_thinking_space_resources(&pool).await?;

        let create_follow_ups = CreateFollowUpQuestions {
            root_question_id: tool_config.root_questions.first().unwrap().id,
            follow_up_questions: vec![
                "A follow up question".to_string(),
                "Another follow up question".to_string(),
            ],
            workflow_step_id,
        };

        let created_follow_ups = create(&pool, user_id, &create_follow_ups).await?;

        let follow_ups = get_by_id(&pool, created_follow_ups.id).await?;

        assert_eq!(created_follow_ups.id, follow_ups.id, "ids don't match");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_follow_up_questions_for_workflow_step(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (user_a_id, workflow_step_id, tool_config) =
            create_thinking_space_resources(&pool).await?;

        let user_b = users::create_guest_user(&pool).await?;

        let params_a = CreateFollowUpQuestions {
            root_question_id: tool_config.root_questions.first().unwrap().id,
            follow_up_questions: vec![
                "A follow up question".to_string(),
                "Another follow up question".to_string(),
            ],
            workflow_step_id,
        };
        create(&pool, user_a_id, &params_a).await?;

        let params_b = CreateFollowUpQuestions {
            root_question_id: tool_config.root_questions.first().unwrap().id,
            follow_up_questions: vec![
                "A follow up question".to_string(),
                "Another follow up question".to_string(),
            ],
            workflow_step_id,
        };
        create(&pool, user_a_id, &params_b).await?;

        let params_c = CreateFollowUpQuestions {
            root_question_id: tool_config.root_questions.first().unwrap().id,
            follow_up_questions: vec![
                "A follow up question".to_string(),
                "Another follow up question".to_string(),
            ],
            workflow_step_id,
        };
        create(&pool, user_b.id, &params_c).await?;

        let filter_options = ThinkingSpaceFollowUpQuestionFilterOptions {
            user_id: Some(user_b.id),
            ..Default::default()
        };
        let follow_ups = list(&pool, &workflow_step_id, filter_options).await?;

        assert_eq!(follow_ups.len(), 1, "incorrect total");
        assert!(
            !follow_ups.iter().any(|f| f.user_id == user_a_id),
            "should not include user_a follow_ups"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_follow_up_questions_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (user_id, workflow_step_id, tool_config) =
            create_thinking_space_resources(&pool).await?;

        let create_follow_ups = CreateFollowUpQuestions {
            root_question_id: tool_config.root_questions.first().unwrap().id,
            follow_up_questions: vec![
                "A follow up question".to_string(),
                "Another follow up question".to_string(),
            ],
            workflow_step_id,
        };

        let follow_ups = create(&pool, user_id, &create_follow_ups).await?;

        delete(&pool, follow_ups.id).await?;

        let err = get_by_id(&pool, follow_ups.id).await.unwrap_err();

        match err {
            ModelError::Data(DataError::ResourceNotFound(message)) => {
                assert!(message.contains("Thinking Space Follow Up Questions"))
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
