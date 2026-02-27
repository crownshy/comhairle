use std::sync::Arc;

use aide::axum::{
    routing::{get_with, put_with},
    ApiRouter,
};
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
        conversation::{ConversationFilterOptions, ConversationOrderOptions},
        pagination::{OrderParams, PageOptions, PaginatedResults},
        users::{UpdateUserRequest, UpgradeAccountRequest},
    },
    routes::{
        conversations::dto::{ConversationDto, LocalizedConversationDto},
        user::dto::UserDto,
    },
    ComhairleState,
};

pub mod dto;

use super::auth::{is_user_admin, RequiredAdminUser, RequiredUser};

pub async fn get_user_owned_conversations(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    OrderParams(order_options): OrderParams<ConversationOrderOptions>,
    Query(filter_options): Query<ConversationFilterOptions>,
    Query(page_options): Query<PageOptions>,
) -> Result<(StatusCode, Json<PaginatedResults<LocalizedConversationDto>>), ComhairleError> {
    let conversations = models::conversation::list_owned(
        &state.db,
        user.id,
        page_options,
        order_options,
        filter_options,
        Some("en".to_string()),
    )
    .await?
    .into();
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

pub async fn get_conversations_user_participating_in(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
) -> Result<(StatusCode, Json<Vec<ConversationDto>>), ComhairleError> {
    let conversations = models::conversation::list_for_user_participation(&state.db, &user.id)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok((StatusCode::OK, Json(conversations)))
}

pub async fn get_user_roles(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
) -> Result<(StatusCode, Json<Vec<UserRoles>>), ComhairleError> {
    let mut roles = vec![];

    if is_user_admin(&user, &state.config) {
        roles.push(UserRoles {
            resource: ResourceType::Site,
            roles: vec![ResourceRole::Admin],
        });
    }

    Ok((StatusCode::OK, Json(roles)))
}

pub async fn update_user_details(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Json(update_request): Json<UpdateUserRequest>,
) -> Result<(StatusCode, Json<UserDto>), ComhairleError> {
    let updated_user = models::users::update_user(&user.id, &update_request, &state.db).await?;
    let user: UserDto = updated_user.into();
    Ok((StatusCode::OK, Json(user)))
}

pub async fn upgrade_account(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Json(upgrade_request): Json<UpgradeAccountRequest>,
) -> Result<(StatusCode, Json<UserDto>), ComhairleError> {
    let upgraded_user =
        models::users::upgrade_account(&user.id, &upgrade_request, &state.db).await?;
    let user: UserDto = upgraded_user.into();
    Ok((StatusCode::OK, Json(user)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/roles",
            get_with(get_user_roles, |op| {
                op.id("GetUserRoles")
                    .tag("User")
                    .description("Gets a list of roles the current user has")
                    .security_requirement("JWT")
                    .response::<201, Json<Vec<UserRoles>>>()
            }),
        )
        .api_route(
            "/conversations",
            get_with(get_conversations_user_participating_in, |op| {
                op.id("GetConversationsUserIsParticipatingIn")
                    .tag("User")
                    .description(
                        "Returns a list of all the conversations the user has taken part in",
                    )
                    .security_requirement("JWT")
                    .response::<201, Json<Vec<ConversationDto>>>()
            }),
        )
        .api_route(
            "/owned_conversations",
            get_with(get_user_owned_conversations, |op| {
                op.id("GetOwnedConversations")
                    .tag("User")
                    .description("Gets a list of the conversations a user owns")
                    .security_requirement("JWT")
                    .response::<201, Json<PaginatedResults<LocalizedConversationDto>>>()
            }),
        )
        .api_route(
            "/details",
            put_with(update_user_details, |op| {
                op.id("UpdateUserDetails")
                    .tag("User")
                    .description("Update user details (username and/or password)")
                    .security_requirement("JWT")
                    .response::<200, Json<UserDto>>()
            }),
        )
        .api_route(
            "/upgrade",
            put_with(upgrade_account, |op| {
                op.id("UpgradeAccount")
                    .tag("User")
                    .description("Upgrade anonymous account to email/password account")
                    .security_requirement("JWT")
                    .response::<200, Json<UserDto>>()
            }),
        )
        .with_state(state)
}
