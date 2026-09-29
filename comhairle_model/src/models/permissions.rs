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
//! 2. Map new roles to their resource type in [`Role::resource_type`] and their allowed
//!    actions in [`Role::actions`]; map new actions to their resource type in
//!    [`Action::resource_type`].
//! 3. If the resource is addressed by a path, add an extractor struct via
//!    `define_owned_resource!` / `define_unowned_resource!` so [`crate::routes::auth::authorize`]
//!    can resolve its id and owner.
//!
//! ## Helper Macros
//! - `define_owned_resource!`   : Defines a resource struct with an owner and implements the `ExtractResourceId` and `OwnedResource` traits for it.
//! - `define_unowned_resource!` : Defines a resource struct without an owner and implements the `ExtractResourceId` and `OwnedResource` traits for it.

use crate::models::error::ModelError;
use crate::models::error::ValidationError;
use crate::models::error::{DataError, PermissionError};

use crate::models::organization::OrganizationIden;
use crate::models::users::UserIden;
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

use crate::models::pagination::{PageOptions, PaginatedResults};
// ------------------ //
// * RESOURCE TYPES * //
// ------------------ //
//
// - A resource type is a string that identifies a category of resources in the system.
// - A resource ID is a UUID that uniquely identifies a specific resource within its category.
//

/// A trait for extracting owner_id for a resource if available
pub trait OwnedResource {
    fn owner_id(&self) -> Option<Uuid> {
        None
    }
}

// -- ENUM -- //

/// The set of resource categories that permissions can be granted on.
///
/// The string form (via [`AsRefStr`] / [`Display`]) is what is persisted in the
/// `resource_permissions.resource_type` column, so those values are load bearing.
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
pub enum ResourceType {
    System,
    Conversation,
    Organization,
    #[cfg(any(test, feature = "test-util"))]
    Test,
}

impl ResourceType {
    /// Returns every resource type, for discovery endpoints.
    pub fn all() -> impl Iterator<Item = ResourceType> {
        ResourceType::iter()
    }
}

// -- SYSTEM RESOURCE -- //

/// The global system resource uses a fixed ID as a workaround to avoid having a
/// separate table for system-level permissions.
pub const SYSTEM_RESOURCE_ID: Uuid = Uuid::nil();

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
    OrganizationAdmin,
    #[serde(rename = "content_editor")]
    #[strum(serialize = "content_editor")]
    ConversationContentEditor,
    ConversationCoHost,
    #[cfg(any(test, feature = "test-util"))]
    Tester,
}

impl Role {
    /// The resource type this role applies to.
    pub fn resource_type(self) -> ResourceType {
        match self {
            Role::SuperAdmin | Role::Admin => ResourceType::System,
            Role::OrganizationAdmin => ResourceType::Organization,
            Role::ConversationContentEditor | Role::ConversationCoHost => {
                ResourceType::Conversation
            }
            #[cfg(any(test, feature = "test-util"))]
            Role::Tester => ResourceType::Test,
        }
    }

    /// The actions this role is permitted to perform (the permission policy).
    pub fn actions(self) -> &'static [Action] {
        match self {
            Role::SuperAdmin => &[
                Action::ListPermission,
                Action::GrantPermission,
                Action::RevokePermission,
            ],
            Role::Admin => &[],
            Role::OrganizationAdmin => &[
                Action::OrganizationRead,
                Action::OrganizationUpdate,
                Action::OrganizationDelete,
            ],
            Role::ConversationContentEditor => {
                &[Action::ConversationRead, Action::ConversationUpdate]
            }
            Role::ConversationCoHost => &[Action::ConversationRead],
            #[cfg(any(test, feature = "test-util"))]
            Role::Tester => &[],
        }
    }

    /// Builds a [`PermissionTriplet`] for this role on a specific resource.
    pub fn triplet(self, resource_id: &Uuid) -> PermissionTriplet<'_> {
        if self.resource_type() == ResourceType::System && *resource_id != SYSTEM_RESOURCE_ID {
            panic!(
                "Cannot create a triplet for a system role with a specific resource ID. Use `system_triplet()` instead."
            );
        }
        PermissionTriplet(self.resource_type().into(), resource_id, self.into())
    }

    /// Builds a [`PermissionTriplet`] for this role on the global system resource.
    pub fn system_triplet(self) -> PermissionTriplet<'static> {
        if self.resource_type() != ResourceType::System {
            panic!("Cannot create a system triplet for a non-system role.");
        }
        PermissionTriplet(
            ResourceType::System.into(),
            &SYSTEM_RESOURCE_ID,
            self.into(),
        )
    }

    /// Returns every role, for discovery endpoints.
    pub fn all() -> impl Iterator<Item = Role> {
        Role::iter()
    }

    /// Returns the roles that apply to a given resource type.
    pub fn for_resource_type(resource_type: ResourceType) -> impl Iterator<Item = Role> {
        Role::iter().filter(move |role| role.resource_type() == resource_type)
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
    ListPermission,
    GrantPermission,
    RevokePermission,
    ConversationAdmin,
    ConversationRead,
    ConversationUpdate,
    OrganizationRead,
    OrganizationCreate,
    OrganizationUpdate,
    OrganizationDelete,
}

impl Action {
    /// The resource type this action applies to.
    pub fn resource_type(self) -> ResourceType {
        match self {
            Action::ListPermission
            | Action::GrantPermission
            | Action::RevokePermission
            | Action::OrganizationCreate => ResourceType::System,
            Action::ConversationRead | Action::ConversationUpdate | Action::ConversationAdmin => {
                ResourceType::Conversation
            }
            Action::OrganizationRead | Action::OrganizationUpdate | Action::OrganizationDelete => {
                ResourceType::Organization
            }
        }
    }

    /// Returns every action, for discovery endpoints.
    pub fn all() -> impl Iterator<Item = Action> {
        Action::iter()
    }

    /// Returns the actions that apply to a given resource type.
    pub fn for_resource_type(resource_type: ResourceType) -> impl Iterator<Item = Action> {
        Action::iter().filter(move |action| action.resource_type() == resource_type)
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
pub struct PermissionTriplet<'a>(
    pub &'a str,  // resource_type
    pub &'a Uuid, // resource_id
    pub &'a str,  // role_name
);

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

/// Loads all role names assigned to a specific actor on a specific resource from Postgres.
///
/// This is the uncached read; the caching layer on top lives in
/// `services::permissions`, which is also where invalidation happens.
#[instrument(err(Debug), skip(db))]
pub async fn fetch_actor_roles_for_resource(
    db: &PgPool,
    resource_type: &str,
    resource_id: &Uuid,
    actor_id: UserOrOrganizationId,
) -> Result<Vec<String>, ModelError> {
    let mut query = Query::select();
    query
        .column(ResourcePermissionIden::RoleName)
        .from(ResourcePermissionIden::Table)
        .and_where(Expr::col(ResourcePermissionIden::ResourceId).eq(*resource_id))
        .and_where(Expr::col(ResourcePermissionIden::ResourceType).eq(resource_type));

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
        .map_err(DataError::DatabaseError)?
        .into_iter()
        .map(|(role_name,)| role_name)
        .collect();

    Ok(roles)
}

/// Grants a role to a user or organization on a specific resource.
///
/// # Errors
///
/// * Returns [`PermissionError::RoleAlreadyGranted`] if the role is already
/// assigned.
/// * Returns [`DataError::DatabaseError`] if there is an error interacting
/// with the database.
///
/// This only writes the row. Invalidating the cached role list is the caller's
/// job; `services::permissions::grant_role` does both.
#[instrument(err(Debug), skip(db))]
pub async fn grant_role(
    db: &PgPool,
    request: GrantRoleRequest<'_>,
) -> Result<ResourcePermission, PermissionError> {
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
        .fetch_optional(db)
        .await
        .map_err(DataError::DatabaseError)?;

    let permission =
        response.ok_or_else(|| PermissionError::RoleAlreadyGranted(role_name.to_string()))?;

    Ok(permission)
}

/// Revokes a role from a user or organization on a specific resource.
///
/// # Errors
///
/// * Returns [`PermissionError::RoleNotFound`] if the role was not previously
/// granted.
/// * Returns [`DataError::DatabaseError`] if there is an error interactin
/// with the database.
///
/// This only deletes the row. Invalidating the cached role list is the caller's
/// job; `services::permissions::revoke_role` does both.
#[instrument(err(Debug), skip(db))]
pub async fn revoke_role(
    db: &PgPool,
    request: RevokeRoleRequest<'_>,
) -> Result<(), PermissionError> {
    let PermissionTriplet(resource_type, resource_id, role_name) = request.permission_triplet;

    let mut tx = db.begin().await.map_err(DataError::DatabaseError)?;

    if resource_type == ResourceType::System.as_ref() && role_name == Role::SuperAdmin.as_ref() {
        let mut count_query = Query::select();
        count_query
            .expr(sea_query::Expr::cust("count(*)"))
            .from(ResourcePermissionIden::Table)
            .and_where(
                Expr::col(ResourcePermissionIden::ResourceType).eq(ResourceType::System.as_ref()),
            )
            .and_where(Expr::col(ResourcePermissionIden::RoleName).eq(Role::SuperAdmin.as_ref()));

        let (sql, values) = count_query.build_sqlx(PostgresQueryBuilder);

        let count: i64 = sqlx::query_scalar_with(&sql, values)
            .fetch_one(&mut *tx)
            .await
            .map_err(DataError::DatabaseError)?;

        if count <= 1 {
            return Err(PermissionError::CannotRevokeLastSuperAdmin);
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
        .map_err(DataError::DatabaseError)?;

    if response.rows_affected() == 0 {
        return Err(PermissionError::RoleNotFound(role_name.to_string()));
    }

    tx.commit().await.map_err(DataError::DatabaseError)?;

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
/// Returns [`DataError::DatabaseError`] if there is an error querying the database.
#[instrument(err(Debug), skip(db))]
pub async fn list_permissions(
    db: &PgPool,
    request: ListPermissionsFilters<'_>,
) -> Result<PaginatedResults<ResourcePermission>, ModelError> {
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
        .fetch_paginated_results(db, query)
        .await
        .map_err(|err| DataError::DatabaseError(err).into())
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
    resource_type: &str,
    resource_id: Uuid,
    role_name: Option<&str>,
) -> Result<Vec<UserWithPermissionDto>, ModelError> {
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

#[instrument(err(Debug), skip(db))]
pub async fn list_organizations_with_permission(
    db: &PgPool,
    resource_type: &str,
    resource_id: Uuid,
    role_name: Option<&str>,
) -> Result<Vec<OrganizationWithPermissionDto>, ModelError> {
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
) -> Result<Vec<ResourcePermission>, ModelError> {
    let Ok(action_enum) = action.parse::<Action>() else {
        return Err(ValidationError::BadRequest(format!("Invalid action: {}", action)).into());
    };

    let roles = Role::for_resource_type(action_enum.resource_type())
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
        .map_err(DataError::DatabaseError)?;

    Ok(permissions)
}
