//! Tests for [`crate::models::email_template_config`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use crate::models::error::DataError;
    use crate::models::model_test_helpers::setup_default_app_and_session;

    use crate::models::email_template_config::*;
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
    async fn should_create_email_config(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, current_user, _) = session.current_user(&app).await?;

        let params = CreateEmailTemplateConfig {
            slots: EmailTemplateSlots::ConversationInvite(DefaultEmailSlots {
                heading: "<h1>You're invite to a conversation</h1>".to_string(),
                intro: "<p>You have been selected to take part in a public engagement</p>"
                    .to_string(),
                body: "<p>Test body content</p>".to_string(),
                footer: "<p>Thank you for your time</p>".to_string(),
            }),
            subject: None,
        };

        let email_config = create(&pool, current_user.id, &params).await?;

        assert_eq!(
            email_config.owner_id, current_user.id,
            "owner_id doesn't match"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_email_config(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, current_user, _) = session.current_user(&app).await?;

        let create_slots = EmailTemplateSlots::ConversationInvite(DefaultEmailSlots {
            heading: "<h1>You're invite to a conversation</h1>".to_string(),
            intro: "<p>You have been selected to take part in a public engagement</p>".to_string(),
            body: "<p>Test body content</p>".to_string(),
            footer: "<p>Thank you for your time</p>".to_string(),
        });

        let params = CreateEmailTemplateConfig {
            slots: create_slots.clone(),
            subject: None,
        };

        let new_email_config = create(&pool, current_user.id, &params).await?;

        assert_eq!(
            new_email_config.slots, create_slots,
            "incorrect slots before update"
        );

        let update_slots = EmailTemplateSlots::ConversationInvite(DefaultEmailSlots {
            heading: "<h1>You're not invite to a conversation</h1>".to_string(),
            intro: "<p>You have not been selected to take part in a public engagement</p>"
                .to_string(),
            body: "<p>Test body content</p>".to_string(),
            footer: "<p>Thank you for your time</p>".to_string(),
        });

        let email_config = update(
            &pool,
            new_email_config.id,
            &UpdateEmailTemplateConfig {
                slots: Some(update_slots.clone()),
                ..Default::default()
            },
        )
        .await?;

        assert_eq!(
            email_config.slots, update_slots,
            "incorrect slots after update"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_email_config_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, current_user, _) = session.current_user(&app).await?;

        let params = CreateEmailTemplateConfig {
            slots: EmailTemplateSlots::ConversationInvite(DefaultEmailSlots {
                heading: "<h1>You're invite to a conversation</h1>".to_string(),
                intro: "<p>You have been selected to take part in a public engagement</p>"
                    .to_string(),
                body: "<p>Test body content</p>".to_string(),
                footer: "<p>Thank you for your time</p>".to_string(),
            }),
            subject: None,
        };

        let new_email_config = create(&pool, current_user.id, &params).await?;

        let email_config = get_by_id(&pool, new_email_config.id).await?;

        assert_eq!(new_email_config.id, email_config.id, "ids don't match");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_optionally_get_email_config_user_and_email_type(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, current_user, _) = session.current_user(&app).await?;

        let params = CreateEmailTemplateConfig {
            slots: EmailTemplateSlots::ConversationInvite(DefaultEmailSlots {
                heading: "<h1>You're invite to a conversation</h1>".to_string(),
                intro: "<p>You have been selected to take part in a public engagement</p>"
                    .to_string(),
                body: "<p>Test body content</p>".to_string(),
                footer: "<p>Thank you for your time</p>".to_string(),
            }),
            subject: None,
        };

        create(&pool, current_user.id, &params).await?;

        let email_config = get_by_type_user(
            &pool,
            current_user.id,
            &SCHEMA_CONVERSATION_INVITE.email_type,
        )
        .await?;

        assert!(email_config.is_some(), "existing config not found");

        let email_config = get_by_type_user(
            &pool,
            Uuid::new_v4(),
            &SCHEMA_CONVERSATION_INVITE.email_type,
        )
        .await?;

        assert!(email_config.is_none(), "random user config found");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_email_configs(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, user, _) = session.current_user(&app).await?;

        let default_slots = DefaultEmailSlots {
            heading: "<h1>Test heading</h1>".to_string(),
            intro: "<p>Test intro</p>".to_string(),
            body: "<p>Test body</p>".to_string(),
            footer: "<p>Test footer</p>".to_string(),
        };

        let params_a = CreateEmailTemplateConfig {
            slots: EmailTemplateSlots::EventRegistrationConfirmation(default_slots.clone()),
            subject: None,
        };
        create(&pool, user.id, &params_a).await?;
        let params_b = CreateEmailTemplateConfig {
            slots: EmailTemplateSlots::ConversationInvite(default_slots.clone()),
            subject: None,
        };
        create(&pool, user.id, &params_b).await?;

        let filter_options = EmailTemplateConfigFilterOptions {
            email_type: Some(EmailType::EventRegistrationConfirmation),
        };
        let email_configs = list(&pool, &user.id, filter_options).await?;

        assert_eq!(email_configs.len(), 1, "incorrect total");
        assert!(
            !email_configs
                .iter()
                .any(|c| c.email_type == EmailType::ConversationInvite),
            "incorrectly email_type included"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_email_config(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, current_user, _) = session.current_user(&app).await?;

        let params = CreateEmailTemplateConfig {
            slots: EmailTemplateSlots::ConversationInvite(DefaultEmailSlots {
                heading: "<h1>You're invite to a conversation</h1>".to_string(),
                intro: "<p>You have been selected to take part in a public engagement</p>"
                    .to_string(),
                body: "<p>Test body content</p>".to_string(),
                footer: "<p>Thank you for your time</p>".to_string(),
            }),
            subject: None,
        };

        let email_config = create(&pool, current_user.id, &params).await?;

        delete(&pool, email_config.id).await?;

        let err = get_by_id(&pool, email_config.id).await.unwrap_err();

        match err {
            ModelError::Data(DataError::ResourceNotFound(e)) => {
                assert_eq!(
                    e,
                    "Email Template Config".to_string(),
                    "incorrect error message"
                );
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
