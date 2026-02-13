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
        report_impact::{CreateImpactDTO, PartialReportImpact},
    },
    routes::report_impacts::dto::ReportImpactDto,
    ComhairleState,
};

use super::auth::RequiredAdminUser;

pub mod dto;

async fn create_impact(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, report_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(user): RequiredAdminUser,
    Json(create_request): Json<CreateImpactDTO>,
) -> Result<(StatusCode, Json<ReportImpactDto>), ComhairleError> {
    let conversation = models::conversation::get_by_id(&state.db, &conversation_id).await?;
    if conversation.owner_id != user.id {
        return Err(ComhairleError::UserIsNotConversationOwner);
    }

    let impact = models::report_impact::create(&state.db, create_request, &report_id, &user.id)
        .await?
        .into();

    Ok((StatusCode::CREATED, Json(impact)))
}

async fn update_impact(
    State(state): State<Arc<ComhairleState>>,
    Path((_, _, impact_id)): Path<(Uuid, Uuid, Uuid)>,
    RequiredAdminUser(user): RequiredAdminUser,
    Json(update_request): Json<PartialReportImpact>,
) -> Result<(StatusCode, Json<ReportImpactDto>), ComhairleError> {
    let impact = models::report_impact::update(&state.db, update_request, &impact_id, &user.id)
        .await?
        .into();
    Ok((StatusCode::OK, Json(impact)))
}

async fn list_impacts_for_conversation(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, report_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<ReportImpactDto>>), ComhairleError> {
    let conversation = models::conversation::get_by_id(&state.db, &conversation_id).await?;
    if conversation.owner_id != user.id {
        return Err(ComhairleError::UserIsNotConversationOwner);
    }

    let impacts = models::report_impact::get_for_report(&state.db, &report_id)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok((StatusCode::OK, Json(impacts)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_impact, |op| {
                op.id("CreateImpact")
                    .summary("Create an impact on a report")
                    .response::<201, Json<ReportImpactDto>>()
            }),
        )
        .api_route(
            "/",
            put_with(update_impact, |op| {
                op.id("UpdateImpact")
                    .summary("Update an impact")
                    .response::<201, Json<ReportImpactDto>>()
            }),
        )
        .api_route(
            "/",
            get_with(list_impacts_for_conversation, |op| {
                op.id("ListImpactsForReport")
                    .summary("Return a list of impacts for a report")
                    .response::<200, Json<Vec<ReportImpactDto>>>()
            }),
        )
        .with_state(state)
}
