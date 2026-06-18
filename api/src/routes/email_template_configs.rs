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
    let custom_slots_map = slots.mailer_slots_map();
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
                    .response::<200, Json<[EmailTypeSchema; 3]>>()
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
