//! Tests for [`crate::models::thinking_space_answer`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use crate::models::error::DataError;
    use serde_json::json;

    use crate::{
        models::{
            model_test_helpers::{
                get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
            },
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
    use crate::models::request_context::{ClientIp, ClientUserAgent};
    use crate::models::thinking_space_answer::*;
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
    async fn should_create_new_thinking_space_answer(pool: PgPool) -> Result<(), Box<dyn Error>> {
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
                    "tool_setup": thinking_space_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        let user = users::create_guest_user(&pool).await?;

        let create_answer = CreateAnswer {
            question: "A test question".to_string(),
            answer: "A test answer".to_string(),
            ..Default::default()
        };
        let answer = create(&pool, &workflow_step.id, &user.id, &create_answer).await?;

        assert_eq!(
            answer.workflow_step_id, workflow_step.id,
            "incorrect workflow_step_id"
        );
        assert_eq!(answer.status, AnswerStatus::Pending, "incorrect status");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_fail_thinking_space_answer_creation_if_missing_follow_up_params(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
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
                    "tool_setup": thinking_space_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        let user = users::create_guest_user(&pool).await?;

        let root_create = CreateAnswer {
            question: "A root question".to_string(),
            answer: "A root answer".to_string(),
            ..Default::default()
        };
        let root_answer = create(&pool, &workflow_step.id, &user.id, &root_create).await?;

        let follow_up_create = CreateAnswer {
            question: "A follow up question".to_string(),
            answer: "A follow up answer".to_string(),
            is_follow_up: Some(true),
            ..Default::default()
        };
        let err = create(&pool, &workflow_step.id, &user.id, &follow_up_create)
            .await
            .unwrap_err();

        let bad_request_message =
            "Follow up question must contain root_question_id and is_follow_up".to_string();
        match err {
            ModelError::Validation(ValidationError::BadRequest(message)) => {
                assert_eq!(message, bad_request_message, "incorrect error message")
            }
            _ => panic!("Expected bad request message"),
        }

        let follow_up_create = CreateAnswer {
            question: "A follow up question".to_string(),
            answer: "A follow up answer".to_string(),
            root_question_id: Some(root_answer.id),
            ..Default::default()
        };
        let err = create(&pool, &workflow_step.id, &user.id, &follow_up_create)
            .await
            .unwrap_err();

        let bad_request_message =
            "Follow up question must contain root_question_id and is_follow_up".to_string();
        match err {
            ModelError::Validation(ValidationError::BadRequest(message)) => {
                assert_eq!(message, bad_request_message, "incorrect error message")
            }
            _ => panic!("Expected bad request message"),
        }

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_thinking_space_answer_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
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
                    "tool_setup": thinking_space_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        let user = users::create_guest_user(&pool).await?;

        let create_answer = CreateAnswer {
            question: "A test question".to_string(),
            answer: "A test answer".to_string(),
            ..Default::default()
        };
        let new_answer = create(&pool, &workflow_step.id, &user.id, &create_answer).await?;

        let answer = get_by_id(&pool, &new_answer.id).await?;

        assert_eq!(new_answer.id, answer.id, "ids don't match");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_thinking_space_answers(pool: PgPool) -> Result<(), Box<dyn Error>> {
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
                    "tool_setup": thinking_space_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        // User a
        let user_a = users::create_guest_user(&pool).await?;
        let create_a_a = CreateAnswer {
            question: "A root question".to_string(),
            answer: "A root answer".to_string(),
            ..Default::default()
        };
        let answer_a_a = create(&pool, &workflow_step.id, &user_a.id, &create_a_a).await?;

        let create_a_b = CreateAnswer {
            question: "A follow up question".to_string(),
            answer: "A follow up answer".to_string(),
            is_follow_up: Some(true),
            root_question_id: Some(answer_a_a.id),
            other_questions: Some(vec!["An unanswered question".to_string()]),
            ..Default::default()
        };
        let _answer_a_b = create(&pool, &workflow_step.id, &user_a.id, &create_a_b).await?;

        let create_a_c = CreateAnswer {
            question: "A root question".to_string(),
            answer: "A root answer".to_string(),
            ..Default::default()
        };
        let answer_a_c = create(&pool, &workflow_step.id, &user_a.id, &create_a_c).await?;

        // User b
        let user_b = users::create_guest_user(&pool).await?;
        let create_b_a = CreateAnswer {
            question: "A root question".to_string(),
            answer: "A root answer".to_string(),
            ..Default::default()
        };
        let answer_b_a = create(&pool, &workflow_step.id, &user_b.id, &create_b_a).await?;

        let create_b_b = CreateAnswer {
            question: "A follow up question".to_string(),
            answer: "A follow up answer".to_string(),
            is_follow_up: Some(true),
            root_question_id: Some(answer_b_a.id),
            other_questions: Some(vec!["An unanswered question".to_string()]),
            ..Default::default()
        };
        let answer_b_b = create(&pool, &workflow_step.id, &user_b.id, &create_b_b).await?;

        let filter_options = ThinkingSpaceAnswerFilterOptions {
            user_id: Some(user_a.id),
            ..Default::default()
        };
        let user_a_answers = list(&pool, &workflow_step.id, filter_options).await?;

        let filter_options = ThinkingSpaceAnswerFilterOptions {
            user_id: Some(user_b.id),
            status: None,
        };
        let user_b_answers = list(&pool, &workflow_step.id, filter_options).await?;

        assert_eq!(user_a_answers.len(), 3, "incorrect total user a");
        assert_eq!(user_b_answers.len(), 2, "incorrect total user b");

        let update_answer = UpdateAnswer {
            status: Some(AnswerStatus::Approved),
            ..Default::default()
        };
        update(&pool, &answer_a_c.id, &update_answer).await?;
        update(&pool, &answer_b_b.id, &update_answer).await?;

        let filter_options = ThinkingSpaceAnswerFilterOptions {
            status: Some(AnswerStatus::Approved),
            ..Default::default()
        };
        let approved_answers = list(&pool, &workflow_step.id, filter_options).await?;

        assert_eq!(approved_answers.len(), 2, "incorrect total approved");
        assert!(approved_answers.iter().any(|a| a.id == answer_a_c.id));
        assert!(approved_answers.iter().any(|a| a.id == answer_b_b.id));

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_thinking_space_answer(pool: PgPool) -> Result<(), Box<dyn Error>> {
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
                    "tool_setup": thinking_space_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        let user = users::create_guest_user(&pool).await?;

        let create_answer = CreateAnswer {
            question: "A test question".to_string(),
            answer: "A test answer".to_string(),
            ..Default::default()
        };
        let new_answer = create(&pool, &workflow_step.id, &user.id, &create_answer).await?;

        let update_answer = UpdateAnswer {
            status: Some(AnswerStatus::Approved),
            answer: Some("Something different".to_string()),
        };

        let answer = update(&pool, &new_answer.id, &update_answer).await?;

        assert_eq!(answer.status, AnswerStatus::Approved, "incorrect status");
        assert_eq!(
            answer.answer,
            "Something different".to_string(),
            "incorrect answer"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_thinking_space_answer_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
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
                    "tool_setup": thinking_space_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        let user = users::create_guest_user(&pool).await?;

        let create_answer = CreateAnswer {
            question: "A test question".to_string(),
            answer: "A test answer".to_string(),
            ..Default::default()
        };
        let new_answer = create(&pool, &workflow_step.id, &user.id, &create_answer).await?;

        delete(&pool, &new_answer.id).await?;

        let err = get_by_id(&pool, &new_answer.id).await.unwrap_err();

        match err {
            ModelError::Data(DataError::ResourceNotFound(message)) => {
                assert!(message.contains("Thinking Space Answer"))
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
