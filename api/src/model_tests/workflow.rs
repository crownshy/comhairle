//! Tests for [`crate::models::workflow`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use crate::{
        models::model_test_helpers::setup_default_app_and_session,
        routes::{
            conversations::dto::ConversationDto, regions::dto::RegionDto,
            workflows::dto::WorkflowDto,
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
    use crate::models::workflow::*;
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_workflow_with_region_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, convo_res, _) = session.create_random_conversation(&app).await?;
        let conversation: ConversationDto = serde_json::from_value(convo_res)?;
        let (_, region_res, _) = session.create_random_region(&app).await?;
        let region: RegionDto = serde_json::from_value(region_res)?;

        let (_, user, _) = session.current_user(&app).await?;

        let params = CreateWorkflow {
            name: "test_workflow".to_string(),
            description: "a test workflow".to_string(),
            is_active: true,
            is_public: true,
            auto_login: false,
            region_id: Some(region.id),
        };

        let workflow = create(&pool, &params, Some(conversation.id), None, user.id).await?;

        assert_eq!(
            workflow.region_id.unwrap(),
            region.id,
            "incorrect region_id"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_workflow_region_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, convo_res, _) = session.create_random_conversation(&app).await?;
        let conversation: ConversationDto = serde_json::from_value(convo_res)?;
        let (_, region_res, _) = session.create_random_region(&app).await?;
        let region: RegionDto = serde_json::from_value(region_res)?;
        let (_, workflow_res, _) = session
            .create_random_workflow(&app, &conversation.id.to_string())
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(workflow_res)?;

        assert!(
            workflow.region_id.is_none(),
            "incorrect region_id before update"
        );

        let workflow = update(
            &pool,
            workflow.id,
            &PartialWorkflow {
                region_id: Some(region.id),
                ..Default::default()
            },
        )
        .await?;

        assert_eq!(
            workflow.region_id.unwrap(),
            region.id,
            "incorrect region_id"
        );

        Ok(())
    }
}
