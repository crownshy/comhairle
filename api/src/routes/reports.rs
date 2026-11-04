use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::{get_with, post_with, put_with};
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::middleware::from_fn_with_state;
use hyper::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::middleware::permissions::{PermissionRequirement, authorize as permission_middleware};
use crate::models::permissions::ConversationResource;
use crate::models::permissions::conversation::Action;
use crate::models::report::{FullReportDto, PartialReport, ReportWithTranslations};
use crate::routes::auth::{OptionalUser, RequiredAdminUser, is_user_admin};
use crate::routes::reports::dto::{LocalizedReportDto, ReportDto, ReportEmbedsDto};
use crate::routes::translations::LocaleExtractor;
use crate::tools::public_report_data::embeds::embedded_section_blocks;
use crate::{ComhairleState, models};

pub mod dto;

#[derive(Deserialize, Debug, JsonSchema)]
struct GetReportQuery {
    #[serde(rename = "withTranslations", default)]
    with_translations: bool,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(untagged)]
pub enum ReportView {
    WithTranslations(ReportWithTranslations),
    Localized(LocalizedReportDto),
}

#[instrument(err(Debug), skip(state))]
async fn get_report(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    Query(query): Query<GetReportQuery>,
    OptionalUser(user): OptionalUser,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<(StatusCode, Json<FullReportDto>), ComhairleError> {
    let should_return_with_translations = if let Some(user) = user {
        query.with_translations && is_user_admin(&state, &user).await
    } else {
        false
    };

    if should_return_with_translations {
        let raw_report = models::report::get_for_conversation(&state.db, conversation_id).await?;
        let report_with_translations =
            ReportWithTranslations::from_original(&state.db, raw_report, &locale).await?;
        let full_report = FullReportDto::from_report(
            &state.db,
            ReportView::WithTranslations(report_with_translations),
        )
        .await?;

        Ok((StatusCode::OK, Json(full_report)))
    } else {
        let report =
            models::report::get_localized_for_conversation(&state.db, conversation_id, &locale)
                .await?;
        let full_report =
            FullReportDto::from_report(&state.db, ReportView::Localized(report.into())).await?;

        Ok((StatusCode::OK, Json(full_report)))
    }
}

#[instrument(err(Debug), skip(state))]
async fn update_report(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    Json(update): Json<PartialReport>,
) -> Result<(StatusCode, Json<ReportDto>), ComhairleError> {
    let updated_report = models::report::update(&state.db, conversation_id, update)
        .await?
        .into();
    Ok((StatusCode::OK, Json(updated_report)))
}

#[instrument(err(Debug), skip(state))]
async fn create_report(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<(StatusCode, Json<FullReportDto>), ComhairleError> {
    models::report::create_for_conversation(&state.db, conversation_id, &locale).await?;
    let raw_report = models::report::get_for_conversation(&state.db, conversation_id).await?;

    let report_with_translations =
        ReportWithTranslations::from_original(&state.db, raw_report, &locale).await?;
    let full_report = FullReportDto::from_report(
        &state.db,
        ReportView::WithTranslations(report_with_translations),
    )
    .await?;

    Ok((StatusCode::CREATED, Json(full_report)))
}

#[instrument(err(Debug), skip(state))]
async fn get_report_embeds(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ReportEmbedsDto>), ComhairleError> {
    let report = match models::report::get_for_conversation(&state.db, conversation_id).await {
        Ok(report) => report,
        Err(ComhairleError::ResourceNotFound(_)) => {
            let no_report = ReportEmbedsDto {
                is_public: false,
                section_blocks: vec![],
            };
            return Ok((StatusCode::OK, Json(no_report)));
        }
        Err(err) => return Err(err),
    };
    let section_blocks = embedded_section_blocks(&state.db, &report).await?;
    Ok((
        StatusCode::OK,
        Json(ReportEmbedsDto {
            is_public: report.is_public,
            section_blocks,
        }),
    ))
}

pub fn router() -> ApiRouter<Arc<ComhairleState>> {
    let requirement = PermissionRequirement::<ConversationResource>::new;

    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_report, |op| {
                op.id("GenerateReportForConversation")
                    .summary("Generates a report for this conversation")
                    .response::<201, Json<FullReportDto>>()
            }),
        )
        .api_route(
            "/",
            put_with(update_report, |op| {
                op.id("UpdateReport")
                    .summary("Update a report")
                    .response::<201, Json<ReportDto>>()
            }),
        )
        .api_route(
            "/",
            get_with(get_report, |op| {
                op.id("GetReportForConversation")
                    .summary("Return the report of a given conversation")
                    .response::<200, Json<FullReportDto>>()
            }),
        )
        .api_route(
            "/embeds",
            get_with(get_report_embeds, |op| {
                op.id("GetReportEmbeds")
                    .summary("List the section blocks the conversation's report embeds")
                    .response::<200, Json<ReportEmbedsDto>>()
            })
            .route_layer(from_fn_with_state(
                requirement(Action::Read),
                permission_middleware::<ConversationResource>,
            )),
        )
}
