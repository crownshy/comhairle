use std::sync::Arc;

use aide::axum::{
    ApiRouter,
    routing::{get_with, post_with, put_with},
};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use hyper::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::models;
use crate::models::report::{FullReportDto, PartialReport, ReportWithTranslations};
use crate::routes::auth::{OptionalUser, RequiredAdminUser, is_user_admin};
use crate::routes::reports::dto::{LocalizedReportDto, ReportDto};
use crate::routes::translations::LocaleExtractor;
use crate::{ComhairleState, error::ComhairleError};

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

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
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
        .with_state(state)
}
