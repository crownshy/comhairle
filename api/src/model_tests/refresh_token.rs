//! Tests for [`crate::models::refresh_token`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use crate::models::model_test_helpers::{get_random_user_id, setup_default_app_and_session};

    #[allow(unused_imports)]
    use crate::models::error::{
        AuthError, ConversationError, DataError, EventError, InviteError, ModelError,
        PermissionError, ReportError, UserError, ValidationError, WorkflowError,
    };
    #[allow(unused_imports)]
    use crate::models::moderation_status::ModerationStatus;
    #[allow(unused_imports)]
    use crate::models::pagination::{Order, PageOptions, PaginatedResults};
    use crate::models::refresh_token::*;
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
    use chrono::{DateTime, Duration, Utc};
    #[allow(unused_imports)]
    use schemars::JsonSchema;
    #[allow(unused_imports)]
    use sqlx::PgPool;
    #[allow(unused_imports)]
    use uuid::Uuid;

    use std::error::Error;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_refresh_token(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let ip_addr = ClientIp("127.0.0.1".to_string());
        let user_agent = ClientUserAgent(Some("Mozilla/5.0 (X11; Linux x86_64)".to_string()));
        let user_id = get_random_user_id(&app, &mut session).await?;

        let now = Utc::now();

        let token = create(
            &pool,
            CreateRefreshToken {
                user_id,
                ip_addr: &ip_addr,
                user_agent: &user_agent,
                family_id: None,
                custom_expiry: None,
            },
        )
        .await?;

        assert_eq!(
            token.ip_address.unwrap(),
            ip_addr.0,
            "ip addresses don't match"
        );
        assert!(
            token.expires_at > now + Duration::days(6)
                && token.expires_at < now + Duration::days(8),
            "expires_at incorrect"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_return_none_if_no_token_present(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let result = get_by_id(&pool, Uuid::new_v4()).await?;

        assert!(result.is_none());

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_revoke_all_tokens_for_user(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let ip_addr = ClientIp("127.0.0.1".to_string());
        let user_agent = ClientUserAgent(Some("Mozilla/5.0 (X11; Linux x86_64)".to_string()));
        let user_id = get_random_user_id(&app, &mut session).await?;

        let token_a = create(
            &pool,
            CreateRefreshToken {
                user_id,
                ip_addr: &ip_addr,
                user_agent: &user_agent,
                family_id: None,
                custom_expiry: None,
            },
        )
        .await?;
        let token_b = create(
            &pool,
            CreateRefreshToken {
                user_id,
                ip_addr: &ip_addr,
                user_agent: &user_agent,
                family_id: None,
                custom_expiry: None,
            },
        )
        .await?;

        assert!(
            token_a.revoked_at.is_none(),
            "token_a revoked before update"
        );
        assert!(
            token_b.revoked_at.is_none(),
            "token_b revoked before update"
        );

        revoke_for_user(&pool, user_id, "test revoke for user").await?;

        let token_a = get_by_id(&pool, token_a.id).await?.unwrap();
        let token_b = get_by_id(&pool, token_b.id).await?.unwrap();

        assert!(
            token_a.revoked_at.is_some(),
            "token_a not revoked after update"
        );
        assert!(
            token_b.revoked_at.is_some(),
            "token_b not revoked after update"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_revoke_all_tokens_in_family(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let ip_addr = ClientIp("127.0.0.1".to_string());
        let user_agent = ClientUserAgent(Some("Mozilla/5.0 (X11; Linux x86_64)".to_string()));
        let user_id = get_random_user_id(&app, &mut session).await?;

        let unrelated_token = create(
            &pool,
            CreateRefreshToken {
                user_id,
                ip_addr: &ip_addr,
                user_agent: &user_agent,
                family_id: None,
                custom_expiry: None,
            },
        )
        .await?;
        let family_token_a = create(
            &pool,
            CreateRefreshToken {
                user_id,
                ip_addr: &ip_addr,
                user_agent: &user_agent,
                family_id: None,
                custom_expiry: None,
            },
        )
        .await?;
        let family_token_b = create(
            &pool,
            CreateRefreshToken {
                user_id,
                ip_addr: &ip_addr,
                user_agent: &user_agent,
                family_id: Some(family_token_a.family_id),
                custom_expiry: None,
            },
        )
        .await?;
        let family_token_c = create(
            &pool,
            CreateRefreshToken {
                user_id,
                ip_addr: &ip_addr,
                user_agent: &user_agent,
                family_id: Some(family_token_b.family_id),
                custom_expiry: None,
            },
        )
        .await?;

        let revoked_tokens = revoke_family(&pool, family_token_a.family_id, "test").await?;

        assert_eq!(revoked_tokens.len(), 3, "incorrect no affected tokens");
        assert!(
            revoked_tokens
                .iter()
                .all(|t| t.revoked_at.is_some() && t.revoked_reason.as_ref().unwrap() == "test"),
            "revoked columns incorrect"
        );
        assert!(
            !revoked_tokens.iter().any(|t| t.id == unrelated_token.id),
            "unrelated token updated"
        );
        assert!(
            unrelated_token.revoked_at.is_none(),
            "unrelated_token revoked"
        );
        assert!(
            revoked_tokens.iter().any(|t| t.id == family_token_a.id),
            "family_token_a missing"
        );
        assert!(
            revoked_tokens.iter().any(|t| t.id == family_token_b.id),
            "family_token_b missing"
        );
        assert!(
            revoked_tokens.iter().any(|t| t.id == family_token_c.id),
            "family_token_c missing"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_rotate_valid_token(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let ip_addr = ClientIp("127.0.0.1".to_string());
        let user_agent = ClientUserAgent(Some("Mozilla/5.0 (X11; Linux x86_64)".to_string()));
        let user_id = get_random_user_id(&app, &mut session).await?;

        let original_token = create(
            &pool,
            CreateRefreshToken {
                user_id,
                ip_addr: &ip_addr,
                user_agent: &user_agent,
                family_id: None,
                custom_expiry: None,
            },
        )
        .await?;

        let rotation_token =
            rotate(&pool, original_token.id, user_id, &ip_addr, &user_agent).await?;

        let original_token = get_by_id(&pool, original_token.id).await?.unwrap();

        assert_eq!(
            rotation_token.family_id, original_token.family_id,
            "family_id mismatch"
        );
        assert!(rotation_token.revoked_at.is_none(), "new token is revoked");
        assert!(
            original_token.revoked_at.is_some(),
            "original token not revoked"
        );
        assert_eq!(
            original_token.revoked_reason.unwrap(),
            "rotated",
            "incorrect revoked_reason"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_detect_reuse_of_revoked_token(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let ip_addr = ClientIp("127.0.0.1".to_string());
        let user_agent = ClientUserAgent(Some("Mozilla/5.0 (X11; Linux x86_64)".to_string()));
        let user_id = get_random_user_id(&app, &mut session).await?;

        let original_token = create(
            &pool,
            CreateRefreshToken {
                user_id,
                ip_addr: &ip_addr,
                user_agent: &user_agent,
                family_id: None,
                custom_expiry: None,
            },
        )
        .await?;
        revoke_for_user(&pool, user_id, "test reuse detection").await?;

        let err = rotate(&pool, original_token.id, user_id, &ip_addr, &user_agent)
            .await
            .unwrap_err();

        assert!(
            matches!(
                err,
                AuthError::SessionRefreshFailure(RefreshFailure::ReuseDetected)
            ),
            "incorrect error type"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_detect_user_mismatch(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let ip_addr = ClientIp("127.0.0.1".to_string());
        let user_agent = ClientUserAgent(Some("Mozilla/5.0 (X11; Linux x86_64)".to_string()));
        let user_a_id = get_random_user_id(&app, &mut session).await?;
        let user_b_id = get_random_user_id(&app, &mut session).await?;

        let original_token = create(
            &pool,
            CreateRefreshToken {
                user_id: user_a_id,
                ip_addr: &ip_addr,
                user_agent: &user_agent,
                family_id: None,
                custom_expiry: None,
            },
        )
        .await?;

        let err = rotate(&pool, original_token.id, user_b_id, &ip_addr, &user_agent)
            .await
            .unwrap_err();

        assert!(
            matches!(
                err,
                AuthError::SessionRefreshFailure(RefreshFailure::OwnershipMismatch)
            ),
            "incorrect error type"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_return_not_found_error(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let ip_addr = ClientIp("127.0.0.1".to_string());
        let user_agent = ClientUserAgent(Some("Mozilla/5.0 (X11; Linux x86_64)".to_string()));
        let user_id = get_random_user_id(&app, &mut session).await?;

        let err = rotate(&pool, Uuid::new_v4(), user_id, &ip_addr, &user_agent)
            .await
            .unwrap_err();

        assert!(
            matches!(
                err,
                AuthError::SessionRefreshFailure(RefreshFailure::NotFound)
            ),
            "incorrect error type"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_detect_expired_token(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let ip_addr = ClientIp("127.0.0.1".to_string());
        let user_agent = ClientUserAgent(Some("Mozilla/5.0 (X11; Linux x86_64)".to_string()));
        let user_id = get_random_user_id(&app, &mut session).await?;

        let token = create(
            &pool,
            CreateRefreshToken {
                user_id,
                ip_addr: &ip_addr,
                user_agent: &user_agent,
                family_id: None,
                custom_expiry: Some(Utc::now() - Duration::hours(1)),
            },
        )
        .await?;

        let err = rotate(&pool, token.id, user_id, &ip_addr, &user_agent)
            .await
            .unwrap_err();

        assert!(
            matches!(
                err,
                AuthError::SessionRefreshFailure(RefreshFailure::Expired)
            ),
            "incorrect error type"
        );

        Ok(())
    }
}
