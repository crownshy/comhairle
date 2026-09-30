//! Tests for [`crate::models::users`], kept in the api crate because they
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
    #[allow(unused_imports)]
    use crate::models::user_progress::ProgressStatus;
    #[allow(unused_imports)]
    use crate::models::users::User;
    use crate::models::users::*;
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
        models::{
            model_test_helpers::setup_default_app_and_session,
            users::{Resource, Role, add_user_resource_role, create_user, user_has_resource_role},
        },
        routes::{auth::SignupRequest, organizations::dto::OrganizationDto},
        setup_server,
        test_helpers::{UserSession, test_state},
    };
    use std::error::Error;
    use std::sync::Arc;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn should_create_otp_user(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = create_otp_user(
            &OtpSignupRequest {
                email: "test_otp@test.com".to_string(),
                username: None,
            },
            &pool,
        )
        .await?;

        assert_eq!(user.auth_type, UserAuthType::Otp, "incorrect auth_type");
        assert_eq!(
            user.email,
            Some("test_otp@test.com".to_string()),
            "incorrect email"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_record_and_hide_signup_ip(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = create_user(
            &SignupRequest {
                username: "ip_user".to_string(),
                password: "test_pw".to_string(),
                email: "ip_user@test.com".to_string(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;

        assert!(user.signup_ip.is_none(), "signup_ip unset before recording");
        assert!(
            user.signup_user_agent.is_none(),
            "signup_user_agent unset before recording"
        );

        set_signup_metadata(
            &user.id,
            "203.0.113.7",
            Some("Mozilla/5.0 (Test) Firefox/152.0"),
            &pool,
        )
        .await?;

        let stored = get_user_by_id(&user.id, &pool).await?;
        assert_eq!(
            stored.signup_ip.as_deref(),
            Some("203.0.113.7"),
            "signup_ip should be persisted"
        );
        assert_eq!(
            stored.signup_user_agent.as_deref(),
            Some("Mozilla/5.0 (Test) Firefox/152.0"),
            "signup_user_agent should be persisted"
        );

        // The IP and browser signature must never leak through API serialization.
        let json = serde_json::to_value(&stored)?;
        assert!(
            json.get("signup_ip").is_none(),
            "signup_ip must not be serialized"
        );
        assert!(
            json.get("signup_user_agent").is_none(),
            "signup_user_agent must not be serialized"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn user_has_resource_role_tests(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool.clone()).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (status, conversation, _) = session
            .create_conversation(
                &app,
                serde_json::json! ({
                    "title" : "Test conversation",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : false,
                    "is_live" : true,
                    "is_invite_only" : false,
                    "primary_locale" : "en",
                    "supported_languages" : ["en"],
                    "slug" : "new_conversation"
                }),
            )
            .await?;
        assert_eq!(status, 201, "should be able to create a conversation");
        let conversation_id = Uuid::parse_str(conversation["id"].as_str().unwrap())?;

        let mut session = UserSession::new(
            "test_user",
            crate::test_helpers::TEST_PASSWORD,
            "test.user@gmail.com",
        );
        session.signup(&app).await?;

        add_user_resource_role(
            Resource::Conversation,
            &conversation_id,
            Role::Contributor,
            &session.id.unwrap(),
            &pool,
        )
        .await?;

        assert!(
            user_has_resource_role(
                Resource::Conversation,
                &conversation_id,
                &[Role::Contributor],
                &session.id.unwrap(),
                &pool.clone(),
            )
            .await?,
            "true when user has role",
        );
        assert!(
            !user_has_resource_role(
                Resource::Conversation,
                &conversation_id,
                &[Role::Contributor],
                &Uuid::parse_str("5FDFC2CE-C7F5-43DB-AA1F-0A8698E76D2E").unwrap(),
                &pool.clone(),
            )
            .await?,
            "false when no user with that ID",
        );
        assert!(
            !user_has_resource_role(
                Resource::Conversation,
                &Uuid::parse_str("5FDFC2CE-C7F5-43DB-AA1F-0A8698E76D2E").unwrap(),
                &[Role::Contributor],
                &session.id.unwrap(),
                &pool.clone(),
            )
            .await?,
            "false when no conversation with that ID",
        );
        assert!(
            !user_has_resource_role(
                Resource::Conversation,
                &conversation_id,
                &[Role::Owner],
                &session.id.unwrap(),
                &pool.clone(),
            )
            .await?,
            "false when wrong role kind",
        );
        assert!(
            user_has_resource_role(
                Resource::Conversation,
                &conversation_id,
                &[Role::Owner, Role::Contributor],
                &session.id.unwrap(),
                &pool.clone(),
            )
            .await?,
            "true when user could be multiple roles and has one",
        );

        add_user_resource_role(
            Resource::Conversation,
            &conversation_id,
            Role::Translator,
            &session.id.unwrap(),
            &pool,
        )
        .await?;

        assert!(
            user_has_resource_role(
                Resource::Conversation,
                &conversation_id,
                &[Role::Translator],
                &session.id.unwrap(),
                &pool.clone(),
            )
            .await?,
            "true when user has multiple roles and one is required",
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_user_with_organization(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut admin_session) = setup_default_app_and_session(&pool).await?;
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

        assert!(
            user.organization_id.is_none(),
            "incorrect organization id before update"
        );

        let updated_user = update_user(
            &user.id,
            &UpdateUserRequest {
                organization_id: Some(organization.id),
                ..Default::default()
            },
            &pool,
        )
        .await?;

        assert!(
            updated_user.organization_id.is_some(),
            "incorrect organization id after update"
        );

        Ok(())
    }
}
