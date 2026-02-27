use aide::axum::{
    routing::{get_with, put_with},
    ApiRouter,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::ComhairleState;
use crate::{
    models::user_progress::{self, ProgressStatus},
    routes::user_progress::dto::UserProgressDto,
};

use super::auth::RequiredUser;

pub mod dto;

/// Get the progress for a user on a workflow step
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
pub async fn update_user_progress(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((_, _, workflow_step_id)): Path<(Uuid, Uuid, Uuid)>,
    Json(status): Json<ProgressStatus>,
) -> Result<(StatusCode, Json<UserProgressDto>), ComhairleError> {
    let user_progress =
        user_progress::update(&state.db, &user.id, &workflow_step_id, status).await?;

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
        test_helpers::{extract, test_state, UserSession},
    };

    #[sqlx::test]
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

        let mut user_session =
            UserSession::new("regular_user", "test_password", "regular_user@gmail.com");
        user_session.signup(&app).await?;

        // Sign up for the workflow

        let url = format!("/conversation/{conversation_id}/workflow/{workflow_id}/register");
        user_session.post(&app, &url, Body::empty()).await?;

        // Update the status for a user on a given step
        let url = format!(
            "/conversation/{conversation_id}/workflow/{workflow_id}/progress/{workflow_step_id}"
        );

        let (status, progress, _) = user_session
            .put(&app, &url, json!("done").to_string().into())
            .await?;

        let new_status: String = extract("status", &progress);
        assert_eq!(status, StatusCode::OK, "should respone with created");
        assert_eq!(new_status, "done", "should have the correct status");

        Ok(())
    }
}
