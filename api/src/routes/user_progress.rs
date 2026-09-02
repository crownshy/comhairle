use aide::axum::{
    ApiRouter,
    routing::{get_with, put_with},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use std::sync::Arc;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::ComhairleState;
use crate::{
    error::ComhairleError,
    models::user_progress::{self, UpdateUserProgress},
    models::workflow_step,
    routes::user_progress::dto::UserProgressDto,
};

use super::auth::RequiredUser;

pub mod dto;

/// Get the progress for a user on a workflow step
#[instrument(err(Debug), skip(state))]
async fn get_user_progress_for_workflow(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((_, workflow_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<Vec<UserProgressDto>>), ComhairleError> {
    info!(
        "Attempting to sigun up user {} to workflow {workflow_id}",
        user.id
    );
    let user_progress =
        user_progress::list_for_user_on_workflow(&state.db, &user.id, &workflow_id).await?;

    let user_progress: Vec<UserProgressDto> = user_progress.into_iter().map(Into::into).collect();
    Ok((StatusCode::OK, Json(user_progress)))
}

/// Set the progress for the current user on a workflow step
///
/// Refuses to record progress for a sealed participant. The seal is evaluated against the
/// state *before* this write, which matters: the write that marks the final step done is the
/// one that brings the seal into existence, so checking afterwards would reject the very
/// request that completes the flow and nobody could ever finish. See ADR-0016.
#[instrument(err(Debug), skip(state))]
pub async fn update_user_progress(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((_, workflow_id, workflow_step_id)): Path<(Uuid, Uuid, Uuid)>,
    Json(payload): Json<UpdateUserProgress>,
) -> Result<(StatusCode, Json<UserProgressDto>), ComhairleError> {
    // The seal is evaluated for the workflow in the path, but the write below is keyed on the
    // step alone. Left unchecked, a sealed participant could send the id of some other
    // workflow they have not finished and have the gate wave the write through.
    let step = workflow_step::get_by_id(&state.db, &workflow_step_id).await?;
    if step.workflow_id != workflow_id {
        return Err(ComhairleError::BadRequest(format!(
            "workflow step {workflow_step_id} does not belong to workflow {workflow_id}"
        )));
    }

    if user_progress::is_sealed(&state.db, &user.id, &workflow_id).await? {
        // Already sealed going in. A write that changes nothing is still allowed through as a
        // no-op: the client retrying the `done` write that completed the flow (a double
        // submit, a flaky connection) has in fact succeeded, and answering it with an error
        // would show a failure to someone who finished.
        let existing = user_progress::get(&state.db, &user.id, &workflow_step_id).await?;

        return match existing {
            Some(row) if payload.is_noop_for(&row) => Ok((StatusCode::OK, Json(row.into()))),
            _ => Err(ComhairleError::ParticipantSealed),
        };
    }

    let user_progress =
        user_progress::update(&state.db, &user.id, &workflow_step_id, &payload).await?;

    let user_progress: UserProgressDto = user_progress.into();
    Ok((StatusCode::OK, Json(user_progress)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(get_user_progress_for_workflow, |op| {
                op.id("GetUserProgress")
                    .summary("Get the users progress on this workflow")
                    .response::<200, Json<Vec<UserProgressDto>>>()
            }),
        )
        .api_route(
            "/{workflow_step_id}",
            put_with(update_user_progress, |op| {
                op.id("SetUserProgress")
                    .summary("Set the user progress for a given workflow step")
                    .response::<200, Json<UserProgressDto>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use std::{error::Error, sync::Arc};

    use axum::{body::Body, http::StatusCode};
    use serde_json::json;
    use sqlx::PgPool;

    use crate::{
        setup_server,
        test_helpers::{UserSession, extract, test_state},
    };

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn should_be_able_to_register_a_user_for_a_workflow(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_user_session = UserSession::new_admin();

        admin_user_session.signup(&app).await?;

        let (_, conversation, _) = admin_user_session.create_random_conversation(&app).await?;
        let conversation_id: String = extract("id", &conversation);

        let (_, workflow, _) = admin_user_session
            .create_random_workflow(&app, &conversation_id)
            .await?;

        let workflow_id: String = extract("id", &workflow);

        let steps = admin_user_session
            .create_random_workflow_steps(&app, &conversation_id, &workflow_id, 10)
            .await?;

        let target_step: serde_json::Value = steps.get(3).unwrap().to_owned();
        let workflow_step_id: String = extract("id", &target_step);

        let mut user_session = UserSession::new(
            "regular_user",
            crate::test_helpers::TEST_PASSWORD,
            "regular_user@gmail.com",
        );
        user_session.signup(&app).await?;

        // Sign up for the workflow

        let url = format!("/conversation/{conversation_id}/workflow/{workflow_id}/register");
        user_session.post(&app, &url, Body::empty()).await?;

        // Update the status for a user on a given step
        let url = format!(
            "/conversation/{conversation_id}/workflow/{workflow_id}/progress/{workflow_step_id}"
        );

        let (status, progress, _) = user_session
            .put(&app, &url, json!({"status": "done"}).to_string().into())
            .await?;

        let new_status: String = extract("status", &progress);
        assert_eq!(status, StatusCode::OK, "should respone with created");
        assert_eq!(new_status, "done", "should have the correct status");

        Ok(())
    }

    /// The seal is evaluated for the workflow in the path while the write is keyed on the step,
    /// so the two have to be checked against each other. Otherwise a sealed participant could
    /// name a workflow they have not finished and get their write through anyway.
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn should_reject_progress_for_a_step_in_another_workflow(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_user_session = UserSession::new_admin();
        admin_user_session.signup(&app).await?;

        let (_, conversation, _) = admin_user_session.create_random_conversation(&app).await?;
        let conversation_id: String = extract("id", &conversation);

        let (_, workflow, _) = admin_user_session
            .create_random_workflow(&app, &conversation_id)
            .await?;
        let workflow_id: String = extract("id", &workflow);

        let (_, other_workflow, _) = admin_user_session
            .create_random_workflow(&app, &conversation_id)
            .await?;
        let other_workflow_id: String = extract("id", &other_workflow);

        let other_steps = admin_user_session
            .create_random_workflow_steps(&app, &conversation_id, &other_workflow_id, 1)
            .await?;
        let other_step_id: String = extract("id", other_steps.first().unwrap());

        let mut user_session = UserSession::new(
            "regular_user",
            crate::test_helpers::TEST_PASSWORD,
            "regular_user@gmail.com",
        );
        user_session.signup(&app).await?;

        for id in [&workflow_id, &other_workflow_id] {
            let url = format!("/conversation/{conversation_id}/workflow/{id}/register");
            user_session.post(&app, &url, Body::empty()).await?;
        }

        // The step belongs to `other_workflow_id`, but the path names `workflow_id`.
        let url = format!(
            "/conversation/{conversation_id}/workflow/{workflow_id}/progress/{other_step_id}"
        );

        let (status, _, _) = user_session
            .put(&app, &url, json!({"status": "done"}).to_string().into())
            .await?;

        assert_eq!(
            status,
            StatusCode::BAD_REQUEST,
            "a step from another workflow must be refused"
        );

        Ok(())
    }
}
