use std::sync::Arc;

use aide::axum::{
    ApiRouter,
    routing::{get_with, put_with},
};
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    ComhairleState,
    error::ComhairleError,
    models::{
        self,
        conversation::{ConversationFilterOptions, ConversationOrderOptions},
        media::{FromWithMedia, MediaResolver},
        organization::{self, OrganizationFilterOptions, OrganizationOrderOptions},
        pagination::{OrderParams, PageOptions, PaginatedResults},
        permissions::{Action, can_perform_resource_action},
        users::{UpdateUserRequest, UpgradeAccountRequest},
    },
    routes::{
        conversations::dto::LocalizedConversationDto, organizations::dto::LocalizedOrganizationDto,
        user::dto::UserDto,
    },
};

pub mod dto;

use super::auth::{RequiredAdminUser, RequiredUser, is_user_admin};
use super::translations::LocaleExtractor;

#[instrument(err(Debug), skip(state))]
pub async fn get_user_owned_conversations(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    OrderParams(order_options): OrderParams<ConversationOrderOptions>,
    Query(filter_options): Query<ConversationFilterOptions>,
    Query(page_options): Query<PageOptions>,
) -> Result<(StatusCode, Json<PaginatedResults<LocalizedConversationDto>>), ComhairleError> {
    let results = models::conversation::list_owned(
        &state.db,
        user.id,
        page_options,
        order_options,
        filter_options,
        Some("en".to_string()),
    )
    .await?;

    let media = MediaResolver::load(
        &state.db,
        &results
            .records
            .iter()
            .filter_map(|c| c.image)
            .collect::<Vec<_>>(),
    )
    .await?;

    let results_with_media: PaginatedResults<LocalizedConversationDto> =
        FromWithMedia::from_with_media(
            results,
            &media,
            &state.config.default_conversation_image_url,
        );

    Ok((StatusCode::OK, Json(results_with_media)))
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct PermittedConversationsQuery {
    pub role_name: String,
}

#[instrument(err(Debug), skip(state))]
pub async fn get_user_permitted_conversations(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    OrderParams(order_options): OrderParams<ConversationOrderOptions>,
    Query(filter_options): Query<ConversationFilterOptions>,
    Query(page_options): Query<PageOptions>,
    Query(PermittedConversationsQuery { role_name }): Query<PermittedConversationsQuery>,
) -> Result<(StatusCode, Json<PaginatedResults<LocalizedConversationDto>>), ComhairleError> {
    let results = models::conversation::list_for_permitted_user(
        &state.db,
        user.id,
        page_options,
        order_options,
        filter_options,
        &role_name,
        Some("en".to_string()),
    )
    .await?;

    let media = MediaResolver::load(
        &state.db,
        &results
            .records
            .iter()
            .filter_map(|c| c.image)
            .collect::<Vec<_>>(),
    )
    .await?;

    let results_with_media: PaginatedResults<LocalizedConversationDto> =
        FromWithMedia::from_with_media(
            results,
            &media,
            &state.config.default_conversation_image_url,
        );

    Ok((StatusCode::OK, Json(results_with_media)))
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

#[instrument(err(Debug), skip(state))]
pub async fn get_conversations_user_participating_in(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<(StatusCode, Json<Vec<LocalizedConversationDto>>), ComhairleError> {
    let conversations =
        models::conversation::list_for_user_participation(&state.db, &user.id, &locale).await?;

    let media = MediaResolver::load(
        &state.db,
        &conversations
            .iter()
            .filter_map(|c| c.image)
            .collect::<Vec<_>>(),
    )
    .await?;

    let conversations = conversations
        .into_iter()
        .map(|c| {
            FromWithMedia::from_with_media(c, &media, &state.config.default_conversation_image_url)
        })
        .collect();

    Ok((StatusCode::OK, Json(conversations)))
}

#[instrument(err(Debug), skip(state))]
pub async fn get_user_roles(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
) -> Result<(StatusCode, Json<Vec<UserRoles>>), ComhairleError> {
    let mut roles = vec![];

    if is_user_admin(&state, &user).await {
        roles.push(UserRoles {
            resource: ResourceType::Site,
            roles: vec![ResourceRole::Admin],
        });
    }

    Ok((StatusCode::OK, Json(roles)))
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserOrganizationAccess {
    pub organization: LocalizedOrganizationDto,
    pub is_associated: bool,
    pub can_update: bool,
    pub can_delete: bool,
    pub can_manage_team: bool,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserOrganizationsResponse {
    pub organizations: Vec<UserOrganizationAccess>,
    pub can_create_organization: bool,
}

#[instrument(err(Debug), skip(state))]
pub async fn get_user_organizations(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<(StatusCode, Json<UserOrganizationsResponse>), ComhairleError> {
    let results = organization::list(
        &state.db,
        PageOptions {
            offset: None,
            limit: Some(500),
        },
        OrganizationFilterOptions::default(),
        OrganizationOrderOptions::default(),
        &locale,
    )
    .await?;

    let all_organizations = results
        .records
        .into_iter()
        .map(LocalizedOrganizationDto::from)
        .collect::<Vec<_>>();

    let mut organizations = Vec::with_capacity(all_organizations.len());
    for organization in &all_organizations {
        let is_associated = user
            .organization_id
            .is_some_and(|organization_id| organization_id == organization.id);

        let can_update = can_perform_resource_action(
            &state,
            &organization.id,
            Action::OrganizationUpdate,
            &user.id,
            user.organization_id.as_ref(),
            None,
        )
        .await?;

        let can_delete = can_perform_resource_action(
            &state,
            &organization.id,
            Action::OrganizationDelete,
            &user.id,
            user.organization_id.as_ref(),
            None,
        )
        .await?;

        organizations.push(UserOrganizationAccess {
            organization: organization.clone(),
            is_associated,
            can_update,
            can_delete,
            can_manage_team: can_update,
        });
    }

    let can_create_organization = can_perform_resource_action(
        &state,
        &Uuid::nil(),
        Action::OrganizationCreate,
        &user.id,
        user.organization_id.as_ref(),
        None,
    )
    .await?;

    Ok((
        StatusCode::OK,
        Json(UserOrganizationsResponse {
            organizations,
            can_create_organization,
        }),
    ))
}

#[instrument(err(Debug), skip(state))]
pub async fn update_user_details(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Json(update_request): Json<UpdateUserRequest>,
) -> Result<(StatusCode, Json<UserDto>), ComhairleError> {
    let updated_user = models::users::update_user(&user.id, &update_request, &state.db).await?;
    let user: UserDto = updated_user.into();
    Ok((StatusCode::OK, Json(user)))
}

#[instrument(err(Debug), skip(state))]
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
                    .response::<200, Json<Vec<LocalizedConversationDto>>>()
            }),
        )
        .api_route(
            "/owned_conversations",
            get_with(get_user_owned_conversations, |op| {
                op.id("GetOwnedConversations")
                    .tag("User")
                    .description("Gets a list of the conversations a user owns")
                    .security_requirement("JWT")
                    .response::<200, Json<PaginatedResults<LocalizedConversationDto>>>()
            }),
        )
        .api_route(
            "/permitted_conversations",
            get_with(get_user_permitted_conversations, |op| {
                op.id("GetPermittedConversations")
                    .tag("User")
                    .description("Gets a list of the conversations a user is permitted access to")
                    .security_requirement("JWT")
                    .response::<200, Json<PaginatedResults<LocalizedConversationDto>>>()
            }),
        )
        .api_route(
            "/organizations",
            get_with(get_user_organizations, |op| {
                op.id("GetUserOrganizations")
                    .tag("User")
                    .description("Gets the organizations associated with the current user and those they can manage")
                    .security_requirement("JWT")
                    .response::<200, Json<UserOrganizationsResponse>>()
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
