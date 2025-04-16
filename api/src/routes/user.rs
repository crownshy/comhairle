use std::sync::Arc;

use aide::axum::{routing::get_with, ApiRouter};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    error::ComhairleError,
    models::{
        self,
        conversation::{Conversation, ConversationFilterOptions, ConversationOrderOptions},
        pagination::{OrderParams, PageOptions, PaginatedResults},
    },
    ComhairleState,
};

use super::auth::RequiredUser;

pub async fn get_user_owned_conversations(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    OrderParams(order_options): OrderParams<ConversationOrderOptions>,
    Query(filter_options): Query<ConversationFilterOptions>,
    Query(page_options): Query<PageOptions>,
) -> Result<(StatusCode, Json<PaginatedResults<Conversation>>), ComhairleError> {
    let conversations = models::conversation::list_owned(
        &state.db,
        user.id,
        page_options,
        order_options,
        filter_options,
    )
    .await?;
    Ok((StatusCode::OK, Json(conversations)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/owned_conversations",
            get_with(get_user_owned_conversations, |op| {
                op.id("GetOwnedConversations")
                    .description("Gets a list of the conversations a user owns")
                    .response::<201, Json<PaginatedResults<Conversation>>>()
            }),
        )
        .with_state(state)
}
