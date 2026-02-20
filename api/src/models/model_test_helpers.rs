use std::{error::Error, sync::Arc};

use crate::{
    routes::{
        conversations::dto::ConversationDto, user::dto::UserDto, workflows::dto::WorkflowDto,
    },
    setup_server,
    test_helpers::{test_state, UserSession},
};

use axum::Router;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn setup_default_app_and_session(
    pool: &PgPool,
) -> Result<(Router, UserSession), Box<dyn Error>> {
    let state = test_state().db(pool.clone()).call()?;
    let app = setup_server(Arc::new(state)).await?;

    let mut session = UserSession::new_admin();
    session.signup(&app).await?;

    Ok((app, session))
}

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

pub async fn get_random_conversation_id(
    app: &Router,
    session: &mut UserSession,
) -> Result<Uuid, Box<dyn Error>> {
    let (_, response, _) = session.create_random_conversation(app).await?;
    let conversation: ConversationDto = serde_json::from_value(response)?;

    Ok(conversation.id)
}

pub async fn get_random_user_id(
    app: &Router,
    session: &mut UserSession,
) -> Result<Uuid, Box<dyn Error>> {
    let (_, response, _) = session.signup_annon(app).await?;
    let user: UserDto = serde_json::from_value(serde_json::to_value(response)?)?;

    Ok(user.id)
}
