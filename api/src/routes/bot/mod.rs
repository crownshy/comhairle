use std::sync::Arc;

use aide::axum::ApiRouter;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::ComhairleState;

pub mod agent_sessions;
pub mod agents;
pub mod chats;
pub mod documents;
pub mod knowledge_bases;

#[derive(Deserialize, Debug, JsonSchema, Default, PartialEq)]
pub struct GetQueryParams {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub order_by: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .nest_api_service(
            "/agents",
            agents::router(state.clone()).nest_api_service(
                "/{agent_id}/sessions",
                agent_sessions::router(state.clone()),
            ),
        )
        .nest_api_service("/chats", chats::router(state.clone()))
        .nest_api_service(
            "/knowledge_bases",
            knowledge_bases::router(state.clone()).nest_api_service(
                "/{knowledge_base_id}/documents",
                documents::router(state.clone()),
            ),
        )
        .with_state(state)
}
