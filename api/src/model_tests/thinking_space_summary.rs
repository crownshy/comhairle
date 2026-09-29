//! Tests for [`crate::models::thinking_space_summary`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use crate::models::error::DataError;
    use crate::{
        models::{
            model_test_helpers::{
                get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
            },
            user_progress::{self, ProgressStatus, UpdateUserProgress},
            users,
        },
        routes::workflow_steps::dto::WorkflowStepDto,
        test_helpers::thinking_space_tool_config,
    };

    #[allow(unused_imports)]
    use crate::models::error::{
        AuthError, ConversationError, EventError, InviteError, ModelError, PermissionError,
        ReportError, UserError, ValidationError, WorkflowError,
    };
    #[allow(unused_imports)]
    use crate::models::moderation_status::ModerationStatus;
    #[allow(unused_imports)]
    use crate::models::pagination::{Order, PageOptions, PaginatedResults};
    #[allow(unused_imports)]
    #[allow(unused_imports)]
    use crate::models::proposal_section;
    #[allow(unused_imports)]
    use crate::models::request_context::{ClientIp, ClientUserAgent};
    use crate::models::thinking_space_summary::*;
    #[allow(unused_imports)]
    use crate::models::users::User;
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

    use serde_json::json;
    use std::error::Error;

    async fn create_thinking_space_resources(
        pool: &PgPool,
    ) -> Result<(Uuid, Uuid), Box<dyn Error>> {
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

        Ok((user.id, workflow_step.id))
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_thinking_space_summary(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (user_id, workflow_step_id) = create_thinking_space_resources(&pool).await?;

        let create_summary = CreateSummary {
            summary: "Some summary text".to_string(),
            is_ai_generated: Some(true),
        };

        let summary = create(&pool, user_id, workflow_step_id, &create_summary).await?;

        assert_eq!(
            summary.summary,
            "Some summary text".to_string(),
            "incorrect summary text"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_thinking_space_summary(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (user_id, workflow_step_id) = create_thinking_space_resources(&pool).await?;

        let create_summary = CreateSummary {
            summary: "Some summary text".to_string(),
            ..Default::default()
        };

        let summary = create(&pool, user_id, workflow_step_id, &create_summary).await?;

        assert_eq!(
            summary.summary,
            "Some summary text".to_string(),
            "incorrect summary text before update"
        );

        let update_summary = UpdateSummary {
            summary: "Some updated text".to_string(),
        };

        let summary = update(&pool, summary.id, &update_summary).await?;

        assert_eq!(
            summary.summary,
            "Some updated text".to_string(),
            "incorrect summary text after update"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_thinking_space_summary_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (user_id, workflow_step_id) = create_thinking_space_resources(&pool).await?;

        let create_summary = CreateSummary {
            summary: "Some summary text".to_string(),
            is_ai_generated: Some(true),
        };

        let created_summary = create(&pool, user_id, workflow_step_id, &create_summary).await?;

        let summary = get_by_id(&pool, created_summary.id).await?;

        assert_eq!(created_summary.id, summary.id, "ids don't match");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_thinking_space_summaries(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (user_a_id, workflow_step_id) = create_thinking_space_resources(&pool).await?;

        let user_b = users::create_guest_user(&pool).await?;

        let params_a = CreateSummary {
            summary: "Summary_a".to_string(),
            ..Default::default()
        };
        create(&pool, user_a_id, workflow_step_id, &params_a).await?;

        let params_b = CreateSummary {
            summary: "Summary_b".to_string(),
            is_ai_generated: Some(true),
        };
        create(&pool, user_a_id, workflow_step_id, &params_b).await?;
        let params_c = CreateSummary {
            summary: "Summary_c".to_string(),
            is_ai_generated: Some(true),
        };
        create(&pool, user_a_id, workflow_step_id, &params_c).await?;
        let params_d = CreateSummary {
            summary: "Summary_d".to_string(),
            is_ai_generated: Some(true),
        };
        create(&pool, user_a_id, workflow_step_id, &params_d).await?;
        let params_e = CreateSummary {
            summary: "Summary_e".to_string(),
            is_ai_generated: Some(true),
        };
        let summary_e = create(&pool, user_b.id, workflow_step_id, &params_e).await?;

        let filter_options = ThinkingSpaceSummaryFilterOptions {
            user_id: Some(user_a_id),
            is_ai_generated: Some(true),
            ..Default::default()
        };
        let summaries = list(&pool, &workflow_step_id, filter_options).await?;

        assert!(
            summaries.iter().all(|s| s.is_ai_generated),
            "not all edited summaries"
        );
        assert!(
            !summaries.iter().any(|s| s.id == summary_e.id),
            "user_b summary is included"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_filter_by_is_shared_with_organizer(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (user_a_id, workflow_step_id) = create_thinking_space_resources(&pool).await?;
        let user_b = users::create_guest_user(&pool).await?;

        // user_a: permission granted -> should be included when filtering true
        let summary_shared = create(
            &pool,
            user_a_id,
            workflow_step_id,
            &CreateSummary {
                summary: "Summary_shared".to_string(),
                ..Default::default()
            },
        )
        .await?;

        // user_b: permission explicitly denied -> should be excluded when filtering true
        let summary_not_shared = create(
            &pool,
            user_b.id,
            workflow_step_id,
            &CreateSummary {
                summary: "Summary_not_shared".to_string(),
                ..Default::default()
            },
        )
        .await?;

        // Create user_progress records, defaulting to permissions = true, and
        // update to permission = false for user_b.
        user_progress::create(&pool, &user_a_id, &workflow_step_id, ProgressStatus::Done).await?;
        user_progress::create(&pool, &user_b.id, &workflow_step_id, ProgressStatus::Done).await?;
        let not_shared_params = UpdateUserProgress {
            permission_to_share_with_organizers: Some(false),
            ..Default::default()
        };
        user_progress::update(&pool, &user_b.id, &workflow_step_id, &not_shared_params).await?;

        let filter_options = ThinkingSpaceSummaryFilterOptions {
            is_shared_with_organizer: Some(true),
            ..Default::default()
        };
        let summaries = list(&pool, &workflow_step_id, filter_options).await?;

        assert!(
            summaries.iter().any(|s| s.id == summary_shared.id),
            "summary with granted permission is missing"
        );
        assert!(
            !summaries.iter().any(|s| s.id == summary_not_shared.id),
            "summary with denied permission is included"
        );

        // Inverse filter should return the opposite set
        let filter_options = ThinkingSpaceSummaryFilterOptions {
            is_shared_with_organizer: Some(false),
            ..Default::default()
        };
        let summaries = list(&pool, &workflow_step_id, filter_options).await?;

        assert!(
            summaries.iter().any(|s| s.id == summary_not_shared.id),
            "summary with denied permission is missing from false filter"
        );
        assert!(
            !summaries.iter().any(|s| s.id == summary_shared.id),
            "summary with granted permission is included in false filter"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_thinking_space_summary(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (user_id, workflow_step_id) = create_thinking_space_resources(&pool).await?;

        let create_summary = CreateSummary {
            summary: "Some summary text".to_string(),
            is_ai_generated: Some(true),
        };

        let summary = create(&pool, user_id, workflow_step_id, &create_summary).await?;

        delete(&pool, summary.id).await?;

        let err = get_by_id(&pool, summary.id).await.unwrap_err();

        match err {
            ModelError::Data(DataError::ResourceNotFound(message)) => {
                assert!(message.contains("Thinking Space Summary"))
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
