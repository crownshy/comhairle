//! Tests for [`crate::models::workflow_step`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use crate::{
        models::{
            self,
            conversation::{self, PartialConversation},
            model_test_helpers::{get_random_conversation_id, setup_default_app_and_session},
            user_progress::UpdateUserProgress,
            workflow,
        },
        routes::{
            auth::SignupRequest, workflow_steps::dto::WorkflowStepDto, workflows::dto::WorkflowDto,
        },
    };

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
    use crate::models::workflow_step::*;
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
    use std::error::Error;
    #[allow(unused_imports)]
    use uuid::Uuid;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_step_for_live_conversation(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let _conversation = conversation::update(
            &pool,
            &conversation_id,
            &PartialConversation {
                is_live: Some(true),
                ..Default::default()
            },
        )
        .await?;

        let (_, workflow_res, _) = session
            .create_random_workflow(&app, &conversation_id.to_string())
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(workflow_res)?;
        let steps_res = session
            .create_random_workflow_steps(
                &app,
                &conversation_id.to_string(),
                &workflow.id.to_string(),
                1,
            )
            .await?;
        let step: WorkflowStepDto = serde_json::from_value(steps_res.first().unwrap().to_owned())?;

        assert!(step.tool_config.is_some(), "missing tool_config");
        assert_eq!(
            step.preview_tool_config,
            step.tool_config.unwrap(),
            "tool_config's don't match"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_can_revisit_field(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, workflow_res, _) = session
            .create_random_workflow(&app, &conversation_id.to_string())
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(workflow_res)?;
        let steps_res = session
            .create_random_workflow_steps(
                &app,
                &conversation_id.to_string(),
                &workflow.id.to_string(),
                1,
            )
            .await?;
        let step: WorkflowStepDto = serde_json::from_value(steps_res.first().unwrap().to_owned())?;

        assert!(!step.can_revisit, "incorrect can_revisit before update");

        let step = update(
            &pool,
            &step.id,
            &workflow.id,
            &PartialWorkflowStep {
                can_revisit: Some(true),
                ..Default::default()
            },
        )
        .await?;

        assert!(step.can_revisit, "incorrect can_revisit after update");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_localized_steps(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, workflow_res, _) = session
            .create_random_workflow(&app, &conversation_id.to_string())
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(workflow_res)?;
        let _ = session
            .create_random_workflow_steps(
                &app,
                &conversation_id.to_string(),
                &workflow.id.to_string(),
                5,
            )
            .await?;

        let steps = list_localized(&pool, &workflow.id, "en").await?;

        assert_eq!(steps.len(), 5, "incorrect number of steps");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_backfill_user_progress_for_existing_participants_when_step_added(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, workflow_res, _) = session
            .create_random_workflow(&app, &conversation_id.to_string())
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(workflow_res)?;

        // Create a step, register the user, then add another step — the
        // newly added step should produce a user_progress row for the
        // already-registered user.
        let _ = session
            .create_random_workflow_steps(
                &app,
                &conversation_id.to_string(),
                &workflow.id.to_string(),
                1,
            )
            .await?;

        let user = models::users::create_user(
            &SignupRequest {
                username: "backfill_user".to_string(),
                password: "test_pw".to_string(),
                avatar_url: None,
                email: "backfill_email".to_string(),
            },
            &pool,
        )
        .await?;
        workflow::register_user(&pool, &workflow.id, &user).await?;

        let initial_progress =
            models::user_progress::list_for_user_on_workflow(&pool, &user.id, &workflow.id).await?;
        assert_eq!(
            initial_progress.len(),
            1,
            "expected one progress row after register"
        );

        let new_steps_res = session
            .create_random_workflow_steps(
                &app,
                &conversation_id.to_string(),
                &workflow.id.to_string(),
                1,
            )
            .await?;
        let new_step: WorkflowStepDto =
            serde_json::from_value(new_steps_res.first().unwrap().to_owned())?;

        let progress_after =
            models::user_progress::list_for_user_on_workflow(&pool, &user.id, &workflow.id).await?;
        assert_eq!(
            progress_after.len(),
            2,
            "new step should have produced a progress row for the registered user"
        );
        let backfilled = progress_after
            .iter()
            .find(|p| p.workflow_step_id == new_step.id)
            .expect("progress row for newly added step is missing");
        assert_eq!(
            backfilled.status,
            ProgressStatus::NotStarted,
            "backfilled progress should be NotStarted"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_localized_steps_with_user_progress(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, workflow_res, _) = session
            .create_random_workflow(&app, &conversation_id.to_string())
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(workflow_res)?;
        let steps = session
            .create_random_workflow_steps(
                &app,
                &conversation_id.to_string(),
                &workflow.id.to_string(),
                5,
            )
            .await?;
        let first_step: WorkflowStepDto = serde_json::from_value(steps.first().unwrap().clone())?;
        let second_step: WorkflowStepDto = serde_json::from_value(steps.get(1).unwrap().clone())?;
        let user = models::users::create_user(
            &SignupRequest {
                username: "test_user".to_string(),
                password: "test_pw".to_string(),
                avatar_url: None,
                email: "test_email".to_string(),
            },
            &pool,
        )
        .await?;
        workflow::register_user(&pool, &workflow.id, &user).await?;

        let params = UpdateUserProgress {
            status: Some(ProgressStatus::Done),
            ..Default::default()
        };
        models::user_progress::update(&pool, &user.id, &first_step.id, &params).await?;
        let params = UpdateUserProgress {
            status: Some(ProgressStatus::InProgress),
            ..Default::default()
        };
        models::user_progress::update(&pool, &user.id, &second_step.id, &params).await?;

        let steps = list_localized_with_progress(&pool, &workflow.id, "en", &user.id).await?;
        assert_eq!(
            steps[0].status,
            ProgressStatus::Done,
            "incorrect first step progess status"
        );
        assert_eq!(
            steps[1].status,
            ProgressStatus::InProgress,
            "incorrect second step progess status"
        );
        assert_eq!(
            steps[2].status,
            ProgressStatus::NotStarted,
            "incorrect third step progess status"
        );
        assert_eq!(
            steps[3].status,
            ProgressStatus::NotStarted,
            "incorrect fourth step progess status"
        );
        assert_eq!(
            steps[4].status,
            ProgressStatus::NotStarted,
            "incorrect fifth step progess status"
        );

        Ok(())
    }
}
