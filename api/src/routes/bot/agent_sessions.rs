use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with, put_with},
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

async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path((agent_id, session_id)): Path<(String, String)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<ComhairleAgentSession>), ComhairleError> {
    todo!();
}

async fn list(
    State(state): State<Arc<ComhairleState>>,
    Path(agent_id): Path<String>,
    Query(params): Query<GetQueryParams>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<ComhairleAgentSession>>), ComhairleError> {
    todo!();
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Default)]
pub struct CreateAgentSessionRequest {
    pub name: String,
    pub user_id: Option<String>,
}

async fn create(
    State(state): State<Arc<ComhairleState>>,
    Path(agent_id): Path<String>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<CreateAgentSessionRequest>,
) -> Result<(StatusCode, Json<ComhairleAgentSession>), ComhairleError> {
    todo!();
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone, PartialEq, Default)]
pub struct UpdateAgentSessionRequest {
    pub name: Option<String>,
}

async fn update(
    State(state): State<Arc<ComhairleState>>,
    Path((agent_id, session_id)): Path<(String, String)>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<UpdateAgentSessionRequest>,
) -> Result<(StatusCode, Json<ComhairleAgentSession>), ComhairleError> {
    todo!();
}

async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path((agent_id, session_id)): Path<(String, String)>,
) -> Result<StatusCode, ComhairleError> {
    todo!();
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
        .api_route(
            "/{session_id}",
            put_with(update, |op| {
                op.id("UpdateAgentSessions")
                    .tag("Bot Agent Sessions")
                    .security_requirement("JWT")
                    .summary("Update an agent session")
                    .response::<200, Json<ComhairleAgentSession>>()
            }),
        )
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
