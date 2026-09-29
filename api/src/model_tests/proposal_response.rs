//! Tests for [`crate::models::proposal_response`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use crate::{
        models::{
            model_test_helpers::{
                get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
            },
            proposal::{self, CreateProposal},
            users,
        },
        routes::user::dto::UserDto,
        test_helpers::TEST_PASSWORD,
        tools::ToolConfig,
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
    use crate::models::proposal_response::*;
    #[allow(unused_imports)]
    use crate::models::request_context::{ClientIp, ClientUserAgent};
    #[allow(unused_imports)]
    use crate::models::user_progress::ProgressStatus;
    #[allow(unused_imports)]
    use crate::models::users::User;
    #[allow(unused_imports)]
    use crate::models::{proposal_section, user_progress};
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
    async fn should_create_new_proposal_response(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let workflow_step = session
            .create_prioritization_workflow_step(&app, &conversation_id, &workflow_id)
            .await?;

        let (proposal, _) = proposal::create(
            &pool,
            &workflow_step.id,
            &CreateProposal {
                title: "A new proposal".to_string(),
                sections: vec!["Test proposal".to_string()],
            },
            "en",
        )
        .await?;

        let (_, value, _) = session
            .login(&app, "admin@crown-shy.com", TEST_PASSWORD)
            .await?;
        let user: UserDto = serde_json::from_value(value)?;

        let tool_config = match workflow_step.preview_tool_config {
            ToolConfig::Prioritization(config) => config,
            _ => panic!("Incorrect tool_config type"),
        };

        let create_response = CreateResponse {
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: (-1.0_f64).into(),
                    section_id: None,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.5_f64.into(),
                    section_id: None,
                },
            ],
        };

        let proposal_response = create(&pool, &proposal.id, &user.id, &create_response).await?;

        assert_eq!(
            proposal_response.proposal_id, proposal.id,
            "incorrect proposal_id"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_proposal_responses(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let workflow_step = session
            .create_prioritization_workflow_step(&app, &conversation_id, &workflow_id)
            .await?;

        let (proposal_a, _) = proposal::create(
            &pool,
            &workflow_step.id,
            &CreateProposal {
                title: "Proposal A".to_string(),
                sections: vec!["Proposal A".to_string()],
            },
            "en",
        )
        .await?;
        let (proposal_b, _) = proposal::create(
            &pool,
            &workflow_step.id,
            &CreateProposal {
                title: "Proposal B".to_string(),
                sections: vec!["Proposal B".to_string()],
            },
            "en",
        )
        .await?;

        let tool_config = match workflow_step.preview_tool_config {
            ToolConfig::Prioritization(config) => config,
            _ => panic!("Incorrect tool_config type"),
        };

        let user_a = users::create_guest_user(&pool).await?;
        let create_response_a_a = CreateResponse {
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: (-1.0_f64).into(),
                    section_id: None,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.5_f64.into(),
                    section_id: None,
                },
            ],
        };
        let create_response_a_b = CreateResponse {
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: 0.5_f64.into(),
                    section_id: None,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.2_f64.into(),
                    section_id: None,
                },
            ],
        };
        create(&pool, &proposal_a.id, &user_a.id, &create_response_a_a).await?;
        create(&pool, &proposal_b.id, &user_a.id, &create_response_a_b).await?;

        let user_b = users::create_guest_user(&pool).await?;
        let create_response_b_a = CreateResponse {
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: (-1.0_f64).into(),
                    section_id: None,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.5_f64.into(),
                    section_id: None,
                },
            ],
        };
        let create_response_b_b = CreateResponse {
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: 0.5_f64.into(),
                    section_id: None,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.2_f64.into(),
                    section_id: None,
                },
            ],
        };
        create(&pool, &proposal_a.id, &user_b.id, &create_response_b_a).await?;
        create(&pool, &proposal_b.id, &user_b.id, &create_response_b_b).await?;

        let user_c = users::create_guest_user(&pool).await?;
        let create_response_c_a = CreateResponse {
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: (-1.0_f64).into(),
                    section_id: None,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.5_f64.into(),
                    section_id: None,
                },
            ],
        };
        let create_response_c_b = CreateResponse {
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: 0.5_f64.into(),
                    section_id: None,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.2_f64.into(),
                    section_id: None,
                },
            ],
        };
        create(&pool, &proposal_a.id, &user_c.id, &create_response_c_a).await?;
        create(&pool, &proposal_b.id, &user_c.id, &create_response_c_b).await?;

        let filter_options = ProposalResponseFilterOptions;
        let order_options = ProposalResponseOrderOptions;
        let proposal_a_responses = list(
            &pool,
            &proposal_a.id,
            filter_options.clone(),
            order_options.clone(),
        )
        .await?;

        let proposal_b_responses = list(
            &pool,
            &proposal_b.id,
            filter_options.clone(),
            order_options.clone(),
        )
        .await?;

        assert_eq!(proposal_a_responses.len(), 3, "incorrect a total");
        assert_eq!(proposal_b_responses.len(), 3, "incorrect b total");

        Ok(())
    }
}
