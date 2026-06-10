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
use tracing::instrument;
use uuid::Uuid;

use crate::{
    models::email_template_config::{
        self, CreateEmailTemplateConfig, EmailTemplateConfigFilterOptions,
        UpdateEmailTemplateConfig,
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
    let email_configs = email_template_config::list(&state.db, filter_options)
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
        .with_state(state)
}
