use std::sync::Arc;

use aide::axum::{
    routing::{get_with, post_with, put_with},
    ApiRouter,
};
use axum::{
    extract::{Path, State},
    Json,
};
use hyper::StatusCode;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        self,
        report::{FullReportDto, PartialReport},
    },
    routes::reports::dto::ReportDto,
    ComhairleState,
};

pub mod dto;

async fn get_report(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
) -> Result<(StatusCode, Json<FullReportDto>), ComhairleError> {
    let report = models::report::get_for_conversation(&state.db, &conversation_id).await?;
    let report = FullReportDto::from_report(&state.db, report).await?;
    Ok((StatusCode::OK, Json(report)))
}

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

async fn create_report(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
) -> Result<(StatusCode, Json<FullReportDto>), ComhairleError> {
    let report = models::report::create_for_conversation(&state.db, conversation_id).await?;
    let report = FullReportDto::from_report(&state.db, report).await?;
    Ok((StatusCode::CREATED, Json(report)))
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
