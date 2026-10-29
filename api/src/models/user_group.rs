use std::collections::HashSet;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Alias, Expr, Func, JoinType, OnConflict, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::models::organization::OrganizationIden;

pub const CACHE_TTL_SECONDS: u64 = 300;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, FromRow)]
#[enum_def(table_name = "user_group")]
pub struct UserGroup {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct CachedMemberships {
    version: i64,
    groups: Vec<Uuid>,
}

pub fn membership_cache_key(user_id: Uuid) -> String {
    format!("memberships:v1:user:{user_id}")
}

/// The two User groups every Organization owns. Members hold the view role and
/// Admins hold the admin role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationGroup {
    Members,
    Admins,
}

impl OrganizationGroup {
    fn column(self) -> OrganizationIden {
        match self {
            Self::Members => OrganizationIden::UserGroupId,
            Self::Admins => OrganizationIden::AdminGroupId,
        }
    }
}

#[derive(sea_query::Iden)]
enum UserGroupMemberIden {
    #[iden = "user_group_member"]
    Table,
    GroupId,
    UserId,
}

#[derive(sea_query::Iden)]
enum UserGroupMembershipVersionIden {
    #[iden = "user_group_membership_version"]
    Table,
    UserId,
    Version,
}

#[derive(sea_query::Iden)]
enum SystemGroupPermissionsIden {
    #[iden = "system_group_permissions"]
    Table,
    GroupId,
    RoleName,
}

#[derive(sea_query::Iden)]
enum OrganizationUserPermissionsIden {
    #[iden = "organization_user_permissions"]
    Table,
    UserId,
    ResourceId,
}

#[derive(sea_query::Iden)]
enum ComhairleUserIden {
    #[iden = "comhairle_user"]
    Table,
    Id,
    Username,
    Email,
    OrganizationId,
}

pub async fn organization_group_for(
    db: &sqlx::PgPool,
    organization_id: Uuid,
    group: OrganizationGroup,
) -> Result<Uuid, ComhairleError> {
    let (sql, values) = Query::select()
        .column(group.column())
        .from(OrganizationIden::Table)
        .and_where(Expr::col(OrganizationIden::Id).eq(organization_id))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_scalar_with(&sql, values)
        .fetch_one(db)
        .await
        .map_err(ComhairleError::DatabaseError)
}

/// The Organization's members group.
pub async fn organization_group(
    db: &sqlx::PgPool,
    organization_id: Uuid,
) -> Result<Uuid, ComhairleError> {
    organization_group_for(db, organization_id, OrganizationGroup::Members).await
}

pub async fn memberships(
    state: &Arc<ComhairleState>,
    user_id: Uuid,
) -> Result<Vec<Uuid>, ComhairleError> {
    let (sql, values) = Query::select()
        .expr(Func::coalesce([
            Expr::col((
                UserGroupMembershipVersionIden::Table,
                UserGroupMembershipVersionIden::Version,
            ))
            .into(),
            Expr::val(0).into(),
        ]))
        .column((UserGroupMemberIden::Table, UserGroupMemberIden::GroupId))
        .from(ComhairleUserIden::Table)
        .join(
            JoinType::LeftJoin,
            UserGroupMembershipVersionIden::Table,
            Expr::col((
                UserGroupMembershipVersionIden::Table,
                UserGroupMembershipVersionIden::UserId,
            ))
            .equals((ComhairleUserIden::Table, ComhairleUserIden::Id)),
        )
        .join(
            JoinType::LeftJoin,
            UserGroupMemberIden::Table,
            Expr::col((UserGroupMemberIden::Table, UserGroupMemberIden::UserId))
                .equals((ComhairleUserIden::Table, ComhairleUserIden::Id)),
        )
        .and_where(Expr::col((ComhairleUserIden::Table, ComhairleUserIden::Id)).eq(user_id))
        .order_by(
            (UserGroupMemberIden::Table, UserGroupMemberIden::GroupId),
            sea_query::Order::Asc,
        )
        .build_sqlx(PostgresQueryBuilder);
    let rows: Vec<(i64, Option<Uuid>)> = sqlx::query_as_with(&sql, values)
        .fetch_all(&state.db)
        .await?;
    let version = rows.first().map_or(0, |(version, _)| *version);
    let cache_key = membership_cache_key(user_id);
    if let Some(connection) = &state.redis_conn
        && let Ok(Some(raw)) = connection.get(&cache_key).await
        && let Ok(cached) = serde_json::from_str::<CachedMemberships>(&raw)
        && cached.version == version
    {
        return Ok(cached.groups);
    }

    let memberships = CachedMemberships {
        version,
        groups: rows
            .into_iter()
            .filter_map(|(_, group_id)| group_id)
            .collect(),
    };
    if let Some(connection) = &state.redis_conn
        && let Ok(serialized) = serde_json::to_string(&memberships)
    {
        let _ = connection
            .set_ex(&cache_key, &serialized, CACHE_TTL_SECONDS)
            .await;
    }
    Ok(memberships.groups)
}

pub async fn invalidate_memberships(state: &Arc<ComhairleState>, user_id: Uuid) {
    if let Some(connection) = &state.redis_conn {
        let _ = connection.del(&membership_cache_key(user_id)).await;
    }
}

/// Returns the ids of the given organizations the user belongs to, in one query.
pub async fn organization_ids_with_membership(
    db: &sqlx::PgPool,
    organization_ids: &[Uuid],
    user_id: Uuid,
) -> Result<HashSet<Uuid>, ComhairleError> {
    if organization_ids.is_empty() {
        return Ok(HashSet::new());
    }

    let (sql, values) = Query::select()
        .column((OrganizationIden::Table, OrganizationIden::Id))
        .from(UserGroupMemberIden::Table)
        .join(
            JoinType::InnerJoin,
            OrganizationIden::Table,
            Expr::col((OrganizationIden::Table, OrganizationIden::UserGroupId))
                .equals((UserGroupMemberIden::Table, UserGroupMemberIden::GroupId))
                .or(
                    Expr::col((OrganizationIden::Table, OrganizationIden::AdminGroupId))
                        .equals((UserGroupMemberIden::Table, UserGroupMemberIden::GroupId)),
                ),
        )
        .and_where(Expr::col((UserGroupMemberIden::Table, UserGroupMemberIden::UserId)).eq(user_id))
        .and_where(
            Expr::col((OrganizationIden::Table, OrganizationIden::Id))
                .is_in(organization_ids.iter().copied()),
        )
        .build_sqlx(PostgresQueryBuilder);

    let ids: Vec<(Uuid,)> = sqlx::query_as_with(&sql, values).fetch_all(db).await?;
    Ok(ids.into_iter().map(|(id,)| id).collect())
}

pub async fn organization_groups_of_user(
    db: &sqlx::PgPool,
    organization_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<OrganizationGroup>, ComhairleError> {
    let member = Query::select()
        .expr(Expr::val(1))
        .from(UserGroupMemberIden::Table)
        .and_where(Expr::col(UserGroupMemberIden::UserId).eq(user_id))
        .and_where(
            Expr::col(UserGroupMemberIden::GroupId)
                .equals((OrganizationIden::Table, OrganizationIden::UserGroupId)),
        )
        .to_owned();
    let administrator = Query::select()
        .expr(Expr::val(1))
        .from(UserGroupMemberIden::Table)
        .and_where(Expr::col(UserGroupMemberIden::UserId).eq(user_id))
        .and_where(
            Expr::col(UserGroupMemberIden::GroupId)
                .equals((OrganizationIden::Table, OrganizationIden::AdminGroupId)),
        )
        .to_owned();
    let (sql, values) = Query::select()
        .expr(Expr::exists(member))
        .expr(Expr::exists(administrator))
        .from(OrganizationIden::Table)
        .and_where(Expr::col(OrganizationIden::Id).eq(organization_id))
        .build_sqlx(PostgresQueryBuilder);
    let (member, administrator): (bool, bool) =
        sqlx::query_as_with(&sql, values).fetch_one(db).await?;
    Ok([
        (member, OrganizationGroup::Members),
        (administrator, OrganizationGroup::Admins),
    ]
    .into_iter()
    .filter_map(|(belongs, group)| belongs.then_some(group))
    .collect())
}

#[derive(Debug, FromRow)]
pub struct OrganizationTeamMember {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
    pub is_member: bool,
    pub is_admin: bool,
}

pub async fn organization_team(
    db: &sqlx::PgPool,
    organization_id: Uuid,
) -> Result<Vec<OrganizationTeamMember>, ComhairleError> {
    let membership = Alias::new("membership");
    let is_member = Func::cust("bool_or").arg(
        Expr::col((membership.clone(), UserGroupMemberIden::GroupId))
            .equals((OrganizationIden::Table, OrganizationIden::UserGroupId)),
    );
    let is_admin = Func::cust("bool_or").arg(
        Expr::col((membership.clone(), UserGroupMemberIden::GroupId))
            .equals((OrganizationIden::Table, OrganizationIden::AdminGroupId)),
    );
    let (sql, values) = Query::select()
        .columns([
            (ComhairleUserIden::Table, ComhairleUserIden::Id),
            (ComhairleUserIden::Table, ComhairleUserIden::Username),
            (ComhairleUserIden::Table, ComhairleUserIden::Email),
        ])
        .expr_as(is_member, Alias::new("is_member"))
        .expr_as(is_admin, Alias::new("is_admin"))
        .from(OrganizationIden::Table)
        .join_as(
            JoinType::InnerJoin,
            UserGroupMemberIden::Table,
            membership.clone(),
            Expr::col((membership.clone(), UserGroupMemberIden::GroupId))
                .equals((OrganizationIden::Table, OrganizationIden::UserGroupId))
                .or(
                    Expr::col((membership.clone(), UserGroupMemberIden::GroupId))
                        .equals((OrganizationIden::Table, OrganizationIden::AdminGroupId)),
                ),
        )
        .join(
            JoinType::InnerJoin,
            ComhairleUserIden::Table,
            Expr::col((ComhairleUserIden::Table, ComhairleUserIden::Id))
                .equals((membership.clone(), UserGroupMemberIden::UserId)),
        )
        .and_where(Expr::col((OrganizationIden::Table, OrganizationIden::Id)).eq(organization_id))
        .group_by_columns([
            (ComhairleUserIden::Table, ComhairleUserIden::Id),
            (ComhairleUserIden::Table, ComhairleUserIden::Username),
            (ComhairleUserIden::Table, ComhairleUserIden::Email),
        ])
        .order_by(
            (ComhairleUserIden::Table, ComhairleUserIden::Email),
            sea_query::Order::Asc,
        )
        .order_by(
            (ComhairleUserIden::Table, ComhairleUserIden::Id),
            sea_query::Order::Asc,
        )
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with(&sql, values)
        .fetch_all(db)
        .await
        .map_err(ComhairleError::DatabaseError)
}

/// Add a user to one of an Organization's groups. Administrators are always members, so
/// joining the administrators group also joins the members group.
pub async fn add_organization_member(
    state: &Arc<ComhairleState>,
    organization_id: Uuid,
    group: OrganizationGroup,
    user_id: Uuid,
    caller_id: Uuid,
) -> Result<(), ComhairleError> {
    use super::permissions::organization::Action;
    use super::permissions::{self};
    if user_id == caller_id {
        return Err(ComhairleError::UserNotAuthorized);
    }
    let mut transaction = state.db.begin().await?;
    permissions::lock_permission_mutation(&mut transaction).await?;
    permissions::authorize_transaction(
        &mut transaction,
        organization_id,
        Action::AddMember,
        caller_id,
    )
    .await?;
    let (sql, values) = Query::select()
        .columns([
            OrganizationIden::UserGroupId,
            OrganizationIden::AdminGroupId,
        ])
        .from(OrganizationIden::Table)
        .and_where(Expr::col(OrganizationIden::Id).eq(organization_id))
        .build_sqlx(PostgresQueryBuilder);
    let (members_group, admins_group): (Uuid, Uuid) = sqlx::query_as_with(&sql, values)
        .fetch_one(&mut *transaction)
        .await?;
    let target_groups = match group {
        OrganizationGroup::Members => vec![members_group],
        OrganizationGroup::Admins => vec![members_group, admins_group],
    };
    let superadmin_permission = Query::select()
        .expr(Expr::val(1))
        .from(SystemGroupPermissionsIden::Table)
        .and_where(Expr::col(SystemGroupPermissionsIden::GroupId).is_in(target_groups.clone()))
        .and_where(Expr::col(SystemGroupPermissionsIden::RoleName).eq("super_admin"))
        .to_owned();
    let (sql, values) = Query::select()
        .expr(Expr::exists(superadmin_permission))
        .build_sqlx(PostgresQueryBuilder);
    let grants_superadmin: bool = sqlx::query_scalar_with(&sql, values)
        .fetch_one(&mut *transaction)
        .await?;
    if grants_superadmin {
        permissions::authorize_transaction(
            &mut transaction,
            permissions::SYSTEM_RESOURCE_ID,
            permissions::system::Action::GrantPermission,
            caller_id,
        )
        .await?;
    }
    let mut insert = Query::insert()
        .into_table(UserGroupMemberIden::Table)
        .columns([UserGroupMemberIden::GroupId, UserGroupMemberIden::UserId])
        .to_owned();
    for group_id in target_groups {
        insert.values([group_id.into(), user_id.into()])?;
    }
    insert.on_conflict(OnConflict::new().do_nothing().to_owned());
    let (sql, values) = insert.build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values)
        .execute(&mut *transaction)
        .await?;
    transaction
        .commit()
        .await
        .map_err(permissions::permission_database_error)?;
    invalidate_memberships(state, user_id).await;
    Ok(())
}

/// Remove a user from one of an Organization's groups. Leaving the administrators group
/// demotes the user to member; leaving the members group removes them from the
/// Organization entirely, including the administrators group and any direct grants.
pub async fn remove_organization_member(
    state: &Arc<ComhairleState>,
    organization_id: Uuid,
    group: OrganizationGroup,
    user_id: Uuid,
    caller_id: Uuid,
) -> Result<(), ComhairleError> {
    use super::permissions::organization::Action;
    use super::permissions::{self, ActorId};
    if user_id == caller_id {
        return Err(ComhairleError::UserNotAuthorized);
    }
    let mut transaction = state.db.begin().await?;
    permissions::lock_permission_mutation(&mut transaction).await?;
    permissions::authorize_transaction(
        &mut transaction,
        organization_id,
        Action::RemoveMember,
        caller_id,
    )
    .await?;
    let (sql, values) = Query::select()
        .columns([
            OrganizationIden::UserGroupId,
            OrganizationIden::AdminGroupId,
        ])
        .from(OrganizationIden::Table)
        .and_where(Expr::col(OrganizationIden::Id).eq(organization_id))
        .build_sqlx(PostgresQueryBuilder);
    let (members_group, admins_group): (Uuid, Uuid) = sqlx::query_as_with(&sql, values)
        .fetch_one(&mut *transaction)
        .await?;
    let target_groups = match group {
        OrganizationGroup::Members => vec![members_group, admins_group],
        OrganizationGroup::Admins => vec![admins_group],
    };
    let (sql, values) = Query::delete()
        .from_table(UserGroupMemberIden::Table)
        .and_where(Expr::col(UserGroupMemberIden::UserId).eq(user_id))
        .and_where(Expr::col(UserGroupMemberIden::GroupId).is_in(target_groups))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values)
        .execute(&mut *transaction)
        .await?;
    if group == OrganizationGroup::Members {
        let (sql, values) = Query::delete()
            .from_table(OrganizationUserPermissionsIden::Table)
            .and_where(Expr::col(OrganizationUserPermissionsIden::UserId).eq(user_id))
            .and_where(Expr::col(OrganizationUserPermissionsIden::ResourceId).eq(organization_id))
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values)
            .execute(&mut *transaction)
            .await?;
        let (sql, values) = Query::update()
            .table(ComhairleUserIden::Table)
            .value(ComhairleUserIden::OrganizationId, Option::<Uuid>::None)
            .and_where(Expr::col(ComhairleUserIden::Id).eq(user_id))
            .and_where(Expr::col(ComhairleUserIden::OrganizationId).eq(organization_id))
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values)
            .execute(&mut *transaction)
            .await?;
    }
    transaction
        .commit()
        .await
        .map_err(permissions::permission_database_error)?;
    invalidate_memberships(state, user_id).await;
    if group == OrganizationGroup::Members {
        permissions::invalidate_actor_roles(
            state,
            "organization",
            &organization_id,
            ActorId::User(user_id),
        )
        .await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::model_test_helpers::{
        get_random_organization_id, get_random_user_id, setup_default_app_and_session,
    };
    use crate::redis_connection::MockRedis;
    use crate::test_helpers::test_state;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn memberships_support_multiple_organizations_and_validate_cached_versions(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let first_organization = get_random_organization_id(&app, &mut session).await?;
        let second_organization = get_random_organization_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;
        let first_group = organization_group(&pool, first_organization).await?;
        let second_group = organization_group(&pool, second_organization).await?;
        let state = Arc::new(
            test_state()
                .db(pool.clone())
                .redis_conn(Arc::new(MockRedis::new()))
                .call()?,
        );
        assert!(memberships(&state, user_id).await?.is_empty());
        let (sql, values) = Query::insert()
            .into_table(UserGroupMemberIden::Table)
            .columns([UserGroupMemberIden::GroupId, UserGroupMemberIden::UserId])
            .values([first_group.into(), user_id.into()])?
            .values([second_group.into(), user_id.into()])?
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values).execute(&pool).await?;
        assert_eq!(memberships(&state, user_id).await?.len(), 2);
        let (sql, values) = Query::delete()
            .from_table(UserGroupMemberIden::Table)
            .and_where(Expr::col(UserGroupMemberIden::GroupId).eq(first_group))
            .and_where(Expr::col(UserGroupMemberIden::UserId).eq(user_id))
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values).execute(&pool).await?;
        assert_eq!(memberships(&state, user_id).await?, vec![second_group]);
        Ok(())
    }
}
