use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with, put_with},
    ApiRouter,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use axum_extra::extract::CookieJar;
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use crate::{error::ComhairleError, routes::bot::GetQueryParams, ComhairleState};

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    jar: CookieJar,
    params: Query<GetQueryParams>,
) -> Result<(CookieJar, StatusCode), ComhairleError> {
    todo!();
}

#[derive(Deserialize, Debug, JsonSchema)]
struct CreateChatRequest {
    name: String,
    knowledge_base_ids: Option<Vec<String>>,
    llm_model: Option<String>,
}

#[instrument(err(Debug), skip(state))]
async fn create(
    State(state): State<Arc<ComhairleState>>,
    jar: CookieJar,
    Json(payload): Json<CreateChatRequest>,
) -> Result<(CookieJar, StatusCode), ComhairleError> {
    todo!();
}

#[derive(Deserialize, Debug, JsonSchema)]
struct UpdateChatRequest {
    name: Option<String>,
    knowledge_base_ids: Option<Vec<String>>,
    llm_model: Option<String>,
}

#[instrument(err(Debug), skip(state))]
async fn update(
    State(state): State<Arc<ComhairleState>>,
    jar: CookieJar,
    Path(chat_id): Path<String>,
    Json(payload): Json<UpdateChatRequest>,
) -> Result<(CookieJar, StatusCode), ComhairleError> {
    todo!();
}

#[instrument(err(Debug), skip(state))]
async fn delete(
    State(state): State<Arc<ComhairleState>>,
    jar: CookieJar,
    Path(chat_id): Path<String>,
) -> Result<(CookieJar, StatusCode), ComhairleError> {
    todo!();
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(get, |op| {
                op.id("GetChats")
                    .summary("Get a list of chat bots")
                    .response::<200, ()>()
            }),
        )
        .api_route(
            "/",
            post_with(create, |op| {
                op.id("CreateChat")
                    .summary("Create a new chat bot")
                    .response::<201, ()>()
            }),
        )
        .api_route(
            "/{chat_id}",
            put_with(update, |op| {
                op.id("UpdateChat")
                    .summary("Update a chat bot")
                    .response::<200, ()>()
            }),
        )
        .api_route(
            "/{chat_id}",
            delete_with(delete, |op| {
                op.id("DeleteChat")
                    .summary("Delete a chat bot")
                    .response::<204, ()>()
            }),
        )
        .with_state(state)
}
