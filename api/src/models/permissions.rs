//! Resource-specific permission persistence and shared authorization.
//!
//! Conversation, Organization, and System modules each define User and UserGroup
//! records with SQLx row decoding and SeaQuery table identifiers, plus typed policies.
//! Writes target these six tables directly. `ResourcePermission` is a shared API
//! projection, not a table; cross-resource reads combine concrete-table queries.

use std::sync::Arc;

pub mod assignments;
pub mod conversation;
pub mod organization;
pub mod system;

use crate::models::organization::OrganizationIden;
use crate::models::users::UserIden;
use crate::redis_connection::RedisConnection;
use aide::OperationIo;
use axum::extract::{FromRequestParts, Path};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::JoinType;
use sea_query::{
    Alias, DynIden, Expr, IntoIden, OnConflict, PostgresQueryBuilder, Query, SelectStatement,
    UnionType,
};
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
use crate::models::{
    self,
    pagination::{PageOptions, PaginatedResults},
};

// ---------- //
// * MACROS * //
// ---------- //

macro_rules! impl_permission_assignment {
    ($record:ident, $identifier:ident, $resource_type:expr, $recipient_field:ident, $recipient_variant:ident, $recipient_column:ident) => {
        super::impl_permission_assignment!(
            $record,
            $identifier,
            $resource_type,
            $recipient_field,
            $recipient_variant,
            $recipient_column,
            |assignment: &Self| assignment.resource_id
        );
    };
    ($record:ident, $identifier:ident, $resource_type:expr, $recipient_field:ident, $recipient_variant:ident, $recipient_column:ident, $resource_id:expr) => {
        impl super::PermissionAssignment for $record {
            const RESOURCE_TYPE: super::ResourceType = $resource_type;

            fn table() -> sea_query::DynIden {
                sea_query::IntoIden::into_iden($identifier::Table)
            }

            fn recipient_column() -> sea_query::DynIden {
                sea_query::IntoIden::into_iden($identifier::$recipient_column)
            }

            fn into_resource_permission(self) -> super::ResourcePermission {
                let (user_id, group_id) =
                    match super::ActorId::$recipient_variant(self.$recipient_field) {
                        super::ActorId::User(user_id) => (Some(user_id), None),
                        super::ActorId::Group(group_id) => (None, Some(group_id)),
                    };

                super::ResourcePermission {
                    id: self.id,
                    user_id,
                    group_id,
                    resource_id: ($resource_id)(&self),
                    resource_type: $resource_type.to_string(),
                    role_name: self.role_name,
                    granted_by: self.granted_by,
                    grant_reason: self.grant_reason,
                    granted_at: self.granted_at,
                }
            }
        }
    };
}

pub(super) use impl_permission_assignment;

// ------------------ //
// * RESOURCE TYPES * //
// ------------------ //
//
// - A resource type is a string that identifies a category of resources in the system.
// - A resource ID is a UUID that uniquely identifies a specific resource within its category.
//

// -- TRAITS -- //

/// A trait representing an action that can be performed on a resource.
pub trait PermissionAction:
    Copy + Eq + AsRef<str> + std::fmt::Debug + Send + Sync + 'static
{
    const RESOURCE_TYPE: ResourceType;
    type Role: PermissionRole<Action = Self> + std::str::FromStr;
}

/// A trait representing a role that can be assigned permissions for a specific action on a resource.
pub trait PermissionRole: Copy + AsRef<str> + Into<&'static str> + IntoEnumIterator {
    type Action: PermissionAction;

    fn actions(self) -> &'static [Self::Action];

    fn allows(self, action: Self::Action) -> bool {
        self.actions().contains(&action)
    }

    fn triplet(self, resource_id: &Uuid) -> Result<PermissionTriplet<'_>, ComhairleError> {
        validate_permission_resource(Self::Action::RESOURCE_TYPE.as_ref(), resource_id)?;

        Ok(PermissionTriplet(
            Self::Action::RESOURCE_TYPE.into(),
            resource_id,
            self.into(),
        ))
    }

    fn system_triplet(self) -> Result<PermissionTriplet<'static>, ComhairleError> {
        if Self::Action::RESOURCE_TYPE != ResourceType::System {
            return Err(ComhairleError::BadRequest(
                "Cannot create a System triplet for a non-System role".into(),
            ));
        }

        self.triplet(&SYSTEM_RESOURCE_ID)
    }
}

/// A trait representing an assignment of a permission to a recipient for a specific resource type.
pub trait PermissionAssignment:
    for<'row> sqlx::FromRow<'row, sqlx::postgres::PgRow> + Send + Unpin
{
    const RESOURCE_TYPE: ResourceType;

    fn table() -> sea_query::DynIden;
    fn recipient_column() -> sea_query::DynIden;
    fn into_resource_permission(self) -> ResourcePermission;
}

pub trait PermissionResource:
    FromRequestParts<Arc<ComhairleState>, Rejection = ComhairleError> + 'static + Send + Sync
{
    type Action: PermissionAction;

    fn resource_id(&self) -> Uuid;

    fn owner_id(&self) -> Option<Uuid> {
        None
    }
}

// -- ENUM -- //

/// The set of resource categories that permissions can be granted on.
///
/// The string form (via [`AsRefStr`] / [`Display`]) is what is persisted in the
/// resource-specific table selection, so those values are load bearing.
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
}

// -- SYSTEM RESOURCE -- //

/// Shared API and cache metadata identify the global System with the nil UUID.
/// System permission tables do not store a resource ID.
pub const SYSTEM_RESOURCE_ID: Uuid = Uuid::nil();

#[derive(Debug, OperationIo)]
pub struct SystemResource {
    pub resource_id: Uuid,
}

impl PermissionResource for SystemResource {
    type Action = system::Action;

    fn resource_id(&self) -> Uuid {
        self.resource_id
    }
}

impl FromRequestParts<Arc<ComhairleState>> for SystemResource {
    type Rejection = ComhairleError;

    async fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self {
            resource_id: SYSTEM_RESOURCE_ID,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct PermissionTargetPath {
    pub resource_type: String,
    pub resource_id: Uuid,
}

#[derive(Debug, OperationIo)]
pub struct PermissionTargetResource {
    pub resource_type: String,
    pub resource_id: Uuid,
    pub owner_id: Option<Uuid>,
}

impl FromRequestParts<Arc<ComhairleState>> for PermissionTargetResource {
    type Rejection = ComhairleError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<ComhairleState>,
    ) -> Result<Self, Self::Rejection> {
        let Path(PermissionTargetPath {
            resource_type,
            resource_id,
        }) = Path::<PermissionTargetPath>::from_request_parts(parts, state)
            .await
            .map_err(|_| {
                ComhairleError::ResourceNotFound(
                    "Path must contain resource_type and resource_id".to_string(),
                )
            })?;

        let owner_id = if resource_type == ResourceType::Conversation.as_ref() {
            models::conversation::get_by_id(&state.db, &resource_id)
                .await
                .ok()
                .map(|conversation| conversation.owner_id)
        } else {
            None
        };

        Ok(PermissionTargetResource {
            resource_type,
            resource_id,
            owner_id,
        })
    }
}

// -- CONVERSATION RESOURCE -- //

/// A struct representing the path parameters for a conversation resource.
#[derive(Deserialize)]
pub struct ConversationPath {
    pub conversation_id: Uuid,
}

#[derive(Deserialize)]
struct ConversationResourcePath {
    conversation_id: models::conversation::IdOrSlug,
}

#[derive(Debug, OperationIo)]
pub struct ConversationResource {
    pub conversation_id: Uuid,
    pub owner_id: Uuid,
}

impl FromRequestParts<Arc<ComhairleState>> for ConversationResource {
    type Rejection = ComhairleError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<ComhairleState>,
    ) -> Result<Self, Self::Rejection> {
        let Path(ConversationResourcePath { conversation_id }) =
            Path::<ConversationResourcePath>::from_request_parts(parts, state)
                .await
                .map_err(|_| {
                    ComhairleError::ResourceNotFound(
                        "Path must contain a conversation_id".to_string(),
                    )
                })?;

        let conversation =
            models::conversation::get_by_id_or_slug(&state.db, &conversation_id).await?;

        Ok(ConversationResource {
            conversation_id: conversation.id,
            owner_id: conversation.owner_id,
        })
    }
}

impl PermissionResource for ConversationResource {
    type Action = conversation::Action;

    fn resource_id(&self) -> Uuid {
        self.conversation_id
    }

    fn owner_id(&self) -> Option<Uuid> {
        Some(self.owner_id)
    }
}

// ----------------------- //
// * PERMISSION HANDLING * //
// ----------------------- //
//
// - Permission handling involves granting, revoking, and checking permissions for Users and User groups.
//

/// The triplet associated with a permission for a resource.
#[derive(Debug)]
pub struct PermissionTriplet<'a>(
    pub &'a str,  // resource_type
    pub &'a Uuid, // resource_id
    pub &'a str,  // role_name
);

/// Read and API projection of a resource-specific User or UserGroup assignment.
#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
pub struct ResourcePermission {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub group_id: Option<Uuid>,
    pub resource_id: Uuid,
    pub resource_type: String,
    pub role_name: String,
    pub granted_by: Option<Uuid>,
    pub grant_reason: String,
    pub granted_at: DateTime<Utc>,
}

#[derive(sea_query::Iden)]
pub enum ResourcePermissionIden {
    Id,
    UserId,
    GroupId,
    ResourceId,
    ResourceType,
    RoleName,
    GrantedBy,
    GrantReason,
    GrantedAt,
}

fn table_identifiers<Model: PermissionAssignment>() -> (DynIden, DynIden) {
    (Model::table(), Model::recipient_column())
}

pub fn permission_table(
    resource_type: &str,
    actor: ActorId,
) -> Result<(DynIden, DynIden), ComhairleError> {
    match (resource_type, actor) {
        ("conversation", ActorId::User(_)) => {
            Ok(table_identifiers::<conversation::UserPermission>())
        }
        ("conversation", ActorId::Group(_)) => {
            Ok(table_identifiers::<conversation::GroupPermission>())
        }
        ("organization", ActorId::User(_)) => {
            Ok(table_identifiers::<organization::UserPermission>())
        }
        ("organization", ActorId::Group(_)) => {
            Ok(table_identifiers::<organization::GroupPermission>())
        }
        ("system", ActorId::User(_)) => Ok(table_identifiers::<system::UserPermission>()),
        ("system", ActorId::Group(_)) => Ok(table_identifiers::<system::GroupPermission>()),
        _ => Err(ComhairleError::BadRequest(format!(
            "Unknown permission resource type: {resource_type}"
        ))),
    }
}

pub fn permission_select(resource_type: Option<&str>) -> Result<SelectStatement, ComhairleError> {
    let mut combined: Option<SelectStatement> = None;
    for resource in ["conversation", "organization", "system"] {
        if resource_type.is_some_and(|requested| requested != resource) {
            continue;
        }
        for actor in [ActorId::User(Uuid::nil()), ActorId::Group(Uuid::nil())] {
            let (table, recipient_column) = permission_table(resource, actor)?;
            let mut projection = Query::select();
            projection.from(table).column(ResourcePermissionIden::Id);
            if resource == "system" {
                projection.expr_as(
                    Expr::val(SYSTEM_RESOURCE_ID),
                    ResourcePermissionIden::ResourceId,
                );
            } else {
                projection.column(ResourcePermissionIden::ResourceId);
            }
            projection
                .columns([
                    ResourcePermissionIden::RoleName,
                    ResourcePermissionIden::GrantedBy,
                    ResourcePermissionIden::GrantReason,
                    ResourcePermissionIden::GrantedAt,
                ])
                .expr_as(Expr::val(resource), ResourcePermissionIden::ResourceType);
            match actor {
                ActorId::User(_) => {
                    projection
                        .expr_as(Expr::col(recipient_column), ResourcePermissionIden::UserId)
                        .expr_as(Expr::cust("NULL::uuid"), ResourcePermissionIden::GroupId);
                }
                ActorId::Group(_) => {
                    projection
                        .expr_as(Expr::cust("NULL::uuid"), ResourcePermissionIden::UserId)
                        .expr_as(Expr::col(recipient_column), ResourcePermissionIden::GroupId);
                }
            }
            match &mut combined {
                Some(combined) => {
                    combined.union(UnionType::All, projection.to_owned());
                }
                None => combined = Some(projection),
            }
        }
    }
    let combined = combined
        .ok_or_else(|| ComhairleError::BadRequest("Unknown permission resource type".into()))?;
    Ok(Query::select()
        .columns(DEFAULT_COLUMNS)
        .from_subquery(combined, Alias::new("permission_assignments"))
        .to_owned())
}

const DEFAULT_COLUMNS: [ResourcePermissionIden; 9] = [
    ResourcePermissionIden::Id,
    ResourcePermissionIden::UserId,
    ResourcePermissionIden::GroupId,
    ResourcePermissionIden::ResourceId,
    ResourcePermissionIden::ResourceType,
    ResourcePermissionIden::RoleName,
    ResourcePermissionIden::GrantedBy,
    ResourcePermissionIden::GrantReason,
    ResourcePermissionIden::GrantedAt,
];

/// Represents either a user or an organization for role assignment purposes.
#[derive(Debug, Copy, Clone)]
pub enum ActorId {
    User(Uuid),
    Group(Uuid),
}

/// Request struct for granting a role to a user or organization on a resource.
#[derive(Debug)]
pub struct GrantRoleRequest<'request> {
    pub actor_id: ActorId,
    pub granted_by: &'request Uuid,
    pub grant_reason: &'request str,
    pub permission_triplet: PermissionTriplet<'request>,
}

/// Request struct for revoking a role from a user or organization on a resource.
#[derive(Debug)]
pub struct RevokeRoleRequest<'request> {
    pub actor_id: ActorId,
    pub permission_triplet: PermissionTriplet<'request>,
}

/// Generates a cache key for storing all assigned role names for an actor on a resource.
fn role_list_cache_key(resource_type: &str, resource_id: &Uuid, actor_id: &ActorId) -> String {
    match *actor_id {
        ActorId::User(user_id) => {
            format!("roles:v1:{resource_type}:{resource_id}:user:{user_id}")
        }
        ActorId::Group(group_id) => {
            format!("roles:v1:{resource_type}:{resource_id}:group:{group_id}")
        }
    }
}

/// Deletes a cache key from Redis.
async fn cache_delete(conn: &dyn RedisConnection, key: &str) {
    let _ = conn.del(key).await;
}

/// Reads a cached role list for an actor on a resource.
#[derive(Debug, Serialize, Deserialize)]
struct CachedRoles {
    version: i64,
    roles: Vec<String>,
}

async fn cache_get_role_list(conn: &dyn RedisConnection, key: &str) -> Option<CachedRoles> {
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
    version: i64,
) {
    let serialized = match serde_json::to_string(&CachedRoles {
        version,
        roles: roles.to_vec(),
    }) {
        Ok(value) => value,
        Err(_) => return,
    };
    let _ = conn
        .set_ex(key, &serialized, models::user_group::CACHE_TTL_SECONDS)
        .await;
}

/// Loads all role names assigned to a specific actor on a specific resource from Postgres.
fn validate_permission_resource(
    resource_type: &str,
    resource_id: &Uuid,
) -> Result<(), ComhairleError> {
    if resource_type == "system" && *resource_id != SYSTEM_RESOURCE_ID {
        return Err(ComhairleError::BadRequest(
            "System permissions are global; use the nil UUID".into(),
        ));
    }
    Ok(())
}

#[instrument(err(Debug), skip(db))]
async fn fetch_actor_roles_for_resource<'connection>(
    db: impl sqlx::Executor<'connection, Database = sqlx::Postgres>,
    resource_type: &str,
    resource_id: &Uuid,
    actor_id: ActorId,
) -> Result<Vec<String>, ComhairleError> {
    validate_permission_resource(resource_type, resource_id)?;
    let (table, recipient_column) = permission_table(resource_type, actor_id)?;
    let recipient_id = match actor_id {
        ActorId::User(user_id) => user_id,
        ActorId::Group(group_id) => group_id,
    };
    let mut query = Query::select();
    query
        .column(ResourcePermissionIden::RoleName)
        .from(table)
        .and_where(Expr::col(recipient_column).eq(recipient_id));
    if resource_type != "system" {
        query.and_where(Expr::col(ResourcePermissionIden::ResourceId).eq(*resource_id));
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
    resource_type: &str,
    resource_id: &Uuid,
    actor_id: ActorId,
) -> Result<Vec<String>, ComhairleError> {
    let cache_key = role_list_cache_key(resource_type, resource_id, &actor_id);
    let version = actor_permission_version(&state.db, resource_type, resource_id, actor_id).await?;

    if let Some(conn) = &state.redis_conn {
        if let Some(cached_roles) = cache_get_role_list(conn.as_ref(), &cache_key).await
            && cached_roles.version == version
        {
            return Ok(cached_roles.roles);
        }
    }

    let mut transaction = state.db.begin().await?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *transaction)
        .await?;
    let version =
        actor_permission_version(&mut *transaction, resource_type, resource_id, actor_id).await?;
    let roles =
        fetch_actor_roles_for_resource(&mut *transaction, resource_type, resource_id, actor_id)
            .await?;
    transaction.commit().await?;

    if let Some(conn) = &state.redis_conn {
        cache_set_role_list(conn.as_ref(), &cache_key, &roles, version).await;
    }

    Ok(roles)
}

pub async fn actor_permission_version<'connection>(
    db: impl sqlx::Executor<'connection, Database = sqlx::Postgres>,
    resource_type: &str,
    resource_id: &Uuid,
    actor_id: ActorId,
) -> Result<i64, ComhairleError> {
    let (recipient_type, recipient_id) = match actor_id {
        ActorId::User(user_id) => ("user", user_id),
        ActorId::Group(group_id) => ("group", group_id),
    };
    Ok(sqlx::query_scalar(
        "SELECT coalesce((SELECT version FROM permission_set_version WHERE resource_type = $1
         AND resource_id = $2 AND recipient_type = $3 AND recipient_id = $4), 0)",
    )
    .bind(resource_type)
    .bind(resource_id)
    .bind(recipient_type)
    .bind(recipient_id)
    .fetch_one(db)
    .await?)
}

pub async fn invalidate_actor_roles(
    state: &Arc<ComhairleState>,
    resource_type: &str,
    resource_id: &Uuid,
    actor_id: ActorId,
) -> Result<(), ComhairleError> {
    if let Some(connection) = &state.redis_conn {
        cache_delete(
            connection.as_ref(),
            &role_list_cache_key(resource_type, resource_id, &actor_id),
        )
        .await;
    }
    Ok(())
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
    request: GrantRoleRequest<'_>,
) -> Result<ResourcePermission, ComhairleError> {
    validate_permission_resource(request.permission_triplet.0, request.permission_triplet.1)?;
    match (request.permission_triplet.0, request.actor_id) {
        ("conversation", ActorId::User(_)) => {
            insert_assignment::<conversation::UserPermission>(state, request).await
        }
        ("conversation", ActorId::Group(_)) => {
            insert_assignment::<conversation::GroupPermission>(state, request).await
        }
        ("organization", ActorId::User(_)) => {
            insert_assignment::<organization::UserPermission>(state, request).await
        }
        ("organization", ActorId::Group(_)) => {
            insert_assignment::<organization::GroupPermission>(state, request).await
        }
        ("system", ActorId::User(_)) => {
            insert_assignment::<system::UserPermission>(state, request).await
        }
        ("system", ActorId::Group(_)) => {
            insert_assignment::<system::GroupPermission>(state, request).await
        }
        _ => Err(ComhairleError::BadRequest(
            "Unknown permission resource type".into(),
        )),
    }
}

async fn insert_assignment<Model: PermissionAssignment>(
    state: &Arc<ComhairleState>,
    request: GrantRoleRequest<'_>,
) -> Result<ResourcePermission, ComhairleError> {
    let recipient_id = match request.actor_id {
        ActorId::User(user_id) => user_id,
        ActorId::Group(group_id) => group_id,
    };

    let PermissionTriplet(resource_type, resource_id, role_name) = request.permission_triplet;

    let mut columns = vec![Model::recipient_column()];
    let mut values = vec![recipient_id.into()];
    if Model::RESOURCE_TYPE != ResourceType::System {
        columns.push(ResourcePermissionIden::ResourceId.into_iden());
        values.push((*resource_id).into());
    }
    columns.extend([
        ResourcePermissionIden::RoleName.into_iden(),
        ResourcePermissionIden::GrantedBy.into_iden(),
        ResourcePermissionIden::GrantReason.into_iden(),
    ]);
    values.extend([
        role_name.into(),
        (*request.granted_by).into(),
        request.grant_reason.to_owned().into(),
    ]);
    let mut returning = columns.clone();
    returning.insert(0, ResourcePermissionIden::Id.into_iden());
    returning.push(ResourcePermissionIden::GrantedAt.into_iden());
    let mut query = Query::insert();
    query
        .into_table(Model::table())
        .columns(columns)
        .values_panic(values)
        .on_conflict(OnConflict::new().do_nothing().to_owned())
        .returning(Query::returning().columns(returning));

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let response = sqlx::query_as_with::<_, Model, _>(&sql, values)
        .fetch_optional(&state.db)
        .await
        .map_err(ComhairleError::DatabaseError)?;

    let permission =
        response.ok_or_else(|| ComhairleError::RoleAlreadyGranted(role_name.to_string()))?;

    invalidate_actor_roles(state, resource_type, resource_id, request.actor_id).await?;

    Ok(permission.into_resource_permission())
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
    request: RevokeRoleRequest<'_>,
) -> Result<(), ComhairleError> {
    let PermissionTriplet(resource_type, resource_id, role_name) = request.permission_triplet;

    validate_permission_resource(resource_type, resource_id)?;
    let (table, recipient_column) = permission_table(resource_type, request.actor_id)?;
    let recipient_id = match request.actor_id {
        ActorId::User(user_id) => user_id,
        ActorId::Group(group_id) => group_id,
    };
    let mut tx = state
        .db
        .begin()
        .await
        .map_err(ComhairleError::DatabaseError)?;

    let mut query = Query::delete();
    query
        .from_table(table)
        .and_where(Expr::col(ResourcePermissionIden::RoleName).eq(role_name))
        .and_where(Expr::col(recipient_column).eq(recipient_id));
    if resource_type != "system" {
        query.and_where(Expr::col(ResourcePermissionIden::ResourceId).eq(*resource_id));
    }

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let response = sqlx::query_with(&sql, values)
        .execute(&mut *tx)
        .await
        .map_err(ComhairleError::DatabaseError)?;

    if response.rows_affected() == 0 {
        return Err(ComhairleError::RoleNotFound(role_name.to_string()));
    }

    tx.commit().await.map_err(permission_database_error)?;

    invalidate_actor_roles(state, resource_type, resource_id, request.actor_id).await?;

    Ok(())
}

/// Filters for listing permissions, allowing optional filtering and pagination
/// via `page_options`.
#[derive(Debug, Default)]
pub struct ListPermissionsFilters<'request> {
    pub resource_type: Option<&'request str>,
    pub resource_id: Option<&'request Uuid>,
    pub actor: Option<ActorId>,
    pub role_name: Option<&'request str>,
    pub page_options: PageOptions,
}

pub fn permission_database_error(error: sqlx::Error) -> ComhairleError {
    match error
        .as_database_error()
        .and_then(|error| error.constraint())
    {
        Some("platform_superadmin_continuity") => ComhairleError::CannotRevokeLastSuperAdmin,
        Some("organization_admin_continuity") => {
            ComhairleError::Conflict("The Organization must retain an administrator".into())
        }
        _ => ComhairleError::DatabaseError(error),
    }
}

pub async fn lock_permission_mutation(
    connection: &mut sqlx::PgConnection,
) -> Result<(), ComhairleError> {
    sqlx::query("UPDATE permission_mutation_guard SET generation = generation + 1 WHERE singleton")
        .execute(connection)
        .await?;
    Ok(())
}

pub async fn authorize_transaction<Action: PermissionAction>(
    connection: &mut sqlx::PgConnection,
    resource_id: Uuid,
    action: Action,
    caller_id: Uuid,
) -> Result<(), ComhairleError> {
    let superadmin_roles =
        transaction_roles(&mut *connection, "system", SYSTEM_RESOURCE_ID, caller_id).await?;

    if superadmin_roles.iter().any(|role| role == "super_admin") {
        return Ok(());
    }

    if Action::RESOURCE_TYPE == ResourceType::Conversation {
        let owner_id: Option<Uuid> =
            sqlx::query_scalar("SELECT owner_id FROM conversation WHERE id = $1")
                .bind(resource_id)
                .fetch_optional(&mut *connection)
                .await?;
        if owner_id == Some(caller_id) {
            return Ok(());
        }
    }

    let roles = transaction_roles(
        connection,
        Action::RESOURCE_TYPE.as_ref(),
        resource_id,
        caller_id,
    )
    .await?;

    if roles.iter().any(|role| {
        role.parse::<Action::Role>()
            .is_ok_and(|role| role.allows(action))
    }) {
        Ok(())
    } else {
        Err(ComhairleError::UserNotAuthorized)
    }
}

async fn transaction_roles(
    connection: &mut sqlx::PgConnection,
    resource_type: &str,
    resource_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<String>, ComhairleError> {
    let query = permission_select(Some(resource_type))?
        .and_where(Expr::col(ResourcePermissionIden::ResourceId).eq(resource_id))
        .and_where(Expr::cust_with_values("(user_id = $1 OR group_id IN (SELECT group_id FROM user_group_member WHERE user_id = $1))", [user_id]))
        .to_owned();

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    Ok(
        sqlx::query_as_with::<_, ResourcePermission, _>(&sql, values)
            .fetch_all(connection)
            .await?
            .into_iter()
            .map(|permission| permission.role_name)
            .collect(),
    )
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
    request: ListPermissionsFilters<'_>,
) -> Result<PaginatedResults<ResourcePermission>, ComhairleError> {
    let mut query = permission_select(request.resource_type)?;

    if let Some(resource_type) = request.resource_type {
        query.and_where(Expr::col(ResourcePermissionIden::ResourceType).eq(resource_type));
    }
    if let Some(resource_id) = request.resource_id {
        query.and_where(Expr::col(ResourcePermissionIden::ResourceId).eq(*resource_id));
    }

    match request.actor {
        Some(ActorId::User(user_id)) => {
            query.and_where(Expr::col(ResourcePermissionIden::UserId).eq(user_id));
        }
        Some(ActorId::Group(group_id)) => {
            query.and_where(Expr::col(ResourcePermissionIden::GroupId).eq(group_id));
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
    permission_triplet: PermissionTriplet<'_>,
    user_id: &Uuid,
) -> Result<bool, ComhairleError> {
    let PermissionTriplet(resource_type, resource_id, role_name) = permission_triplet;

    let user_roles =
        get_actor_roles_for_resource(state, resource_type, resource_id, ActorId::User(*user_id))
            .await?;
    if user_roles
        .iter()
        .any(|cached_role| cached_role == role_name)
    {
        return Ok(true);
    }
    for group_id in models::user_group::memberships(state, *user_id).await? {
        let roles = get_actor_roles_for_resource(
            state,
            resource_type,
            resource_id,
            ActorId::Group(group_id),
        )
        .await?;
        if roles.iter().any(|cached_role| cached_role == role_name) {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Check whether a user, or their organization, can perform an action on a resource.
#[instrument(err(Debug), skip(state))]
pub async fn can_perform_action<Action: PermissionAction>(
    state: &Arc<ComhairleState>,
    resource_id: &Uuid,
    action: Action,
    user_id: &Uuid,
    owner_id: Option<&Uuid>,
) -> Result<bool, ComhairleError> {
    if owner_id.is_some_and(|resource_owner_id| resource_owner_id == user_id) {
        return Ok(true);
    }

    // Bypass permission checks for super admins
    if has_resource_permission(state, system::Role::SuperAdmin.system_triplet()?, user_id).await? {
        return Ok(true);
    }

    let resource_type = Action::RESOURCE_TYPE;

    let mut roles = get_actor_roles_for_resource(
        state,
        resource_type.as_ref(),
        resource_id,
        ActorId::User(*user_id),
    )
    .await?;

    for group_id in models::user_group::memberships(state, *user_id).await? {
        let org_roles = get_actor_roles_for_resource(
            state,
            resource_type.as_ref(),
            resource_id,
            ActorId::Group(group_id),
        )
        .await?;
        roles.extend(org_roles);
    }

    roles.dedup();

    Ok(roles.iter().any(|role_name| {
        role_name
            .parse::<Action::Role>()
            .is_ok_and(|role| role.allows(action))
    }))
}

pub async fn can_perform_target_action(
    state: &Arc<ComhairleState>,
    target: &PermissionTargetResource,
    action: &str,
    user_id: &Uuid,
) -> Result<bool, ComhairleError> {
    let resource_type = target
        .resource_type
        .parse::<ResourceType>()
        .map_err(|_| ComhairleError::BadRequest("Unknown permission resource type".into()))?;

    match resource_type {
        ResourceType::Conversation => {
            can_perform_action(
                state,
                &target.resource_id,
                action
                    .parse::<conversation::Action>()
                    .map_err(|_| ComhairleError::UserNotAuthorized)?,
                user_id,
                target.owner_id.as_ref(),
            )
            .await
        }
        ResourceType::Organization => {
            can_perform_action(
                state,
                &target.resource_id,
                action
                    .parse::<organization::Action>()
                    .map_err(|_| ComhairleError::UserNotAuthorized)?,
                user_id,
                target.owner_id.as_ref(),
            )
            .await
        }
        ResourceType::System => {
            can_perform_action(
                state,
                &target.resource_id,
                action
                    .parse::<system::Action>()
                    .map_err(|_| ComhairleError::UserNotAuthorized)?,
                user_id,
                target.owner_id.as_ref(),
            )
            .await
        }
    }
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
) -> Result<Vec<UserWithPermissionDto>, ComhairleError> {
    validate_permission_resource(resource_type, &resource_id)?;
    let (table, _) = permission_table(resource_type, ActorId::User(Uuid::nil()))?;
    let mut query = Query::select()
        .from(table.clone())
        .join(
            JoinType::InnerJoin,
            UserIden::Table,
            Expr::col((UserIden::Table, UserIden::Id))
                .equals((table.clone(), ResourcePermissionIden::UserId)),
        )
        .columns([
            (UserIden::Table, UserIden::Id),
            (UserIden::Table, UserIden::Username),
            (UserIden::Table, UserIden::Email),
        ])
        .column((table.clone(), ResourcePermissionIden::RoleName))
        .to_owned();
    if resource_type != "system" {
        query.and_where(
            Expr::col((table.clone(), ResourcePermissionIden::ResourceId))
                .eq(resource_id.to_owned()),
        );
    }

    if let Some(role_name) = role_name {
        query = query
            .and_where(
                Expr::col((table.clone(), ResourcePermissionIden::RoleName))
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
) -> Result<Vec<OrganizationWithPermissionDto>, ComhairleError> {
    validate_permission_resource(resource_type, &resource_id)?;
    let (table, _) = permission_table(resource_type, ActorId::Group(Uuid::nil()))?;
    let mut query = Query::select()
        .from(table.clone())
        .join(
            JoinType::InnerJoin,
            OrganizationIden::Table,
            Expr::col((OrganizationIden::Table, OrganizationIden::UserGroupId))
                .equals((table.clone(), ResourcePermissionIden::GroupId)),
        )
        .columns([
            (OrganizationIden::Table, OrganizationIden::Id),
            (OrganizationIden::Table, OrganizationIden::Name),
        ])
        .column((table.clone(), ResourcePermissionIden::RoleName))
        .to_owned();
    if resource_type != "system" {
        query.and_where(
            Expr::col((table.clone(), ResourcePermissionIden::ResourceId))
                .eq(resource_id.to_owned()),
        );
    }

    if let Some(role_name) = role_name {
        query = query
            .and_where(
                Expr::col((table.clone(), ResourcePermissionIden::RoleName))
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
    action: &str,
) -> Result<Vec<ResourcePermission>, ComhairleError> {
    let (resource_type, roles) = if let Ok(action) = action.parse::<system::Action>() {
        (ResourceType::System, roles_for_action(action))
    } else if let Ok(action) = action.parse::<conversation::Action>() {
        (ResourceType::Conversation, roles_for_action(action))
    } else if let Ok(action) = action.parse::<organization::Action>() {
        (ResourceType::Organization, roles_for_action(action))
    } else {
        return Err(ComhairleError::BadRequest(format!(
            "Invalid action: {action}"
        )));
    };

    let mut query = permission_select(Some(resource_type.as_ref()))?;

    query
        .and_where(Expr::col(ResourcePermissionIden::RoleName).is_in(roles))
        .and_where(Expr::cust_with_values("(user_id = $1 OR group_id IN (SELECT group_id FROM user_group_member WHERE user_id = $1))", [user_id]));

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let permissions = query_as_with(&sql, values)
        .fetch_all(db)
        .await
        .map_err(ComhairleError::DatabaseError)?;

    Ok(permissions)
}

pub fn roles_for_action<A: PermissionAction>(action: A) -> Vec<String> {
    A::Role::iter()
        .filter(|role| role.allows(action))
        .map(|role| role.as_ref().to_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    async fn create_legacy_permission_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
        for migration in crate::SQLX_MIGRATOR
            .iter()
            .filter(|migration| migration.version < 20261001120000)
        {
            sqlx::raw_sql(&migration.sql).execute(pool).await?;
        }
        Ok(())
    }

    #[sqlx::test(migrations = false)]
    async fn legacy_migration_preserves_all_grants_and_audit_fields(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        create_legacy_permission_schema(&pool).await?;
        let user_id: Uuid = sqlx::query_scalar(
            "INSERT INTO comhairle_user (auth_type) VALUES ('guest') RETURNING id",
        )
        .fetch_one(&pool)
        .await?;
        let description = models::translations::new_translation(
            &pool,
            "en",
            "Legacy Organization",
            models::translations::TextFormat::Plain,
        )
        .await?;
        let organization_id: Uuid = sqlx::query_scalar("INSERT INTO organization (name, description, mission, org_type) VALUES ('Legacy Organization', $1, $1, 'other') RETURNING id")
            .bind(description.id).fetch_one(&pool).await?;
        sqlx::query("UPDATE comhairle_user SET organization_id = $1 WHERE id = $2")
            .bind(organization_id)
            .bind(user_id)
            .execute(&pool)
            .await?;
        let granted_at: DateTime<Utc> = "2026-09-30T12:00:00Z".parse()?;
        let mut expected = Vec::new();
        for (resource_type, resource_id, role_name) in [
            ("conversation", Uuid::new_v4(), "content_editor"),
            ("organization", organization_id, "organization_admin"),
            ("system", SYSTEM_RESOURCE_ID, "super_admin"),
        ] {
            for actor in [ActorId::User(user_id), ActorId::Group(organization_id)] {
                let grant_id = Uuid::new_v4();
                let (recipient_user, recipient_organization) = match actor {
                    ActorId::User(user_id) => (Some(user_id), None),
                    ActorId::Group(group_id) => (None, Some(group_id)),
                };
                sqlx::query("INSERT INTO resource_permissions (id, user_id, organization_id, resource_id, resource_type, role_name, granted_by, grant_reason, granted_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, 'Original grant reason', $8)")
                    .bind(grant_id).bind(recipient_user).bind(recipient_organization).bind(resource_id)
                    .bind(resource_type).bind(role_name).bind(user_id).bind(granted_at).execute(&pool).await?;
                expected.push((actor, resource_type, resource_id, role_name, grant_id));
            }
        }
        for migration in crate::SQLX_MIGRATOR
            .iter()
            .filter(|migration| migration.version >= 20261001120000)
        {
            sqlx::raw_sql(&migration.sql).execute(&pool).await?;
        }
        for (actor, resource_type, resource_id, role_name, grant_id) in expected {
            let (table, recipient_column) = permission_table(resource_type, actor)?;
            let mut query = Query::select();
            query.columns([ResourcePermissionIden::Id.into_iden(), recipient_column]);
            if resource_type == "system" {
                query.expr_as(
                    Expr::val(SYSTEM_RESOURCE_ID),
                    ResourcePermissionIden::ResourceId,
                );
            } else {
                query.column(ResourcePermissionIden::ResourceId);
            }
            let (sql, values) = query
                .columns([
                    ResourcePermissionIden::RoleName.into_iden(),
                    ResourcePermissionIden::GrantedBy.into_iden(),
                    ResourcePermissionIden::GrantReason.into_iden(),
                    ResourcePermissionIden::GrantedAt.into_iden(),
                ])
                .from(table)
                .and_where(Expr::col(ResourcePermissionIden::Id).eq(grant_id))
                .build_sqlx(PostgresQueryBuilder);
            let stored: (
                Uuid,
                Uuid,
                Uuid,
                String,
                Option<Uuid>,
                String,
                DateTime<Utc>,
            ) = sqlx::query_as_with(&sql, values).fetch_one(&pool).await?;
            let recipient_id = match actor {
                ActorId::User(user_id) => user_id,
                ActorId::Group(group_id) => group_id,
            };
            assert_eq!(
                stored,
                (
                    grant_id,
                    recipient_id,
                    resource_id,
                    role_name.into(),
                    Some(user_id),
                    "Original grant reason".into(),
                    granted_at
                )
            );
        }
        let membership: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM user_group_member WHERE user_id = $1 AND group_id = $2)",
        )
        .bind(user_id)
        .bind(organization_id)
        .fetch_one(&pool)
        .await?;
        assert!(membership);
        let unified_absent: bool =
            sqlx::query_scalar("SELECT to_regclass('resource_permissions') IS NULL")
                .fetch_one(&pool)
                .await?;
        assert!(unified_absent);
        Ok(())
    }

    #[sqlx::test(migrations = false)]
    async fn legacy_migration_rejects_unknown_types_without_losing_grants(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        create_legacy_permission_schema(&pool).await?;
        let user_id: Uuid = sqlx::query_scalar(
            "INSERT INTO comhairle_user (auth_type) VALUES ('guest') RETURNING id",
        )
        .fetch_one(&pool)
        .await?;
        let grant_id: Uuid = sqlx::query_scalar("INSERT INTO resource_permissions (user_id, resource_id, resource_type, role_name, grant_reason)
            VALUES ($1, $2, 'unsupported', 'legacy_role', 'Preserve me') RETURNING id")
            .bind(user_id).bind(Uuid::new_v4()).fetch_one(&pool).await?;
        let migration = crate::SQLX_MIGRATOR
            .iter()
            .find(|migration| migration.version == 20261001120000)
            .unwrap();
        let mut transaction = pool.begin().await?;
        let result = sqlx::raw_sql(&migration.sql)
            .execute(&mut *transaction)
            .await;
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Unsupported legacy permission resource types")
        );
        transaction.rollback().await?;
        let preserved: Uuid =
            sqlx::query_scalar("SELECT id FROM resource_permissions WHERE id = $1")
                .bind(grant_id)
                .fetch_one(&pool)
                .await?;
        assert_eq!(preserved, grant_id);
        Ok(())
    }
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn six_resource_tables_replace_unified_storage(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let grantor = session.id.unwrap();
        let organization_id = get_random_organization_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;
        let group_id = models::user_group::organization_group(&pool, organization_id).await?;
        let state = Arc::new(test_state().db(pool).call()?);
        let unified_absent: bool = sqlx::query_scalar("SELECT to_regclass('resource_permissions') IS NULL AND to_regclass('unclassified_resource_permissions') IS NULL")
            .fetch_one(&state.db).await?;
        assert!(unified_absent);
        for (resource_type, resource_id, role) in [
            ("conversation", Uuid::new_v4(), "content_editor"),
            ("organization", organization_id, "organization_admin"),
            ("system", SYSTEM_RESOURCE_ID, "admin"),
        ] {
            for actor in [ActorId::User(user_id), ActorId::Group(group_id)] {
                let assignment = grant_role(
                    &state,
                    GrantRoleRequest {
                        actor_id: actor,
                        granted_by: &grantor,
                        grant_reason: "Split table round trip",
                        permission_triplet: PermissionTriplet(resource_type, &resource_id, role),
                    },
                )
                .await?;
                assert_eq!(assignment.resource_type, resource_type);
                assert_eq!(assignment.resource_id, resource_id);
                assert_eq!(assignment.role_name, role);
                assert_eq!(assignment.granted_by, Some(grantor));
                assert_eq!(assignment.grant_reason, "Split table round trip");
                let (table, _) = permission_table(resource_type, actor)?;
                let (sql, values) = Query::select()
                    .column(ResourcePermissionIden::Id)
                    .from(table)
                    .and_where(Expr::col(ResourcePermissionIden::Id).eq(assignment.id))
                    .build_sqlx(PostgresQueryBuilder);
                let stored: Uuid = sqlx::query_scalar_with(&sql, values)
                    .fetch_one(&state.db)
                    .await?;
                assert_eq!(stored, assignment.id);
            }
        }
        Ok(())
    }
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn database_prevents_removing_last_effective_group_superadmin(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let user_id = session.id.unwrap();
        let organization_id = get_random_organization_id(&app, &mut session).await?;
        let group_id = models::user_group::organization_group(&pool, organization_id).await?;
        sqlx::query(
            "INSERT INTO system_group_permissions (group_id, role_name, grant_reason)
                         VALUES ($1, 'super_admin', 'Continuity test')",
        )
        .bind(group_id)
        .execute(&pool)
        .await?;
        sqlx::query(
            "DELETE FROM system_user_permissions WHERE user_id = $1 AND role_name = 'super_admin'",
        )
        .bind(user_id)
        .execute(&pool)
        .await?;
        let result =
            sqlx::query("DELETE FROM user_group_member WHERE group_id = $1 AND user_id = $2")
                .bind(group_id)
                .bind(user_id)
                .execute(&pool)
                .await;
        assert_eq!(
            result
                .unwrap_err()
                .as_database_error()
                .unwrap()
                .constraint(),
            Some("platform_superadmin_continuity")
        );
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn concurrent_superadmin_removals_leave_one_actual_user(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let first_user = session.id.unwrap();
        let second_user = get_random_user_id(&app, &mut session).await?;
        sqlx::query(
            "INSERT INTO system_user_permissions (user_id, role_name, grant_reason)
                         VALUES ($1, 'super_admin', 'Concurrency test')",
        )
        .bind(second_user)
        .execute(&pool)
        .await?;
        let first_pool = pool.clone();
        let first = tokio::spawn(async move {
            sqlx::query("DELETE FROM system_user_permissions WHERE user_id = $1 AND role_name = 'super_admin'")
                    .bind(first_user).execute(&first_pool).await
        });
        let second = tokio::spawn(async move {
            sqlx::query("DELETE FROM system_user_permissions WHERE user_id = $1 AND role_name = 'super_admin'")
                    .bind(second_user).execute(&pool).await
        });
        let (first, second) = tokio::join!(first, second);
        assert_ne!(first?.is_ok(), second?.is_ok());
        Ok(())
    }
    use super::*;
    use crate::error::ComhairleError;
    use crate::models::model_test_helpers::{
        get_random_organization_id, get_random_user_id, setup_default_app_and_session,
    };
    use crate::redis_connection::{MockRedis, RedisConnection};
    use crate::test_helpers::{TEST_RESOURCE_TYPE, TEST_ROLE_NAME, TestRole, test_state};

    use sea_query::DeleteStatement;
    use sqlx::PgPool;

    const OTHER_ROLE_NAME: &str = "other_role";

    #[test]
    fn policy_discovery_uses_typed_roles() {
        assert_eq!(
            roles_for_action(conversation::Action::Read),
            vec![
                "content_editor".to_owned(),
                "conversation_co_host".to_owned(),
            ]
        );
        assert_eq!(
            roles_for_action(conversation::Action::Update),
            vec!["content_editor".to_owned(),]
        );
        assert_eq!(
            roles_for_action(organization::Action::Update),
            vec!["organization_admin".to_owned(),]
        );
        assert_eq!(
            roles_for_action(organization::Action::GrantPermission),
            vec!["organization_admin".to_owned(),]
        );
        assert_eq!(
            roles_for_action(system::Action::OrganizationCreate),
            vec!["super_admin".to_owned(),]
        );
        assert!(roles_for_action(conversation::Action::Admin).is_empty());
    }

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
            actor_id: ActorId::User(user_id),
            permission_triplet: TestRole::make_triplet(&resource_id),
            granted_by: &session.id.unwrap(),
            grant_reason: "Testing",
        };

        let assignment = grant_role(&state, grant_request).await?;
        assert_eq!(assignment.user_id, Some(user_id));
        assert_eq!(assignment.group_id, None);
        assert_eq!(assignment.resource_type, TestRole::resource_type());
        assert_eq!(assignment.role_name, TestRole::name());

        // Check that the user has the role
        let has_permission =
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id).await?;
        assert!(has_permission);

        // Check that the user does not have a different role
        let has_wrong_permission =
            has_resource_permission(&state, OtherRole::make_triplet(&resource_id), &user_id)
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
        // Create test organization and user
        let org_id = get_random_organization_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;

        // Mock conversation
        let resource_id = Uuid::new_v4();

        // Grant a role to the organization
        let grant_request = GrantRoleRequest {
            actor_id: ActorId::Group(
                models::user_group::organization_group(&state.db, org_id).await?,
            ),
            permission_triplet: TestRole::make_triplet(&resource_id),
            granted_by: &session.id.unwrap(),
            grant_reason: "Testing",
        };

        let assignment = grant_role(&state, grant_request).await?;
        assert_eq!(assignment.user_id, None);
        assert_eq!(
            assignment.group_id,
            Some(models::user_group::organization_group(&state.db, org_id).await?)
        );
        assert_eq!(assignment.resource_type, TestRole::resource_type());
        assert_eq!(assignment.role_name, TestRole::name());

        assert!(
            !has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id,)
                .await?
        );
        sqlx::query("INSERT INTO user_group_member (group_id, user_id) SELECT user_group_id, $2 FROM organization WHERE id = $1")
            .bind(org_id).bind(user_id).execute(&state.db).await?;
        let has_permission =
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id).await?;
        assert!(has_permission);

        // Check that the user does not have a different role
        let has_wrong_permission =
            has_resource_permission(&state, OtherRole::make_triplet(&resource_id), &user_id)
                .await?;
        assert!(!has_wrong_permission);

        sqlx::query("DELETE FROM user_group_member WHERE user_id = $1")
            .bind(user_id)
            .execute(&state.db)
            .await?;
        assert!(
            !has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id,)
                .await?
        );

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
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id).await?;
        assert!(!has_permission);

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: ActorId::Group(
                    models::user_group::organization_group(&state.db, organization_id).await?,
                ),
                permission_triplet: TestRole::make_triplet(&resource_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Non-member must not inherit group permissions",
            },
        )
        .await?;

        // Check that the user does not have any roles through the organization
        let has_org_permission =
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id).await?;
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
                actor_id: ActorId::User(user_id),
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
                actor_id: ActorId::User(user_id),
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
                actor_id: ActorId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing",
            },
        )
        .await?;

        // Confirm permission is granted.
        assert!(
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id).await?
        );

        // Revoke the role.
        revoke_role(
            &state,
            RevokeRoleRequest {
                actor_id: ActorId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_id),
            },
        )
        .await?;

        // Confirm permission no longer granted.
        assert!(
            !has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id)
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
                actor_id: ActorId::User(user.id),
                permission_triplet: system::Role::SuperAdmin.system_triplet()?,
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
                actor_id: ActorId::User(user_id),
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
                actor_id: ActorId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_1_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing",
            },
        )
        .await?;

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: ActorId::Group(
                    models::user_group::organization_group(&state.db, org_id).await?,
                ),
                permission_triplet: OtherRole::make_triplet(&resource_1_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing",
            },
        )
        .await?;

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: ActorId::User(user_id),
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
            actor: Some(ActorId::User(user_id)),
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
            actor: Some(ActorId::Group(
                models::user_group::organization_group(&state.db, org_id).await?,
            )),
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
                actor_id: ActorId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing",
            },
        )
        .await?;

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: ActorId::User(user_id),
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
                actor: Some(ActorId::User(user_id)),
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
                actor_id: ActorId::User(user_id),
                permission_triplet: conversation::Role::CoHost.triplet(&resource_1_id)?,
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing",
            },
        )
        .await?;

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: ActorId::User(user_id),
                permission_triplet: conversation::Role::ContentEditor.triplet(&resource_2_id)?,
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing",
            },
        )
        .await?;

        // Grant the organization admin role to the user for resource_3
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: ActorId::User(user_id),
                permission_triplet: organization::Role::Admin.triplet(&resource_3_id)?,
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing",
            },
        )
        .await?;

        // List permissions which permit the user to perform the "conversation_read" action
        let permissions =
            list_permissions_by_action(&state.db, user_id, "conversation_read").await?;

        assert_eq!(permissions.len(), 2);
        assert!(permissions.iter().any(|p| p.resource_id == resource_1_id));
        assert!(permissions.iter().any(|p| p.resource_id == resource_2_id));

        // List permissions which permit the user to perform the "conversation_update" action
        let permissions =
            list_permissions_by_action(&state.db, user_id, "conversation_update").await?;

        assert_eq!(permissions.len(), 1);
        assert_eq!(permissions[0].resource_id, resource_2_id);

        // List permissions which permit the user to perform the "organization_update" action
        let permissions =
            list_permissions_by_action(&state.db, user_id, "organization_update").await?;

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
                actor_id: ActorId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing cache",
            },
        )
        .await?;

        // First check: hits the DB and populates the cache.
        let first =
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id).await?;
        assert!(first, "expected permission to be present");

        let key = role_list_cache_key(
            TestRole::resource_type(),
            &resource_id,
            &ActorId::User(user_id),
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
        sqlx::query("DELETE FROM conversation_user_permissions WHERE user_id = $1 AND resource_id = $2 AND role_name = $3")
            .bind(user_id)
            .bind(resource_id)
            .bind(TestRole::name())
            .execute(&pool)
            .await?;

        let second =
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id).await?;
        assert!(
            !second,
            "a database version change must invalidate a cached positive result"
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
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id).await?;
        assert!(!initial_has_role, "expected no role before grant");

        let role_list_key = role_list_cache_key(
            TestRole::resource_type(),
            &resource_id,
            &ActorId::User(user_id),
        );
        let cached_before_grant = mock.get_value(&role_list_key).await;
        let cached: CachedRoles = serde_json::from_str(&cached_before_grant.unwrap())?;
        assert_eq!(cached.version, 0);
        assert!(cached.roles.is_empty());

        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: ActorId::User(user_id),
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
                actor_id: ActorId::User(user_id),
                permission_triplet: TestRole::make_triplet(&resource_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing role list cache invalidation on revoke",
            },
        )
        .await?;

        let has_role =
            has_resource_permission(&state, TestRole::make_triplet(&resource_id), &user_id).await?;
        assert!(has_role, "expected role to exist before revoke");

        let role_list_key = role_list_cache_key(
            TestRole::resource_type(),
            &resource_id,
            &ActorId::User(user_id),
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
                actor_id: ActorId::User(user_id),
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
                actor_id: ActorId::User(user_id),
                permission_triplet: conversation::Role::ContentEditor.triplet(&resource_id)?,
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing action role list cache",
            },
        )
        .await?;

        let first = can_perform_action(
            &state,
            &resource_id,
            conversation::Action::Read,
            &user_id,
            None,
        )
        .await?;
        assert!(first, "expected first action check to be allowed");

        let role_list_key = role_list_cache_key(
            ResourceType::Conversation.as_ref(),
            &resource_id,
            &ActorId::User(user_id),
        );
        let cached_roles = mock.get_value(&role_list_key).await;
        assert!(
            cached_roles
                .as_ref()
                .is_some_and(|raw| raw.contains(conversation::Role::ContentEditor.as_ref())),
            "expected cached role list to include content editor role"
        );

        let (sql, values) = DeleteStatement::new()
            .from_table(conversation::UserPermissionIden::Table)
            .and_where(Expr::col("user_id").eq(user_id))
            .and_where(Expr::col("resource_id").eq(resource_id))
            .and_where(Expr::col("role_name").eq(conversation::Role::ContentEditor.as_ref()))
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values).execute(&pool).await?;

        let second = can_perform_action(
            &state,
            &resource_id,
            conversation::Action::Read,
            &user_id,
            None,
        )
        .await?;
        assert!(
            !second,
            "a revoked role must not authorize an action from an outdated cache"
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
            actor_id: ActorId::User(user_a_id),
            permission_triplet: TestRole::make_triplet(&resource_id),
            granted_by: &session.id.unwrap(),
            grant_reason: "Testing",
        };
        grant_role(&state, grant_request_a).await?;

        let grant_request_b = GrantRoleRequest {
            actor_id: ActorId::User(user_b_id),
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
                actor_id: ActorId::User(user_id),
                permission_triplet: conversation::Role::ContentEditor.triplet(&resource_id)?,
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing action checks",
            },
        )
        .await?;

        let can_read = can_perform_action(
            &state,
            &resource_id,
            conversation::Action::Read,
            &user_id,
            None,
        )
        .await?;
        assert!(can_read, "content editor should allow read action");

        let can_update = can_perform_action(
            &state,
            &resource_id,
            conversation::Action::Update,
            &user_id,
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
                actor_id: ActorId::User(user_id),
                permission_triplet: OtherRole::make_triplet(&resource_id),
                granted_by: &session.id.unwrap(),
                grant_reason: "Testing action checks",
            },
        )
        .await?;

        let can_read = can_perform_action(
            &state,
            &resource_id,
            conversation::Action::Read,
            &user_id,
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
