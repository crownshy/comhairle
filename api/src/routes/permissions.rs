use std::sync::Arc;

use aide::axum::{
    ApiRouter,
    routing::{delete_with, get_with, post_with, put_with},
};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    middleware::{from_fn, from_fn_with_state},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::middleware::permissions::{
    PermissionRequirement, PermissionUserFilter, authenticate, authorize, authorize_or_self,
};
use crate::models::pagination::{PageOptions, PaginatedResults};
use crate::models::permissions::system;
use crate::models::permissions::{
    self, ActorId, GrantRoleRequest, ListPermissionsFilters, PermissionManagementAction,
    PermissionTargetResource, PermissionTriplet, RevokeRoleRequest, SystemResource,
    UserWithPermissionDto, grant_role, list_permissions, revoke_role,
};
use crate::models::users;
use crate::routes::auth::RequiredUser;

/// Represents the resource type and ID for a permission operation.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TargetResourceId {
    pub resource_type: String,
    pub resource_id: Uuid,
}

/// Represents a request body for granting a permission to a user or organization.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct GrantPermissionBody {
    pub user_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub user_email: Option<String>,
    pub role_name: String,
    pub grant_reason: String,
}

/// Represents a request query for revoking a permission from a user or organization.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RevokePermissionQuery {
    pub user_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub role_name: String,
}

/// Represents a request query for listing permissions, with optional filters.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ListPermissionsQuery {
    pub user_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub role_name: Option<String>,
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

/// Represents a request query for listing permissions by action, with optional filters.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ListPermissionsByActionQuery {
    pub user_id: Option<Uuid>,
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

impl PermissionUserFilter for Query<ListPermissionsByActionQuery> {
    fn is_caller(&self, caller_id: &Uuid) -> bool {
        self.0.user_id.is_none_or(|user_id| user_id == *caller_id)
    }
}

/// Resolves the actor from the optional user_id / organization_id fields.
///
/// # Errors
///
/// * Returns [`ComhairleError::BadRequest`] if more than one of user_id, organization_id or user_email are provided.
/// * Returns [`ComhairleError::NoUserFoundForEmail`] if no user found for a given user_email.
async fn resolve_actor(
    db: &PgPool,
    user_id: Option<Uuid>,
    organization_id: Option<Uuid>,
    user_email: Option<String>,
    allow_none: bool,
) -> Result<Option<ActorId>, ComhairleError> {
    match (user_id, organization_id, user_email) {
        (Some(uid), None, None) => Ok(Some(ActorId::User(uid))),
        (None, Some(oid), None) => Ok(Some(ActorId::Group(
            crate::models::user_group::organization_group(db, oid).await?,
        ))),
        (None, None, Some(u_email)) => {
            let user = users::get_user_by_email(&u_email, db).await?;

            Ok(Some(ActorId::User(user.id)))
        }
        (None, None, None) if allow_none => Ok(None),
        _ => Err(ComhairleError::BadRequest(
            "Only one of user_id, organization_id or user_email can be provided".into(),
        )),
    }
}

/// Grants a role to a user or organization on a specific resource.
///
/// # Errors
///
/// * Returns [`ComhairleError::RoleAlreadyGranted`] if the role was already granted to the actor.
/// * Returns [`ComhairleError::BadRequest`] if more than one of user_id, organization_id or user_email are provided in the request body.
/// * Returns [`ComhairleError::DatabaseError`] if there is an error querying the database.
#[instrument(err(Debug), skip(state))]
async fn grant(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(caller): RequiredUser,
    Path(path): Path<TargetResourceId>,
    Json(body): Json<GrantPermissionBody>,
) -> Result<(StatusCode, Json<permissions::ResourcePermission>), ComhairleError> {
    let actor_id = resolve_actor(
        &state.db,
        body.user_id,
        body.organization_id,
        body.user_email,
        false,
    )
    .await?
    .unwrap();

    let permission_triplet =
        PermissionTriplet(&path.resource_type, &path.resource_id, &body.role_name);
    let permission = grant_role(
        &state,
        GrantRoleRequest {
            actor_id,
            permission_triplet,
            granted_by: &caller.id,
            grant_reason: &body.grant_reason,
        },
    )
    .await?;

    match actor_id {
        ActorId::User(uid) => {
            let user = users::get_user_by_id(&uid, &state.db).await?;

            if let Some(email) = user.email {
                state
                    .mailer
                    .send_permission_notification_email(&email, &permission, "granted")?;
            }
        }
        ActorId::Group(_) => {}
    }

    Ok((StatusCode::CREATED, Json(permission)))
}

/// Revokes a role from a user or organization on a specific resource.
///
/// # Errors
///
/// * Returns [`ComhairleError::RoleNotFound`] if the role does not exist for the resource type.
/// * Returns [`ComhairleError::BadRequest`] if both user_id and organization_id are provided in the query parameters.
/// * Returns [`ComhairleError::DatabaseError`] if there is an error querying the database.
#[instrument(err(Debug), skip(state))]
async fn revoke(
    State(state): State<Arc<ComhairleState>>,
    Path(path): Path<TargetResourceId>,
    Query(query): Query<RevokePermissionQuery>,
) -> Result<StatusCode, ComhairleError> {
    let actor_id = resolve_actor(&state.db, query.user_id, query.organization_id, None, false)
        .await?
        .unwrap();

    let permission_triplet =
        PermissionTriplet(&path.resource_type, &path.resource_id, &query.role_name);
    revoke_role(
        &state,
        RevokeRoleRequest {
            actor_id,
            permission_triplet,
        },
    )
    .await?;

    Ok(StatusCode::OK)
}

/// Lists permissions for all resources with optional filtering. Supports pagination via offset and limit query parameters.
///
/// # Errors
///
/// * Returns [`ComhairleError::BadRequest`] if both user_id and organization_id are provided.
/// * Returns [`ComhairleError::DatabaseError`] if there is an error querying the database.
#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    Query(query): Query<ListPermissionsQuery>,
) -> Result<
    (
        StatusCode,
        Json<PaginatedResults<permissions::ResourcePermission>>,
    ),
    ComhairleError,
> {
    let actor = resolve_actor(&state.db, query.user_id, query.organization_id, None, true).await?;
    let page_options = PageOptions {
        limit: query.limit,
        offset: query.offset,
    };

    let request = ListPermissionsFilters {
        actor,
        role_name: query.role_name.as_deref(),
        page_options,
        ..Default::default()
    };

    let page = list_permissions(&state, request).await?;

    Ok((StatusCode::OK, Json(page)))
}

/// Lists the resources of the specified type that the caller can perform the specified action on.
///
/// # Errors
///
/// * Returns [`ComhairleError::BadRequest`] if the action is not provided or invalid.
/// * Returns [`ComhairleError::DatabaseError`] if there is an error querying the database.
#[instrument(err(Debug), skip(state))]
async fn list_permissions_by_action(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(caller): RequiredUser,
    Path(action): Path<String>,
    Query(query): Query<ListPermissionsByActionQuery>,
) -> Result<(StatusCode, Json<Vec<permissions::ResourcePermission>>), ComhairleError> {
    let permissions = permissions::list_permissions_by_action(
        &state.db,
        query.user_id.unwrap_or(caller.id),
        &action,
    )
    .await?;

    Ok((StatusCode::OK, Json(permissions)))
}

/// Lists permissions for a specific resource with optional filtering. Supports pagination via offset and limit query parameters.
///
/// # Errors
///
/// * Returns [`ComhairleError::BadRequest`] if both user_id and organization_id are provided.
/// * Returns [`ComhairleError::DatabaseError`] if there is an error querying the database.
#[instrument(err(Debug), skip(state))]
async fn list_for_resource(
    State(state): State<Arc<ComhairleState>>,
    Path(path): Path<TargetResourceId>,
    Query(query): Query<ListPermissionsQuery>,
) -> Result<
    (
        StatusCode,
        Json<PaginatedResults<permissions::ResourcePermission>>,
    ),
    ComhairleError,
> {
    let actor = resolve_actor(&state.db, query.user_id, query.organization_id, None, true).await?;
    let page_options = PageOptions {
        limit: query.limit,
        offset: query.offset,
    };

    let request = ListPermissionsFilters {
        actor,
        role_name: query.role_name.as_deref(),
        page_options,
        resource_type: Some(&path.resource_type),
        resource_id: Some(&path.resource_id),
    };

    let page = list_permissions(&state, request).await?;

    Ok((StatusCode::OK, Json(page)))
}

#[instrument(err(Debug), skip(state))]
async fn list_users_with_permission(
    State(state): State<Arc<ComhairleState>>,
    Path(path): Path<TargetResourceId>,
    Query(query): Query<ListPermissionsQuery>,
) -> Result<(StatusCode, Json<Vec<UserWithPermissionDto>>), ComhairleError> {
    let users_with_permission = permissions::list_users_with_permission(
        &state.db,
        &path.resource_type,
        path.resource_id,
        query.role_name.as_deref(),
    )
    .await?;

    Ok((StatusCode::OK, Json(users_with_permission)))
}

/// Creates the permissions API router with all the defined routes and their corresponding handlers.
#[derive(Debug, Deserialize, JsonSchema)]
struct AssignmentPath {
    resource_type: permissions::ResourceType,
    resource_id: Uuid,
    recipient_type: String,
    recipient_id: Uuid,
}

impl AssignmentPath {
    fn recipient(&self) -> Result<permissions::assignments::Recipient, ComhairleError> {
        match self.recipient_type.as_str() {
            "user" => Ok(permissions::assignments::Recipient::User(self.recipient_id)),
            "group" => Ok(permissions::assignments::Recipient::Group(
                self.recipient_id,
            )),
            _ => Err(ComhairleError::BadRequest(
                "Recipient must be a User or UserGroup".into(),
            )),
        }
    }
}

#[instrument]
async fn catalogue(Path(resource_type): Path<permissions::ResourceType>) -> Json<Vec<String>> {
    Json(permissions::assignments::role_catalogue(resource_type))
}

#[instrument(err(Debug), skip(state))]
async fn load_assignments(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(caller): RequiredUser,
    Path(path): Path<AssignmentPath>,
) -> Result<Json<permissions::assignments::Snapshot>, ComhairleError> {
    Ok(Json(
        permissions::assignments::load(
            &state,
            path.resource_type,
            path.resource_id,
            caller.id,
            path.recipient()?,
        )
        .await?,
    ))
}

#[instrument(err(Debug), skip(state, request))]
async fn save_assignments(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(caller): RequiredUser,
    Path(path): Path<AssignmentPath>,
    Json(request): Json<permissions::assignments::SaveAssignments>,
) -> Result<Json<permissions::assignments::Snapshot>, ComhairleError> {
    Ok(Json(
        permissions::assignments::save(
            &state,
            path.resource_type,
            path.resource_id,
            caller.id,
            path.recipient()?,
            request,
        )
        .await?,
    ))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/{resource_type}/roles",
            get_with(catalogue, |operation| {
                operation
                    .id("GetPermissionRoles")
                    .tag("Permissions")
                    .security_requirement("JWT")
            })
            .route_layer(from_fn(authenticate::<RequiredUser>)),
        )
        .api_route(
            "/{resource_type}/{resource_id}/assignments/{recipient_type}/{recipient_id}",
            get_with(load_assignments, |operation| {
                operation
                    .id("GetPermissionAssignments")
                    .tag("Permissions")
                    .security_requirement("JWT")
            })
            .route_layer(from_fn_with_state(
                PermissionRequirement::<PermissionTargetResource>::new(
                    PermissionManagementAction::List,
                ),
                authorize::<PermissionTargetResource>,
            )),
        )
        .api_route(
            "/{resource_type}/{resource_id}/assignments/{recipient_type}/{recipient_id}",
            put_with(save_assignments, |operation| {
                operation
                    .id("SavePermissionAssignments")
                    .tag("Permissions")
                    .security_requirement("JWT")
            })
            .route_layer(from_fn_with_state(
                PermissionRequirement::<PermissionTargetResource>::new(
                    PermissionManagementAction::Grant,
                ),
                authorize::<PermissionTargetResource>,
            )),
        )
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListPermissions")
                    .tag("Permissions")
                    .summary("List all permissions")
                    .description(
                        "Returns role assignments using offset-based pagination. \
                        Optionally filter by user_id, organization_id, or role_name. \
                        Use the `offset` and `limit` query params to page through results.",
                    )
                    .security_requirement("JWT")
                    .response::<200, Json<PaginatedResults<permissions::ResourcePermission>>>()
            })
            .route_layer(from_fn_with_state(
                PermissionRequirement::<SystemResource>::new(system::Action::ListPermission),
                authorize::<SystemResource>,
            )),
        )
        .api_route(
            "/by-action/{action}",
            get_with(list_permissions_by_action, |op| {
                op.id("ListPermissionsByAction")
                    .tag("Permissions")
                    .summary("List resources by action")
                    .description(
                        "Returns resources of the specified type that the caller can perform \
                        the specified action on. Optionally filter by user_id. Use the `offset` \
                        and `limit` query params to page through results.",
                    )
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<permissions::ResourcePermission>>>()
            })
            .route_layer(from_fn_with_state(
                PermissionRequirement::<SystemResource>::new(system::Action::ListPermission),
                authorize_or_self::<SystemResource, Query<ListPermissionsByActionQuery>>,
            )),
        )
        .api_route(
            "/{resource_type}/{resource_id}",
            get_with(list_for_resource, |op| {
                op.id("ListResourcePermissions")
                    .tag("Permissions")
                    .summary("List permissions for a resource")
                    .description(
                        "Returns role assignments for a specific resource using \
                        offset-based pagination. Optionally filter by user_id, \
                        organization_id, or role_name. The caller must own the \
                        resource or have permission to list its role assignments.",
                    )
                    .security_requirement("JWT")
                    .response::<200, Json<PaginatedResults<permissions::ResourcePermission>>>()
            })
            .route_layer(from_fn_with_state(
                PermissionRequirement::<PermissionTargetResource>::new(
                    PermissionManagementAction::List,
                ),
                authorize::<PermissionTargetResource>,
            )),
        )
        .api_route(
            "/{resource_type}/{resource_id}",
            post_with(grant, |op| {
                op.id("GrantPermission")
                    .tag("Permissions")
                    .summary("Grant a role on a resource")
                    .description(
                        "Grants a role to a user or organisation on a resource. \
                        The caller must own the resource or have permission to grant roles on it.",
                    )
                    .security_requirement("JWT")
                    .response::<201, Json<permissions::ResourcePermission>>()
            })
            .route_layer(from_fn_with_state(
                PermissionRequirement::<PermissionTargetResource>::new(
                    PermissionManagementAction::Grant,
                ),
                authorize::<PermissionTargetResource>,
            )),
        )
        .api_route(
            "/{resource_type}/{resource_id}",
            delete_with(revoke, |op| {
                op.id("RevokePermission")
                    .tag("Permissions")
                    .summary("Revoke a role from a resource")
                    .description(
                        "Revokes a role from a user or organisation on a resource. \
                        The actor (user_id or organization_id) and role_name are \
                        provided as query parameters. The caller must own the \
                        resource or have permission to revoke roles on it.",
                    )
                    .security_requirement("JWT")
                    .response::<200, ()>()
            })
            .route_layer(from_fn_with_state(
                PermissionRequirement::<PermissionTargetResource>::new(
                    PermissionManagementAction::Revoke,
                ),
                authorize::<PermissionTargetResource>,
            )),
        )
        .api_route(
            "/{resource_type}/{resource_id}/users",
            get_with(list_users_with_permission, |op| {
                op.id("ListUsersWithPermission")
                    .tag("Permissions")
                    .summary("List users with permissions")
                    .description(
                        "List users with a give permission (role + resource_type) \
                        for a given resource",
                    )
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<UserWithPermissionDto>>>()
            })
            .route_layer(from_fn_with_state(
                PermissionRequirement::<PermissionTargetResource>::new(
                    PermissionManagementAction::List,
                ),
                authorize::<PermissionTargetResource>,
            )),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use crate::models::model_test_helpers::get_random_conversation_id;
    use crate::models::pagination::PaginatedResults;
    use crate::models::permissions::{
        self, ActorId, GrantRoleRequest, ListPermissionsFilters, PermissionRole, PermissionTriplet,
        ResourcePermission, grant_role, has_resource_permission, list_permissions, system::Role,
    };
    use crate::routes::permissions::{
        GrantPermissionBody, ListPermissionsQuery, RevokePermissionQuery,
    };
    use crate::test_helpers::{test_config, test_state};
    use crate::{setup_server, test_helpers::UserSession};

    use axum::body::Body;
    use hyper::StatusCode;
    use sqlx::PgPool;
    use std::sync::Arc;

    // Role definitions for testing
    const RESOURCE_TYPE: &str = "conversation";

    struct TestRole;

    impl TestRole {
        fn name() -> &'static str {
            "editor"
        }

        fn make_triplet(resource_id: &uuid::Uuid) -> PermissionTriplet<'_> {
            PermissionTriplet(RESOURCE_TYPE, resource_id, "editor")
        }
    }

    // Helper functions
    fn revoke_url(resource: (&str, &uuid::Uuid), query: &RevokePermissionQuery) -> String {
        let mut url = format!("/permissions/{}/{}?", resource.0, resource.1);
        if let Some(user_id) = query.user_id {
            url.push_str(&format!("user_id={}&", user_id));
        }
        if let Some(org_id) = query.organization_id {
            url.push_str(&format!("organization_id={}&", org_id));
        }
        url.push_str(&format!("role_name={}&", query.role_name));
        // Remove trailing '&' or '?' if present
        url.trim_end_matches('&').trim_end_matches('?').to_string()
    }

    fn list_query_url(base_url: &str, query: &ListPermissionsQuery) -> String {
        let mut url = format!("{base_url}?");
        if let Some(user_id) = query.user_id {
            url.push_str(&format!("user_id={}&", user_id));
        }
        if let Some(org_id) = query.organization_id {
            url.push_str(&format!("organization_id={}&", org_id));
        }
        if let Some(role_name) = &query.role_name {
            url.push_str(&format!("role_name={}&", role_name));
        }
        if let Some(offset) = query.offset {
            url.push_str(&format!("offset={}&", offset));
        }
        if let Some(limit) = query.limit {
            url.push_str(&format!("limit={}&", limit));
        }
        // Remove trailing '&' or '?' if present
        url.trim_end_matches('&').trim_end_matches('?').to_string()
    }

    #[test]
    fn action_query_identifies_self_filters() {
        use axum::extract::Query;

        use crate::middleware::permissions::PermissionUserFilter;
        use crate::routes::permissions::ListPermissionsByActionQuery;

        let caller_id = uuid::Uuid::new_v4();
        for (user_id, expected) in [
            (None, true),
            (Some(caller_id), true),
            (Some(uuid::Uuid::new_v4()), false),
        ] {
            let query = Query(ListPermissionsByActionQuery {
                user_id,
                offset: None,
                limit: None,
            });
            assert_eq!(query.is_caller(&caller_id), expected);
        }
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn resource_owner_can_manage_permissions_without_system_authority(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let app = setup_server(state.clone()).await?;
        let mut owner = UserSession::new_admin();
        owner.signup(&app).await?;
        let owner_id = owner.id.ok_or("missing owner ID")?;
        assert!(
            !has_resource_permission(&state, Role::SuperAdmin.system_triplet()?, &owner_id).await?
        );
        let (_, response, _) = owner.create_random_unlaunched_conversation(&app).await?;
        let conversation: crate::routes::conversations::dto::ConversationDto =
            serde_json::from_value(response)?;
        let target = format!("/permissions/conversation/{}", conversation.id);
        let mut editor = UserSession::new_guest();
        editor.signup_guest(&app).await?;
        let editor_id = editor.id.ok_or("missing editor ID")?;
        let assignments = format!("{target}/assignments/user/{editor_id}");
        let body = GrantPermissionBody {
            user_id: Some(editor_id),
            organization_id: None,
            user_email: None,
            role_name: "content_editor".into(),
            grant_reason: "Owner granting draft access".into(),
        };
        let (status, response, _) = owner
            .post(&app, &target, serde_json::to_vec(&body)?.into())
            .await?;
        assert_eq!(status, StatusCode::CREATED, "{response}");

        for url in [
            target.clone(),
            format!("{target}/users"),
            assignments.clone(),
        ] {
            let (status, response, _) = owner.get(&app, &url).await?;
            assert_eq!(status, StatusCode::OK, "{url}: {response}");
            let (status, response, _) = editor.get(&app, &url).await?;
            assert_eq!(status, StatusCode::FORBIDDEN, "{url}: {response}");
        }
        let (_, response, _) = owner.get(&app, &assignments).await?;
        let snapshot: permissions::assignments::Snapshot = serde_json::from_value(response)?;
        assert_eq!(snapshot.roles, vec!["content_editor"]);
        let save = permissions::assignments::SaveAssignments {
            expected_version: snapshot.version,
            roles: vec![],
            grant_reason: "Owner removing draft access".into(),
        };
        let (status, response, _) = editor
            .post(&app, &target, serde_json::to_vec(&body)?.into())
            .await?;
        assert_eq!(status, StatusCode::FORBIDDEN, "{response}");
        let (status, response, _) = editor
            .put(&app, &assignments, serde_json::to_vec(&save)?.into())
            .await?;
        assert_eq!(status, StatusCode::FORBIDDEN, "{response}");
        let revoke = format!("{target}?user_id={editor_id}&role_name=content_editor");
        let (status, response, _) = editor.delete(&app, &revoke).await?;
        assert_eq!(status, StatusCode::FORBIDDEN, "{response}");

        let (status, response, _) = owner
            .put(&app, &assignments, serde_json::to_vec(&save)?.into())
            .await?;
        assert_eq!(status, StatusCode::OK, "{response}");
        let snapshot: permissions::assignments::Snapshot = serde_json::from_value(response)?;
        assert!(snapshot.roles.is_empty());
        let (status, response, _) = owner
            .post(&app, &target, serde_json::to_vec(&body)?.into())
            .await?;
        assert_eq!(status, StatusCode::CREATED, "{response}");
        let (status, response, _) = owner.delete(&app, &revoke).await?;
        assert_eq!(status, StatusCode::OK, "{response}");
        let (status, response, _) = editor
            .get_conversation(&app, &conversation.id.to_string())
            .await?;
        assert_eq!(status, StatusCode::FORBIDDEN, "{response:?}");

        let system_target = format!("/permissions/system/{}", permissions::SYSTEM_RESOURCE_ID);
        for url in ["/permissions", system_target.as_str()] {
            let (status, response, _) = owner.get(&app, url).await?;
            assert_eq!(status, StatusCode::FORBIDDEN, "{url}: {response}");
        }
        let system_body = GrantPermissionBody {
            role_name: "super_admin".into(),
            ..body
        };
        let (status, response, _) = owner
            .post(
                &app,
                &system_target,
                serde_json::to_vec(&system_body)?.into(),
            )
            .await?;
        assert_eq!(status, StatusCode::FORBIDDEN, "{response}");
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn middleware_allows_permitted_callers_to_list_another_user(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::models::model_test_helpers::{
            get_random_conversation_id, get_random_user_id, setup_default_app_and_session,
        };

        let (app, mut owner) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut owner).await?;
        let mut guest = UserSession::new_guest();
        let user_id = get_random_user_id(&app, &mut guest).await?;
        let body = GrantPermissionBody {
            user_id: Some(user_id),
            organization_id: None,
            user_email: None,
            role_name: "content_editor".into(),
            grant_reason: "Test listing another user's permissions".into(),
        };
        let (status, response, _) = owner
            .post(
                &app,
                &format!("/permissions/conversation/{conversation_id}"),
                serde_json::to_vec(&body)?.into(),
            )
            .await?;
        assert_eq!(status, StatusCode::CREATED, "{response}");
        let granted: ResourcePermission = serde_json::from_value(response)?;

        let (status, response, _) = owner
            .get(
                &app,
                &format!("/permissions/by-action/conversation_read?user_id={user_id}"),
            )
            .await?;
        assert_eq!(status, StatusCode::OK, "{response}");
        let permissions: Vec<ResourcePermission> = serde_json::from_value(response)?;
        assert_eq!(permissions.len(), 1);
        assert_eq!(permissions[0].id, granted.id);
        assert_eq!(permissions[0].user_id, Some(user_id));
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_admin_user_should_have_system_admin_role(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(test_state().db(pool).config(config).call()?);
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (_, user, _) = session.current_user(&app).await?;

        // Check if the user has the system admin role
        let has_admin_role =
            has_resource_permission(&state, Role::Admin.system_triplet()?, &user.id).await?;

        assert!(
            has_admin_role,
            "Admin users should be auto-assigned the system admin role"
        );

        Ok(())
    }

    // Grant permission
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn middleware_rejects_unauthenticated_requests(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::models::model_test_helpers::{
            get_random_conversation_id, setup_default_app_and_session,
        };

        let (app, mut owner) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut owner).await?;
        let mut anonymous = UserSession::new_admin();
        for url in [
            "/permissions".to_owned(),
            "/permissions/by-action/conversation_read".to_owned(),
            format!("/permissions/conversation/{conversation_id}"),
            format!("/conversation/{conversation_id}/cohosts"),
            format!("/conversation/{conversation_id}/moderation_policies"),
        ] {
            let (status, response, _) = anonymous.get(&app, &url).await?;
            assert_eq!(status, StatusCode::UNAUTHORIZED, "{url}: {response}");
        }
        let (status, _, _) = anonymous
            .post(
                &app,
                &format!("/permissions/conversation/{conversation_id}"),
                "{invalid".into(),
            )
            .await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn middleware_denies_unpermitted_requests_but_allows_self_listing(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::models::model_test_helpers::{
            get_random_conversation_id, get_random_user_id, setup_default_app_and_session,
        };

        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let owner_id = session.id.ok_or("missing owner ID")?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;
        let target = format!("/permissions/conversation/{conversation_id}");
        let assignments = format!("{target}/assignments/user/{user_id}");

        for url in [
            "/permissions".to_owned(),
            target.clone(),
            format!("{target}/users"),
            assignments.clone(),
            format!("/permissions/by-action/conversation_read?user_id={owner_id}"),
            format!("/conversation/{conversation_id}/cohosts"),
            format!("/conversation/{conversation_id}/moderation_policies"),
        ] {
            let (status, response, _) = session.get(&app, &url).await?;
            assert_eq!(status, StatusCode::FORBIDDEN, "{url}: {response}");
        }
        let body = GrantPermissionBody {
            user_id: Some(user_id),
            organization_id: None,
            user_email: None,
            role_name: "content_editor".into(),
            grant_reason: "Unauthorized request".into(),
        };
        let (status, _, _) = session
            .post(&app, &target, serde_json::to_vec(&body)?.into())
            .await?;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let (status, _, _) = session
            .delete(
                &app,
                &format!("{target}?user_id={user_id}&role_name=content_editor"),
            )
            .await?;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let (status, _, _) = session
            .put(
                &app,
                &assignments,
                serde_json::json!({
                    "expected_version": 0,
                    "roles": ["content_editor"],
                    "grant_reason": "Unauthorized request",
                })
                .to_string()
                .into(),
            )
            .await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        for url in [
            "/permissions/by-action/conversation_read".to_owned(),
            format!("/permissions/by-action/conversation_read?user_id={user_id}"),
        ] {
            let (status, response, _) = session.get(&app, &url).await?;
            assert_eq!(status, StatusCode::OK, "{url}: {response}");
            assert!(response.as_array().is_some_and(Vec::is_empty));
        }
        let (status, _, _) = session
            .get(
                &app,
                "/permissions/by-action/conversation_read?user_id=invalid",
            )
            .await?;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        let granted: bool = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM conversation_user_permissions WHERE user_id = $1 AND resource_id = $2)")
            .bind(user_id).bind(conversation_id).fetch_one(&pool).await?;
        assert!(!granted);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_post_permission(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(test_state().db(pool).config(config).call()?);
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (_, user, _) = session.current_user(&app).await?;

        // Assign super admin role to the user to ensure they can grant/revoke permissions
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: ActorId::User(user.id),
                permission_triplet: Role::SuperAdmin.system_triplet()?,
                granted_by: &user.id,
                grant_reason: "Testing".into(),
            },
        )
        .await?;

        // Create a resource to grant permissions on
        let resource_id = get_random_conversation_id(&app, &mut session).await?;

        // Grant a permission
        let grant_body = GrantPermissionBody {
            user_id: Some(user.id),
            organization_id: None,
            user_email: None,
            role_name: "editor".into(),
            grant_reason: "Testing".into(),
        };

        let body = Body::from(serde_json::to_string(&grant_body)?);

        let (status, _response, _message) = session
            .post(
                &app,
                &format!("/permissions/{}/{}", RESOURCE_TYPE, resource_id),
                body,
            )
            .await?;

        assert_eq!(status, StatusCode::CREATED);

        let body = Body::from(serde_json::to_string(&grant_body)?);

        // Re-grant the same permission and expect a 400 Bad Request
        let (status, _response, _message) = session
            .post(
                &app,
                &format!("/permissions/{}/{}", RESOURCE_TYPE, resource_id),
                body,
            )
            .await?;

        assert_eq!(status, StatusCode::CONFLICT);

        Ok(())
    }

    // Revoke permission
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_delete_permission(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(test_state().db(pool).config(config).call()?);
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (_, user, _) = session.current_user(&app).await?;

        // Assign super admin role to the user to ensure they can grant/revoke permissions
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: ActorId::User(user.id),
                permission_triplet: Role::SuperAdmin.system_triplet()?,
                granted_by: &user.id,
                grant_reason: "Testing".into(),
            },
        )
        .await?;

        // Create a resource to grant permissions on
        let resource_id = get_random_conversation_id(&app, &mut session).await?;

        // Grant editor role to revoke
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: ActorId::User(user.id),
                permission_triplet: TestRole::make_triplet(&resource_id),
                granted_by: &user.id,
                grant_reason: "Testing".into(),
            },
        )
        .await?;

        // Revoke the permission via query parameters
        let revoke_query = RevokePermissionQuery {
            user_id: Some(user.id),
            organization_id: None,
            role_name: "editor".into(),
        };

        let revoke_url = revoke_url((RESOURCE_TYPE, &resource_id), &revoke_query);

        let (status, _response, _message) = session.delete(&app, &revoke_url).await?;

        assert_eq!(status, StatusCode::OK);

        // Revoking again should return NOT_FOUND
        let (status, _response, _message) = session.delete(&app, &revoke_url).await?;

        assert_eq!(status, StatusCode::NOT_FOUND);

        Ok(())
    }

    // List permissions (general)
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_get_permissions(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(test_state().db(pool).config(config).call()?);
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (_, user, _) = session.current_user(&app).await?;

        // Assign super admin role to the user to ensure they can grant/revoke permissions
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: ActorId::User(user.id),
                permission_triplet: Role::SuperAdmin.system_triplet()?,
                granted_by: &user.id,
                grant_reason: "Testing".into(),
            },
        )
        .await?;

        // Create a resource to grant permissions on
        let resource_id = get_random_conversation_id(&app, &mut session).await?;

        // Grant additional permissions
        for i in 0..5 {
            let permission_triplet =
                PermissionTriplet(&RESOURCE_TYPE, &resource_id, &format!("Role{}", i));
            grant_role(
                &state,
                GrantRoleRequest {
                    actor_id: ActorId::User(user.id),
                    permission_triplet,
                    granted_by: &user.id,
                    grant_reason: "Testing".into(),
                },
            )
            .await?;
        }

        // List permissions with pagination
        let query = ListPermissionsQuery {
            user_id: None,
            organization_id: None,
            role_name: None,
            offset: Some(0),
            limit: Some(4),
        };

        let list_url = list_query_url("/permissions", &query);

        let (status, response, _message) = session.get(&app, &list_url).await?;

        assert_eq!(status, StatusCode::OK);

        let permissions_page =
            serde_json::from_value::<PaginatedResults<ResourcePermission>>(response)?;

        assert_eq!(permissions_page.total, 7);
        assert_eq!(permissions_page.records.len(), 4);

        let next_query = ListPermissionsQuery {
            offset: Some(4),
            ..query
        };
        let next_list_url = list_query_url("/permissions", &next_query);

        let (status, response, _message) = session.get(&app, &next_list_url).await?;

        assert_eq!(status, StatusCode::OK);

        let next_permissions_page =
            serde_json::from_value::<PaginatedResults<ResourcePermission>>(response)?;

        assert_eq!(next_permissions_page.records.len(), 3);

        Ok(())
    }

    // List permissions (for resource)
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_get_permissions_for_resource(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(test_state().db(pool).config(config).call()?);
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (_, user, _) = session.current_user(&app).await?;

        // Assign super admin role to the user to ensure they can grant/revoke permissions
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: ActorId::User(user.id),
                permission_triplet: Role::SuperAdmin.system_triplet()?,
                granted_by: &user.id,
                grant_reason: "Testing".into(),
            },
        )
        .await?;

        // Create two resources to grant permissions on
        let resource_id = get_random_conversation_id(&app, &mut session).await?;
        let resource_id_2 = get_random_conversation_id(&app, &mut session).await?;

        // Grant additional permissions
        for i in 0..5 {
            let permission_triplet =
                PermissionTriplet(&RESOURCE_TYPE, &resource_id, &format!("Role{}", i));
            grant_role(
                &state,
                GrantRoleRequest {
                    actor_id: ActorId::User(user.id),
                    permission_triplet,
                    granted_by: &user.id,
                    grant_reason: "Testing".into(),
                },
            )
            .await?;
        }

        for i in 0..3 {
            let permission_triplet =
                PermissionTriplet(&RESOURCE_TYPE, &resource_id_2, &format!("OtherRole{}", i));
            grant_role(
                &state,
                GrantRoleRequest {
                    actor_id: ActorId::User(user.id),
                    permission_triplet,
                    granted_by: &user.id,
                    grant_reason: "Testing".into(),
                },
            )
            .await?;
        }

        // List permissions for the resource with pagination
        let query = ListPermissionsQuery {
            user_id: None,
            organization_id: None,
            role_name: None,
            offset: Some(0),
            limit: Some(4),
        };

        let list_url = list_query_url(
            &format!("/permissions/{}/{}", RESOURCE_TYPE, resource_id),
            &query,
        );

        let (status, response, _message) = session.get(&app, &list_url).await?;

        assert_eq!(status, StatusCode::OK);

        let permissions_page =
            serde_json::from_value::<PaginatedResults<ResourcePermission>>(response)?;

        assert_eq!(permissions_page.total, 5);
        assert_eq!(permissions_page.records.len(), 4);

        let next_query = ListPermissionsQuery {
            offset: Some(4),
            ..query
        };
        let next_list_url = list_query_url(
            &format!("/permissions/{}/{}", RESOURCE_TYPE, resource_id),
            &next_query,
        );

        let (status, response, _message) = session.get(&app, &next_list_url).await?;

        assert_eq!(status, StatusCode::OK);

        let next_permissions_page =
            serde_json::from_value::<PaginatedResults<ResourcePermission>>(response)?;

        assert_eq!(next_permissions_page.records.len(), 1);

        Ok(())
    }

    // List permissions for a resource with filters
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_get_permissions_for_resource_with_filters(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(test_state().db(pool).config(config).call()?);
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (_, user, _) = session.current_user(&app).await?;

        // Assign super admin role to the user to ensure they can grant/revoke permissions
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: ActorId::User(user.id),
                permission_triplet: Role::SuperAdmin.system_triplet()?,
                granted_by: &user.id,
                grant_reason: "Testing".into(),
            },
        )
        .await?;

        // Create a resource to grant permissions on
        let resource_id = get_random_conversation_id(&app, &mut session).await?;
        let resource_2_id = get_random_conversation_id(&app, &mut session).await?;

        // Grant additional permissions
        for i in 0..5 {
            let permission_triplet =
                PermissionTriplet(RESOURCE_TYPE, &resource_id, &format!("Role{}", i));
            grant_role(
                &state,
                GrantRoleRequest {
                    actor_id: ActorId::User(user.id),
                    permission_triplet,
                    granted_by: &user.id,
                    grant_reason: "Testing".into(),
                },
            )
            .await?;
        }

        // Grant role "Role1" on a different resource to ensure filtering works
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: ActorId::User(user.id),
                permission_triplet: PermissionTriplet(RESOURCE_TYPE, &resource_2_id, "Role1"),
                granted_by: &user.id,
                grant_reason: "Testing".into(),
            },
        )
        .await?;

        // List permissions for the resource with a filter on role_name
        let query = ListPermissionsQuery {
            user_id: None,
            organization_id: None,
            role_name: Some("Role1".into()),
            offset: Some(0),
            limit: Some(10),
        };

        let list_url = list_query_url(
            &format!("/permissions/{}/{}", RESOURCE_TYPE, resource_id),
            &query,
        );

        let (status, response, _message) = session.get(&app, &list_url).await?;

        assert_eq!(status, StatusCode::OK);

        let permissions_page =
            serde_json::from_value::<PaginatedResults<ResourcePermission>>(response)?;

        assert_eq!(permissions_page.total, 1);
        assert_eq!(permissions_page.records.len(), 1);
        assert_eq!(permissions_page.records[0].role_name, "Role1");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_permissions_audit_trail(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(test_state().db(pool).config(config).call()?);
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (_, user, _) = session.current_user(&app).await?;

        // Assign super admin role to the user to ensure they can grant/revoke permissions
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: ActorId::User(user.id),
                permission_triplet: Role::SuperAdmin.system_triplet()?,
                granted_by: &user.id,
                grant_reason: "Testing".into(),
            },
        )
        .await?;

        // Create a resource to grant permissions on
        let resource_id = get_random_conversation_id(&app, &mut session).await?;

        // Grant a permission
        let grant_body = GrantPermissionBody {
            user_id: Some(user.id),
            organization_id: None,
            user_email: None,
            role_name: TestRole::name().into(),
            grant_reason: "Testing".into(),
        };

        let body = Body::from(serde_json::to_string(&grant_body)?);

        let pre_grant = chrono::Utc::now();
        let (status, _response, _message) = session
            .post(
                &app,
                &format!("/permissions/{}/{}", RESOURCE_TYPE, resource_id),
                body,
            )
            .await?;
        let post_grant = chrono::Utc::now();

        assert_eq!(status, StatusCode::CREATED);

        // Get the permission from the database to check the audit trail
        let permissions = list_permissions(
            &state,
            ListPermissionsFilters {
                actor: Some(ActorId::User(user.id)),
                role_name: Some(TestRole::name()),
                resource_type: Some(RESOURCE_TYPE),
                resource_id: Some(&resource_id),
                page_options: Default::default(),
            },
        )
        .await?;

        assert!(permissions.records.len() > 0);

        let permission = &permissions.records[0];
        assert_eq!(
            permission.granted_by,
            Some(user.id),
            "granted by does not match"
        );
        assert_eq!(
            permission.grant_reason, "Testing",
            "grant reason does not match"
        );
        assert!(
            permission.granted_at >= pre_grant,
            "permission granted_at is at or after pre_grant"
        );
        assert!(
            permission.granted_at <= post_grant,
            "permission granted_at is at or before post_grant"
        );

        Ok(())
    }
}
