//! Tests for [`crate::models::proposal`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use crate::models::error::DataError;
    use serde_json::json;

    use crate::{
        models::model_test_helpers::{
            get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
        },
        routes::workflow_steps::dto::WorkflowStepDto,
        test_helpers::prioritization_tool_config,
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
    use crate::models::proposal::*;
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_new_proposal(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
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
                    "tool_setup": prioritization_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        let create_proposal = CreateProposal {
            title: "Test proposal".to_string(),
            sections: vec!["Test proposal".to_string()],
        };
        let (proposal, sections) = create(&pool, &workflow_step.id, &create_proposal, "en").await?;

        assert_eq!(
            proposal.workflow_step_id, workflow_step.id,
            "incorrect workflow_step_id"
        );
        assert_eq!(sections.len(), 1, "expected one section");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_localized_proposal(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
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
                    "tool_setup": prioritization_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        let create_proposal = CreateProposal {
            title: "Test proposal".to_string(),
            sections: vec!["Test proposal body".to_string()],
        };
        let (new_proposal, _) = create(&pool, &workflow_step.id, &create_proposal, "en").await?;

        let proposal = get_localized_by_id(&pool, &new_proposal.id, "en").await?;

        assert_eq!(
            proposal.title,
            "Test proposal".to_string(),
            "incorrect title"
        );

        let sections = proposal_section::list_localized(&pool, &new_proposal.id, "en").await?;
        assert_eq!(sections.len(), 1, "incorrect number of sections");
        assert_eq!(
            sections[0].body,
            "Test proposal body".to_string(),
            "incorrect body"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_proposals(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
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
                    "tool_setup": prioritization_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        let create_proposal_1 = CreateProposal {
            title: "Proposal A".to_string(),
            sections: vec!["Proposal A".to_string()],
        };
        let create_proposal_2 = CreateProposal {
            title: "Proposal B".to_string(),
            sections: vec!["Proposal B".to_string()],
        };
        let create_proposal_3 = CreateProposal {
            title: "Proposal C".to_string(),
            sections: vec!["Proposal C".to_string()],
        };
        create(&pool, &workflow_step.id, &create_proposal_1, "en").await?;
        create(&pool, &workflow_step.id, &create_proposal_2, "en").await?;
        create(&pool, &workflow_step.id, &create_proposal_3, "en").await?;

        let proposals = list_localized(&pool, &workflow_step.id, "en").await?;

        assert_eq!(proposals.len(), 3, "incorrect number of proposals");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_proposal(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
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
                    "tool_setup": prioritization_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        let create_proposal = CreateProposal {
            title: "Test proposal".to_string(),
            sections: vec!["Test proposal".to_string()],
        };
        let (proposal, _) = create(&pool, &workflow_step.id, &create_proposal, "en").await?;

        delete(&pool, &proposal.id).await?;

        let err = get_localized_by_id(&pool, &proposal.id, "en")
            .await
            .unwrap_err();

        match err {
            ModelError::Data(DataError::ResourceNotFound(message)) => {
                assert!(message.contains("Proposal"))
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
