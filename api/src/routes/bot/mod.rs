use std::sync::Arc;

use aide::axum::ApiRouter;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::ComhairleState;

pub mod chats;
pub mod documents;
pub mod knowledge_bases;
pub mod sessions;

#[derive(Deserialize, Debug, JsonSchema)]
pub struct GetQueryParams {
    page: Option<String>,
    page_size: Option<String>,
    order_by: Option<String>,
    name: Option<String>,
    id: Option<String>,
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .nest_service(
            "/chats",
            chats::router(state.clone())
                .nest_service("/{chat_id}/sessions", sessions::router(state.clone())),
        )
        .nest_service(
            "/knowledge_bases",
            knowledge_bases::router(state.clone()).nest_service(
                "/{knowledge_base_id}/documents",
                documents::router(state.clone()),
            ),
        )
        .with_state(state)
}
