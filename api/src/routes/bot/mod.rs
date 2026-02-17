use std::sync::Arc;

use aide::axum::ApiRouter;

use crate::ComhairleState;

pub mod agent_sessions;
pub mod agents;

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .nest_api_service(
            "/agents",
            agents::router(state.clone()).nest_api_service(
                "/{agent_id}/sessions",
                agent_sessions::router(state.clone()),
            ),
        )
        .with_state(state)
}
