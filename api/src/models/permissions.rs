use std::sync::Arc;

use crate::models::users::UserIden;
use crate::redis_connection::RedisConnection;
use axum::extract::FromRequestParts;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, OnConflict, PostgresQueryBuilder, Query, enum_def};
use sea_query::{IdenStatic, JoinType};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::{PgPool, Row, query_as_with};
use tracing::instrument;
use uuid::Uuid;

use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::models::pagination::{PageOptions, PaginatedResults};

/// Represents the system administrator role.
#[derive(Debug)]
pub struct SystemAdminRole;

impl NamedRole for SystemAdminRole {
    fn name() -> &'static str {
        "admin"
    }
}

impl SystemResourceRole for SystemAdminRole {}

/// Represents the system resource, which is a global resource used for system-level permissions.
#[derive(Debug)]
pub struct SystemResource;

impl FromRequestParts<Arc<ComhairleState>> for SystemResource {
    type Rejection = ComhairleError;

    async fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self, Self::Rejection> {
        Ok(SystemResource)
    }
}

/// A trait for extracting a resource ID from a request.
pub trait ExtractResourceId: FromRequestParts<Arc<ComhairleState>> + 'static + Send + Sync {
    fn resource_id(&self) -> Uuid;
}

impl ExtractResourceId for SystemResource {
    fn resource_id(&self) -> Uuid {
        SYSTEM_RESOURCE_ID
    }
}

/// The triplet associated with a permission for a resource.
#[derive(Debug)]
pub struct PermissionTriplet<'a>(
    pub &'a str,  // resource_type
    pub &'a Uuid, // resource_id
    pub &'a str,  // role_name
);

/// A trait for roles that have a name, used for permission checks.
pub trait NamedRole: Send + Sync {
    /// Returns the name of the role as a static string.
    fn name() -> &'static str;
}

/// The resource type associated with system-level permissions.
pub const SYSTEM_RESOURCE_TYPE: &str = "system";
/// The global system resource uses a fixed ID as a workaround to avoid having a
/// separate table for system-level permissions.
pub const SYSTEM_RESOURCE_ID: Uuid = Uuid::nil();

/// Represents a role that can be assigned to a user or organization on the global
/// system resource.
pub trait SystemResourceRole: NamedRole {
    /// Returns a `PermissionTriplet` for the global system resource.
    fn make_system_triplet() -> PermissionTriplet<'static> {
        PermissionTriplet(SYSTEM_RESOURCE_TYPE, &SYSTEM_RESOURCE_ID, Self::name())
    }
}

/// The resource type associated with conversation-level permissions.
pub const CONVERSATION_RESOURCE_TYPE: &str = "conversation";

/// Grants read and update access to a resource, but not full write access —
/// only a subset of update operations are permitted (e.g. editing content), while
/// others (e.g. `launch`) are excluded and require a higher-privileged role.
pub const CONTENT_EDITOR_ROLE: &str = "content_editor";

#[derive(Debug)]
pub struct ConversationContentEditor;

impl NamedRole for ConversationContentEditor {
    fn name() -> &'static str {
        "content_editor"
    }
}

impl ResourceRole for ConversationContentEditor {
    fn resource_type() -> &'static str {
        CONVERSATION_RESOURCE_TYPE
    }
}

/// Represents a role that can be assigned to a user or organization on a specific
/// resource.
pub trait ResourceRole: NamedRole {
    /// Returns a `PermissionTriplet` for the given resource ID, combining the
    /// resource type, resource ID, and role name.
    fn make_triplet<'a>(resource_id: &'a Uuid) -> PermissionTriplet<'a> {
        PermissionTriplet(Self::resource_type(), resource_id, Self::name())
    }

    /// Returns the resource type associated with the role.
    fn resource_type() -> &'static str;
}

// Automatically implement `ResourceRole` for any type that implements `SystemResourceRole`.
impl<SystemRole: SystemResourceRole> ResourceRole for SystemRole {
    fn resource_type() -> &'static str {
        SYSTEM_RESOURCE_TYPE
    }
}

/// Represents a role assignment for a user or organization on a specific resource.
#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "resource_permissions")]
pub struct ResourcePermission {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub resource_id: Uuid,
    pub resource_type: String,
    pub role_name: String,
    pub granted_by: Option<Uuid>,
    pub grant_reason: String,
    pub granted_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [ResourcePermissionIden; 9] = [
    ResourcePermissionIden::Id,
    ResourcePermissionIden::UserId,
    ResourcePermissionIden::OrganizationId,
    ResourcePermissionIden::ResourceId,
    ResourcePermissionIden::ResourceType,
    ResourcePermissionIden::RoleName,
    ResourcePermissionIden::GrantedBy,
    ResourcePermissionIden::GrantReason,
    ResourcePermissionIden::GrantedAt,
];

/// Represents either a user or an organization for role assignment purposes.
#[derive(Debug, Copy, Clone)]
pub enum UserOrOrganizationId {
    User(Uuid),
    Org(Uuid),
}

/// Request struct for granting a role to a user or organization on a resource.
#[derive(Debug)]
pub struct GrantRoleRequest<'request> {
    pub actor_id: UserOrOrganizationId,
    pub granted_by: &'request Uuid,
    pub grant_reason: &'request str,
    pub permission_triplet: PermissionTriplet<'request>,
}

/// Request struct for revoking a role from a user or organization on a resource.
#[derive(Debug)]
pub struct RevokeRoleRequest<'request> {
    pub actor_id: UserOrOrganizationId,
    pub permission_triplet: PermissionTriplet<'request>,
}

/// Generates a cache key for storing permission checks in Redis.
fn permission_cache_key(
    permission_triplet: &PermissionTriplet<'_>,
    actor_id: &UserOrOrganizationId,
) -> String {
    let PermissionTriplet(resource_type, resource_id, role_name) = *permission_triplet;
    match *actor_id {
        UserOrOrganizationId::User(user_id) => {
            format!("perm:v1:{resource_type}:{resource_id}:{role_name}:user:{user_id}")
        }
        UserOrOrganizationId::Org(org_id) => {
            format!("perm:v1:{resource_type}:{resource_id}:{role_name}:org:{org_id}")
        }
    }
}

/// Checks the Redis cache for a permission check result.
async fn cache_get(
    conn: &dyn RedisConnection,
    user_key: &str,
    org_key: Option<&str>,
) -> Option<bool> {
    let mut keys = vec![user_key];
    if let Some(k) = org_key {
        keys.push(k);
    }
    let results = match conn.get_multi(&keys).await {
        Ok(v) => v,
        Err(_) => return None,
    };
    let user_result = results.first().cloned().flatten();
    let org_result = if results.len() > 1 {
        results.get(1).cloned().flatten()
    } else {
        None
    };

    match (user_result.as_deref(), org_result.as_deref()) {
        (Some("1"), _) | (_, Some("1")) => Some(true),
        (None, None) => None,
        _ => Some(false),
    }
}

/// Sets a permission check result in the Redis cache.
async fn cache_set(conn: &dyn RedisConnection, key: &str, value: bool, ttl_secs: u64) {
    let val = if value { "1" } else { "0" };
    let _ = conn.set_ex(key, val, ttl_secs).await;
}

/// Deletes a permission check result from the Redis cache.
async fn cache_delete(conn: &dyn RedisConnection, key: &str) {
    let _ = conn.del(key).await;
}

/// Grants a role to a user or organization on a specific resource.
///
/// # Errors
///
/// * Returns [`ComhairleError::RoleAlreadyGranted`] if the role is already
/// assigned.
/// * Returns [`ComhairleError::DatabaseError`] if there is an error interacting
/// with the database.
pub async fn grant_role(
    state: &Arc<ComhairleState>,
    request: GrantRoleRequest<'_>,
) -> Result<ResourcePermission, ComhairleError> {
    let (user_id, organization_id) = match request.actor_id {
        UserOrOrganizationId::User(user_id) => (Some(user_id), None),
        UserOrOrganizationId::Org(org_id) => (None, Some(org_id)),
    };

    let PermissionTriplet(resource_type, resource_id, role_name) = request.permission_triplet;

    let mut query = Query::insert();
    query
        .into_table(ResourcePermissionIden::Table)
        .columns([
            ResourcePermissionIden::UserId,
            ResourcePermissionIden::OrganizationId,
            ResourcePermissionIden::ResourceId,
            ResourcePermissionIden::ResourceType,
            ResourcePermissionIden::RoleName,
            ResourcePermissionIden::GrantedBy,
            ResourcePermissionIden::GrantReason,
        ])
        .values_panic([
            user_id.into(),
            organization_id.into(),
            (*resource_id).into(),
            resource_type.into(),
            role_name.into(),
            (*request.granted_by).into(),
            request.grant_reason.to_owned().into(),
        ])
        .on_conflict(OnConflict::new().do_nothing().to_owned())
        .returning(Query::returning().columns(DEFAULT_COLUMNS));

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let response = sqlx::query_as_with::<_, ResourcePermission, _>(&sql, values)
        .fetch_optional(&state.db)
        .await
        .map_err(ComhairleError::DatabaseError)?;

    let permission =
        response.ok_or_else(|| ComhairleError::RoleAlreadyGranted(role_name.to_string()))?;

    if let Some(conn) = &state.redis_conn {
        let key = permission_cache_key(&request.permission_triplet, &request.actor_id);
        cache_delete(conn.as_ref(), &key).await;
    }

    Ok(permission)
}

/// Revokes a role from a user or organization on a specific resource.
///
/// # Errors
///
/// * Returns [`ComhairleError::RoleNotFound`] if the role was not previously
/// granted.
/// * Returns [`ComhairleError::DatabaseError`] if there is an error interactin
/// with the database.
pub async fn revoke_role(
    state: &Arc<ComhairleState>,
    request: RevokeRoleRequest<'_>,
) -> Result<(), ComhairleError> {
    let PermissionTriplet(resource_type, resource_id, role_name) = request.permission_triplet;

    let mut tx = state
        .db
        .begin()
        .await
        .map_err(ComhairleError::DatabaseError)?;

    if resource_type == SYSTEM_RESOURCE_TYPE && role_name == "admin" {
        let mut count_query = Query::select();
        count_query
            .expr(sea_query::Expr::cust("count(*)"))
            .from(ResourcePermissionIden::Table)
            .and_where(Expr::col(ResourcePermissionIden::ResourceType).eq(SYSTEM_RESOURCE_TYPE))
            .and_where(Expr::col(ResourcePermissionIden::RoleName).eq("admin"));

        let (sql, values) = count_query.build_sqlx(PostgresQueryBuilder);

        let count: i64 = sqlx::query_scalar_with(&sql, values)
            .fetch_one(&mut *tx)
            .await
            .map_err(ComhairleError::DatabaseError)?;

        if count <= 1 {
            return Err(ComhairleError::CannotRevokeLastAdmin);
        }
    }

    let mut query = Query::delete();
    query
        .from_table(ResourcePermissionIden::Table)
        .and_where(Expr::col(ResourcePermissionIden::ResourceId).eq(*resource_id))
        .and_where(Expr::col(ResourcePermissionIden::ResourceType).eq(resource_type))
        .and_where(Expr::col(ResourcePermissionIden::RoleName).eq(role_name));

    match request.actor_id {
        UserOrOrganizationId::User(user_id) => {
            query.and_where(Expr::col(ResourcePermissionIden::UserId).eq(user_id));
        }
        UserOrOrganizationId::Org(org_id) => {
            query.and_where(Expr::col(ResourcePermissionIden::OrganizationId).eq(org_id));
        }
    };

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let response = sqlx::query_with(&sql, values)
        .execute(&mut *tx)
        .await
        .map_err(ComhairleError::DatabaseError)?;

    if response.rows_affected() == 0 {
        return Err(ComhairleError::RoleNotFound(role_name.to_string()));
    }

    tx.commit().await.map_err(ComhairleError::DatabaseError)?;

    if let Some(conn) = &state.redis_conn {
        let key = permission_cache_key(&request.permission_triplet, &request.actor_id);
        cache_delete(conn.as_ref(), &key).await;
    }

    Ok(())
}

/// Filters for listing permissions, allowing optional filtering and pagination
/// via `page_options`.
#[derive(Debug, Default)]
pub struct ListPermissionsFilters<'request> {
    pub resource_type: Option<&'request str>,
    pub resource_id: Option<&'request Uuid>,
    pub actor: Option<UserOrOrganizationId>,
    pub role_name: Option<&'request str>,
    pub page_options: PageOptions,
}

/// Lists permissions using optional page-based pagination, with optional
/// filtering by resource, actor (user or organization), and role name.
///
/// # Errors
///
/// Returns [`ComhairleError::DatabaseError`] if there is an error querying the database.
pub async fn list_permissions(
    state: &Arc<ComhairleState>,
    request: ListPermissionsFilters<'_>,
) -> Result<PaginatedResults<ResourcePermission>, ComhairleError> {
    let mut query = Query::select();
    query
        .from(ResourcePermissionIden::Table)
        .columns(DEFAULT_COLUMNS);

    if let Some(resource_type) = request.resource_type {
        query.and_where(Expr::col(ResourcePermissionIden::ResourceType).eq(resource_type));
    }
    if let Some(resource_id) = request.resource_id {
        query.and_where(Expr::col(ResourcePermissionIden::ResourceId).eq(*resource_id));
    }

    match request.actor {
        Some(UserOrOrganizationId::User(user_id)) => {
            query.and_where(Expr::col(ResourcePermissionIden::UserId).eq(user_id));
        }
        Some(UserOrOrganizationId::Org(org_id)) => {
            query.and_where(Expr::col(ResourcePermissionIden::OrganizationId).eq(org_id));
        }
        None => {}
    }

    if let Some(role_name) = request.role_name {
        query.and_where(Expr::col(ResourcePermissionIden::RoleName).eq(role_name));
    }

    request
        .page_options
        .fetch_paginated_results(&state.db, query)
        .await
        .map_err(ComhairleError::DatabaseError)
}

/// Check whether a user, or their organization, has a specific role on a resource.
pub async fn has_resource_permission(
    state: &Arc<ComhairleState>,
    permission_triplet: PermissionTriplet<'_>,
    user_id: &Uuid,
    organization_id: Option<&Uuid>,
) -> Result<bool, ComhairleError> {
    let redis_conn = state.redis_conn.clone();
    if let Some(ref conn) = redis_conn {
        let key = permission_cache_key(&permission_triplet, &UserOrOrganizationId::User(*user_id));
        let org_key = organization_id.map(|org_id| {
            permission_cache_key(&permission_triplet, &UserOrOrganizationId::Org(*org_id))
        });
        if let Some(cached) = cache_get(conn.as_ref(), &key, org_key.as_deref()).await {
            return Ok(cached);
        }
    }

    let PermissionTriplet(resource_type, resource_id, role_name) = permission_triplet;

    let mut query = Query::select();
    query
        .expr(Expr::val(1))
        .from(ResourcePermissionIden::Table)
        .and_where(Expr::col(ResourcePermissionIden::ResourceId).eq(*resource_id))
        .and_where(Expr::col(ResourcePermissionIden::ResourceType).eq(resource_type))
        .and_where(Expr::col(ResourcePermissionIden::RoleName).eq(role_name))
        .limit(1);

    match organization_id {
        Some(organization_id) => {
            query.and_where(
                Expr::col(ResourcePermissionIden::UserId)
                    .eq(*user_id)
                    .or(Expr::col(ResourcePermissionIden::OrganizationId).eq(*organization_id)),
            );
        }
        None => {
            query.and_where(Expr::col(ResourcePermissionIden::UserId).eq(*user_id));
        }
    }

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let response = sqlx::query_with(&sql, values)
        .fetch_optional(&state.db)
        .await
        .map_err(ComhairleError::DatabaseError)?;

    let result = response.is_some();

    if let Some(ref conn) = redis_conn {
        // If organization_id is not None and the returned PgRow has a populated organization_id, cache the result for the organization only.
        // Otherwise, cache the result for the user only.
        let is_user = if organization_id.is_some() {
            match response {
                Some(row) => {
                    let org_id_in_row: Option<Uuid> = row
                        .try_get(ResourcePermissionIden::OrganizationId.as_str())
                        .unwrap_or(None);
                    org_id_in_row.is_none()
                }
                None => true,
            }
        } else {
            true
        };
        let key = if is_user {
            permission_cache_key(&permission_triplet, &UserOrOrganizationId::User(*user_id))
        } else {
            permission_cache_key(
                &permission_triplet,
                &UserOrOrganizationId::Org(*organization_id.unwrap()),
            )
        };
        cache_set(
            conn.as_ref(),
            &key,
            result,
            state.config.redis_cache_ttl_secs,
        )
        .await;
    }

    Ok(result)
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserWithPermissionDto {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
    pub role_name: String,
}

#[instrument(err(Debug))]
pub async fn list_users_with_permission(
    db: &PgPool,
    resource_type: &str,
    resource_id: Uuid,
    role_name: Option<&str>,
) -> Result<Vec<UserWithPermissionDto>, ComhairleError> {
    let mut query = Query::select()
        .from(ResourcePermissionIden::Table)
        .join(
            JoinType::InnerJoin,
            UserIden::Table,
            Expr::col((UserIden::Table, UserIden::Id)).equals((
                ResourcePermissionIden::Table,
                ResourcePermissionIden::UserId,
            )),
        )
        .columns([
            (UserIden::Table, UserIden::Id),
            (UserIden::Table, UserIden::Username),
            (UserIden::Table, UserIden::Email),
        ])
        .column((
            ResourcePermissionIden::Table,
            ResourcePermissionIden::RoleName,
        ))
        .and_where(
            Expr::col((
                ResourcePermissionIden::Table,
                ResourcePermissionIden::ResourceType,
            ))
            .eq(resource_type.to_owned()),
        )
        .and_where(
            Expr::col((
                ResourcePermissionIden::Table,
                ResourcePermissionIden::ResourceId,
            ))
            .eq(resource_id.to_owned()),
        )
        .to_owned();

    if let Some(role_name) = role_name {
        query = query
            .and_where(
                Expr::col((
                    ResourcePermissionIden::Table,
                    ResourcePermissionIden::RoleName,
                ))
                .eq(role_name.to_owned()),
            )
            .to_owned();
    }

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let users_with_permission = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(users_with_permission)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ComhairleError;
    use crate::models::model_test_helpers::{
        get_random_organization_id, get_random_user_id, setup_default_app_and_session,
    };
    use crate::redis_connection::{MockRedis, RedisConnection};
    use crate::test_helpers::{TEST_RESOURCE_TYPE, TEST_ROLE_NAME, TestRole, test_state};

    use sqlx::PgPool;

    const OTHER_ROLE_NAME: &str = "other_role";
    struct OtherRole;

    impl NamedRole for OtherRole {
        fn name() -> &'static str {
            OTHER_ROLE_NAME
        }
    }

    impl ResourceRole for OtherRole {
        fn resource_type() -> &'static str {
            TEST_RESOURCE_TYPE
        }
    }

    #[sqlx::test]
    async fn test_grant_and_check_user_role(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
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

    #[sqlx::test]
    async fn test_grant_and_check_organization_role(
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

    #[sqlx::test]
    async fn test_unauthorized_access(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
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

    #[sqlx::test]
    async fn test_grant_role_already_granted(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
            matches!(err, ComhairleError::RoleAlreadyGranted(_)),
            "Expected RoleAlreadyGranted, got {err:?}"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn test_revoke_user_role(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
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

    #[sqlx::test]
    async fn test_revoke_last_admin_fails(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let (app, mut session) = setup_default_app_and_session(&state.db).await?;

        let (_, user, _) = session.current_user(&app).await?;

        // Attempt to revoke the admin role, which should be the only one
        let result = revoke_role(
            &state,
            RevokeRoleRequest {
                actor_id: UserOrOrganizationId::User(user.id),
                permission_triplet: SystemAdminRole::make_system_triplet(),
            },
        )
        .await;

        assert!(
            matches!(result, Err(ComhairleError::CannotRevokeLastAdmin)),
            "Expected CannotRevokeLastAdmin, got {result:?}"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn test_revoke_role_not_found(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
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
            matches!(err, ComhairleError::RoleNotFound(_)),
            "Expected RoleNotFound, got {err:?}"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn test_list_permissions(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
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
        let all_permissions = list_permissions(&state, request).await?;
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
        let permissions = list_permissions(&state, request).await?;
        assert_eq!(permissions.records.len(), 2);

        // List permissions for the user
        let request = ListPermissionsFilters {
            actor: Some(UserOrOrganizationId::User(user_id)),
            ..Default::default()
        };
        let user_permissions = list_permissions(&state, request).await?;
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
        let org_permissions = list_permissions(&state, request).await?;
        assert_eq!(org_permissions.records.len(), 1);
        assert_eq!(org_permissions.records[0].resource_id, resource_1_id);
        assert_eq!(org_permissions.records[0].role_name, OTHER_ROLE_NAME);

        // Filter by role_name: only the two OtherRole assignments granted above.
        let request = ListPermissionsFilters {
            role_name: Some(OTHER_ROLE_NAME),
            ..Default::default()
        };
        let viewer_permissions = list_permissions(&state, request).await?;
        assert_eq!(viewer_permissions.records.len(), 2);
        assert!(
            viewer_permissions
                .records
                .iter()
                .all(|p| p.role_name == OTHER_ROLE_NAME)
        );

        Ok(())
    }

    #[sqlx::test]
    async fn test_list_permissions_offset_pagination(
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
            let page = list_permissions(&state, request).await?;

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

    #[sqlx::test]
    async fn test_permission_is_cached_after_first_call(
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

        let key = permission_cache_key(
            &TestRole::make_triplet(&resource_id),
            &UserOrOrganizationId::User(user_id),
        );
        let cached = mock.get_value(&key).await;
        assert_eq!(
            cached.as_deref(),
            Some("1"),
            "expected cache key to hold \"1\" after first positive check"
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

    #[sqlx::test]
    async fn test_cache_invalidated_on_grant(
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

        // First check: no permission exists; false result is cached.
        let first =
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id, None)
                .await?;
        assert!(!first, "expected no permission to exist yet");

        let key = permission_cache_key(
            &TestRole::make_triplet(&resource_id),
            &UserOrOrganizationId::User(user_id),
        );
        let cached = mock.get_value(&key).await;
        assert_eq!(
            cached.as_deref(),
            Some("0"),
            "expected cache key to hold \"0\" after first negative check"
        );

        // Grant the role, which should invalidate the cache.
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing cache invalidation",
            },
        )
        .await?;

        let after_grant = mock.get_value(&key).await;
        assert!(
            after_grant.is_none(),
            "expected cache key to be deleted after grant_role, got {after_grant:?}"
        );

        // Permission check now re-queries the DB and should return true.
        let second =
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id, None)
                .await?;
        assert!(second, "expected permission to be present after grant");

        Ok(())
    }

    #[sqlx::test]
    async fn test_cache_invalidated_on_revoke(
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

        // Grant the role and confirm it is cached.
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing cache invalidation",
            },
        )
        .await?;

        let first =
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id, None)
                .await?;
        assert!(first, "expected permission to be present");

        let key = permission_cache_key(
            &TestRole::make_triplet(&resource_id),
            &UserOrOrganizationId::User(user_id),
        );
        let cached = mock.get_value(&key).await;
        assert_eq!(
            cached.as_deref(),
            Some("1"),
            "expected cache key to hold \"1\" after positive check"
        );

        // Revoke the role, which should invalidate the cache.
        revoke_role(
            &state,
            RevokeRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_id),
            },
        )
        .await?;

        let after_revoke = mock.get_value(&key).await;
        assert!(
            after_revoke.is_none(),
            "expected cache key to be deleted after revoke_role, got {after_revoke:?}"
        );

        // Permission check now re-queries the DB and should return false.
        let second =
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id, None)
                .await?;
        assert!(!second, "expected no permission after revoke");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_users_with_permission(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
            "user_b incorrectly included"
        );
        assert!(
            users_with_permission
                .iter()
                .all(|u| u.role_name == TestRole::name()),
            "wrong role_name"
        );

        Ok(())
    }
}
