//! Tests for [`crate::models::recruitment_target`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    #[allow(unused_imports)]
    use crate::models::error::{
        AuthError, ConversationError, DataError, EventError, InviteError, ModelError,
        PermissionError, ReportError, UserError, ValidationError, WorkflowError,
    };
    use crate::models::model_test_helpers::setup_default_app_and_session;
    #[allow(unused_imports)]
    use crate::models::moderation_status::ModerationStatus;
    #[allow(unused_imports)]
    use crate::models::pagination::{Order, PageOptions, PaginatedResults};
    use crate::models::recruitment_target::*;
    #[allow(unused_imports)]
    use crate::models::request_context::{ClientIp, ClientUserAgent};
    #[allow(unused_imports)]
    use crate::models::user_progress::ProgressStatus;
    #[allow(unused_imports)]
    use crate::models::users::User;
    #[allow(unused_imports)]
    use crate::models::{proposal_section, user_progress, users};
    use crate::routes::{conversations::dto::ConversationDto, workflows::dto::WorkflowDto};
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

    async fn make_workflow(pool: &PgPool) -> Result<Uuid, Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(pool).await?;
        let (_, conversation, _) = session.create_random_conversation(&app).await?;
        let conversation: ConversationDto = serde_json::from_value(conversation)?;
        let (_, workflow, _) = session
            .create_random_workflow(&app, &conversation.id.to_string())
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(workflow)?;
        Ok(workflow.id)
    }

    #[sqlx::test]
    async fn should_create_a_recruitment_target(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let workflow_id = make_workflow(&pool).await?;

        let target = create(
            &pool,
            &workflow_id,
            &CreateRecruitmentTarget {
                metric: "age".into(),
                bucket: "18-24".into(),
                target_count: 30,
            },
        )
        .await?;

        assert_eq!(target.workflow_id, workflow_id);
        assert_eq!(target.metric, "age");
        assert_eq!(target.bucket, "18-24");
        assert_eq!(target.target_count, 30);

        Ok(())
    }

    #[sqlx::test]
    async fn should_upsert_when_same_metric_bucket(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let workflow_id = make_workflow(&pool).await?;

        let first = create(
            &pool,
            &workflow_id,
            &CreateRecruitmentTarget {
                metric: "age".into(),
                bucket: "18-24".into(),
                target_count: 30,
            },
        )
        .await?;

        let second = create(
            &pool,
            &workflow_id,
            &CreateRecruitmentTarget {
                metric: "age".into(),
                bucket: "18-24".into(),
                target_count: 45,
            },
        )
        .await?;

        assert_eq!(first.id, second.id, "should be the same row");
        assert_eq!(second.target_count, 45, "target count should be updated");

        let all = list_for_workflow(&pool, &workflow_id).await?;
        assert_eq!(all.len(), 1, "should only have one row");

        Ok(())
    }

    #[sqlx::test]
    async fn should_list_targets_for_workflow(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let workflow_id = make_workflow(&pool).await?;

        create(
            &pool,
            &workflow_id,
            &CreateRecruitmentTarget {
                metric: "age".into(),
                bucket: "18-24".into(),
                target_count: 10,
            },
        )
        .await?;
        create(
            &pool,
            &workflow_id,
            &CreateRecruitmentTarget {
                metric: "gender".into(),
                bucket: "female".into(),
                target_count: 50,
            },
        )
        .await?;

        let targets = list_for_workflow(&pool, &workflow_id).await?;

        assert_eq!(targets.len(), 2);
        Ok(())
    }

    #[sqlx::test]
    async fn should_update_a_target(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let workflow_id = make_workflow(&pool).await?;
        let target = create(
            &pool,
            &workflow_id,
            &CreateRecruitmentTarget {
                metric: "age".into(),
                bucket: "18-24".into(),
                target_count: 10,
            },
        )
        .await?;

        let updated = update(
            &pool,
            &target.id,
            &PartialRecruitmentTarget {
                target_count: Some(20),
                ..PartialRecruitmentTarget::default()
            },
        )
        .await?;

        assert_eq!(updated.target_count, 20);
        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_a_target(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let workflow_id = make_workflow(&pool).await?;
        let target = create(
            &pool,
            &workflow_id,
            &CreateRecruitmentTarget {
                metric: "age".into(),
                bucket: "18-24".into(),
                target_count: 10,
            },
        )
        .await?;

        delete(&pool, &target.id).await?;

        let result = get_by_id(&pool, &target.id).await;
        assert!(result.is_err(), "target should be gone");
        Ok(())
    }

    #[sqlx::test]
    async fn should_cascade_delete_when_workflow_deleted(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let workflow_id = make_workflow(&pool).await?;
        let target = create(
            &pool,
            &workflow_id,
            &CreateRecruitmentTarget {
                metric: "age".into(),
                bucket: "18-24".into(),
                target_count: 10,
            },
        )
        .await?;

        // The workflow route auto-assigns the created workflow as the
        // conversation's default_workflow_id, and that FK has no ON DELETE
        // clause. Clear it so the workflow row can actually be deleted.
        sqlx::query(
            "UPDATE conversation SET default_workflow_id = NULL WHERE default_workflow_id = $1",
        )
        .bind(workflow_id)
        .execute(&pool)
        .await?;

        crate::models::workflow::delete(&pool, &workflow_id).await?;

        let result = get_by_id(&pool, &target.id).await;
        assert!(
            result.is_err(),
            "target should cascade-delete with workflow"
        );
        Ok(())
    }
}
