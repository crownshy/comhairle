use std::sync::Arc;

use aide::axum::ApiRouter;

use crate::ComhairleState;

pub mod agent_sessions;
pub mod agents;
pub mod chats;
pub mod knowledge_bases;

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
        .nest_api_service("/knowledge_bases", knowledge_bases::router(state.clone()))
        .with_state(state)
}
