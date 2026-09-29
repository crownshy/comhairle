//! Tests for [`crate::models::otp`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    #[allow(unused_imports)]
    use crate::models::error::{
        AuthError, ConversationError, DataError, EventError, InviteError, ModelError,
        PermissionError, ReportError, UserError, ValidationError, WorkflowError,
    };
    #[allow(unused_imports)]
    use crate::models::moderation_status::ModerationStatus;
    use crate::models::otp::*;
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
    use chrono::{DateTime, Duration, Utc};
    #[allow(unused_imports)]
    use schemars::JsonSchema;
    #[allow(unused_imports)]
    use sqlx::PgPool;
    #[allow(unused_imports)]
    use uuid::Uuid;

    use crate::routes::auth::OtpSignupRequest;

    use std::error::Error;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_otp_for_user_with_expiry(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = users::create_otp_user(
            &OtpSignupRequest {
                email: "test@user.com".to_string(),
                username: None,
            },
            &pool,
        )
        .await?;

        let before = Utc::now();
        let otp = create(&pool, &user.id, None, None).await?;
        let after = Utc::now();

        let expiry_duration = Duration::minutes(10);
        let lower = before + expiry_duration;
        let upper = after + expiry_duration;

        assert_eq!(otp.user_id, user.id, "incorrect user_id");
        assert_eq!(otp.status, OtpStatus::Pending, "incorrect default status");
        assert!(otp.expires_at >= lower);
        assert!(otp.expires_at <= upper);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_mark_existing_otps_as_error_when_new_otp_created(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let user = users::create_otp_user(
            &OtpSignupRequest {
                email: "test@user.com".to_string(),
                username: None,
            },
            &pool,
        )
        .await?;

        let first = create(&pool, &user.id, None, None).await?;
        let second = create(&pool, &user.id, None, None).await?;
        let third = create(&pool, &user.id, None, None).await?;

        let first = get_by_id(&pool, &first.id).await?;
        let second = get_by_id(&pool, &second.id).await?;

        assert_eq!(first.status, OtpStatus::Error, "incorrect first status");
        assert_eq!(second.status, OtpStatus::Error, "incorrect second status");
        assert_eq!(third.status, OtpStatus::Pending, "incorrect third status");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_fail_otp_create_for_guest_users(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = users::create_guest_user(&pool).await?;

        let error = create(&pool, &user.id, None, None).await.unwrap_err();

        match error {
            ModelError::User(UserError::WrongUserType) => return Ok(()),
            _ => panic!("Wrong error type"),
        }
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_otp_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = users::create_otp_user(
            &OtpSignupRequest {
                email: "test@user.com".to_string(),
                username: None,
            },
            &pool,
        )
        .await?;

        let new_otp = create(&pool, &user.id, None, None).await?;

        let otp = get_by_id(&pool, &new_otp.id).await?;

        assert_eq!(otp.id, new_otp.id, "ids don't match");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_otp_by_user_code(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = users::create_otp_user(
            &OtpSignupRequest {
                email: "test@user.com".to_string(),
                username: None,
            },
            &pool,
        )
        .await?;

        let new_otp = create(&pool, &user.id, None, None).await?;

        let now = Utc::now();
        let otp = accept(&pool, &user.id, &new_otp.code, now).await?;

        assert_eq!(otp.user_id, user.id, "incorrect user_id");
        assert_eq!(otp.code, new_otp.code, "incorrect code");
        assert_eq!(otp.status, OtpStatus::Accepted, "incorrect status");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_return_error_for_expired_otps(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = users::create_otp_user(
            &OtpSignupRequest {
                email: "test@user.com".to_string(),
                username: None,
            },
            &pool,
        )
        .await?;

        let new_otp = create(&pool, &user.id, None, None).await?;

        let now = Utc::now() + Duration::minutes(11);
        let error = accept(&pool, &user.id, &new_otp.code, now)
            .await
            .unwrap_err();

        match error {
            ModelError::Data(DataError::ResourceNotFound(e)) => {
                assert_eq!(
                    e,
                    "One Time Passcode".to_string(),
                    "incorrect error message"
                );
            }
            _ => panic!("Wrong error type"),
        }

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_return_error_if_user_or_code_incorrect(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let user = users::create_otp_user(
            &OtpSignupRequest {
                email: "test@user.com".to_string(),
                username: None,
            },
            &pool,
        )
        .await?;

        let new_otp = create(&pool, &user.id, None, None).await?;

        let now = Utc::now();
        let first_error = accept(&pool, &Uuid::new_v4(), &new_otp.code, now)
            .await
            .unwrap_err();
        let second_error = accept(&pool, &user.id, "wrong code", now)
            .await
            .unwrap_err();

        match (first_error, second_error) {
            (
                ModelError::Data(DataError::ResourceNotFound(first)),
                ModelError::Data(DataError::ResourceNotFound(second)),
            ) => {
                let message = "One Time Passcode".to_string();
                assert_eq!(first, message, "incorrect error message");
                assert_eq!(second, message, "incorrect error message")
            }
            _ => panic!("Wrong error type"),
        }

        Ok(())
    }
}
