use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::{delete_with, get_with, post_with, put_with};
use axum::Json;
use axum::extract::{Path, State};
use axum::middleware::from_fn_with_state;
use hyper::StatusCode;
use tracing::instrument;
use uuid::Uuid;

use super::auth::RequiredUser;
use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::middleware::permissions::{PermissionRequirement, authorize as permission_middleware};
use crate::models::permissions::ConversationResource;
use crate::models::permissions::conversation::Action;
use crate::models::report_impact::{CreateImpactDTO, PartialReportImpact};
use crate::models::{self};
use crate::routes::report_impacts::dto::ReportImpactDto;

pub mod dto;

/// The permission middleware checks the conversation, so the report has to belong to it.
async fn check_report_in_conversation(
    state: &ComhairleState,
    conversation_id: Uuid,
    report_id: &Uuid,
) -> Result<(), ComhairleError> {
    let report = models::report::get_for_conversation(&state.db, conversation_id).await?;
    if report.id != *report_id {
        return Err(ComhairleError::ResourceNotFound("Report".into()));
    }
    Ok(())
}

#[instrument(err(Debug), skip(state))]
async fn create_impact(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, report_id)): Path<(Uuid, Uuid)>,
    RequiredUser(user): RequiredUser,
    Json(create_request): Json<CreateImpactDTO>,
) -> Result<(StatusCode, Json<ReportImpactDto>), ComhairleError> {
    check_report_in_conversation(&state, conversation_id, &report_id).await?;
    let impact = models::report_impact::create(&state.db, create_request, &report_id, &user.id)
        .await?
        .into();
    Ok((StatusCode::CREATED, Json(impact)))
}

#[instrument(err(Debug), skip(state))]
async fn update_impact(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, report_id, impact_id)): Path<(Uuid, Uuid, Uuid)>,
    Json(update_request): Json<PartialReportImpact>,
) -> Result<(StatusCode, Json<ReportImpactDto>), ComhairleError> {
    check_report_in_conversation(&state, conversation_id, &report_id).await?;
    let impact = models::report_impact::update(&state.db, update_request, &impact_id, &report_id)
        .await?
        .into();
    Ok((StatusCode::OK, Json(impact)))
}

#[instrument(err(Debug), skip(state))]
async fn delete_impact(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, report_id, impact_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<StatusCode, ComhairleError> {
    check_report_in_conversation(&state, conversation_id, &report_id).await?;
    models::report_impact::delete(&state.db, &impact_id, &report_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[instrument(err(Debug), skip(state))]
async fn list_impacts_for_report(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, report_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<Vec<ReportImpactDto>>), ComhairleError> {
    check_report_in_conversation(&state, conversation_id, &report_id).await?;
    let impacts = models::report_impact::get_for_report(&state.db, &report_id)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok((StatusCode::OK, Json(impacts)))
}

/// Impacts are private to the people running the conversation, so every route needs
/// permission to update it.
pub fn router() -> ApiRouter<Arc<ComhairleState>> {
    let requirement = PermissionRequirement::<ConversationResource>::new;
    let can_update = || {
        from_fn_with_state(
            requirement(Action::Update),
            permission_middleware::<ConversationResource>,
        )
    };

    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_impact, |op| {
                op.id("CreateImpact")
                    .summary("Create an impact on a report")
                    .response::<201, Json<ReportImpactDto>>()
            })
            .route_layer(can_update()),
        )
        .api_route(
            "/",
            get_with(list_impacts_for_report, |op| {
                op.id("ListImpactsForReport")
                    .summary("Return a list of impacts for a report")
                    .response::<200, Json<Vec<ReportImpactDto>>>()
            })
            .route_layer(can_update()),
        )
        .api_route(
            "/{impact_id}",
            put_with(update_impact, |op| {
                op.id("UpdateImpact")
                    .summary("Update an impact")
                    .response::<200, Json<ReportImpactDto>>()
            })
            .route_layer(can_update()),
        )
        .api_route(
            "/{impact_id}",
            delete_with(delete_impact, |op| {
                op.id("DeleteImpact")
                    .summary("Delete an impact")
                    .response::<204, ()>()
            })
            .route_layer(can_update()),
        )
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use serde_json::json;
    use sqlx::PgPool;

    use crate::models::model_test_helpers::{
        get_random_conversation_id, setup_default_app_and_session,
    };
    use crate::test_helpers::UserSession;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn only_people_running_the_conversation_see_and_change_impacts(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut owner) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut owner).await?;
        owner
            .create_random_workflow(&app, &conversation_id.to_string())
            .await?;
        let report_url = format!("/conversation/{conversation_id}/report");
        let (status, report, _) = owner.post(&app, &report_url, "".into()).await?;
        assert!(status.is_success(), "report not created");
        let impacts_url = format!("{report_url}/{}/impacts", report["id"].as_str().unwrap());

        let (status, impact, _) = owner
            .post(
                &app,
                &impacts_url,
                json!({ "title": "Debated", "details": "In council", "kind": "debate" })
                    .to_string()
                    .into(),
            )
            .await?;
        assert_eq!(status, 201, "impact not created");
        let impact_url = format!("{impacts_url}/{}", impact["id"].as_str().unwrap());

        let (status, updated, _) = owner
            .put(
                &app,
                &impact_url,
                json!({ "title": "Debated twice" }).to_string().into(),
            )
            .await?;
        assert_eq!(status, 200, "impact not updated");
        assert_eq!(updated["title"], "Debated twice");

        let mut guest = UserSession::new_guest();
        let (status, _, _) = guest
            .put(
                &app,
                &report_url,
                json!({ "is_public": true }).to_string().into(),
            )
            .await?;
        assert!(status.is_client_error(), "a guest updated the report");
        let (_, guest_view, _) = guest.get(&app, &report_url).await?;
        assert_eq!(guest_view["impacts"], json!([]), "a guest saw impacts");
        let (status, _, _) = guest.delete(&app, &impact_url).await?;
        assert!(status.is_client_error(), "a guest deleted an impact");

        let (_, owner_view, _) = owner.get(&app, &report_url).await?;
        assert_eq!(owner_view["impacts"].as_array().map(Vec::len), Some(1));

        let (status, _, _) = owner.delete(&app, &impact_url).await?;
        assert_eq!(status, 204, "impact not deleted");
        let (_, owner_view, _) = owner.get(&app, &report_url).await?;
        assert_eq!(owner_view["impacts"], json!([]));

        Ok(())
    }
}
