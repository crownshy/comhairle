//! # Permissions model
//!
//! This module defines the permissions model for the Comhairle application, including resource types, roles, actions, and permission handling.
//!
//! ## Definitions:
//! - Resource Type      : A string that identifies a category of resources in the system (e.g., "system", "conversation").
//! - Resource ID        : A UUID that uniquely identifies a specific resource within its category.
//! - Role               : An alias for a group of permissions that can be assigned to a user or organization on a specific resource.
//! - Action             : A specific operation that can be performed on a resource, such as reading, updating, or deleting it.
//! - Policy             : Defines which actions are allowed for a specific resource type on a per-role basis.
//! - Permission Triplet : A combination of resource type, resource ID, and role name that uniquely identifies a permission assignment.
//!
//! ## Adding New Resource Types, Roles, and Actions
//! Resource types, roles, and actions are each a single enum ([`ResourceType`],
//! [`Role`], [`Action`]). To extend the model:
//! 1. Add a variant to the relevant enum, preserving any persisted string value via
//!    `#[strum(serialize = "...")]` / `#[serde(rename = "...")]`.
//! 2. If a route needs to authorize against a specific resource, fetch it with
//!    [`models::resources::get_by_id`] and pass it to
//!    [`crate::routes::auth::authorize`] / [`can_perform_resource_action`].

use std::sync::Arc;

use crate::models::organization::OrganizationIden;
use crate::models::users::UserIden;
use crate::redis_connection::RedisConnection;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::JoinType;
use sea_query::{Expr, OnConflict, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::{PgPool, query_as_with};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};
use tracing::instrument;
use uuid::Uuid;

use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::models::pagination::{PageOptions, PaginatedResults};

// ------------------ //
// * RESOURCES * //
// ------------------ //
//
// - A resource ID is a UUID that uniquely identifies a specific resource within its category.
//

/// The global system resource uses a fixed ID as a workaround to avoid having a
/// separate table for system-level permissions. It is registered in
/// [`models::resources`] like any other resource, with no owner.
pub const SYSTEM_RESOURCE_ID: Uuid = Uuid::nil();

/// Path parameters for routes scoped to a conversation.
#[derive(Debug, Deserialize)]
pub struct ConversationPath {
    pub conversation_id: Uuid,
}

// --------- //
// * ROLES * //
// --------- //
//
// - A role is an alias for a group of permissions that can be assigned to a user or organization on a specific resource.
//

/// The set of roles that can be assigned to a user or organization.
///
/// The string form (via [`AsRefStr`] / [`Display`]) is what is persisted in the
/// `resource_permissions.role_name` column, so those values are load bearing.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
    EnumIter,
    IntoStaticStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Role {
    SuperAdmin,
    Admin,
    Editor,
    Moderator,
    Translator,
    Exporter,
    Viewer,
    #[cfg(test)]
    Tester,
}

impl Role {
    /// The actions this role is permitted to perform (the permission policy).
    pub fn actions(self) -> Vec<Action> {
        match self {
            // SuperAdmin is short-circuited, so this role does not need to list any actions.
            Role::SuperAdmin => vec![],
            // Admin can perform all possible actions on resources where they have access.
            Role::Admin => Action::iter().collect::<Vec<_>>(),
            Role::Editor => vec![Action::View, Action::Edit],
            Role::Moderator => vec![Action::Moderate, Action::View],
            Role::Translator => vec![Action::Translate, Action::View],
            Role::Exporter => vec![Action::Export, Action::View],
            Role::Viewer => vec![Action::View],
            #[cfg(test)]
            Role::Tester => vec![],
        }
    }

    /// Builds a [`ResourcePermissionTarget`] for this role on a specific resource.
    pub fn target(self, resource_id: Uuid) -> ResourcePermissionTarget {
        ResourcePermissionTarget(resource_id, self.to_string())
    }

    /// Builds a [`ResourcePermissionTarget`] for this role on the global system resource.
    pub fn system_target(self) -> ResourcePermissionTarget {
        ResourcePermissionTarget(SYSTEM_RESOURCE_ID, self.to_string())
    }

    /// Returns every role, for discovery endpoints.
    pub fn all() -> impl Iterator<Item = Role> {
        Role::iter()
    }
}

// ----------- //
// * ACTIONS * //
// ----------- //
//
// - An action represents a specific operation that can be performed on a resource, such as reading, updating, or deleting it.
//

/// The set of actions that can be performed on a resource.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
    EnumIter,
    IntoStaticStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Action {
    Admin, // Permission to perform administrative actions on a resource (create, delete, manage permissions, etc.)
    Edit,  // Permission to edit content associated with a resource.
    View,  // Permission to view content associated with a resource.
    Moderate, // Permission to moderate content associated with a resource.
    Translate, // Permission to translate content associated with a resource.
    Export, // Permission to export content associated with a resource.
}

impl Action {
    /// Returns every action, for discovery endpoints.
    pub fn all() -> impl Iterator<Item = Action> {
        Action::iter()
    }
}

// ----------------------- //
// * PERMISSION HANDLING * //
// ----------------------- //
//
// - Permission handling involves granting, revoking, and checking permissions for users and organizations on specific resources.
//

/// The triplet associated with a permission for a resource.
#[derive(Debug)]
pub struct ResourcePermissionTarget(
    pub Uuid,   // resource_id
    pub String, // role_name
);

/// Represents a role assignment for a user or organization on a specific resource.
#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "resource_permissions")]
pub struct ResourcePermission {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub resource_id: Uuid,
    pub role_name: String,
    pub granted_by: Option<Uuid>,
    pub grant_reason: String,
    pub granted_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [ResourcePermissionIden; 8] = [
    ResourcePermissionIden::Id,
    ResourcePermissionIden::UserId,
    ResourcePermissionIden::OrganizationId,
    ResourcePermissionIden::ResourceId,
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
pub struct GrantRoleRequest {
    pub actor_id: UserOrOrganizationId,
    pub granted_by: Uuid,
    pub grant_reason: String,
    pub permission_target: ResourcePermissionTarget,
}

/// Request struct for revoking a role from a user or organization on a resource.
#[derive(Debug)]
pub struct RevokeRoleRequest {
    pub actor_id: UserOrOrganizationId,
    pub permission_target: ResourcePermissionTarget,
}

/// Generates a cache key for storing all assigned role names for an actor on a resource.
fn role_list_cache_key(resource_id: Uuid, actor_id: UserOrOrganizationId) -> String {
    match actor_id {
        UserOrOrganizationId::User(user_id) => {
            format!("roles:v1:{resource_id}:user:{user_id}")
        }
        UserOrOrganizationId::Org(org_id) => {
            format!("roles:v1:{resource_id}:org:{org_id}")
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

/// Loads all role names assigned to a specific actor on a specific resource from Postgres.
#[instrument(err(Debug), skip(db))]
async fn fetch_actor_roles_for_resource(
    db: &PgPool,
    resource_id: Uuid,
    actor_id: UserOrOrganizationId,
) -> Result<Vec<String>, ComhairleError> {
    let mut query = Query::select();
    query
        .column(ResourcePermissionIden::RoleName)
        .from(ResourcePermissionIden::Table)
        .and_where(Expr::col(ResourcePermissionIden::ResourceId).eq(resource_id));

    match actor_id {
        UserOrOrganizationId::User(user_id) => {
            query.and_where(Expr::col(ResourcePermissionIden::UserId).eq(user_id));
        }
        UserOrOrganizationId::Org(org_id) => {
            query.and_where(Expr::col(ResourcePermissionIden::OrganizationId).eq(org_id));
        }
    }

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let roles: Vec<String> = sqlx::query_as_with::<_, (String,), _>(&sql, values)
        .fetch_all(db)
        .await
        .map_err(ComhairleError::DatabaseError)?
        .into_iter()
        .map(|(role_name,)| role_name)
        .collect();

    Ok(roles)
}

/// Returns role names for an actor on a resource, using Redis cache when available.
#[instrument(err(Debug), skip(state))]
async fn get_actor_roles_for_resource(
    state: &Arc<ComhairleState>,
    resource_id: Uuid,
    actor_id: UserOrOrganizationId,
) -> Result<Vec<String>, ComhairleError> {
    let cache_key = role_list_cache_key(resource_id, actor_id);

    if let Some(conn) = &state.redis_conn {
        if let Some(cached_roles) = cache_get_role_list(conn.as_ref(), &cache_key).await {
            return Ok(cached_roles);
        }
    }

    let roles = fetch_actor_roles_for_resource(&state.db, resource_id, actor_id).await?;

    if let Some(conn) = &state.redis_conn {
        cache_set_role_list(
            conn.as_ref(),
            &cache_key,
            &roles,
            state.config.redis_cache_ttl_secs,
        )
        .await;
    }

    Ok(roles)
}

/// Grants a role to a user or organization on a specific resource.
///
/// # Errors
///
/// * Returns [`ComhairleError::RoleAlreadyGranted`] if the role is already
/// assigned.
/// * Returns [`ComhairleError::DatabaseError`] if there is an error interacting
/// with the database.
#[instrument(err(Debug), skip(state))]
pub async fn grant_role(
    state: &Arc<ComhairleState>,
    request: GrantRoleRequest,
) -> Result<ResourcePermission, ComhairleError> {
    let (user_id, organization_id) = match request.actor_id {
        UserOrOrganizationId::User(user_id) => (Some(user_id), None),
        UserOrOrganizationId::Org(org_id) => (None, Some(org_id)),
    };

    let ResourcePermissionTarget(resource_id, role_name) = request.permission_target;

    // Check if the role is already granted to the user or organization on the resource.
    let mut check_query = Query::select();
    check_query
        .column(ResourcePermissionIden::ResourceId)
        .from(ResourcePermissionIden::Table)
        .and_where(Expr::col(ResourcePermissionIden::ResourceId).eq(resource_id))
        .and_where(Expr::col(ResourcePermissionIden::RoleName).eq(role_name.clone()));

    if let Some(uid) = user_id {
        check_query.and_where(Expr::col(ResourcePermissionIden::UserId).eq(uid));
    } else if let Some(oid) = organization_id {
        check_query.and_where(Expr::col(ResourcePermissionIden::OrganizationId).eq(oid));
    }

    let (check_sql, check_values) = check_query.build_sqlx(PostgresQueryBuilder);

    let existing_role = sqlx::query_with(&check_sql, check_values)
        .fetch_optional(&state.db)
        .await
        .map_err(ComhairleError::DatabaseError)?;

    if existing_role.is_some() {
        return Err(ComhairleError::RoleAlreadyGranted(role_name.to_string()));
    }

    // Insert the new role assignment into the database.
    let mut query = Query::insert();
    query
        .into_table(ResourcePermissionIden::Table)
        .columns([
            ResourcePermissionIden::UserId,
            ResourcePermissionIden::OrganizationId,
            ResourcePermissionIden::ResourceId,
            ResourcePermissionIden::RoleName,
            ResourcePermissionIden::GrantedBy,
            ResourcePermissionIden::GrantReason,
        ])
        .values_panic([
            user_id.into(),
            organization_id.into(),
            (resource_id).into(),
            role_name.clone().into(),
            (request.granted_by).into(),
            request.grant_reason.into(),
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
        let role_list_key = role_list_cache_key(resource_id, request.actor_id);
        cache_delete(conn.as_ref(), &role_list_key).await;
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
#[instrument(err(Debug), skip(state))]
pub async fn revoke_role(
    state: &Arc<ComhairleState>,
    request: RevokeRoleRequest,
) -> Result<(), ComhairleError> {
    let ResourcePermissionTarget(resource_id, role_name) = request.permission_target;

    let mut tx = state
        .db
        .begin()
        .await
        .map_err(ComhairleError::DatabaseError)?;

    if resource_id == SYSTEM_RESOURCE_ID && role_name == Role::SuperAdmin.as_ref() {
        let mut count_query = Query::select();
        count_query
            .expr(sea_query::Expr::cust("count(*)"))
            .from(ResourcePermissionIden::Table)
            .and_where(Expr::col(ResourcePermissionIden::RoleName).eq(Role::SuperAdmin.as_ref()));

        let (sql, values) = count_query.build_sqlx(PostgresQueryBuilder);

        let count: i64 = sqlx::query_scalar_with(&sql, values)
            .fetch_one(&mut *tx)
            .await
            .map_err(ComhairleError::DatabaseError)?;

        if count <= 1 {
            return Err(ComhairleError::CannotRevokeLastSuperAdmin);
        }
    }

    let mut query = Query::delete();
    query
        .from_table(ResourcePermissionIden::Table)
        .and_where(Expr::col(ResourcePermissionIden::ResourceId).eq(resource_id))
        .and_where(Expr::col(ResourcePermissionIden::RoleName).eq(role_name.clone()));

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
        return Err(ComhairleError::RoleNotFound(role_name));
    }

    tx.commit().await.map_err(ComhairleError::DatabaseError)?;

    if let Some(conn) = &state.redis_conn {
        let role_list_key = role_list_cache_key(resource_id, request.actor_id);
        cache_delete(conn.as_ref(), &role_list_key).await;
    }

    Ok(())
}

/// Filters for listing permissions, allowing optional filtering and pagination
/// via `page_options`.
#[derive(Debug, Default)]
pub struct ListPermissionsFilters {
    pub resource_id: Option<Uuid>,
    pub actor: Option<UserOrOrganizationId>,
    pub role_name: Option<String>,
    pub page_options: PageOptions,
}

/// Lists permissions using optional page-based pagination, with optional
/// filtering by resource, actor (user or organization), and role name.
///
/// # Errors
///
/// Returns [`ComhairleError::DatabaseError`] if there is an error querying the database.
#[instrument(err(Debug), skip(state))]
pub async fn list_permissions(
    state: &Arc<ComhairleState>,
    request: ListPermissionsFilters,
) -> Result<PaginatedResults<ResourcePermission>, ComhairleError> {
    let mut query = Query::select();
    query
        .from(ResourcePermissionIden::Table)
        .columns(DEFAULT_COLUMNS);

    if let Some(resource_id) = request.resource_id {
        query.and_where(Expr::col(ResourcePermissionIden::ResourceId).eq(resource_id));
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
#[instrument(err(Debug), skip(state))]
pub async fn has_resource_permission(
    state: &Arc<ComhairleState>,
    permission_target: ResourcePermissionTarget,
    user_id: Uuid,
    organization_id: Option<Uuid>,
) -> Result<bool, ComhairleError> {
    let ResourcePermissionTarget(resource_id, role_name) = permission_target;

    let user_roles =
        get_actor_roles_for_resource(state, resource_id, UserOrOrganizationId::User(user_id))
            .await?;
    let org_roles = match organization_id {
        Some(org_id) => Some(
            get_actor_roles_for_resource(state, resource_id, UserOrOrganizationId::Org(org_id))
                .await?,
        ),
        None => None,
    };

    let user_has_role = user_roles
        .iter()
        .any(|cached_role| *cached_role == role_name);
    let org_has_role = org_roles
        .as_ref()
        .is_some_and(|roles| roles.iter().any(|cached_role| *cached_role == role_name));

    Ok(user_has_role || org_has_role)
}

/// Check whether a user, or their organization, can perform an action on a resource.
#[instrument(err(Debug), skip(state))]
pub async fn can_perform_resource_action(
    state: &Arc<ComhairleState>,
    resource_id: Uuid,
    action: Action,
    user_id: Uuid,
    organization_id: Option<Uuid>,
) -> Result<bool, ComhairleError> {
    let resource = crate::models::resources::get_by_id(&state.db, resource_id).await?;

    if resource
        .owner_id
        .is_some_and(|resource_owner_id| resource_owner_id == user_id)
    {
        return Ok(true);
    }

    // Bypass permission checks for super admins
    if has_resource_permission(
        state,
        Role::SuperAdmin.system_target(),
        user_id,
        organization_id,
    )
    .await?
    {
        return Ok(true);
    }

    let mut roles =
        get_actor_roles_for_resource(state, resource.id, UserOrOrganizationId::User(user_id))
            .await?;

    if let Some(org_id) = organization_id {
        let org_roles =
            get_actor_roles_for_resource(state, resource.id, UserOrOrganizationId::Org(org_id))
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

#[derive(Serialize, Deserialize, Debug, JsonSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserWithPermissionDto {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
    pub role_name: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationWithPermissionDto {
    pub id: Uuid,
    pub name: String,
    pub role_name: String,
}

#[instrument(err(Debug), skip(db))]
pub async fn list_users_with_permission(
    db: &PgPool,
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

#[instrument(err(Debug), skip(db))]
pub async fn list_organizations_with_permission(
    db: &PgPool,
    resource_id: Uuid,
    role_name: Option<&str>,
) -> Result<Vec<OrganizationWithPermissionDto>, ComhairleError> {
    let mut query = Query::select()
        .from(ResourcePermissionIden::Table)
        .join(
            JoinType::InnerJoin,
            OrganizationIden::Table,
            Expr::col((OrganizationIden::Table, OrganizationIden::Id)).equals((
                ResourcePermissionIden::Table,
                ResourcePermissionIden::OrganizationId,
            )),
        )
        .columns([
            (OrganizationIden::Table, OrganizationIden::Id),
            (OrganizationIden::Table, OrganizationIden::Name),
        ])
        .column((
            ResourcePermissionIden::Table,
            ResourcePermissionIden::RoleName,
        ))
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

    query = query
        .order_by(
            (OrganizationIden::Table, OrganizationIden::Name),
            sea_query::Order::Asc,
        )
        .to_owned();

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let organizations_with_permission = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(organizations_with_permission)
}

#[instrument(err(Debug), skip(db))]
pub async fn list_permissions_by_action(
    db: &PgPool,
    user_id: Uuid,
    organization_id: Option<Uuid>,
    action: &str,
) -> Result<Vec<ResourcePermission>, ComhairleError> {
    let Ok(action_enum) = action.parse::<Action>() else {
        return Err(ComhairleError::BadRequest(format!(
            "Invalid action: {}",
            action
        )));
    };

    let roles = Role::all()
        .filter(|role| role.actions().contains(&action_enum))
        .map(|role| role.as_ref().to_string())
        .collect::<Vec<String>>();

    let mut query = Query::select();

    query
        .from(ResourcePermissionIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(ResourcePermissionIden::RoleName).is_in(roles))
        .and_where(Expr::col(ResourcePermissionIden::UserId).eq(user_id.to_owned()));

    if let Some(org_id) = organization_id {
        query.and_where(Expr::col(ResourcePermissionIden::OrganizationId).eq(org_id.to_owned()));
    }

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let permissions = query_as_with(&sql, values)
        .fetch_all(db)
        .await
        .map_err(ComhairleError::DatabaseError)?;

    Ok(permissions)
}

pub async fn get_actions_for_user_and_resource(
    state: &Arc<ComhairleState>,
    user_id: Uuid,
    organization_id: Option<Uuid>,
    resource_id: Uuid,
) -> Result<Vec<Action>, ComhairleError> {
    let mut roles = list_permissions(
        &state,
        ListPermissionsFilters {
            resource_id: Some(resource_id),
            actor: Some(UserOrOrganizationId::User(user_id)),
            ..Default::default()
        },
    )
    .await?
    .records;

    // GH TODO: When we allow a user to belong to multiple organizations, this will need to iterate through them.
    if let Some(organization_id) = organization_id {
        roles.extend(
            list_permissions(
                &state,
                ListPermissionsFilters {
                    resource_id: Some(resource_id),
                    actor: Some(UserOrOrganizationId::Org(organization_id)),
                    ..Default::default()
                },
            )
            .await?
            .records,
        );
    }

    let mut actions = std::collections::HashSet::new();
    for permission in roles.drain(..) {
        let role = serde_json::from_value::<Role>(serde_json::Value::String(permission.role_name))
            .unwrap();
        for action in role.actions() {
            actions.insert(action);
        }
    }

    Ok(actions.into_iter().collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ComhairleError;
    use crate::models::model_test_helpers::{
        get_random_organization_id, get_random_user_id, setup_default_app_and_session,
    };
    use crate::redis_connection::{MockRedis, RedisConnection};
    use crate::test_helpers::{TEST_ROLE_NAME, TestRole, test_resource, test_state};

    use sea_query::DeleteStatement;
    use sqlx::PgPool;

    const OTHER_ROLE_NAME: &str = "other_role";
    struct OtherRole;

    impl OtherRole {
        fn name() -> String {
            OTHER_ROLE_NAME.to_string()
        }

        fn make_triplet(resource_id: Uuid) -> ResourcePermissionTarget {
            ResourcePermissionTarget(resource_id, OTHER_ROLE_NAME.to_string())
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
            permission_target: TestRole::permission_target(resource_id),
            granted_by: session.id.unwrap(),
            grant_reason: "Testing".to_string().to_string(),
        };

        let assignment = grant_role(&state, grant_request).await?;
        assert_eq!(assignment.user_id, Some(user_id));
        assert_eq!(assignment.organization_id, None);
        assert_eq!(assignment.role_name, TestRole::name());

        // Check that the user has the role
        let has_permission = has_resource_permission(
            &state,
            TestRole::permission_target(resource_id),
            user_id,
            None,
        )
        .await?;
        assert!(has_permission);

        // Check that the user does not have a different role
        let has_wrong_permission =
            has_resource_permission(&state, OtherRole::make_triplet(resource_id), user_id, None)
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
            permission_target: TestRole::permission_target(resource_id),
            granted_by: session.id.unwrap(),
            grant_reason: "Testing".to_string().to_string(),
        };

        let assignment = grant_role(&state, grant_request).await?;
        assert_eq!(assignment.user_id, None);
        assert_eq!(assignment.organization_id, Some(org_id));
        assert_eq!(assignment.role_name, TestRole::name());

        // Check that the user has the role through the organization
        let has_permission = has_resource_permission(
            &state,
            TestRole::permission_target(resource_id),
            user_id,
            Some(org_id),
        )
        .await?;
        assert!(has_permission);

        // Check that the user does not have a different role
        let has_wrong_permission = has_resource_permission(
            &state,
            OtherRole::make_triplet(resource_id),
            user_id,
            Some(org_id),
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
        let has_permission = has_resource_permission(
            &state,
            TestRole::permission_target(resource_id),
            user_id,
            None,
        )
        .await?;
        assert!(!has_permission);

        // Check that the user does not have any roles through the organization
        let has_org_permission = has_resource_permission(
            &state,
            TestRole::permission_target(resource_id),
            user_id,
            Some(organization_id),
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

        let resource = test_resource(&state.db, None).await?;

        // Grant the role for the first time.
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_target: TestRole::permission_target(resource.id),
                granted_by: session.id.unwrap(),
                grant_reason: "Testing".to_string(),
            },
        )
        .await?;

        // Granting the same role again should return RoleAlreadyGranted.
        let result = grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_target: TestRole::permission_target(resource.id),
                granted_by: session.id.unwrap(),
                grant_reason: "Testing".to_string(),
            },
        )
        .await;

        assert!(
            matches!(result, Err(ComhairleError::RoleAlreadyGranted(_))),
            "Expected Err(RoleAlreadyGranted(_)), got {result:?}"
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
                permission_target: TestRole::permission_target(resource_id),
                granted_by: session.id.unwrap(),
                grant_reason: "Testing".to_string(),
            },
        )
        .await?;

        // Confirm permission is granted.
        assert!(
            has_resource_permission(
                &state,
                TestRole::permission_target(resource_id),
                user_id,
                None,
            )
            .await?
        );

        // Revoke the role.
        revoke_role(
            &state,
            RevokeRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_target: TestRole::permission_target(resource_id),
            },
        )
        .await?;

        // Confirm permission no longer granted.
        assert!(
            !has_resource_permission(
                &state,
                TestRole::permission_target(resource_id),
                user_id,
                None,
            )
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
                permission_target: Role::SuperAdmin.system_target(),
            },
        )
        .await;

        assert!(
            matches!(result, Err(ComhairleError::CannotRevokeLastSuperAdmin)),
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
                permission_target: TestRole::permission_target(resource_id),
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
                permission_target: TestRole::permission_target(resource_1_id),
                granted_by: session.id.unwrap(),
                grant_reason: "Testing".to_string(),
            },
        )
        .await?;

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::Org(org_id),
                permission_target: OtherRole::make_triplet(resource_1_id),
                granted_by: session.id.unwrap(),
                grant_reason: "Testing".to_string(),
            },
        )
        .await?;

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_target: OtherRole::make_triplet(resource_2_id),
                granted_by: session.id.unwrap(),
                grant_reason: "Testing".to_string(),
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
            resource_id: Some(resource_1_id),
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
            role_name: Some(OTHER_ROLE_NAME.to_string()),
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
                permission_target: TestRole::permission_target(resource_id),
                granted_by: session.id.unwrap(),
                grant_reason: "Testing".to_string(),
            },
        )
        .await?;

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_target: OtherRole::make_triplet(resource_id),
                granted_by: session.id.unwrap(),
                grant_reason: "Testing".to_string(),
            },
        )
        .await?;

        // Walk the resource scoped listing two records at a time.
        let mut seen = Vec::new();
        let mut offset: Option<u64> = None;
        const LIMIT: usize = 1;
        loop {
            let request = ListPermissionsFilters {
                resource_id: Some(resource_id),
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
                permission_target: Role::Viewer.target(resource_1_id),
                granted_by: session.id.unwrap(),
                grant_reason: "Testing".to_string().to_string(),
            },
        )
        .await?;

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_target: Role::Editor.target(resource_2_id),
                granted_by: session.id.unwrap(),
                grant_reason: "Testing".to_string().to_string(),
            },
        )
        .await?;

        // Grant the organization admin role to the user for resource_3
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_target: Role::Admin.target(resource_3_id),
                granted_by: session.id.unwrap(),
                grant_reason: "Testing".to_string().to_string(),
            },
        )
        .await?;

        // List permissions which permit the user to perform the "view" action
        let permissions = list_permissions_by_action(&state.db, user_id, None, "view").await?;

        eprintln!("Permissions for view action: {:?}", permissions);

        assert_eq!(permissions.len(), 3);
        assert!(permissions.iter().any(|p| p.resource_id == resource_1_id));
        assert!(permissions.iter().any(|p| p.resource_id == resource_2_id));
        assert!(permissions.iter().any(|p| p.resource_id == resource_3_id));

        // List permissions which permit the user to perform the "edit" action
        let permissions = list_permissions_by_action(&state.db, user_id, None, "edit").await?;

        eprintln!("Permissions for edit action: {:?}", permissions);

        assert_eq!(permissions.len(), 2);
        assert!(permissions.iter().any(|p| p.resource_id == resource_2_id));
        assert!(permissions.iter().any(|p| p.resource_id == resource_3_id));

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
                permission_target: TestRole::permission_target(resource_id),
                granted_by: session.id.unwrap(),
                grant_reason: "Testing cache".to_string(),
            },
        )
        .await?;

        // First check: hits the DB and populates the cache.
        let first = has_resource_permission(
            &state,
            TestRole::permission_target(resource_id),
            user_id,
            None,
        )
        .await?;
        assert!(first, "expected permission to be present");

        let key = role_list_cache_key(resource_id, UserOrOrganizationId::User(user_id));
        let cached = mock.get_value(&key).await;
        assert!(
            cached
                .as_ref()
                .is_some_and(|raw| raw.contains(TestRole::name())),
            "expected cache key to contain the granted role after first positive check"
        );

        // Remove the DB row directly – bypassing the permission model so the
        // cache entry is NOT invalidated.
        sqlx::query("DELETE FROM resource_permissions WHERE user_id = $1 AND resource_id = $2 AND role_name = $3")
            .bind(user_id)
            .bind(resource_id)
            .bind(TestRole::name())
            .execute(&pool)
            .await?;

        // Second check: DB row is gone but result should still come from cache.
        let second = has_resource_permission(
            &state,
            TestRole::permission_target(resource_id),
            user_id,
            None,
        )
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

        let initial_has_role = has_resource_permission(
            &state,
            TestRole::permission_target(resource_id),
            user_id,
            None,
        )
        .await?;
        assert!(!initial_has_role, "expected no role before grant");

        let role_list_key = role_list_cache_key(resource_id, UserOrOrganizationId::User(user_id));
        let cached_before_grant = mock.get_value(&role_list_key).await;
        assert_eq!(cached_before_grant.as_deref(), Some("[]"));

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_target: TestRole::permission_target(resource_id),
                granted_by: session.id.unwrap(),
                grant_reason: "Testing role list cache invalidation on grant".to_string(),
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
                permission_target: TestRole::permission_target(resource_id),
                granted_by: session.id.unwrap(),
                grant_reason: "Testing role list cache invalidation on revoke".to_string(),
            },
        )
        .await?;

        let has_role = has_resource_permission(
            &state,
            TestRole::permission_target(resource_id),
            user_id,
            None,
        )
        .await?;
        assert!(has_role, "expected role to exist before revoke");

        let role_list_key = role_list_cache_key(resource_id, UserOrOrganizationId::User(user_id));
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
                permission_target: TestRole::permission_target(resource_id),
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

        let resource = test_resource(&state.db, None).await?;
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_target: Role::Editor.target(resource.id),
                granted_by: session.id.unwrap(),
                grant_reason: "Testing action role list cache".to_string(),
            },
        )
        .await?;

        let first =
            can_perform_resource_action(&state, resource.id, Action::View, user_id, None).await?;
        assert!(first, "expected first action check to be allowed");

        let role_list_key = role_list_cache_key(resource.id, UserOrOrganizationId::User(user_id));
        let cached_roles = mock.get_value(&role_list_key).await;
        assert!(
            cached_roles
                .as_ref()
                .is_some_and(|raw| raw.contains(Role::Editor.as_ref())),
            "expected cached role list to include content editor role"
        );

        let (sql, values) = DeleteStatement::new()
            .from_table("resource_permissions")
            .and_where(Expr::col("user_id").eq(user_id))
            .and_where(Expr::col("resource_id").eq(resource.id))
            .and_where(Expr::col("role_name").eq(Role::Editor.as_ref()))
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values).execute(&pool).await?;

        let second =
            can_perform_resource_action(&state, resource.id, Action::View, user_id, None).await?;
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
            permission_target: TestRole::permission_target(resource_id),
            granted_by: session.id.unwrap(),
            grant_reason: "Testing".to_string().to_string(),
        };
        grant_role(&state, grant_request_a).await?;

        let grant_request_b = GrantRoleRequest {
            actor_id: UserOrOrganizationId::User(user_b_id),
            permission_target: TestRole::permission_target(resource_id),
            granted_by: session.id.unwrap(),
            grant_reason: "Testing".to_string().to_string(),
        };
        grant_role(&state, grant_request_b).await?;

        let users_with_permission =
            list_users_with_permission(&state.db, resource_id, Some(TestRole::name())).await?;

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
    fn test_action_permission_granted_for_editor(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let (app, mut session) = setup_default_app_and_session(&state.db).await?;

        let user_id = get_random_user_id(&app, &mut session).await?;

        let resource = test_resource(&state.db, None).await?;
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_target: Role::Editor.target(resource.id),
                granted_by: session.id.unwrap(),
                grant_reason: "Testing action checks".to_string(),
            },
        )
        .await?;

        let can_read =
            can_perform_resource_action(&state, resource.id, Action::View, user_id, None).await?;
        assert!(can_read, "content editor should allow read action");

        let can_update =
            can_perform_resource_action(&state, resource.id, Action::Edit, user_id, None).await?;
        assert!(can_update, "content editor should allow update action");

        Ok(())
    }
}
