use aide::axum::{
    routing::{delete_with, get_with, post_with},
    ApiRouter,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::models::{user_participation, user_progress, workflow_step};
use crate::ComhairleState;
use crate::{error::ComhairleError, models::user_participation::UserParticipation};

use super::auth::RequiredUser;

/// Register user on workflow
/// This end point will create a user participation
/// entry and a UserProgress entry for each of the
/// workflow_steps in this workflow
async fn register_user_for_workflow(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((_, workflow_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<UserParticipation>), ComhairleError> {
    let user_participation = user_participation::create(&state.db, &user.id, &workflow_id).await?;

    let workflow_steps = workflow_step::list(&state.db, workflow_id).await?;

    for step in workflow_steps {
        user_progress::create(
            &state.db,
            &user.id,
            &step.id,
            user_progress::ProgressStatus::NotStarted,
        )
        .await?;
    }

    Ok((StatusCode::CREATED, Json(user_participation)))
}

/// Delete a specific workflow
async fn deregister_user_on_workflow(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((_, workflow_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<UserParticipation>), ComhairleError> {
    let user_participation = user_participation::delete(&state.db, &user.id, &workflow_id).await?;
    Ok((StatusCode::OK, Json(user_participation)))
}

pub async fn get_user_participation(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((_, workflow_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<Option<UserParticipation>>), ComhairleError> {
    let user_participation = user_participation::get(&state.db, &user.id, &workflow_id).await?;
    Ok((StatusCode::OK, Json(user_participation)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(register_user_for_workflow, |op| {
                op.id("RegisterUserForWorkflow")
                    .summary("Register the currently logged in user for this workflow")
                    .response::<201, Json<UserParticipation>>()
            }),
        )
        .api_route(
            "/",
            delete_with(deregister_user_on_workflow, |op| {
                op.id("UnregisterUserForWorkflow")
                    .summary("Unregisters the current user on this workflow")
                    .response::<200, Json<UserParticipation>>()
            }),
        )
        .api_route(
            "/",
            get_with(get_user_participation, |op| {
                op.id("GetUserParticipation")
                    .summary("Returns the status of the current user on this workflow")
                    .response::<200, Json<Option<UserParticipation>>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::extract;
    use crate::{config, setup_server, test_helpers::UserSession};
    use axum::body::Body;
    use axum::http::StatusCode;

    use sqlx::PgPool;

    use std::error::Error;

    #[sqlx::test]
    fn should_be_able_to_register_a_user_for_a_workflow(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let config = config::load()?;
        let app = setup_server(config, pool).await?;

        let mut admin_user_session = UserSession::new(
            "admin_user".into(),
            "test_password".into(),
            "admin_user@gmail.com".into(),
        );

        admin_user_session.signup(&app).await?;

        let (_, conversation, _) = admin_user_session.create_random_conversation(&app).await?;
        let conversation_id: String = extract("id", &conversation);

        let (_, workflow, _) = admin_user_session
            .create_random_workflow(&app, &conversation_id)
            .await?;

        let workflow_id: String = extract("id", &workflow);

        admin_user_session
            .create_random_workflow_steps(&app, &conversation_id, &workflow_id, 10)
            .await?;

        let mut user_session = UserSession::new(
            "regular_user".into(),
            "test_password".into(),
            "regular_user@gmail.com".into(),
        );

        let url = format!("/conversation/{conversation_id}/workflow/{workflow_id}/participation");
        let (status, _, _) = user_session.post(&app, &url, Body::empty()).await?;

        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "a logged out user should not be able to signup"
        );

        user_session.signup(&app).await?;

        let (status, _, _) = user_session.post(&app, &url, Body::empty()).await?;

        assert_eq!(
            status,
            StatusCode::CREATED,
            "logged in user should be able to signup for a workflow"
        );

        let url = format!("/conversation/{conversation_id}/workflow/{workflow_id}/progress");

        let (status, progress, _) = user_session.get(&app, &url).await?;
        println!("{status}, {progress:#?}");

        assert_eq!(
            status,
            StatusCode::OK,
            "Should be able to get back progress for user"
        );

        let progress: Vec<serde_json::Value> = serde_json::from_value(progress).unwrap();
        let status_codes: Vec<String> = progress
            .iter()
            .map(|a| extract::<String>("status", a))
            .collect();

        let not_started_count = status_codes.iter().filter(|s| *s == "not_started").count();

        assert_eq!(not_started_count, 10, "Should all be not started");
        Ok(())
    }
}
