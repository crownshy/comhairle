pub mod dto;

use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with, put_with},
    ApiRouter,
};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumCount;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    models::email_template_config::{
        self, CreateEmailTemplateConfig, EmailTemplateConfigFilterOptions, EmailTemplateSlots,
        EmailTypeSchema, UpdateEmailTemplateConfig,
    },
    routes::{auth::RequiredAdminUser, email_template_configs::dto::EmailTemplateConfigDto},
    ComhairleError, ComhairleState,
};

#[instrument(err(Debug), skip(state))]
async fn create(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Json(payload): Json<CreateEmailTemplateConfig>,
) -> Result<(StatusCode, Json<EmailTemplateConfigDto>), ComhairleError> {
    let email_config = email_template_config::create(&state.db, user.id, &payload).await?;

    Ok((StatusCode::CREATED, Json(email_config.into())))
}

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Path(email_config_id): Path<Uuid>,
) -> Result<(StatusCode, Json<EmailTemplateConfigDto>), ComhairleError> {
    let email_config = email_template_config::get_by_id(&state.db, email_config_id).await?;

    Ok((StatusCode::OK, Json(email_config.into())))
}

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Query(filter_options): Query<EmailTemplateConfigFilterOptions>,
) -> Result<(StatusCode, Json<Vec<EmailTemplateConfigDto>>), ComhairleError> {
    let email_configs = email_template_config::list(&state.db, &user.id, filter_options)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok((StatusCode::OK, Json(email_configs)))
}

#[instrument(err(Debug), skip(state))]
async fn update(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Path(email_config_id): Path<Uuid>,
    Json(payload): Json<UpdateEmailTemplateConfig>,
) -> Result<(StatusCode, Json<EmailTemplateConfigDto>), ComhairleError> {
    let email_config = email_template_config::update(&state.db, email_config_id, &payload).await?;

    Ok((StatusCode::OK, Json(email_config.into())))
}

#[instrument(err(Debug), skip(state))]
async fn delete(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Path(email_config_id): Path<Uuid>,
) -> Result<(StatusCode, Json<EmailTemplateConfigDto>), ComhairleError> {
    let email_config = email_template_config::delete(&state.db, email_config_id).await?;

    Ok((StatusCode::OK, Json(email_config.into())))
}

#[instrument(err(Debug), skip(state))]
async fn get_schema(
    State(state): State<Arc<ComhairleState>>,
    Path(email_config_id): Path<Uuid>,
    RequiredAdminUser(user): RequiredAdminUser,
) -> Result<(StatusCode, Json<EmailTypeSchema>), ComhairleError> {
    let email_config = email_template_config::get_by_id(&state.db, email_config_id).await?;
    let schema = email_config.slots.schema();

    Ok((StatusCode::OK, Json(schema)))
}

#[instrument(err(Debug))]
async fn list_schemas(
    RequiredAdminUser(user): RequiredAdminUser,
) -> Result<
    (
        StatusCode,
        Json<[EmailTypeSchema; EmailTemplateSlots::COUNT]>,
    ),
    ComhairleError,
> {
    let schemas = EmailTemplateSlots::schemas();

    Ok((StatusCode::OK, Json(schemas)))
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
struct PreviewEmailTemplateConfigRequest {
    slots: EmailTemplateSlots,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
struct PreviewEmailTemplateConfigResponse {
    html: String,
}

#[instrument(err(Debug), skip(state))]
async fn preview(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Json(PreviewEmailTemplateConfigRequest { slots }): Json<PreviewEmailTemplateConfigRequest>,
) -> Result<(StatusCode, Json<PreviewEmailTemplateConfigResponse>), ComhairleError> {
    let template = slots.email_template();
    let custom_slots_map = slots.mailer_context_map();
    let preview_variables_map = slots.preview_variables_map();

    let html =
        state
            .mailer
            .preview_email(template, custom_slots_map, Some(preview_variables_map))?;

    Ok((
        StatusCode::OK,
        Json(PreviewEmailTemplateConfigResponse { html }),
    ))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create, |op| {
                op.id("CreateEmailTemplateConfig")
                    .summary("Create email template config")
                    .description("Create custom content for specific email template")
                    .security_requirement("JWT")
                    .tag("EmailTemplateConfig")
                    .response::<201, Json<EmailTemplateConfigDto>>()
            }),
        )
        .api_route(
            "/{email_config_id}",
            get_with(get, |op| {
                op.id("GetEmailTemplateConfig")
                    .summary("Get email template config")
                    .description("Get custom email template configuration")
                    .security_requirement("JWT")
                    .tag("EmailTemplateConfig")
                    .response::<200, Json<EmailTemplateConfigDto>>()
            }),
        )
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListEmailTemplateConfigs")
                    .summary("List email template configs")
                    .description("List custom email template configurations")
                    .security_requirement("JWT")
                    .tag("EmailTemplateConfig")
                    .response::<200, Json<Vec<EmailTemplateConfigDto>>>()
            }),
        )
        .api_route(
            "/{email_config_id}",
            put_with(update, |op| {
                op.id("UpdateEmailTemplateConfig")
                    .summary("Update email template config")
                    .description("Update custom email template configuration")
                    .security_requirement("JWT")
                    .tag("EmailTemplateConfig")
                    .response::<200, Json<EmailTemplateConfigDto>>()
            }),
        )
        .api_route(
            "/{email_config_id}",
            delete_with(delete, |op| {
                op.id("DeleteEmailTemplateConfig")
                    .summary("Delete email template config")
                    .description("Delete custom email template configuration")
                    .security_requirement("JWT")
                    .tag("EmailTemplateConfig")
                    .response::<200, Json<EmailTemplateConfigDto>>()
            }),
        )
        .api_route(
            "/{email_config_id}/schemas",
            get_with(get_schema, |op| {
                op.id("GetEmailTemplateSchema")
                    .summary("Get email template schema")
                    .description("Get template schemas for an email config")
                    .security_requirement("JWT")
                    .tag("EmailTemplateConfig")
                    .response::<200, Json<EmailTypeSchema>>()
            }),
        )
        .api_route(
            "/schemas",
            get_with(list_schemas, |op| {
                op.id("ListEmailTemplateSchemas")
                    .summary("List email template schemas")
                    .description("List all template schemas for each email template type")
                    .security_requirement("JWT")
                    .tag("EmailTemplateConfig")
                    .response::<200, Json<[EmailTypeSchema; EmailTemplateSlots::COUNT]>>()
            }),
        )
        .api_route(
            "/preview",
            post_with(preview, |op| {
                op.id("PreviewEmailTemplateConfig")
                    .summary("Preview email template config")
                    .description("Preview appearance of custom email before sending")
                    .security_requirement("JWT")
                    .tag("EmailTemplateConfig")
                    .response::<200, Json<PreviewEmailTemplateConfigResponse>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use crate::{
        mailer::MockComhairleMailer,
        models::{
            email_template_config::{DefaultEmailSlots, EmailType},
            model_test_helpers::setup_default_app_and_session,
        },
        setup_server,
        test_helpers::{test_state, UserSession},
    };

    use super::*;

    use std::error::Error;

    use sqlx::PgPool;

    #[sqlx::test]
    async fn should_create_email_template_config(pool: PgPool) -> Result<(), Box<dyn Error>> {
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
            subject: Some("A custom conversation template".to_string()),
        };

        let (_, value, _) = session
            .post(
                &app,
                "/email_template_configs",
                serde_json::to_string(&params)?.into(),
            )
            .await?;
        let email_config: EmailTemplateConfigDto = serde_json::from_value(value)?;

        assert_eq!(
            email_config.subject.unwrap(),
            "A custom conversation template".to_string(),
            "incorrect subject",
        );
        assert_eq!(email_config.owner_id, current_user.id, "incorrect owner_id");

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_email_template_config(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let create_slots = EmailTemplateSlots::ConversationInvite(DefaultEmailSlots {
            heading: "<h1>You're invite to a conversation</h1>".to_string(),
            intro: "<p>You have been selected to take part in a public engagement</p>".to_string(),
            body: "<p>Test body content</p>".to_string(),
            footer: "<p>Thank you for your time</p>".to_string(),
        });

        let params = CreateEmailTemplateConfig {
            slots: create_slots,
            subject: Some("A custom conversation template".to_string()),
        };

        let (_, value, _) = session
            .post(
                &app,
                "/email_template_configs",
                serde_json::to_string(&params)?.into(),
            )
            .await?;
        let created_email_config: EmailTemplateConfigDto = serde_json::from_value(value)?;

        let update_slots = EmailTemplateSlots::ConversationInvite(DefaultEmailSlots {
            heading: "<h1>Updated heading</h1>".to_string(),
            intro: "<p>Updated intro</p>".to_string(),
            body: "<p>Updated body</p>".to_string(),
            footer: "<p>Updated footer</p>".to_string(),
        });
        let params = UpdateEmailTemplateConfig {
            slots: Some(update_slots.clone()),
            subject: None,
        };
        let (_, value, _) = session
            .put(
                &app,
                &format!("/email_template_configs/{}", created_email_config.id),
                serde_json::to_string(&params)?.into(),
            )
            .await?;
        let email_config: EmailTemplateConfigDto = serde_json::from_value(value)?;

        assert_eq!(email_config.slots, update_slots, "incorrect slots");

        Ok(())
    }

    #[sqlx::test]
    async fn should_get_email_template_config_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let params = CreateEmailTemplateConfig {
            slots: EmailTemplateSlots::ConversationInvite(DefaultEmailSlots {
                heading: "<h1>You're invite to a conversation</h1>".to_string(),
                intro: "<p>You have been selected to take part in a public engagement</p>"
                    .to_string(),
                body: "<p>Test body content</p>".to_string(),
                footer: "<p>Thank you for your time</p>".to_string(),
            }),
            subject: Some("A custom conversation template".to_string()),
        };

        let (_, value, _) = session
            .post(
                &app,
                "/email_template_configs",
                serde_json::to_string(&params)?.into(),
            )
            .await?;
        let create_email_config: EmailTemplateConfigDto = serde_json::from_value(value)?;

        let (_, value, _) = session
            .get(
                &app,
                &format!("/email_template_configs/{}", create_email_config.id),
            )
            .await?;
        let email_config: EmailTemplateConfigDto = serde_json::from_value(value)?;

        assert_eq!(email_config.id, create_email_config.id, "ids don't match");

        Ok(())
    }

    #[sqlx::test]
    async fn should_list_email_template_configs(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

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
        session
            .post(
                &app,
                "/email_template_configs",
                serde_json::to_string(&params_a)?.into(),
            )
            .await?;

        let params_b = CreateEmailTemplateConfig {
            slots: EmailTemplateSlots::ConversationInvite(default_slots.clone()),
            subject: None,
        };
        session
            .post(
                &app,
                "/email_template_configs",
                serde_json::to_string(&params_b)?.into(),
            )
            .await?;

        let (_, value, _) = session
            .get(
                &app,
                "/email_template_configs?email_type=event_registration_invite",
            )
            .await?;
        let email_configs: Vec<EmailTemplateConfigDto> = serde_json::from_value(value)?;

        assert_eq!(email_configs.len(), 1, "incorrect total");
        assert!(
            !email_configs
                .iter()
                .any(|c| c.email_type == EmailType::ConversationInvite),
            "incorrectly email_type included"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_email_template_config_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let params = CreateEmailTemplateConfig {
            slots: EmailTemplateSlots::ConversationInvite(DefaultEmailSlots {
                heading: "<h1>You're invite to a conversation</h1>".to_string(),
                intro: "<p>You have been selected to take part in a public engagement</p>"
                    .to_string(),
                body: "<p>Test body content</p>".to_string(),
                footer: "<p>Thank you for your time</p>".to_string(),
            }),
            subject: Some("A custom conversation template".to_string()),
        };

        let (_, value, _) = session
            .post(
                &app,
                "/email_template_configs",
                serde_json::to_string(&params)?.into(),
            )
            .await?;
        let create_email_config: EmailTemplateConfigDto = serde_json::from_value(value)?;

        session
            .delete(
                &app,
                &format!("/email_template_configs/{}", create_email_config.id),
            )
            .await?;

        let (_, response, _) = session
            .get(
                &app,
                &format!("/email_template_configs/{}", create_email_config.id),
            )
            .await?;

        assert_eq!(
            response.get("err").and_then(|v| v.as_str()).unwrap(),
            "Email template config not found",
            "incorrect error message"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_get_associated_schema_for_email_config_type(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let params = CreateEmailTemplateConfig {
            slots: EmailTemplateSlots::ConversationInvite(DefaultEmailSlots {
                heading: "<h1>You're invite to a conversation</h1>".to_string(),
                intro: "<p>You have been selected to take part in a public engagement</p>"
                    .to_string(),
                body: "<p>Test body content</p>".to_string(),
                footer: "<p>Thank you for your time</p>".to_string(),
            }),
            subject: Some("A custom conversation template".to_string()),
        };

        let (_, value, _) = session
            .post(
                &app,
                "/email_template_configs",
                serde_json::to_string(&params)?.into(),
            )
            .await?;
        let email_config: EmailTemplateConfigDto = serde_json::from_value(value)?;

        let (_, response, _) = session
            .get(
                &app,
                &format!("/email_template_configs/{}/schemas", email_config.id),
            )
            .await?;

        assert_eq!(
            response.get("email_type").and_then(|v| v.as_str()).unwrap(),
            "conversation_invite",
            "incorrect email_type from schema"
        );
        assert_eq!(
            response.get("template").and_then(|v| v.as_str()).unwrap(),
            "conversation_invite.html",
            "incorrect template from schema"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_list_schemas_for_email_config_types(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let (_, value, _) = session.get(&app, "/email_template_configs/schemas").await?;

        let arr = value.as_array().unwrap();
        assert_eq!(
            arr.len(),
            EmailTemplateSlots::COUNT,
            "incorrect number of schemas"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_send_preview_html_of_email(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let html_str = "<html><h1>You're invite to a conversation</h1><p>You have been selected to take part in a public engagement</p><p>Test body content</p><p>Thank you for your time</p></html>";
        let mut mailer = MockComhairleMailer::new();
        mailer
            .expect_preview_email()
            .returning(|_, _, _| Ok(html_str.to_string()));
        mailer.expect_send_welcome_email().returning(|_, _| Ok(()));

        let state = test_state()
            .db(pool.clone())
            .mailer(Arc::new(mailer))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let params = PreviewEmailTemplateConfigRequest {
            slots: EmailTemplateSlots::ConversationInvite(DefaultEmailSlots {
                heading: "<h1>You're invite to a conversation</h1>".to_string(),
                intro: "<p>You have been selected to take part in a public engagement</p>"
                    .to_string(),
                body: "<p>Test body content</p>".to_string(),
                footer: "<p>Thank you for your time</p>".to_string(),
            }),
        };

        let (_, value, _) = session
            .post(
                &app,
                "/email_template_configs/preview",
                serde_json::to_string(&params)?.into(),
            )
            .await?;
        let response: PreviewEmailTemplateConfigResponse = serde_json::from_value(value)?;

        assert_eq!(response.html, html_str, "incorrect html response");

        Ok(())
    }
}
