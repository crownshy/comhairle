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
use tracing::instrument;

use crate::{
    bot_service::ComhairleAgent,
    error::ComhairleError,
    routes::{auth::RequiredAdminUser, bot::GetQueryParams},
    ComhairleState,
};

#[instrument(err(Debug), skip(state))]
pub async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path(agent_id): Path<String>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<ComhairleAgent>), ComhairleError> {
    let (_, agent) = state.bot_service.get_agent(&agent_id).await?;

    Ok((StatusCode::OK, Json(agent)))
}

#[instrument(err(Debug), skip(state))]
pub async fn list(
    State(state): State<Arc<ComhairleState>>,
    Query(params): Query<GetQueryParams>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<ComhairleAgent>>), ComhairleError> {
    let (_, agents) = state.bot_service.list_agents(Some(params)).await?;

    Ok((StatusCode::OK, Json(agents)))
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct CreateAgentRequest;

pub async fn create(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<CreateAgentRequest>,
) -> Result<(StatusCode, Json<ComhairleAgent>), ComhairleError> {
    let (_, agent) = state.bot_service.create_agent(payload).await?;

    Ok((StatusCode::CREATED, Json(agent)))
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct UpdateAgentRequest;

pub async fn update(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<UpdateAgentRequest>,
) -> Result<(StatusCode, Json<ComhairleAgent>), ComhairleError> {
    let (_, agent) = state.bot_service.update_agent(payload).await?;

    Ok((StatusCode::OK, Json(agent)))
}

pub async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path(agent_id): Path<String>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<StatusCode, ComhairleError> {
    let _ = state.bot_service.delete_agent(&agent_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListAgents")
                    .tag("Bot Agents")
                    .summary("Get a list of agents")
                    .response::<200, Json<Vec<ComhairleAgent>>>()
            }),
        )
        .api_route(
            "/{agent_id}",
            get_with(get, |op| {
                op.id("GetAgent")
                    .tag("Bot Agents")
                    .summary("Get an agent by id")
                    .response::<200, Json<ComhairleAgent>>()
            }),
        )
        .api_route(
            "/",
            post_with(create, |op| {
                op.id("CreateAgent")
                    .tag("Bot Agents")
                    .summary("Create an agent")
                    .response::<201, Json<ComhairleAgent>>()
            }),
        )
        .api_route(
            "/{agent_id}",
            put_with(update, |op| {
                op.id("UpdateAgent")
                    .tag("Bot Agents")
                    .summary("Update an agent")
                    .response::<200, Json<ComhairleAgent>>()
            }),
        )
        .api_route(
            "/{agent_id}",
            delete_with(delete, |op| {
                op.id("DeleteAgent")
                    .tag("Bot Agents")
                    .summary("Delete an agent")
                    .response::<204, ()>()
            }),
        )
        .with_state(state)
}
