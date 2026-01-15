use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with},
    ApiRouter,
};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    bot_service::ComhairleAgentSession,
    error::ComhairleError,
    routes::{auth::RequiredAdminUser, bot::GetQueryParams},
    ComhairleState,
};

async fn list(
    State(state): State<Arc<ComhairleState>>,
    Path(agent_id): Path<String>,
    Query(params): Query<GetQueryParams>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<ComhairleAgentSession>>), ComhairleError> {
    let (_, sessions) = state
        .bot_service
        .list_agent_session(&agent_id, Some(params))
        .await?;

    Ok((StatusCode::OK, Json(sessions)))
}

async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path((agent_id, session_id)): Path<(String, String)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<ComhairleAgentSession>), ComhairleError> {
    let (_, session) = state
        .bot_service
        .get_agent_session(&session_id, &agent_id)
        .await?;

    Ok((StatusCode::OK, Json(session)))
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Default)]
pub struct CreateAgentSessionRequest;

async fn create(
    State(state): State<Arc<ComhairleState>>,
    Path(agent_id): Path<String>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<CreateAgentSessionRequest>,
) -> Result<(StatusCode, Json<ComhairleAgentSession>), ComhairleError> {
    let (_, session) = state
        .bot_service
        .create_agent_session(&agent_id, payload)
        .await?;

    Ok((StatusCode::CREATED, Json(session)))
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone, PartialEq, Default)]
pub struct UpdateAgentSessionRequest;

async fn update(
    State(state): State<Arc<ComhairleState>>,
    Path((agent_id, session_id)): Path<(String, String)>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<UpdateAgentSessionRequest>,
) -> Result<(StatusCode, Json<ComhairleAgentSession>), ComhairleError> {
    let (_, session) = state
        .bot_service
        .update_agent_session(&session_id, &agent_id, payload)
        .await?;

    Ok((StatusCode::OK, Json(session)))
}

async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path((agent_id, session_id)): Path<(String, String)>,
) -> Result<StatusCode, ComhairleError> {
    let _ = state
        .bot_service
        .delete_agent_session(&session_id, &agent_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone, PartialEq)]
pub struct AgentConversationRequest {
    pub question: String,
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/{session_id}",
            get_with(get, |op| {
                op.id("GetAgentSession")
                    .tag("Bot Agent Sessions")
                    .security_requirement("JWT")
                    .summary("Get an agent session by id")
                    .response::<200, Json<ComhairleAgentSession>>()
            }),
        )
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListAgentSessions")
                    .tag("Bot Agent Sessions")
                    .security_requirement("JWT")
                    .summary("Get a list of agent sessions")
                    .response::<200, Json<Vec<ComhairleAgentSession>>>()
            }),
        )
        .api_route(
            "/",
            post_with(create, |op| {
                op.id("CreateAgentSessions")
                    .tag("Bot Agent Sessions")
                    .security_requirement("JWT")
                    .summary("Create a new session for an agent")
                    .response::<201, Json<ComhairleAgentSession>>()
            }),
        )
        // Not supported by current bot provider
        // .api_route(
        //     "/{session_id}",
        //     put_with(update, |op| {
        //         op.id("UpdateAgentSessions")
        //             .tag("Bot Agent Sessions")
        //             .security_requirement("JWT")
        //             .summary("Update an agent session")
        //             .response::<200, Json<ComhairleAgentSession>>()
        //     }),
        // )
        .api_route(
            "/{session_id}",
            delete_with(delete, |op| {
                op.id("DeleteAgentSessions")
                    .tag("Bot Agent Sessions")
                    .security_requirement("JWT")
                    .summary("Delete an agent session")
                    .response::<204, ()>()
            }),
        )
        .with_state(state)
}
