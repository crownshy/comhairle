//! Tests for [`crate::models::demographics`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use crate::models::model_test_helpers::setup_default_app_and_session;
    use crate::test_helpers::UserSession;

    use crate::models::demographics::*;
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

    // Test can create, get, update and delete a demographics question.
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_create_get_update_delete_demographics_question(
        db: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let slug = "example_slug".to_string();
        let response_type = DemographicsQuestionResponseType::Number;

        // Ensure the question does not exist before creation
        let filters = DemographicsQuestionsFilterOptions {
            question_slug: Some(slug.clone()),
            ..Default::default()
        };
        let response = get_demographics_questions(&db, filters, PageOptions::default()).await?;
        assert_eq!(
            response.total, 0,
            "Question should not exist before creation"
        );

        // Create the demographics question and ensure it can be retrieved successfully
        let new_demographics_question = CreateDemographicsQuestion {
            slug: slug.clone(),
            display_name: "Example Display Name".to_string(),
            response_type: response_type.clone(),
            bucket_config: None,
        };
        let question = create_demographics_question(&db, new_demographics_question).await?;
        let filters = DemographicsQuestionsFilterOptions {
            question_slug: Some(slug.clone()),
            ..Default::default()
        };
        let response = get_demographics_questions(&db, filters, PageOptions::default()).await?;
        assert_eq!(response.total, 1, "Question should exist after creation");
        assert_eq!(vec![question], response.records);

        // Update the demographics question with bucket_config and ensure the changes are reflected
        let string_options = ValueBuckets::String {
            options: vec![StringOption {
                value: "opt1".to_string(),
                label: "Option 1".to_string(),
            }],
        };
        let update = PartialDemographicsQuestion {
            display_name: Some("Updated Display Name".to_string()),
            response_type: Some(DemographicsQuestionResponseType::String),
            bucket_config: Some(Some(sqlx::types::Json(string_options))),
        };
        let updated_question = update_demographics_question(&db, slug.clone(), update).await?;
        assert_eq!(
            updated_question.response_type,
            DemographicsQuestionResponseType::String,
            "Question response type should be updated correctly"
        );
        assert_eq!(
            updated_question.display_name,
            "Updated Display Name".to_string(),
            "Question display name should be updated correctly"
        );
        assert!(
            updated_question.bucket_config.is_some(),
            "Bucket config should be set"
        );

        // Update with bucket_config: Some(None) to clear all options (set bucket_config to NULL)
        let clear_update = PartialDemographicsQuestion {
            display_name: None,
            response_type: None,
            bucket_config: Some(None),
        };
        let cleared_question =
            update_demographics_question(&db, slug.clone(), clear_update).await?;
        assert!(
            cleared_question.bucket_config.is_none(),
            "Bucket config should be cleared to NULL"
        );

        // Delete the demographics question and ensure it is removed successfully
        let response = delete_demographics_question(&db, slug.clone()).await?;
        assert_eq!(
            Some(cleared_question),
            response,
            "Question should be deleted successfully"
        );

        // Ensure the question is actually deleted
        let filters = DemographicsQuestionsFilterOptions {
            question_slug: Some(slug.clone()),
            ..Default::default()
        };
        let response = get_demographics_questions(&db, filters, PageOptions::default()).await?;
        assert_eq!(
            response.total, 0,
            "Question should not exist after deletion"
        );

        Ok(())
    }

    // Test can create a demographics question, associate it with a conversation, and then delete it.
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_create_associate_delete_demographics_question(
        db: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let slug = "example_slug".to_string();
        let response_type = DemographicsQuestionResponseType::Number;

        // Create the demographics question
        let payload = CreateDemographicsQuestion {
            slug: slug.clone(),
            display_name: "Example Display Name".to_string(),
            response_type,
            bucket_config: None,
        };
        let question = create_demographics_question(&db, payload).await?;

        // Create a conversation
        let (app, mut session) = setup_default_app_and_session(&db).await?;
        let (status, response, _) = session.create_random_conversation(&app).await?;
        let conversation_id: Uuid = serde_json::from_value(
            response
                .get("id")
                .cloned()
                .ok_or("Failed to get conversation id")?,
        )?;

        assert!(status.is_success(), "Failed to create random conversation");

        // Ensure the question is initially absent
        let filters = DemographicsQuestionsFilterOptions {
            conversation_id: Some(conversation_id),
            ..Default::default()
        };
        let questions = get_demographics_questions(&db, filters, PageOptions::default()).await?;
        assert_eq!(
            questions.total, 0,
            "Question should not be initially associated with the conversation"
        );

        // Associate the demographics question with the conversation
        let payload = CreateConversationDemographics {
            conversation_id: conversation_id,
            question_slug: question.slug.clone(),
        };
        let response = create_conversation_demographics(&db, payload).await?;

        assert_eq!(
            conversation_id, response.conversation_id,
            "Conversation ID should match after associating demographics question"
        );
        assert_eq!(
            question.slug, response.question_slug,
            "Question slug should match after associating demographics question"
        );

        // Ensure the question is now associated with the conversation
        let filters = DemographicsQuestionsFilterOptions {
            conversation_id: Some(conversation_id),
            ..Default::default()
        };
        let questions = get_demographics_questions(&db, filters, PageOptions::default()).await?;
        assert_eq!(
            questions.total, 1,
            "Question should be associated with the conversation after creation"
        );
        assert_eq!(
            questions.records[0].slug, question.slug,
            "The associated question slug should match the created question slug"
        );

        // Delete the demographics question and ensure it is removed successfully
        let response = delete_demographics_question(&db, question.slug.clone()).await?;
        assert_eq!(
            Some(question.slug.clone()),
            response.map(|r| r.slug),
            "Deleted question slug should match the original question slug"
        );

        // Ensure the question is actually deleted
        let filters = DemographicsQuestionsFilterOptions {
            conversation_id: Some(conversation_id),
            ..Default::default()
        };
        let questions = get_demographics_questions(&db, filters, PageOptions::default()).await?;
        assert_eq!(
            questions.total, 0,
            "Question should not be associated with the conversation after deletion"
        );

        Ok(())
    }

    // Test can create, get, update and delete a demographics response.
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_create_get_update_delete_demographics_response(
        db: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (app, mut session) = setup_default_app_and_session(&db).await?;

        // Get the session user
        let (status, response, _) = session.current_user(&app).await?;
        assert!(status.is_success(), "Failed to get current user");
        let user_id = response.id;

        // Create a demographics question
        let slug = "test_question_slug".to_string();
        let response_type = DemographicsQuestionResponseType::String;
        let payload = CreateDemographicsQuestion {
            slug: slug.clone(),
            display_name: "Example Display Name".to_string(),
            response_type: response_type.clone(),
            bucket_config: None,
        };
        let question = create_demographics_question(&db, payload).await?;

        // Create a demographics response
        let response_value = "test_response".to_string();
        let payload = CreateDemographicsResponse {
            question_slug: question.slug.clone(),
            user_id,
            value: response_value.clone(),
        };
        let demographics_response = create_demographics_response(&db, payload).await?;
        assert_eq!(
            demographics_response.value, response_value,
            "Demographics response value should match the provided response"
        );

        // Get the demographics response by question slug and user ID
        let filters = DemographicsResponsesFilterOptions {
            question_slug: Some(question.slug.clone()),
            user_id: Some(user_id),
            conversation_id: None,
        };
        let fetched_responses =
            get_demographics_responses(&db, filters, PageOptions::default()).await?;
        assert_eq!(
            fetched_responses.total, 1,
            "Fetched demographics responses should not be empty"
        );
        assert_eq!(
            demographics_response, fetched_responses.records[0],
            "Fetched demographics response should match the created response"
        );

        // Update the demographics response
        let new_response_value = "updated_test_response".to_string();
        let payload = PartialDemographicsResponse {
            value: Some(new_response_value.clone()),
        };
        let updated_demographics_response =
            update_demographics_response(&db, question.slug.clone(), user_id, payload).await?;
        assert_eq!(
            new_response_value, updated_demographics_response.value,
            "Updated demographics response value should match the new response"
        );

        // Delete the demographics response
        let deleted_response =
            delete_demographics_response(&db, question.slug.clone(), user_id).await?;
        assert_eq!(
            Some(updated_demographics_response),
            deleted_response,
            "Demographics response should be successfully deleted"
        );

        // Verify the demographics response has been deleted
        let filters = DemographicsResponsesFilterOptions {
            question_slug: Some(question.slug.clone()),
            user_id: Some(user_id),
            conversation_id: None,
        };
        let fetched_responses_after_deletion =
            get_demographics_responses(&db, filters, PageOptions::default()).await?;
        assert_eq!(
            fetched_responses_after_deletion.total, 0,
            "Demographics response should be deleted"
        );

        Ok(())
    }

    // Test can create a demographics response for a specific demographics question, associated with a conversation, and list the response by question, conversation and user.
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_list_response_by_conversation_question_and_user(
        db: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (app, mut session) = setup_default_app_and_session(&db).await?;

        // Get the session user
        let (status, user_dto, _) = session.current_user(&app).await?;
        assert!(status.is_success(), "Failed to get current user");
        let user_id = user_dto.id;

        // Create a conversation
        let (status, response, _) = session.create_random_conversation(&app).await?;
        assert!(status.is_success(), "Failed to create random conversation");
        let conversation_id: Uuid = serde_json::from_value(
            response
                .get("id")
                .cloned()
                .ok_or("Failed to get conversation id")?,
        )?;

        // Create another conversation
        let (status, response, _) = session.create_random_conversation(&app).await?;
        assert!(status.is_success(), "Failed to create random conversation");
        let other_conversation_id: Uuid = serde_json::from_value(
            response
                .get("id")
                .cloned()
                .ok_or("Failed to get other conversation id")?,
        )?;

        // Create a second user for testing
        let mut other_session = UserSession::new_guest();
        let (status, other_user_dto, _) = other_session.signup_guest(&app).await?;
        assert!(status.is_success(), "Failed to create random user");
        let other_user_id: Uuid = serde_json::from_value(
            other_user_dto
                .get("id")
                .cloned()
                .flatten()
                .ok_or("Failed to get other user id")?,
        )?;

        // Create a demographics question
        let question_slug = "test_question".to_string();
        let response_type = DemographicsQuestionResponseType::String;
        let payload = CreateDemographicsQuestion {
            slug: question_slug.clone(),
            display_name: "Test Question".to_string(),
            response_type: response_type.clone(),
            bucket_config: None,
        };
        let question = create_demographics_question(&db, payload).await?;

        // Create another demographics question
        let other_question_slug = "other_test_question".to_string();
        let other_response_type = DemographicsQuestionResponseType::Number;
        let other_payload = CreateDemographicsQuestion {
            slug: other_question_slug.clone(),
            display_name: "Other Test Question".to_string(),
            response_type: other_response_type.clone(),
            bucket_config: None,
        };
        let other_question = create_demographics_question(&db, other_payload).await?;

        // Associate the demographics questions with the conversations
        let _ = create_conversation_demographics(
            &db,
            CreateConversationDemographics {
                conversation_id: conversation_id,
                question_slug: question.slug.clone(),
            },
        )
        .await?;
        let _ = create_conversation_demographics(
            &db,
            CreateConversationDemographics {
                conversation_id: conversation_id,
                question_slug: other_question.slug.clone(),
            },
        )
        .await?;
        let _ = create_conversation_demographics(
            &db,
            CreateConversationDemographics {
                conversation_id: other_conversation_id,
                question_slug: other_question.slug.clone(),
            },
        )
        .await?;

        // Create several demographics responses associated with the question and user
        let response_value = "test_response".to_string();
        let other_response_value = "15".to_string();
        let _ = create_demographics_response(
            &db,
            CreateDemographicsResponse {
                question_slug: question.slug.clone(),
                user_id,
                value: response_value.clone(),
            },
        )
        .await?;
        let _ = create_demographics_response(
            &db,
            CreateDemographicsResponse {
                question_slug: other_question.slug.clone(),
                user_id,
                value: other_response_value.clone(),
            },
        )
        .await?;
        let _ = create_demographics_response(
            &db,
            CreateDemographicsResponse {
                question_slug: question.slug.clone(),
                user_id: other_user_id,
                value: response_value.clone(),
            },
        )
        .await?;

        // List demographics responses by conversation
        let conversation_responses_filters = DemographicsResponsesFilterOptions {
            conversation_id: Some(conversation_id),
            ..Default::default()
        };
        let conversation_responses =
            get_demographics_responses(&db, conversation_responses_filters, PageOptions::default())
                .await?;
        assert_eq!(
            conversation_responses.total, 3,
            "Expected 3 demographics responses for the conversation"
        );

        let other_conversation_responses_filters = DemographicsResponsesFilterOptions {
            conversation_id: Some(other_conversation_id),
            ..Default::default()
        };
        let other_conversation_responses = get_demographics_responses(
            &db,
            other_conversation_responses_filters,
            PageOptions::default(),
        )
        .await?;
        assert_eq!(
            other_conversation_responses.total, 1,
            "Expected 1 demographics response for the other conversation"
        );

        // List demographics responses by question
        let question_responses_filters = DemographicsResponsesFilterOptions {
            question_slug: Some(question.slug.clone()),
            ..Default::default()
        };
        let question_responses =
            get_demographics_responses(&db, question_responses_filters, PageOptions::default())
                .await?;
        assert_eq!(
            question_responses.total, 2,
            "Expected 2 demographics responses for the question"
        );

        let other_question_responses_filters = DemographicsResponsesFilterOptions {
            question_slug: Some(other_question.slug.clone()),
            ..Default::default()
        };
        let other_question_responses = get_demographics_responses(
            &db,
            other_question_responses_filters,
            PageOptions::default(),
        )
        .await?;
        assert_eq!(
            other_question_responses.total, 1,
            "Expected 1 demographics response for the other question"
        );

        // List demographics responses by user
        let user_responses_filters = DemographicsResponsesFilterOptions {
            user_id: Some(user_id),
            ..Default::default()
        };
        let user_responses =
            get_demographics_responses(&db, user_responses_filters, PageOptions::default()).await?;
        assert_eq!(
            user_responses.total, 2,
            "Expected 2 demographics responses for the user"
        );

        let other_user_responses_filters = DemographicsResponsesFilterOptions {
            user_id: Some(other_user_id),
            ..Default::default()
        };
        let other_user_responses =
            get_demographics_responses(&db, other_user_responses_filters, PageOptions::default())
                .await?;
        assert_eq!(
            other_user_responses.total, 1,
            "Expected 1 demographics response for the other user"
        );

        assert_ne!(
            user_responses.total, other_user_responses.total,
            "Expected the user responses and other user responses to be different"
        );

        Ok(())
    }
}
