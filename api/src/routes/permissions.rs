use std::sync::Arc;

use aide::axum::{
    ApiRouter,
    routing::{delete_with, get_with, post_with},
};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::models::permissions::{
    self, Action, GrantRoleRequest, ListPermissionsFilters, ResourcePermissionTarget,
    RevokeRoleRequest, SYSTEM_RESOURCE_ID, UserOrOrganizationId, UserWithPermissionDto,
    list_permissions,
};
use crate::models::{
    pagination::{PageOptions, PaginatedResults},
    users,
};
use crate::routes::auth::{RequiredUser, authorize};
use crate::{
    ComhairleState,
    error::ComhairleError,
    models::permissions::{grant_role, revoke_role},
};

/// Represents the resource type and ID for a permission operation.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TargetResourceId {
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
) -> Result<Option<UserOrOrganizationId>, ComhairleError> {
    match (user_id, organization_id, user_email) {
        (Some(uid), None, None) => Ok(Some(UserOrOrganizationId::User(uid))),
        (None, Some(oid), None) => Ok(Some(UserOrOrganizationId::Org(oid))),
        (None, None, Some(u_email)) => {
            let user = users::get_user_by_email(&u_email, db).await?;

            Ok(Some(UserOrOrganizationId::User(user.id)))
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
    authorize(&state, &caller, Action::Admin, path.resource_id).await?;

    let actor_id = resolve_actor(
        &state.db,
        body.user_id,
        body.organization_id,
        body.user_email,
        false,
    )
    .await?
    .unwrap();

    let permission = grant_role(
        &state,
        GrantRoleRequest {
            actor_id,
            permission_target: ResourcePermissionTarget(path.resource_id, body.role_name),
            granted_by: caller.id,
            grant_reason: body.grant_reason,
        },
    )
    .await?;

    match actor_id {
        UserOrOrganizationId::User(uid) => {
            let user = users::get_user_by_id(&uid, &state.db).await?;

            if let Some(email) = user.email {
                state
                    .mailer
                    .send_permission_notification_email(&email, &permission, "granted")?;
            }
        }
        UserOrOrganizationId::Org(_) => {}
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
    RequiredUser(caller): RequiredUser,
    Path(path): Path<TargetResourceId>,
    Query(query): Query<RevokePermissionQuery>,
) -> Result<StatusCode, ComhairleError> {
    authorize(&state, &caller, Action::Admin, path.resource_id).await?;

    let actor_id = resolve_actor(&state.db, query.user_id, query.organization_id, None, false)
        .await?
        .unwrap();

    revoke_role(
        &state,
        RevokeRoleRequest {
            actor_id,
            permission_target: ResourcePermissionTarget(path.resource_id, query.role_name),
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
    RequiredUser(caller): RequiredUser,
    Query(query): Query<ListPermissionsQuery>,
) -> Result<
    (
        StatusCode,
        Json<PaginatedResults<permissions::ResourcePermission>>,
    ),
    ComhairleError,
> {
    authorize(&state, &caller, Action::Admin, SYSTEM_RESOURCE_ID).await?;

    let actor = resolve_actor(&state.db, query.user_id, query.organization_id, None, true).await?;
    let page_options = PageOptions {
        limit: query.limit,
        offset: query.offset,
    };

    let request = ListPermissionsFilters {
        actor,
        role_name: query.role_name,
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
    if let Err(err) = authorize(&state, &caller, Action::Admin, SYSTEM_RESOURCE_ID).await
        && (!matches!(err, ComhairleError::UserNotAuthorized)
            || caller.id != query.user_id.unwrap_or(caller.id))
    {
        return Err(err);
    };

    let permissions = permissions::list_permissions_by_action(
        &state.db,
        caller.id,
        caller.organization_id,
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
    RequiredUser(caller): RequiredUser,
    Path(path): Path<TargetResourceId>,
    Query(query): Query<ListPermissionsQuery>,
) -> Result<
    (
        StatusCode,
        Json<PaginatedResults<permissions::ResourcePermission>>,
    ),
    ComhairleError,
> {
    authorize(&state, &caller, Action::Admin, path.resource_id).await?;

    let actor = resolve_actor(&state.db, query.user_id, query.organization_id, None, true).await?;
    let page_options = PageOptions {
        limit: query.limit,
        offset: query.offset,
    };

    let request = ListPermissionsFilters {
        actor,
        role_name: query.role_name,
        page_options,
        resource_id: Some(path.resource_id),
    };

    let page = list_permissions(&state, request).await?;

    Ok((StatusCode::OK, Json(page)))
}

#[instrument(err(Debug), skip(state))]
async fn list_users_with_permission(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(caller): RequiredUser,
    Path(path): Path<TargetResourceId>,
    Query(query): Query<ListPermissionsQuery>,
) -> Result<(StatusCode, Json<Vec<UserWithPermissionDto>>), ComhairleError> {
    authorize(&state, &caller, Action::Admin, path.resource_id).await?;

    let users_with_permission = permissions::list_users_with_permission(
        &state.db,
        path.resource_id,
        query.role_name.as_deref(),
    )
    .await?;

    Ok((StatusCode::OK, Json(users_with_permission)))
}

/// Creates the permissions API router with all the defined routes and their corresponding handlers.
pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
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
            }),
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
            }),
        )
        .api_route(
            "/{resource_id}",
            get_with(list_for_resource, |op| {
                op.id("ListResourcePermissions")
                    .tag("Permissions")
                    .summary("List permissions for a resource")
                    .description(
                        "Returns role assignments for a specific resource using \
                        offset-based pagination. Optionally filter by user_id, \
                        organization_id, or role_name. The caller must hold the \
                        Owner role on the resource.",
                    )
                    .security_requirement("JWT")
                    .response::<200, Json<PaginatedResults<permissions::ResourcePermission>>>()
            }),
        )
        .api_route(
            "/{resource_id}",
            post_with(grant, |op| {
                op.id("GrantPermission")
                    .tag("Permissions")
                    .summary("Grant a role on a resource")
                    .description(
                        "Grants a role to a user or organisation on a resource. \
                        The caller must hold the Owner role on the resource.",
                    )
                    .security_requirement("JWT")
                    .response::<201, Json<permissions::ResourcePermission>>()
            }),
        )
        .api_route(
            "/{resource_id}",
            delete_with(revoke, |op| {
                op.id("RevokePermission")
                    .tag("Permissions")
                    .summary("Revoke a role from a resource")
                    .description(
                        "Revokes a role from a user or organisation on a resource. \
                        The actor (user_id or organization_id) and role_name are \
                        provided as query parameters. The caller must hold the \
                        Owner role on the resource.",
                    )
                    .security_requirement("JWT")
                    .response::<200, ()>()
            }),
        )
        .api_route(
            "/{resource_id}/users",
            get_with(list_users_with_permission, |op| {
                op.id("ListUsersWithPermission")
                    .tag("Permissions")
                    .summary("List users with permissions")
                    .description("List users with a given role for a given resource")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<UserWithPermissionDto>>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use crate::models::pagination::PaginatedResults;
    use crate::models::permissions::{
        GrantRoleRequest, ListPermissionsFilters, ResourcePermission, ResourcePermissionTarget,
        Role, UserOrOrganizationId, grant_role, has_resource_permission, list_permissions,
    };
    use crate::routes::permissions::{
        GrantPermissionBody, ListPermissionsQuery, RevokePermissionQuery,
    };
    use crate::test_helpers::{test_config, test_resource, test_state};
    use crate::{setup_server, test_helpers::UserSession};

    use axum::body::Body;
    use hyper::StatusCode;
    use sqlx::PgPool;
    use std::sync::Arc;

    struct TestRole;

    impl TestRole {
        fn name() -> &'static str {
            "editor"
        }

        fn make_triplet(resource_id: uuid::Uuid) -> ResourcePermissionTarget {
            ResourcePermissionTarget(resource_id, "editor".to_string())
        }
    }

    // Helper functions
    fn revoke_url(resource: uuid::Uuid, query: &RevokePermissionQuery) -> String {
        let mut url = format!("/permissions/{}?", resource);
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
        let has_admin_role = has_resource_permission(
            &state,
            Role::Admin.system_target(),
            user.id,
            user.organization_id,
        )
        .await?;

        assert!(
            has_admin_role,
            "Admin users should be auto-assigned the system admin role"
        );

        Ok(())
    }

    // Grant permission
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
                actor_id: UserOrOrganizationId::User(user.id),
                permission_target: Role::SuperAdmin.system_target(),
                granted_by: user.id,
                grant_reason: "Testing".to_string(),
            },
        )
        .await?;

        // Create a resource to grant permissions on
        let resource = test_resource(&state.db, None).await?;

        // Grant a permission
        let grant_body = GrantPermissionBody {
            user_id: Some(user.id),
            organization_id: None,
            user_email: None,
            role_name: "editor".into(),
            grant_reason: "Testing".to_string(),
        };

        let body = Body::from(serde_json::to_string(&grant_body)?);

        let (status, _response, _message) = session
            .post(&app, &format!("/permissions/{}", resource.id), body)
            .await?;

        assert_eq!(status, StatusCode::CREATED);

        let body = Body::from(serde_json::to_string(&grant_body)?);

        // Re-grant the same permission and expect a 400 Bad Request
        let (status, _response, _message) = session
            .post(&app, &format!("/permissions/{}", resource.id), body)
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
                actor_id: UserOrOrganizationId::User(user.id),
                permission_target: Role::SuperAdmin.system_target(),
                granted_by: user.id,
                grant_reason: "Testing".to_string(),
            },
        )
        .await?;

        // Create a resource to grant permissions on
        let resource = test_resource(&state.db, None).await?;

        // Grant editor role to revoke
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user.id),
                permission_target: TestRole::make_triplet(resource.id),
                granted_by: user.id,
                grant_reason: "Testing".to_string(),
            },
        )
        .await?;

        // Revoke the permission via query parameters
        let revoke_query = RevokePermissionQuery {
            user_id: Some(user.id),
            organization_id: None,
            role_name: "editor".into(),
        };

        let revoke_url = revoke_url(resource.id, &revoke_query);

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
                actor_id: UserOrOrganizationId::User(user.id),
                permission_target: Role::SuperAdmin.system_target(),
                granted_by: user.id,
                grant_reason: "Testing".to_string(),
            },
        )
        .await?;

        // Create a resource to grant permissions on
        let resource = test_resource(&state.db, None).await?;

        // Grant additional permissions
        for i in 0..5 {
            grant_role(
                &state,
                GrantRoleRequest {
                    actor_id: UserOrOrganizationId::User(user.id),
                    permission_target: ResourcePermissionTarget(resource.id, format!("Role{}", i)),
                    granted_by: user.id,
                    grant_reason: "Testing".to_string(),
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
                actor_id: UserOrOrganizationId::User(user.id),
                permission_target: Role::SuperAdmin.system_target(),
                granted_by: user.id,
                grant_reason: "Testing".to_string(),
            },
        )
        .await?;

        // Create two resources to grant permissions on
        let resource = test_resource(&state.db, None).await?;
        let resource_2 = test_resource(&state.db, None).await?;

        // Grant additional permissions
        for i in 0..5 {
            grant_role(
                &state,
                GrantRoleRequest {
                    actor_id: UserOrOrganizationId::User(user.id),
                    permission_target: ResourcePermissionTarget(resource.id, format!("Role{}", i)),
                    granted_by: user.id,
                    grant_reason: "Testing".to_string(),
                },
            )
            .await?;
        }

        for i in 0..3 {
            grant_role(
                &state,
                GrantRoleRequest {
                    actor_id: UserOrOrganizationId::User(user.id),
                    permission_target: ResourcePermissionTarget(
                        resource_2.id,
                        format!("OtherRole{}", i),
                    ),
                    granted_by: user.id,
                    grant_reason: "Testing".to_string(),
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

        let list_url = list_query_url(&format!("/permissions/{}", resource.id), &query);

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
        let next_list_url = list_query_url(&format!("/permissions/{}", resource.id), &next_query);

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
                actor_id: UserOrOrganizationId::User(user.id),
                permission_target: Role::SuperAdmin.system_target(),
                granted_by: user.id,
                grant_reason: "Testing".to_string(),
            },
        )
        .await?;

        // Create a resource to grant permissions on
        let resource = test_resource(&state.db, None).await?;
        let resource_2 = test_resource(&state.db, None).await?;

        // Grant additional permissions
        for i in 0..5 {
            grant_role(
                &state,
                GrantRoleRequest {
                    actor_id: UserOrOrganizationId::User(user.id),
                    permission_target: ResourcePermissionTarget(resource.id, format!("Role{}", i)),
                    granted_by: user.id,
                    grant_reason: "Testing".to_string(),
                },
            )
            .await?;
        }

        // Grant role "Role1" on a different resource to ensure filtering works
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user.id),
                permission_target: ResourcePermissionTarget(resource_2.id, "Role1".to_string()),
                granted_by: user.id,
                grant_reason: "Testing".to_string(),
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

        let list_url = list_query_url(&format!("/permissions/{}", resource.id), &query);

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
                actor_id: UserOrOrganizationId::User(user.id),
                permission_target: Role::SuperAdmin.system_target(),
                granted_by: user.id,
                grant_reason: "Testing".to_string(),
            },
        )
        .await?;

        // Create a resource to grant permissions on
        let resource = test_resource(&state.db, None).await?;

        // Grant a permission
        let grant_body = GrantPermissionBody {
            user_id: Some(user.id),
            organization_id: None,
            user_email: None,
            role_name: TestRole::name().into(),
            grant_reason: "Testing".to_string(),
        };

        let body = Body::from(serde_json::to_string(&grant_body)?);

        let pre_grant = chrono::Utc::now();
        let (status, _response, _message) = session
            .post(&app, &format!("/permissions/{}", resource.id), body)
            .await?;
        let post_grant = chrono::Utc::now();

        assert_eq!(status, StatusCode::CREATED);

        // Get the permission from the database to check the audit trail
        let permissions = list_permissions(
            &state,
            ListPermissionsFilters {
                actor: Some(UserOrOrganizationId::User(user.id)),
                role_name: Some(TestRole::name().to_string()),
                resource_id: Some(resource.id),
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
            "permission granted_at is before pre_grant"
        );
        assert!(
            permission.granted_at <= post_grant,
            "permission granted_at is after post_grant"
        );

        Ok(())
    }
}
