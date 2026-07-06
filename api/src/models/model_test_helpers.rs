use std::{error::Error, sync::Arc};

use crate::models::users::{update_user, UpdateUserRequest};
use crate::routes::conversations::dto::ConversationDto;
use crate::routes::organizations::dto::OrganizationDto;
use crate::routes::user::dto::UserDto;
use crate::routes::workflows::dto::WorkflowDto;
use crate::setup_server;
use crate::test_helpers::{test_state, UserSession};

use axum::Router;
use sqlx::PgPool;
use uuid::Uuid;

/// Sets up a default app `Router` and `UserSession` for testing.
pub async fn setup_default_app_and_session(
    pool: &PgPool,
) -> Result<(Router, UserSession), Box<dyn Error>> {
    let state = test_state().db(pool.clone()).call()?;
    let app = setup_server(Arc::new(state)).await?;

    let mut session = UserSession::new_admin();
    session.signup(&app).await?;

    Ok((app, session))
}

/// Creates a new workflow with a random name and returns the ID.
pub async fn get_random_workflow_id(
    app: &Router,
    session: &mut UserSession,
) -> Result<Uuid, Box<dyn Error>> {
    let (_, response, _) = session.create_random_conversation(app).await?;
    let conversation: ConversationDto = serde_json::from_value(response)?;
    let (_, response, _) = session
        .create_random_workflow(app, &conversation.id.to_string())
        .await?;
    let workflow: WorkflowDto = serde_json::from_value(response)?;

    Ok(workflow.id)
}

/// Creates a new conversation with a random name and returns the ID.
pub async fn get_random_conversation_id(
    app: &Router,
    session: &mut UserSession,
) -> Result<Uuid, Box<dyn Error>> {
    let (_, response, _) = session.create_random_conversation(app).await?;
    let conversation: ConversationDto = serde_json::from_value(response)?;

    Ok(conversation.id)
}

/// Creates a new anonymous user and returns the ID.
pub async fn get_random_user_id(
    app: &Router,
    session: &mut UserSession,
) -> Result<Uuid, Box<dyn Error>> {
    let (_, response, _) = session.signup_annon(app).await?;
    let user: UserDto = serde_json::from_value(serde_json::to_value(response)?)?;

    Ok(user.id)
}

/// Creates a new organization and returns the ID.
pub async fn get_random_organization_id(
    app: &Router,
    session: &mut UserSession,
) -> Result<Uuid, Box<dyn Error>> {
    let (_, response, _) = session.create_random_organization(app).await?;
    eprintln!("Organization creation response: {:?}", response);
    let organization: OrganizationDto = serde_json::from_value(response)?;

    Ok(organization.id)
}

/// Adds a user to an organization by updating the user's organization_id field.
pub async fn add_user_to_organization(
    user_id: Uuid,
    organization_id: Uuid,
    db: &PgPool,
) -> Result<(), Box<dyn Error>> {
    update_user(
        &user_id,
        &UpdateUserRequest {
            organization_id: Some(organization_id),
            ..Default::default()
        },
        db,
    )
    .await?;
    Ok(())
}
