use std::sync::Arc;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{ComhairleState, error::ComhairleError};

pub const CACHE_TTL_SECONDS: u64 = 300;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, FromRow)]
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
    fn column(self) -> &'static str {
        match self {
            Self::Members => "user_group_id",
            Self::Admins => "admin_group_id",
        }
    }
}

pub async fn organization_group_for(
    db: &sqlx::PgPool,
    organization_id: Uuid,
    group: OrganizationGroup,
) -> Result<Uuid, ComhairleError> {
    sqlx::query_scalar(&format!(
        "SELECT {} FROM organization WHERE id = $1",
        group.column()
    ))
    .bind(organization_id)
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
    let version: i64 = sqlx::query_scalar(
        "SELECT coalesce((SELECT version FROM user_group_membership_version WHERE user_id = $1), 0)",
    )
    .bind(user_id)
    .fetch_one(&state.db)
    .await?;
    let cache_key = membership_cache_key(user_id);
    if let Some(connection) = &state.redis_conn
        && let Ok(Some(raw)) = connection.get(&cache_key).await
        && let Ok(cached) = serde_json::from_str::<CachedMemberships>(&raw)
        && cached.version == version
    {
        return Ok(cached.groups);
    }

    let memberships = sqlx::query_as::<_, CachedMemberships>(
        "SELECT coalesce((SELECT version FROM user_group_membership_version WHERE user_id = $1), 0) AS version,
         ARRAY(SELECT group_id FROM user_group_member WHERE user_id = $1 ORDER BY group_id) AS groups",
    )
    .bind(user_id)
    .fetch_one(&state.db)
    .await?;
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

pub async fn is_organization_member(
    db: &sqlx::PgPool,
    organization_id: Uuid,
    user_id: Uuid,
) -> Result<bool, ComhairleError> {
    Ok(sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM user_group_member JOIN organization
        ON organization.user_group_id = user_group_member.group_id
        OR organization.admin_group_id = user_group_member.group_id
        WHERE organization.id = $1 AND user_id = $2)",
    )
    .bind(organization_id)
    .bind(user_id)
    .fetch_one(db)
    .await?)
}

pub async fn organization_groups_of_user(
    db: &sqlx::PgPool,
    organization_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<OrganizationGroup>, ComhairleError> {
    let (member, administrator): (bool, bool) = sqlx::query_as(
        "SELECT
            EXISTS (SELECT 1 FROM user_group_member WHERE user_id = $2 AND group_id = organization.user_group_id),
            EXISTS (SELECT 1 FROM user_group_member WHERE user_id = $2 AND group_id = organization.admin_group_id)
         FROM organization WHERE id = $1",
    )
    .bind(organization_id)
    .bind(user_id)
    .fetch_one(db)
    .await?;
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
    sqlx::query_as(
        "SELECT comhairle_user.id, comhairle_user.username, comhairle_user.email,
            bool_or(membership.group_id = organization.user_group_id) AS is_member,
            bool_or(membership.group_id = organization.admin_group_id) AS is_admin
         FROM organization
         JOIN user_group_member membership
           ON membership.group_id IN (organization.user_group_id, organization.admin_group_id)
         JOIN comhairle_user ON comhairle_user.id = membership.user_id
         WHERE organization.id = $1
         GROUP BY comhairle_user.id, comhairle_user.username, comhairle_user.email
         ORDER BY comhairle_user.email, comhairle_user.id",
    )
    .bind(organization_id)
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
    use super::permissions::{self, organization::Action};
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
    let (members_group, admins_group): (Uuid, Uuid) =
        sqlx::query_as("SELECT user_group_id, admin_group_id FROM organization WHERE id = $1")
            .bind(organization_id)
            .fetch_one(&mut *transaction)
            .await?;
    let target_groups = match group {
        OrganizationGroup::Members => vec![members_group],
        OrganizationGroup::Admins => vec![members_group, admins_group],
    };
    let grants_superadmin: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM system_group_permissions
        WHERE group_id = ANY($1) AND role_name = 'super_admin')",
    )
    .bind(&target_groups)
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
    sqlx::query(
        "INSERT INTO user_group_member (group_id, user_id)
         SELECT group_id, $2 FROM unnest($1::uuid[]) AS group_id ON CONFLICT DO NOTHING",
    )
    .bind(&target_groups)
    .bind(user_id)
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
    use super::permissions::{self, ActorId, organization::Action};
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
    let (members_group, admins_group): (Uuid, Uuid) =
        sqlx::query_as("SELECT user_group_id, admin_group_id FROM organization WHERE id = $1")
            .bind(organization_id)
            .fetch_one(&mut *transaction)
            .await?;
    let target_groups = match group {
        OrganizationGroup::Members => vec![members_group, admins_group],
        OrganizationGroup::Admins => vec![admins_group],
    };
    sqlx::query("DELETE FROM user_group_member WHERE user_id = $1 AND group_id = ANY($2)")
        .bind(user_id)
        .bind(&target_groups)
        .execute(&mut *transaction)
        .await?;
    if group == OrganizationGroup::Members {
        sqlx::query(
            "DELETE FROM organization_user_permissions WHERE user_id = $1 AND resource_id = $2",
        )
        .bind(user_id)
        .bind(organization_id)
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "UPDATE comhairle_user SET organization_id = NULL WHERE id = $1 AND organization_id = $2",
        )
        .bind(user_id)
        .bind(organization_id)
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
        sqlx::query("INSERT INTO user_group_member (group_id, user_id) VALUES ($1, $3), ($2, $3)")
            .bind(first_group)
            .bind(second_group)
            .bind(user_id)
            .execute(&pool)
            .await?;
        assert_eq!(memberships(&state, user_id).await?.len(), 2);
        sqlx::query("DELETE FROM user_group_member WHERE group_id = $1 AND user_id = $2")
            .bind(first_group)
            .bind(user_id)
            .execute(&pool)
            .await?;
        assert_eq!(memberships(&state, user_id).await?, vec![second_group]);
        Ok(())
    }
}
