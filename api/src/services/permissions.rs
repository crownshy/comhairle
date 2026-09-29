//! Permission checks and role management, with role-list caching.
//!
//! The permission *data* — the `Role`/`Action`/`ResourceType` vocabulary and the
//! raw `resource_permission` queries — lives in [`crate::models::permissions`].
//! This module is the behaviour on top: it resolves an actor's roles through the
//! Redis cache (falling back to Postgres), answers "can this user do X", and
//! keeps the cache coherent when roles are granted or revoked. It sits here, not
//! in the model layer, so the model crate never depends on Redis.

use tracing::instrument;
use uuid::Uuid;

use crate::ComhairleState;
use crate::models::error::{ModelError, PermissionError};
use crate::models::permissions::{
    Action, GrantRoleRequest, PermissionTriplet, ResourcePermission, RevokeRoleRequest, Role,
    UserOrOrganizationId, fetch_actor_roles_for_resource,
};
use crate::redis_connection::RedisConnection;

/// Generates a cache key for storing all assigned role names for an actor on a resource.
fn role_list_cache_key(
    resource_type: &str,
    resource_id: &Uuid,
    actor_id: &UserOrOrganizationId,
) -> String {
    match *actor_id {
        UserOrOrganizationId::User(user_id) => {
            format!("roles:v1:{resource_type}:{resource_id}:user:{user_id}")
        }
        UserOrOrganizationId::Org(org_id) => {
            format!("roles:v1:{resource_type}:{resource_id}:org:{org_id}")
        }
    }
}

/// Deletes a cache key from Redis.
async fn cache_delete(conn: &dyn RedisConnection, key: &str) {
    let _ = conn.del(key).await;
}

/// Reads a cached role list for an actor on a resource.
async fn cache_get_role_list(conn: &dyn RedisConnection, key: &str) -> Option<Vec<String>> {
    let raw = match conn.get(key).await {
        Ok(Some(value)) => value,
        Ok(None) => return None,
        Err(_) => return None,
    };

    serde_json::from_str(&raw).ok()
}

/// Stores a role list for an actor on a resource.
async fn cache_set_role_list(
    conn: &dyn RedisConnection,
    key: &str,
    roles: &[String],
    ttl_secs: u64,
) {
    let serialized = match serde_json::to_string(roles) {
        Ok(value) => value,
        Err(_) => return,
    };
    let _ = conn.set_ex(key, &serialized, ttl_secs).await;
}

/// Returns role names for an actor on a resource, using the Redis cache when
/// available and falling back to Postgres.
#[instrument(err(Debug), skip(state))]
async fn get_actor_roles_for_resource(
    state: &ComhairleState,
    resource_type: &str,
    resource_id: &Uuid,
    actor_id: UserOrOrganizationId,
) -> Result<Vec<String>, ModelError> {
    let cache_key = role_list_cache_key(resource_type, resource_id, &actor_id);

    if let Some(conn) = state.redis_conn.as_deref() {
        if let Some(cached_roles) = cache_get_role_list(conn, &cache_key).await {
            return Ok(cached_roles);
        }
    }

    let roles =
        fetch_actor_roles_for_resource(&state.db, resource_type, resource_id, actor_id).await?;

    if let Some(conn) = state.redis_conn.as_deref() {
        cache_set_role_list(conn, &cache_key, &roles, state.config.redis_cache_ttl_secs).await;
    }

    Ok(roles)
}

/// Grants a role and invalidates the actor's cached role list.
///
/// See [`crate::models::permissions::grant_role`] for the write itself and its
/// error conditions.
#[instrument(err(Debug), skip(state))]
pub async fn grant_role(
    state: &ComhairleState,
    request: GrantRoleRequest<'_>,
) -> Result<ResourcePermission, PermissionError> {
    let PermissionTriplet(resource_type, resource_id, _) = request.permission_triplet;
    let role_list_key = role_list_cache_key(resource_type, resource_id, &request.actor_id);

    let permission = crate::models::permissions::grant_role(&state.db, request).await?;

    if let Some(conn) = state.redis_conn.as_deref() {
        cache_delete(conn, &role_list_key).await;
    }

    Ok(permission)
}

/// Revokes a role and invalidates the actor's cached role list.
///
/// See [`crate::models::permissions::revoke_role`] for the delete itself and its
/// error conditions.
#[instrument(err(Debug), skip(state))]
pub async fn revoke_role(
    state: &ComhairleState,
    request: RevokeRoleRequest<'_>,
) -> Result<(), PermissionError> {
    let PermissionTriplet(resource_type, resource_id, _) = request.permission_triplet;
    let role_list_key = role_list_cache_key(resource_type, resource_id, &request.actor_id);

    crate::models::permissions::revoke_role(&state.db, request).await?;

    if let Some(conn) = state.redis_conn.as_deref() {
        cache_delete(conn, &role_list_key).await;
    }

    Ok(())
}

/// Check whether a user, or their organization, has a specific role on a resource.
#[instrument(err(Debug), skip(state))]
pub async fn has_resource_permission(
    state: &ComhairleState,
    permission_triplet: PermissionTriplet<'_>,
    user_id: &Uuid,
    organization_id: Option<&Uuid>,
) -> Result<bool, ModelError> {
    let PermissionTriplet(resource_type, resource_id, role_name) = permission_triplet;

    let user_roles = get_actor_roles_for_resource(
        state,
        resource_type,
        resource_id,
        UserOrOrganizationId::User(*user_id),
    )
    .await?;
    let org_roles = match organization_id {
        Some(org_id) => Some(
            get_actor_roles_for_resource(
                state,
                resource_type,
                resource_id,
                UserOrOrganizationId::Org(*org_id),
            )
            .await?,
        ),
        None => None,
    };

    let user_has_role = user_roles
        .iter()
        .any(|cached_role| cached_role == role_name);
    let org_has_role = org_roles
        .as_ref()
        .is_some_and(|roles| roles.iter().any(|cached_role| cached_role == role_name));

    Ok(user_has_role || org_has_role)
}

/// Check whether a user, or their organization, can perform an action on a resource.
#[instrument(err(Debug), skip(state))]
pub async fn can_perform_resource_action(
    state: &ComhairleState,
    resource_id: &Uuid,
    action: Action,
    user_id: &Uuid,
    organization_id: Option<&Uuid>,
    owner_id: Option<&Uuid>,
) -> Result<bool, ModelError> {
    if owner_id.is_some_and(|resource_owner_id| resource_owner_id == user_id) {
        return Ok(true);
    }

    // Bypass permission checks for super admins
    if has_resource_permission(
        state,
        Role::SuperAdmin.system_triplet(),
        user_id,
        organization_id,
    )
    .await?
    {
        return Ok(true);
    }

    let resource_type = action.resource_type();

    let mut roles = get_actor_roles_for_resource(
        state,
        resource_type.as_ref(),
        resource_id,
        UserOrOrganizationId::User(*user_id),
    )
    .await?;

    if let Some(org_id) = organization_id {
        let org_roles = get_actor_roles_for_resource(
            state,
            resource_type.as_ref(),
            resource_id,
            UserOrOrganizationId::Org(*org_id),
        )
        .await?;
        roles.extend(org_roles);
    }

    roles.dedup();

    Ok(roles.iter().any(|role_name| {
        role_name
            .parse::<Role>()
            .is_ok_and(|role| role.actions().contains(&action))
    }))
}
#[cfg(test)]
mod tests {
    use super::*;
    use super::{can_perform_resource_action, grant_role, has_resource_permission, revoke_role};
    use crate::models::model_test_helpers::{
        get_random_organization_id, get_random_user_id, setup_default_app_and_session,
    };
    use crate::models::pagination::PageOptions;
    use crate::models::permissions::*;
    use crate::redis_connection::{MockRedis, RedisConnection};
    use crate::test_helpers::{TEST_RESOURCE_TYPE, TEST_ROLE_NAME, TestRole, test_state};
    use sea_query::{Expr, PostgresQueryBuilder};
    use sea_query_binder::SqlxBinder;
    use std::sync::Arc;

    use sea_query::DeleteStatement;
    use sqlx::PgPool;

    const OTHER_ROLE_NAME: &str = "other_role";
    struct OtherRole;

    impl OtherRole {
        fn name() -> &'static str {
            OTHER_ROLE_NAME
        }

        fn make_triplet(resource_id: &Uuid) -> PermissionTriplet<'_> {
            PermissionTriplet(TEST_RESOURCE_TYPE, resource_id, OTHER_ROLE_NAME)
        }
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_grant_and_check_user_role(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let (app, mut session) = setup_default_app_and_session(&state.db).await?;

        // Create test user and organization
        let user_id = get_random_user_id(&app, &mut session).await?;

        // Mock conversation
        let resource_id = Uuid::new_v4();

        // Grant a role to the user
        let grant_request = GrantRoleRequest {
            actor_id: UserOrOrganizationId::User(user_id),
            permission_triplet: TestRole::make_triplet(&resource_id),
            granted_by: &session.id.unwrap(),
            grant_reason: "Testing",
        };

        let assignment = grant_role(&state, grant_request).await?;
        assert_eq!(assignment.user_id, Some(user_id));
        assert_eq!(assignment.organization_id, None);
        assert_eq!(assignment.resource_type, TestRole::resource_type());
        assert_eq!(assignment.role_name, TestRole::name());

        // Check that the user has the role
        let has_permission =
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id, None)
                .await?;
        assert!(has_permission);

        // Check that the user does not have a different role
        let has_wrong_permission = has_resource_permission(
            &state,
            OtherRole::make_triplet(&resource_id),
            &user_id,
            None,
        )
        .await?;
        assert!(!has_wrong_permission);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_grant_and_check_organization_role(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let state = Arc::new(test_state().db(pool).call()?);

        let (app, mut session) = setup_default_app_and_session(&state.db).await?;

        assert!(
            session.id.is_some(),
            "Session should have a user ID after signup"
        );
        let user_id = session.id.unwrap();

        // Create test organization and user
        let org_id = get_random_organization_id(&app, &mut session).await?;

        // Mock conversation
        let resource_id = Uuid::new_v4();

        // Grant a role to the organization
        let grant_request = GrantRoleRequest {
            actor_id: UserOrOrganizationId::Org(org_id),
            permission_triplet: TestRole::make_triplet(&resource_id),
            granted_by: &session.id.unwrap(),
            grant_reason: "Testing",
        };

        let assignment = grant_role(&state, grant_request).await?;
        assert_eq!(assignment.user_id, None);
        assert_eq!(assignment.organization_id, Some(org_id));
        assert_eq!(assignment.resource_type, TestRole::resource_type());
        assert_eq!(assignment.role_name, TestRole::name());

        // Check that the user has the role through the organization
        let has_permission = has_resource_permission(
            &state,
            TestRole::make_triplet(&resource_id),
            &user_id,
            Some(&org_id),
        )
        .await?;
        assert!(has_permission);

        // Check that the user does not have a different role
        let has_wrong_permission = has_resource_permission(
            &state,
            OtherRole::make_triplet(&resource_id),
            &user_id,
            Some(&org_id),
        )
        .await?;
        assert!(!has_wrong_permission);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_unauthorized_access(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let (app, mut session) = setup_default_app_and_session(&state.db).await?;

        // Create test user and organization
        let organization_id = get_random_organization_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;

        // Mock conversation
        let resource_id = Uuid::new_v4();

        // Check that the user does not have any roles on a random resource
        let has_permission =
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id, None)
                .await?;
        assert!(!has_permission);

        // Check that the user does not have any roles through the organization
        let has_org_permission = has_resource_permission(
            &state,
            TestRole::make_triplet(&resource_id),
            &user_id,
            Some(&organization_id),
        )
        .await?;
        assert!(!has_org_permission);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_grant_role_already_granted(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let (app, mut session) = setup_default_app_and_session(&state.db).await?;

        let user_id = get_random_user_id(&app, &mut session).await?;
        let resource_id = Uuid::new_v4();

        // Grant the role for the first time.
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing",
            },
        )
        .await?;

        // Granting the same role again should return RoleAlreadyGranted.
        let err = grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing",
            },
        )
        .await
        .unwrap_err();

        assert!(
            matches!(err, PermissionError::RoleAlreadyGranted(_)),
            "Expected RoleAlreadyGranted, got {err:?}"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_revoke_user_role(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let (app, mut session) = setup_default_app_and_session(&state.db).await?;

        let user_id = get_random_user_id(&app, &mut session).await?;
        let resource_id = Uuid::new_v4();

        // Grant the role first.
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing",
            },
        )
        .await?;

        // Confirm permission is granted.
        assert!(
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id, None,)
                .await?
        );

        // Revoke the role.
        revoke_role(
            &state,
            RevokeRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_id),
            },
        )
        .await?;

        // Confirm permission no longer granted.
        assert!(
            !has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id, None,)
                .await?
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_revoke_last_admin_fails(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let (app, mut session) = setup_default_app_and_session(&state.db).await?;

        let (_, user, _) = session.current_user(&app).await?;

        // Attempt to revoke the admin role, which should be the only one
        let result = revoke_role(
            &state,
            RevokeRoleRequest {
                actor_id: UserOrOrganizationId::User(user.id),
                permission_triplet: Role::SuperAdmin.system_triplet(),
            },
        )
        .await;

        assert!(
            matches!(result, Err(PermissionError::CannotRevokeLastSuperAdmin)),
            "Expected CannotRevokeLastSuperAdmin, got {result:?}"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_revoke_role_not_found(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let (app, mut session) = setup_default_app_and_session(&state.db).await?;

        let user_id = get_random_user_id(&app, &mut session).await?;
        let resource_id = Uuid::new_v4();

        // Revoking a role that was never granted should return RoleNotFound.
        let err = revoke_role(
            &state,
            RevokeRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_id),
            },
        )
        .await
        .unwrap_err();

        assert!(
            matches!(err, PermissionError::RoleNotFound(_)),
            "Expected RoleNotFound, got {err:?}"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_list_permissions(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let (app, mut session) = setup_default_app_and_session(&state.db).await?;

        let org_id = get_random_organization_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;
        let resource_1_id = Uuid::new_v4();
        let resource_2_id = Uuid::new_v4();

        // Grant roles to both user and organization
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_1_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing",
            },
        )
        .await?;

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::Org(org_id),
                permission_triplet: OtherRole::make_triplet(&resource_1_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing",
            },
        )
        .await?;

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: OtherRole::make_triplet(&resource_2_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing",
            },
        )
        .await?;

        // List all permissions without filters
        let request = ListPermissionsFilters::default();
        let all_permissions = list_permissions(&state.db, request).await?;
        assert!(
            all_permissions.records.len() >= 3,
            "Expected at least 3 permissions, got {}",
            all_permissions.records.len()
        );

        // List all permissions for the resource
        let request = ListPermissionsFilters {
            resource_id: Some(&resource_1_id),
            ..Default::default()
        };
        let permissions = list_permissions(&state.db, request).await?;
        assert_eq!(permissions.records.len(), 2);

        // List permissions for the user
        let request = ListPermissionsFilters {
            actor: Some(UserOrOrganizationId::User(user_id)),
            ..Default::default()
        };
        let user_permissions = list_permissions(&state.db, request).await?;
        assert_eq!(user_permissions.records.len(), 2);
        assert!(
            user_permissions
                .records
                .iter()
                .any(|p| p.resource_id == resource_1_id && p.role_name == TEST_ROLE_NAME)
        );
        assert!(
            user_permissions
                .records
                .iter()
                .any(|p| p.resource_id == resource_2_id && p.role_name == OTHER_ROLE_NAME)
        );

        // List permissions for the organization
        let request = ListPermissionsFilters {
            actor: Some(UserOrOrganizationId::Org(org_id)),
            ..Default::default()
        };
        let org_permissions = list_permissions(&state.db, request).await?;
        assert_eq!(org_permissions.records.len(), 1);
        assert_eq!(org_permissions.records[0].resource_id, resource_1_id);
        assert_eq!(org_permissions.records[0].role_name, OTHER_ROLE_NAME);

        // Filter by role_name: only the two OtherRole assignments granted above.
        let request = ListPermissionsFilters {
            role_name: Some(OTHER_ROLE_NAME),
            ..Default::default()
        };
        let viewer_permissions = list_permissions(&state.db, request).await?;
        assert_eq!(viewer_permissions.records.len(), 2);
        assert!(
            viewer_permissions
                .records
                .iter()
                .all(|p| p.role_name == OTHER_ROLE_NAME)
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_list_permissions_offset_pagination(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let (app, mut session) = setup_default_app_and_session(&state.db).await?;

        let user_id = get_random_user_id(&app, &mut session).await?;
        let resource_id = Uuid::new_v4();

        // Grant distinct roles on the same resource.
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing",
            },
        )
        .await?;

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: OtherRole::make_triplet(&resource_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing",
            },
        )
        .await?;

        // Walk the resource scoped listing two records at a time.
        let mut seen = Vec::new();
        let mut offset: Option<u64> = None;
        const LIMIT: usize = 1;
        loop {
            let request = ListPermissionsFilters {
                resource_type: Some(TEST_RESOURCE_TYPE),
                resource_id: Some(&resource_id),
                actor: Some(UserOrOrganizationId::User(user_id)),
                page_options: PageOptions {
                    offset,
                    limit: Some(LIMIT as u64),
                },
                ..Default::default()
            };
            let page = list_permissions(&state.db, request).await?;

            assert!(page.records.len() <= LIMIT, "page should respect the limit");
            seen.extend(page.records.iter().map(|p| p.role_name.clone()));

            match page.records.len() {
                0 => break,                                                     // no more records
                1..=LIMIT => offset = Some(offset.unwrap_or(0) + LIMIT as u64), // more pages to fetch
                _ => unreachable!("should never return more than the limit"),
            }
        }

        // Every role should be returned exactly once across all pages.
        assert_eq!(seen.len(), 2);
        assert_eq!(
            seen.iter()
                .filter(|r| r.as_str() == TestRole::name())
                .count(),
            1,
            "role {} should appear exactly once across pages",
            TestRole::name()
        );
        assert_eq!(
            seen.iter()
                .filter(|r| r.as_str() == OtherRole::name())
                .count(),
            1,
            "role {} should appear exactly once across pages",
            OtherRole::name()
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_list_permissions_by_action(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let (app, mut session) = setup_default_app_and_session(&state.db).await?;

        let user_id = get_random_user_id(&app, &mut session).await?;

        // Create three test resources
        let resource_1_id = Uuid::new_v4(); // Simulated conversation resource
        let resource_2_id = Uuid::new_v4(); // Simulated conversation resource
        let resource_3_id = Uuid::new_v4(); // Simulated organization resource

        // Grant the conversation co host and conversation content editor roles to the user for resource_1 and resource_2
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: Role::ConversationCoHost.triplet(&resource_1_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing",
            },
        )
        .await?;

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: Role::ConversationContentEditor.triplet(&resource_2_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing",
            },
        )
        .await?;

        // Grant the organization admin role to the user for resource_3
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: Role::OrganizationAdmin.triplet(&resource_3_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing",
            },
        )
        .await?;

        // List permissions which permit the user to perform the "conversation_read" action
        let permissions =
            list_permissions_by_action(&state.db, user_id, None, "conversation_read").await?;

        assert_eq!(permissions.len(), 2);
        assert!(permissions.iter().any(|p| p.resource_id == resource_1_id));
        assert!(permissions.iter().any(|p| p.resource_id == resource_2_id));

        // List permissions which permit the user to perform the "conversation_update" action
        let permissions =
            list_permissions_by_action(&state.db, user_id, None, "conversation_update").await?;

        assert_eq!(permissions.len(), 1);
        assert_eq!(permissions[0].resource_id, resource_2_id);

        // List permissions which permit the user to perform the "organization_update" action
        let permissions =
            list_permissions_by_action(&state.db, user_id, None, "organization_update").await?;

        assert_eq!(permissions.len(), 1);
        assert_eq!(permissions[0].resource_id, resource_3_id);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_role_list_is_cached_after_first_call(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mock = Arc::new(MockRedis::new());
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let state = Arc::new(
            test_state()
                .db(pool.clone())
                .redis_conn(mock.clone())
                .call()?,
        );

        let user_id = get_random_user_id(&app, &mut session).await?;
        let resource_id = Uuid::new_v4();

        // Grant the role so a permission exists.
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing cache",
            },
        )
        .await?;

        // First check: hits the DB and populates the cache.
        let first =
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id, None)
                .await?;
        assert!(first, "expected permission to be present");

        let key = role_list_cache_key(
            TestRole::resource_type(),
            &resource_id,
            &UserOrOrganizationId::User(user_id),
        );
        let cached = mock.get_value(&key).await;
        assert!(
            cached
                .as_ref()
                .is_some_and(|raw| raw.contains(TestRole::name())),
            "expected cache key to contain the granted role after first positive check"
        );

        // Remove the DB row directly – bypassing the permission model so the
        // cache entry is NOT invalidated.
        sqlx::query("DELETE FROM resource_permissions WHERE user_id = $1 AND resource_id = $2 AND resource_type = $3 AND role_name = $4")
            .bind(user_id)
            .bind(resource_id)
            .bind(TestRole::resource_type())
            .bind(TestRole::name())
            .execute(&pool)
            .await?;

        // Second check: DB row is gone but result should still come from cache.
        let second =
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id, None)
                .await?;
        assert!(
            second,
            "expected cached positive result to be returned even though DB row was deleted"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_role_list_cache_invalidated_on_grant(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mock = Arc::new(MockRedis::new());
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let state = Arc::new(
            test_state()
                .db(pool)
                .redis_conn(mock.clone() as Arc<dyn RedisConnection>)
                .call()?,
        );

        let user_id = get_random_user_id(&app, &mut session).await?;
        let resource_id = Uuid::new_v4();

        let initial_has_role =
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id, None)
                .await?;
        assert!(!initial_has_role, "expected no role before grant");

        let role_list_key = role_list_cache_key(
            TestRole::resource_type(),
            &resource_id,
            &UserOrOrganizationId::User(user_id),
        );
        let cached_before_grant = mock.get_value(&role_list_key).await;
        assert_eq!(cached_before_grant.as_deref(), Some("[]"));

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing role list cache invalidation on grant",
            },
        )
        .await?;

        let cached_after_grant = mock.get_value(&role_list_key).await;
        assert!(
            cached_after_grant.is_none(),
            "expected actor role-list cache key to be deleted after grant"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_role_list_cache_invalidated_on_revoke(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mock = Arc::new(MockRedis::new());
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let state = Arc::new(
            test_state()
                .db(pool)
                .redis_conn(mock.clone() as Arc<dyn RedisConnection>)
                .call()?,
        );

        let user_id = get_random_user_id(&app, &mut session).await?;
        let resource_id = Uuid::new_v4();

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing role list cache invalidation on revoke",
            },
        )
        .await?;

        let has_role =
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id, None)
                .await?;
        assert!(has_role, "expected role to exist before revoke");

        let role_list_key = role_list_cache_key(
            TestRole::resource_type(),
            &resource_id,
            &UserOrOrganizationId::User(user_id),
        );
        let cached_before_revoke = mock.get_value(&role_list_key).await;
        assert!(
            cached_before_revoke
                .as_ref()
                .is_some_and(|raw| raw.contains(TEST_ROLE_NAME)),
            "expected role list cache to include the granted role"
        );

        revoke_role(
            &state,
            RevokeRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_id),
            },
        )
        .await?;

        let cached_after_revoke = mock.get_value(&role_list_key).await;
        assert!(
            cached_after_revoke.is_none(),
            "expected actor role-list cache key to be deleted after revoke"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_action_check_uses_cached_role_list(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mock = Arc::new(MockRedis::new());
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let state = Arc::new(
            test_state()
                .db(pool.clone())
                .redis_conn(mock.clone() as Arc<dyn RedisConnection>)
                .call()?,
        );

        let user_id = get_random_user_id(&app, &mut session).await?;
        let resource_id = Uuid::new_v4();

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: Role::ConversationContentEditor.triplet(&resource_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing action role list cache",
            },
        )
        .await?;

        let first = can_perform_resource_action(
            &state,
            &resource_id,
            Action::ConversationRead,
            &user_id,
            None,
            None,
        )
        .await?;
        assert!(first, "expected first action check to be allowed");

        let role_list_key = role_list_cache_key(
            ResourceType::Conversation.as_ref(),
            &resource_id,
            &UserOrOrganizationId::User(user_id),
        );
        let cached_roles = mock.get_value(&role_list_key).await;
        assert!(
            cached_roles
                .as_ref()
                .is_some_and(|raw| raw.contains(Role::ConversationContentEditor.as_ref())),
            "expected cached role list to include content editor role"
        );

        let (sql, values) = DeleteStatement::new()
            .from_table("resource_permissions")
            .and_where(Expr::col("user_id").eq(user_id))
            .and_where(Expr::col("resource_id").eq(resource_id))
            .and_where(Expr::col("resource_type").eq(ResourceType::Conversation.as_ref()))
            .and_where(Expr::col("role_name").eq(Role::ConversationContentEditor.as_ref()))
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values).execute(&pool).await?;

        let second = can_perform_resource_action(
            &state,
            &resource_id,
            Action::ConversationRead,
            &user_id,
            None,
            None,
        )
        .await?;
        assert!(
            second,
            "expected second action check to be served by cached role list"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn should_list_users_with_permission(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let (app, mut session) = setup_default_app_and_session(&state.db).await?;

        let user_a_id = get_random_user_id(&app, &mut session).await?;
        let user_b_id = get_random_user_id(&app, &mut session).await?;
        let user_c_id = get_random_user_id(&app, &mut session).await?;

        let resource_id = Uuid::new_v4();

        let grant_request_a = GrantRoleRequest {
            actor_id: UserOrOrganizationId::User(user_a_id),
            permission_triplet: TestRole::make_triplet(&resource_id),
            granted_by: &session.id.unwrap(),
            grant_reason: "Testing",
        };
        grant_role(&state, grant_request_a).await?;

        let grant_request_b = GrantRoleRequest {
            actor_id: UserOrOrganizationId::User(user_b_id),
            permission_triplet: TestRole::make_triplet(&resource_id),
            granted_by: &session.id.unwrap(),
            grant_reason: "Testing",
        };
        grant_role(&state, grant_request_b).await?;

        let users_with_permission = list_users_with_permission(
            &state.db,
            TestRole::resource_type(),
            resource_id,
            Some(TestRole::name()),
        )
        .await?;

        assert!(
            users_with_permission.iter().any(|u| u.id == user_a_id),
            "missing user_a"
        );
        assert!(
            users_with_permission.iter().any(|u| u.id == user_b_id),
            "missing user_b"
        );
        assert!(
            !users_with_permission.iter().any(|u| u.id == user_c_id),
            "user_c incorrectly included"
        );
        assert!(
            users_with_permission
                .iter()
                .all(|u| u.role_name == TestRole::name()),
            "wrong role_name"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_action_permission_granted_for_content_editor(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let (app, mut session) = setup_default_app_and_session(&state.db).await?;

        let user_id = get_random_user_id(&app, &mut session).await?;
        let resource_id = Uuid::new_v4();

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: Role::ConversationContentEditor.triplet(&resource_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing action checks",
            },
        )
        .await?;

        let can_read = can_perform_resource_action(
            &state,
            &resource_id,
            Action::ConversationRead,
            &user_id,
            None,
            None,
        )
        .await?;
        assert!(can_read, "content editor should allow read action");

        let can_update = can_perform_resource_action(
            &state,
            &resource_id,
            Action::ConversationUpdate,
            &user_id,
            None,
            None,
        )
        .await?;
        assert!(can_update, "content editor should allow update action");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn test_action_permission_denied_when_resource_type_mismatch(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let (app, mut session) = setup_default_app_and_session(&state.db).await?;

        let user_id = get_random_user_id(&app, &mut session).await?;
        let resource_id = Uuid::new_v4();

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: OtherRole::make_triplet(&resource_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing action checks",
            },
        )
        .await?;

        let can_read = can_perform_resource_action(
            &state,
            &resource_id,
            Action::ConversationRead,
            &user_id,
            None,
            None,
        )
        .await?;
        assert!(
            !can_read,
            "conversation read action should be denied for non-conversation role assignment"
        );

        Ok(())
    }
}
