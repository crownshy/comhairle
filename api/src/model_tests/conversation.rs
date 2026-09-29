//! Tests for [`crate::models::conversation`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use fake::{Fake, Faker};
    use serde_json::json;
    use std::sync::Arc;

    use crate::models::model_test_helpers::setup_default_app_and_session;
    use crate::models::permissions::{GrantRoleRequest, Role, UserOrOrganizationId};
    use crate::models::users::{self, UpdateUserRequest, create_user, update_user};
    use crate::routes::auth::SignupRequest;
    use crate::routes::conversations::dto::ConversationDto;
    use crate::routes::organizations::dto::OrganizationDto;
    use crate::services::permissions::grant_role;
    use crate::setup_server;
    use crate::test_helpers::{UserSession, test_state};

    use crate::models::conversation::*;
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
    use crate::models::{proposal_section, user_progress};
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
    async fn should_create_conversation_with_oranganization_id(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool.clone()).call()?;
        let app = setup_server(Arc::new(state.clone())).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let (_, response, _) = admin_session.create_random_organization(&app).await?;
        let organization: OrganizationDto = serde_json::from_value(response)?;
        let user = create_user(
            &SignupRequest {
                username: "test_user".to_string(),
                password: "test_pw".to_string(),
                email: "test_email".to_string(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;
        let user = update_user(
            &user.id,
            &UpdateUserRequest {
                organization_id: Some(organization.id),
                ..Default::default()
            },
            &pool,
        )
        .await?;

        let conversation = crate::services::conversation::create(
            &pool,
            &state.bot_service,
            &state.config,
            &Faker.fake(),
            user.id,
            user.organization_id,
        )
        .await?;

        assert!(
            conversation.organization_id.is_some(),
            "incorrect organization_id"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_filter_conversations_by_case_insensitive_keyword(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let _ = session
            .create_conversation(
                &app,
                json! ({
                    "title" : "A title about the moon",
                    "short_description" : "Scotlands ambitions to leave the planet",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : true,
                    "is_live": true,
                    "is_invite_only" : false,
                    "slug" : "moon_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;
        let _ = session
            .create_conversation(
                &app,
                json! ({
                    "title" : "A conversation about golf",
                    "short_description" : "LIV vs the PGA",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : true,
                    "is_live": true,
                    "is_invite_only" : false,
                    "slug" : "golf_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;
        let _ = session
            .create_conversation(
                &app,
                json! ({
                    "title" : "A conversation about AI",
                    "short_description" : "Some text about artificial intelligence",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : true,
                    "is_live": true,
                    "is_invite_only" : false,
                    "slug" : "ai_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;

        let filter_options_1 = ConversationFilterOptions {
            keyword: Some("moon".to_string()),
            ..Default::default()
        };
        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let order_options = ConversationOrderOptions {
            ..Default::default()
        };

        let results_1 = list(
            &pool,
            page_options.clone(),
            order_options,
            filter_options_1,
            Some("en".to_string()),
        )
        .await?;

        assert_eq!(results_1.total, 1, "incorrect first total");
        assert_eq!(
            results_1.records[0].title,
            "A title about the moon".to_string(),
            "incorrect first title"
        );

        let filter_options_2 = ConversationFilterOptions {
            keyword: Some("liv".to_string()),
            ..Default::default()
        };
        let order_options = ConversationOrderOptions {
            ..Default::default()
        };
        let results_2 = list(
            &pool,
            page_options.clone(),
            order_options,
            filter_options_2,
            Some("en".to_string()),
        )
        .await?;

        assert_eq!(results_2.total, 1, "incorrect second total");
        assert_eq!(
            results_2.records[0].title,
            "A conversation about golf".to_string(),
            "incorrect second title"
        );

        let filter_options_3 = ConversationFilterOptions {
            keyword: Some("intelligence".to_string()),
            ..Default::default()
        };
        let order_options = ConversationOrderOptions {
            ..Default::default()
        };
        let results_3 = list(
            &pool,
            page_options.clone(),
            order_options,
            filter_options_3,
            Some("en".to_string()),
        )
        .await?;

        assert_eq!(results_3.total, 1, "incorrect third total");
        assert_eq!(
            results_3.records[0].title,
            "A conversation about AI".to_string(),
            "incorrect third title"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_conversations_by_organization_id(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, response, _) = session.create_random_organization(&app).await?;
        let organization: OrganizationDto = serde_json::from_value(response)?;

        // Before user has organization_id
        let _ = session.create_random_conversation(&app).await?;
        let _ = session.create_random_conversation(&app).await?;
        let _ = session.create_random_conversation(&app).await?;

        let _ = session
            .put(
                &app,
                "/user/details",
                json!({ "organization_id": organization.id })
                    .to_string()
                    .into(),
            )
            .await?;

        // After user has organization_id
        let (_, response_1, _) = session.create_random_conversation(&app).await?;
        let (_, response_2, _) = session.create_random_conversation(&app).await?;
        let (_, response_3, _) = session.create_random_conversation(&app).await?;
        let conversation_1: ConversationDto = serde_json::from_value(response_1)?;
        let conversation_2: ConversationDto = serde_json::from_value(response_2)?;
        let conversation_3: ConversationDto = serde_json::from_value(response_3)?;

        let page_options = PageOptions {
            limit: None,
            offset: None,
        };
        let order_options = ConversationOrderOptions {
            created_at: Some(Order::Asc),
            title: None,
        };
        let filter_options = ConversationFilterOptions {
            organization_id: Some(organization.id),
            ..Default::default()
        };
        let results = list(
            &pool,
            page_options,
            order_options,
            filter_options,
            Some("en".to_string()),
        )
        .await?;

        assert_eq!(results.total, 3, "incorrect total filtered conversations");
        assert_eq!(
            results.records[0].id, conversation_1.id,
            "incorrect first id"
        );
        assert_eq!(
            results.records[1].id, conversation_2.id,
            "incorrect second id"
        );
        assert_eq!(
            results.records[2].id, conversation_3.id,
            "incorrect third id"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_conversations_for_permitted_user(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let (app, mut session) = setup_default_app_and_session(&state.db).await?;

        let (_, value, _) = session.create_random_conversation(&app).await?;
        let conversation: ConversationDto = serde_json::from_value(value)?;

        let user_a = users::create_guest_user(&state.db).await?;
        let user_b = users::create_guest_user(&state.db).await?;

        let grant_request_a_a = GrantRoleRequest {
            actor_id: UserOrOrganizationId::User(user_a.id),
            permission_triplet: Role::ConversationContentEditor.triplet(&conversation.id),
            granted_by: &session.id.unwrap(),
            grant_reason: "Testing",
        };
        grant_role(&state, grant_request_a_a).await?;

        let grant_request_a_b = GrantRoleRequest {
            actor_id: UserOrOrganizationId::User(user_a.id),
            permission_triplet: Role::Tester.triplet(&conversation.id),
            granted_by: &session.id.unwrap(),
            grant_reason: "Testing",
        };
        grant_role(&state, grant_request_a_b).await?;

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let filter_options = ConversationFilterOptions {
            ..Default::default()
        };
        let order_options = ConversationOrderOptions {
            ..Default::default()
        };

        let results_user_a_a = list_for_permitted_user(
            &state.db,
            user_a.id,
            None,
            false,
            page_options.clone(),
            order_options,
            filter_options,
            Some("en".to_string()),
        )
        .await?;

        let filter_options = ConversationFilterOptions {
            ..Default::default()
        };
        let order_options = ConversationOrderOptions {
            ..Default::default()
        };
        let results_user_b_a = list_for_permitted_user(
            &state.db,
            user_b.id,
            None,
            false,
            page_options.clone(),
            order_options,
            filter_options,
            Some("en".to_string()),
        )
        .await?;

        assert_eq!(
            results_user_a_a.total, 1,
            "incorrect permitted conversation total for user_a"
        );
        assert_eq!(
            results_user_b_a.total, 0,
            "incorrect permitted conversation total for user_b"
        );

        let filter_options = ConversationFilterOptions {
            ..Default::default()
        };
        let order_options = ConversationOrderOptions {
            ..Default::default()
        };
        let results_owner = list_for_permitted_user(
            &state.db,
            session.id.unwrap(),
            None,
            false,
            page_options.clone(),
            order_options,
            filter_options,
            Some("en".to_string()),
        )
        .await?;

        let filter_options = ConversationFilterOptions {
            ..Default::default()
        };
        let order_options = ConversationOrderOptions {
            ..Default::default()
        };
        let results_user_b_b = list_for_permitted_user(
            &state.db,
            user_b.id,
            None,
            false,
            page_options.clone(),
            order_options,
            filter_options,
            Some("en".to_string()),
        )
        .await?;

        assert_eq!(
            results_owner.total, 1,
            "incorrect permitted conversation total for owner"
        );
        assert_eq!(
            results_user_b_b.total, 0,
            "incorrect permitted conversation total for user_b"
        );

        Ok(())
    }
}
