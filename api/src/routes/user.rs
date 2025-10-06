use std::sync::Arc;

use aide::axum::{routing::get_with, ApiRouter};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        self,
        conversation::{Conversation, ConversationFilterOptions, ConversationOrderOptions},
        pagination::{OrderParams, PageOptions, PaginatedResults},
    },
    ComhairleState,
};

use super::auth::{RequiredAdminUser, RequiredUser};

pub async fn get_user_owned_conversations(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
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

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub enum ResourceRole {
    Admin,
    SuperAdmin,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub enum ResourceType {
    Site,
    Conversation(Uuid),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct UserRoles {
    pub resource: ResourceType,
    pub roles: Vec<ResourceRole>,
}

pub async fn get_user_roles(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
) -> Result<(StatusCode, Json<Vec<UserRoles>>), ComhairleError> {
    let mut roles = vec![];

    if let (Some(admin_users), Some(email)) = (&state.config.admin_users, &user.email) {
        if admin_users.contains(&email) {
            roles.push(UserRoles {
                resource: ResourceType::Site,
                roles: vec![ResourceRole::Admin],
            });
        }
    };

    Ok((StatusCode::OK, Json(roles)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/roles",
            get_with(get_user_roles, |op| {
                op.id("GetUserRoles")
                    .description("Gets a list of roles the current user has")
                    .response::<201, Json<Vec<UserRoles>>>()
            }),
        )
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
