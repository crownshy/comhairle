//! Tests for [`crate::models::scheduled_email`], kept in the api crate because they
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
    use crate::models::scheduled_email::*;
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
    async fn should_create_scheduled_email(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let params = CreateScheduledEmail {
            user_email: "test@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::days(1),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_id: Uuid::new_v4(),
                    recipient_id: Uuid::new_v4(),
                    owner_id: Uuid::new_v4(),
                    locale: "en".to_string(),
                },
            },
        };

        let email = create(&pool, params).await?;

        assert_eq!(
            email.user_email,
            "test@test.com".to_string(),
            "incorrect user_email"
        );
        assert!(email.send_at > Utc::now(), "send_at not in the future");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_scheduled_email(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let params = CreateScheduledEmail {
            user_email: "test@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::days(1),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_id: Uuid::new_v4(),
                    recipient_id: Uuid::new_v4(),
                    owner_id: Uuid::new_v4(),
                    locale: "en".to_string(),
                },
            },
        };

        let email = create(&pool, params).await?;

        assert_eq!(
            email.status,
            EmailStatus::Pending,
            "incorrect status before update"
        );

        let update_params = UpdateScheduledEmail {
            status: Some(EmailStatus::Sent),
        };
        let email = update(&pool, email.id, update_params).await?;

        assert_eq!(
            email.status,
            EmailStatus::Sent,
            "incorrect status after update"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_scheduled_email_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let params = CreateScheduledEmail {
            user_email: "test@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::days(1),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_id: Uuid::new_v4(),
                    recipient_id: Uuid::new_v4(),
                    owner_id: Uuid::new_v4(),
                    locale: "en".to_string(),
                },
            },
        };

        let created_email = create(&pool, params).await?;

        let email = get_by_id(&pool, created_email.id).await?;

        assert_eq!(
            email.user_email,
            "test@test.com".to_string(),
            "incorrect user_email"
        );
        assert!(email.send_at > Utc::now(), "send_at not in the future");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_scheduled_emails(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let params_1 = CreateScheduledEmail {
            user_email: "user-1@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::days(1),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_id: Uuid::new_v4(),
                    recipient_id: Uuid::new_v4(),
                    owner_id: Uuid::new_v4(),
                    locale: "en".to_string(),
                },
            },
        };
        let params_2 = CreateScheduledEmail {
            user_email: "user-2@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::days(2),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_id: Uuid::new_v4(),
                    recipient_id: Uuid::new_v4(),
                    owner_id: Uuid::new_v4(),
                    locale: "en".to_string(),
                },
            },
        };
        let params_3 = CreateScheduledEmail {
            user_email: "user-1@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::days(2),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_id: Uuid::new_v4(),
                    recipient_id: Uuid::new_v4(),
                    owner_id: Uuid::new_v4(),
                    locale: "en".to_string(),
                },
            },
        };

        let email_1 = create(&pool, params_1).await?;
        create(&pool, params_2).await?;
        let email_3 = create(&pool, params_3).await?;

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let filter_options = ScheduledEmailFilterOptions {
            user_email: Some("user-1@test.com".to_string()),
        };
        let order_options = ScheduledEmailOrderOptions {
            ..Default::default()
        };
        let results = list(&pool, page_options, filter_options, order_options).await?;

        assert_eq!(results.total, 2, "incorrect total");
        assert!(
            results.records.iter().any(|e| e.id == email_1.id),
            "missing first email"
        );
        assert!(
            results.records.iter().any(|e| e.id == email_3.id),
            "missing third email"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_scheduled_emails_upcoming_2_hours(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let params_23_hours = CreateScheduledEmail {
            user_email: "user-1@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::hours(23),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_id: Uuid::new_v4(),
                    recipient_id: Uuid::new_v4(),
                    owner_id: Uuid::new_v4(),
                    locale: "en".to_string(),
                },
            },
        };
        let params_2_days = CreateScheduledEmail {
            user_email: "user-2@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::days(2),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_id: Uuid::new_v4(),
                    recipient_id: Uuid::new_v4(),
                    owner_id: Uuid::new_v4(),
                    locale: "en".to_string(),
                },
            },
        };
        let params_past = CreateScheduledEmail {
            user_email: "user-1@test.com".to_string(),
            send_at: Utc::now() - chrono::Duration::days(1),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_id: Uuid::new_v4(),
                    recipient_id: Uuid::new_v4(),
                    owner_id: Uuid::new_v4(),
                    locale: "en".to_string(),
                },
            },
        };
        let params_sent = CreateScheduledEmail {
            user_email: "user-1@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::minutes(15),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_id: Uuid::new_v4(),
                    recipient_id: Uuid::new_v4(),
                    owner_id: Uuid::new_v4(),
                    locale: "en".to_string(),
                },
            },
        };
        let params_30_mins = CreateScheduledEmail {
            user_email: "user-1@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::minutes(30),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_id: Uuid::new_v4(),
                    recipient_id: Uuid::new_v4(),
                    owner_id: Uuid::new_v4(),
                    locale: "en".to_string(),
                },
            },
        };

        create(&pool, params_23_hours).await?;
        create(&pool, params_2_days).await?;
        create(&pool, params_past).await?;
        let email_sent = create(&pool, params_sent).await?;
        email_sent.sent(&pool).await?;

        let email_30_mins = create(&pool, params_30_mins).await?;

        let results_2_hours =
            list_upcoming_scheduled_emails(&pool, chrono::Duration::hours(2)).await?;

        assert_eq!(results_2_hours.len(), 1, "incorrect total for 2 hours");
        assert!(
            results_2_hours.iter().all(|e| e.id == email_30_mins.id),
            "incorrect id"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_scheduled_email_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let params = CreateScheduledEmail {
            user_email: "test@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::days(1),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_id: Uuid::new_v4(),
                    recipient_id: Uuid::new_v4(),
                    owner_id: Uuid::new_v4(),
                    locale: "en".to_string(),
                },
            },
        };

        let created_email = create(&pool, params).await?;

        delete(&pool, created_email.id).await?;

        let err = get_by_id(&pool, created_email.id).await.unwrap_err();

        match err {
            ModelError::Data(DataError::ResourceNotFound(e)) => {
                assert_eq!(e, "Scheduled Email".to_string(), "incorrect error message");
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
