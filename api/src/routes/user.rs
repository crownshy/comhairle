use std::sync::Arc;

use aide::axum::{
    ApiRouter,
    routing::{get_with, put_with},
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
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
        permissions::{
            PermissionAction, PermissionRole, SYSTEM_RESOURCE_ID, can_perform_action,
            conversation as conversation_permissions, has_resource_permission,
            organization as organization_permissions, system,
        },
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

#[instrument(err(Debug), skip(state))]
pub async fn get_user_permitted_conversations(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    OrderParams(order_options): OrderParams<ConversationOrderOptions>,
    Query(filter_options): Query<ConversationFilterOptions>,
    Query(page_options): Query<PageOptions>,
) -> Result<(StatusCode, Json<PaginatedResults<LocalizedConversationDto>>), ComhairleError> {
    let is_super_admin =
        has_resource_permission(&state, system::Role::SuperAdmin.system_triplet()?, &user.id)
            .await?;

    let results = models::conversation::list_for_permitted_user(
        &state.db,
        user.id,
        is_super_admin,
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

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(untagged)]
pub enum UserAction {
    Conversation(conversation_permissions::Action),
    Organization(organization_permissions::Action),
    System(system::Action),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserActions {
    pub resource_type: models::permissions::ResourceType,
    pub resource_id: Uuid,
    pub actions: Vec<UserAction>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserActionsPath {
    pub resource_type: models::permissions::ResourceType,
    pub resource_id: Uuid,
}

async fn permitted_actions<Action: PermissionAction + IntoEnumIterator>(
    state: &Arc<ComhairleState>,
    resource_id: &Uuid,
    user_id: &Uuid,
    owner_id: Option<&Uuid>,
) -> Result<Vec<Action>, ComhairleError> {
    let mut actions = Vec::new();
    for action in Action::iter() {
        if can_perform_action(state, resource_id, action, user_id, owner_id).await? {
            actions.push(action);
        }
    }
    Ok(actions)
}

#[instrument(err(Debug), skip(state))]
pub async fn get_user_actions(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(path): Path<UserActionsPath>,
) -> Result<Json<UserActions>, ComhairleError> {
    use models::permissions::ResourceType;

    let actions = match path.resource_type {
        ResourceType::Conversation => {
            let conversation =
                models::conversation::get_by_id(&state.db, &path.resource_id).await?;
            permitted_actions::<conversation_permissions::Action>(
                &state,
                &conversation.id,
                &user.id,
                Some(&conversation.owner_id),
            )
            .await?
            .into_iter()
            .map(UserAction::Conversation)
            .collect()
        }
        ResourceType::Organization => {
            models::organization::get_by_id(&state.db, &path.resource_id).await?;
            permitted_actions::<organization_permissions::Action>(
                &state,
                &path.resource_id,
                &user.id,
                None,
            )
            .await?
            .into_iter()
            .map(UserAction::Organization)
            .collect()
        }
        ResourceType::System => {
            if path.resource_id != SYSTEM_RESOURCE_ID {
                return Err(ComhairleError::BadRequest(
                    "System resource ID must be nil".into(),
                ));
            }
            permitted_actions::<system::Action>(&state, &SYSTEM_RESOURCE_ID, &user.id, None)
                .await?
                .into_iter()
                .map(UserAction::System)
                .collect()
        }
    };

    Ok(Json(UserActions {
        resource_type: path.resource_type,
        resource_id: path.resource_id,
        actions,
    }))
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

        let can_update = can_perform_action(
            &state,
            &organization.id,
            organization_permissions::Action::Update,
            &user.id,
            None,
        )
        .await?;

        let can_delete = can_perform_action(
            &state,
            &organization.id,
            organization_permissions::Action::Delete,
            &user.id,
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

    let can_create_organization = can_perform_action(
        &state,
        &Uuid::nil(),
        system::Action::OrganizationCreate,
        &user.id,
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
            "/actions/{resource_type}/{resource_id}",
            get_with(get_user_actions, |op| {
                op.id("GetUserActions")
                    .tag("User")
                    .description("Gets the current user's effective actions on a resource")
                    .security_requirement("JWT")
                    .response::<200, Json<UserActions>>()
            }),
        )
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;

    use serde_json::json;
    use sqlx::PgPool;

    use crate::models::model_test_helpers::{
        get_random_conversation_id, get_random_organization_id,
    };
    use crate::models::permissions::{ActorId, GrantRoleRequest, grant_role};
    use crate::models::user_group;
    use crate::setup_server;
    use crate::test_helpers::{UserSession, test_state};

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn organization_members_can_read_details_without_admin_access(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let app = setup_server(state.clone()).await?;
        let mut admin = UserSession::new_admin();
        admin.signup(&app).await?;
        let admin_id = admin.id.ok_or("missing administrator id")?;
        let organization_id = get_random_organization_id(&app, &mut admin).await?;
        let other_organization_id = get_random_organization_id(&app, &mut admin).await?;
        let details_url = format!("/organizations/{organization_id}");
        let actions_url = format!("/user/actions/organization/{organization_id}");

        let mut member = UserSession::new_guest();
        member.signup_guest(&app).await?;
        let member_id = member.id.ok_or("missing member id")?;
        let (status, _, _) = member.get(&app, &details_url).await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        user_group::set_organization_member(&state, organization_id, member_id, admin_id, false)
            .await?;
        let (status, response, _) = member.get(&app, &details_url).await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(response["id"], json!(organization_id));
        let (status, response, _) = member.get(&app, &actions_url).await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(response["actions"], json!(["organization_read"]));

        for url in [
            format!("/organizations/{organization_id}/team"),
            format!("/permissions/organization/{organization_id}"),
            format!("/organizations/{other_organization_id}"),
        ] {
            let (status, _, _) = member.get(&app, &url).await?;
            assert_eq!(status, StatusCode::FORBIDDEN, "{url}");
        }

        let (status, _, _) = admin
            .get(&app, &format!("/organizations/{organization_id}/team"))
            .await?;
        assert_eq!(status, StatusCode::OK);

        user_group::remove_organization_member(&state, organization_id, member_id, admin_id)
            .await?;
        let (status, _, _) = member.get(&app, &details_url).await?;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let (status, response, _) = member.get(&app, &actions_url).await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(response["actions"], json!([]));
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn user_actions_include_ownership_direct_and_inherited_roles(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let app = setup_server(state.clone()).await?;
        let mut owner = UserSession::new_admin();
        owner.signup(&app).await?;
        let owner_id = owner.id.ok_or("missing owner id")?;
        let conversation_id = get_random_conversation_id(&app, &mut owner).await?;
        let organization_id = get_random_organization_id(&app, &mut owner).await?;
        let url = format!("/user/actions/conversation/{conversation_id}");

        let (status, response, _) = owner.get(&app, &url).await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(response["resourceId"], json!(conversation_id));
        assert_eq!(response["resourceType"], "conversation");
        assert_eq!(
            response["actions"],
            json!(conversation_permissions::Action::iter().collect::<Vec<_>>())
        );

        let mut editor = UserSession::new_guest();
        editor.signup_guest(&app).await?;
        let editor_id = editor.id.ok_or("missing editor id")?;
        let (status, response, _) = editor.get(&app, &url).await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(response["actions"], json!([]));

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: ActorId::User(editor_id),
                permission_triplet: conversation_permissions::Role::ContentEditor
                    .triplet(&conversation_id)?,
                granted_by: &owner_id,
                grant_reason: "Actions endpoint test",
            },
        )
        .await?;
        let (status, response, _) = editor.get(&app, &url).await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            response["actions"],
            json!(["conversation_read", "conversation_update"])
        );

        let permissions_url = format!("/permissions/conversation/{conversation_id}");
        let (status, _, _) = editor.get(&app, &permissions_url).await?;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let revoke_url = format!("{permissions_url}?user_id={editor_id}&role_name=content_editor");
        let (status, _, _) = editor.delete(&app, &revoke_url).await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        user_group::set_organization_member(&state, organization_id, editor_id, owner_id, false)
            .await?;
        let group_id = user_group::organization_group(&state.db, organization_id).await?;
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: ActorId::Group(group_id),
                permission_triplet: conversation_permissions::Role::Moderator
                    .triplet(&conversation_id)?,
                granted_by: &owner_id,
                grant_reason: "Inherited actions endpoint test",
            },
        )
        .await?;
        let (status, response, _) = editor.get(&app, &url).await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            response["actions"],
            json!([
                "conversation_read",
                "conversation_update",
                "conversation_moderate"
            ])
        );

        user_group::remove_organization_member(&state, organization_id, editor_id, owner_id)
            .await?;
        let (status, response, _) = editor.get(&app, &url).await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            response["actions"],
            json!(["conversation_read", "conversation_update"])
        );

        let (status, _, _) = owner.delete(&app, &revoke_url).await?;
        assert_eq!(status, StatusCode::OK);
        let (status, response, _) = editor.get(&app, &url).await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(response["actions"], json!([]));
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn user_actions_support_system_and_organization_resources(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let app = setup_server(state.clone()).await?;

        let mut admin = UserSession::new_admin();
        admin.signup(&app).await?;
        let admin_id = admin.id.ok_or("missing admin id")?;

        let organization_id = get_random_organization_id(&app, &mut admin).await?;

        let (status, response, _) = admin
            .get(
                &app,
                &format!("/user/actions/organization/{organization_id}"),
            )
            .await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            response["actions"],
            json!(organization_permissions::Action::iter().collect::<Vec<_>>())
        );

        let mut translator = UserSession::new_guest();
        translator.signup_guest(&app).await?;
        let translator_id = translator.id.ok_or("missing translator id")?;

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: ActorId::User(translator_id),
                permission_triplet: system::Role::Translator.system_triplet()?,
                granted_by: &admin_id,
                grant_reason: "System actions endpoint test",
            },
        )
        .await?;

        let (status, response, _) = translator
            .get(&app, &format!("/user/actions/system/{SYSTEM_RESOURCE_ID}"))
            .await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(response["actions"], json!(["translate"]));

        let (_, response, _) = translator
            .get(
                &app,
                &format!("/user/actions/organization/{organization_id}"),
            )
            .await?;
        assert_eq!(response["actions"], json!([]));

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn user_actions_reject_invalid_resources_and_require_authentication(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let app = setup_server(state).await?;

        let mut session = UserSession::new_guest();
        let (status, _, _) = session
            .get(&app, &format!("/user/actions/system/{SYSTEM_RESOURCE_ID}"))
            .await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        session.signup_guest(&app).await?;
        let (status, _, _) = session
            .get(&app, &format!("/user/actions/system/{}", Uuid::new_v4()))
            .await?;
        assert_eq!(status, StatusCode::BAD_REQUEST);

        for resource_type in ["conversation", "organization"] {
            let (status, _, _) = session
                .get(
                    &app,
                    &format!("/user/actions/{resource_type}/{}", Uuid::new_v4()),
                )
                .await?;
            assert_eq!(status, StatusCode::NOT_FOUND);
        }

        let (status, _, _) = session
            .get(&app, &format!("/user/actions/unknown/{SYSTEM_RESOURCE_ID}"))
            .await?;
        assert_eq!(status, StatusCode::BAD_REQUEST);

        Ok(())
    }
}
