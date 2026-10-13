use std::sync::Arc;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum::IntoEnumIterator;
use uuid::Uuid;

use super::{self as permissions, ActorId, ResourceType};
use crate::{ComhairleState, error::ComhairleError};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "id", rename_all = "snake_case")]
pub enum Recipient {
    User(Uuid),
    Group(Uuid),
}

impl From<Recipient> for ActorId {
    fn from(recipient: Recipient) -> Self {
        match recipient {
            Recipient::User(user_id) => Self::User(user_id),
            Recipient::Group(group_id) => Self::Group(group_id),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, FromRow)]
pub struct InheritedRoles {
    pub group_id: Uuid,
    pub group_name: String,
    pub organization_id: Option<Uuid>,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Snapshot {
    pub version: i64,
    pub roles: Vec<String>,
    pub inherited: Vec<InheritedRoles>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SaveAssignments {
    pub expected_version: i64,
    pub roles: Vec<String>,
    pub grant_reason: String,
}

pub fn role_catalogue(resource_type: ResourceType) -> Vec<String> {
    match resource_type {
        ResourceType::Conversation => permissions::conversation::Role::iter()
            .map(|role| role.to_string())
            .collect(),
        ResourceType::Organization => permissions::organization::Role::iter()
            .map(|role| role.to_string())
            .collect(),
        ResourceType::System => permissions::system::Role::iter()
            .map(|role| role.to_string())
            .collect(),
    }
}

pub async fn validate_target(
    connection: &mut sqlx::PgConnection,
    resource_type: ResourceType,
    resource_id: Uuid,
) -> Result<(), ComhairleError> {
    let exists = match resource_type {
        ResourceType::Conversation => {
            sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS (SELECT 1 FROM conversation WHERE id = $1)",
            )
            .bind(resource_id)
            .fetch_one(&mut *connection)
            .await?
        }
        ResourceType::Organization => {
            sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS (SELECT 1 FROM organization WHERE id = $1)",
            )
            .bind(resource_id)
            .fetch_one(&mut *connection)
            .await?
        }
        ResourceType::System => resource_id == permissions::SYSTEM_RESOURCE_ID,
    };
    if exists {
        Ok(())
    } else {
        Err(ComhairleError::ResourceNotFound(resource_type.to_string()))
    }
}

pub async fn authorize(
    connection: &mut sqlx::PgConnection,
    resource_type: ResourceType,
    resource_id: Uuid,
    caller_id: Uuid,
    action: &str,
) -> Result<(), ComhairleError> {
    match resource_type {
        ResourceType::Conversation => {
            permissions::authorize_transaction(
                connection,
                resource_id,
                action
                    .parse::<permissions::conversation::Action>()
                    .map_err(|_| ComhairleError::UserNotAuthorized)?,
                caller_id,
            )
            .await
        }
        ResourceType::Organization => {
            permissions::authorize_transaction(
                connection,
                resource_id,
                action
                    .parse::<permissions::organization::Action>()
                    .map_err(|_| ComhairleError::UserNotAuthorized)?,
                caller_id,
            )
            .await
        }
        ResourceType::System => {
            permissions::authorize_transaction(
                connection,
                resource_id,
                action
                    .parse::<permissions::system::Action>()
                    .map_err(|_| ComhairleError::UserNotAuthorized)?,
                caller_id,
            )
            .await
        }
    }
}

async fn snapshot(
    connection: &mut sqlx::PgConnection,
    resource_type: ResourceType,
    resource_id: Uuid,
    recipient: Recipient,
) -> Result<Snapshot, ComhairleError> {
    let actor = recipient.into();
    let version = permissions::actor_permission_version(
        &mut *connection,
        resource_type.as_ref(),
        &resource_id,
        actor,
    )
    .await?;
    let roles = permissions::fetch_actor_roles_for_resource(
        &mut *connection,
        resource_type.as_ref(),
        &resource_id,
        actor,
    )
    .await?;
    let inherited = match recipient {
        Recipient::Group(_) => Vec::new(),
        Recipient::User(user_id) => {
            let (table, _) =
                permissions::permission_table(resource_type.as_ref(), ActorId::Group(Uuid::nil()))?;
            let table = table.to_string();
            let resource_filter = if resource_type == ResourceType::System {
                ""
            } else {
                "AND permission.resource_id = $2"
            };
            let sql = format!(
            "SELECT user_group.id AS group_id, user_group.name AS group_name, organization.id AS organization_id,
             array_agg(DISTINCT permission.role_name ORDER BY permission.role_name) AS roles
             FROM {table} permission JOIN user_group ON user_group.id = permission.group_id
             JOIN user_group_member membership ON membership.group_id = user_group.id
             LEFT JOIN organization ON organization.user_group_id = user_group.id
               WHERE membership.user_id = $1 {resource_filter}
               GROUP BY user_group.id, user_group.name, organization.id"
           );
            let mut query = sqlx::query_as::<_, InheritedRoles>(&sql).bind(user_id);
            if resource_type != ResourceType::System {
                query = query.bind(resource_id);
            }
            query.fetch_all(&mut *connection).await?
        }
    };
    Ok(Snapshot {
        version,
        roles,
        inherited,
    })
}

pub async fn load(
    state: &Arc<ComhairleState>,
    resource_type: ResourceType,
    resource_id: Uuid,
    caller_id: Uuid,
    recipient: Recipient,
) -> Result<Snapshot, ComhairleError> {
    let mut transaction = state.db.begin().await?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *transaction)
        .await?;
    validate_target(&mut transaction, resource_type, resource_id).await?;
    authorize(
        &mut transaction,
        resource_type,
        resource_id,
        caller_id,
        "list_permission",
    )
    .await?;
    let snapshot = snapshot(&mut transaction, resource_type, resource_id, recipient).await?;
    transaction.commit().await?;
    Ok(snapshot)
}

pub async fn save(
    state: &Arc<ComhairleState>,
    resource_type: ResourceType,
    resource_id: Uuid,
    caller_id: Uuid,
    recipient: Recipient,
    request: SaveAssignments,
) -> Result<Snapshot, ComhairleError> {
    if request
        .roles
        .iter()
        .any(|role| !role_catalogue(resource_type).contains(role))
    {
        return Err(ComhairleError::BadRequest(
            "Role does not belong to this resource type".into(),
        ));
    }
    let mut transaction = state.db.begin().await?;
    permissions::lock_permission_mutation(&mut transaction).await?;
    validate_target(&mut transaction, resource_type, resource_id).await?;
    authorize(
        &mut transaction,
        resource_type,
        resource_id,
        caller_id,
        "grant_permission",
    )
    .await?;
    let current = snapshot(&mut transaction, resource_type, resource_id, recipient).await?;
    if current.version != request.expected_version {
        return Err(ComhairleError::Conflict(
            "Assignments changed. Reload before saving".into(),
        ));
    }
    if resource_type == ResourceType::Organization {
        match recipient {
            Recipient::User(user_id) if user_id == caller_id => {
                return Err(ComhairleError::UserNotAuthorized);
            }
            Recipient::Group(group_id)
                if current.roles.contains(&"organization_admin".into())
                    && !request.roles.contains(&"organization_admin".into()) =>
            {
                let own_membership: bool = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM user_group_member WHERE group_id = $1 AND user_id = $2)")
                    .bind(group_id).bind(caller_id).fetch_one(&mut *transaction).await?;
                if own_membership {
                    return Err(ComhairleError::UserNotAuthorized);
                }
            }
            _ => {}
        }
    }
    let (recipient_type, recipient_id, recipient_column) = match recipient {
        Recipient::User(user_id) => ("user", user_id, "user_id"),
        Recipient::Group(group_id) => ("group", group_id, "group_id"),
    };
    let (table, _) = permissions::permission_table(resource_type.as_ref(), recipient.into())?;
    let table = table.to_string();
    let resource_filter = if resource_type == ResourceType::System {
        ""
    } else {
        "AND resource_id = $3"
    };
    let delete_sql = format!(
        "DELETE FROM {table} WHERE {recipient_column} = $1 AND NOT (role_name = ANY($2)) {resource_filter}"
    );
    let mut delete = sqlx::query(&delete_sql)
        .bind(recipient_id)
        .bind(&request.roles);
    if resource_type != ResourceType::System {
        delete = delete.bind(resource_id);
    }
    delete.execute(&mut *transaction).await?;
    for role in &request.roles {
        let insert_sql = if resource_type == ResourceType::System {
            format!(
                "INSERT INTO {table} ({recipient_column}, role_name, granted_by, grant_reason)
                VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING"
            )
        } else {
            format!("INSERT INTO {table} ({recipient_column}, role_name, granted_by, grant_reason, resource_id)
                VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
        };
        let mut insert = sqlx::query(&insert_sql)
            .bind(recipient_id)
            .bind(role)
            .bind(caller_id)
            .bind(&request.grant_reason);
        if resource_type != ResourceType::System {
            insert = insert.bind(resource_id);
        }
        insert.execute(&mut *transaction).await?;
    }
    sqlx::query("INSERT INTO permission_set_version (resource_type, resource_id, recipient_type, recipient_id)
        VALUES ($1, $2, $3, $4) ON CONFLICT (resource_type, resource_id, recipient_type, recipient_id)
        DO UPDATE SET version = permission_set_version.version + 1")
        .bind(resource_type.as_ref()).bind(resource_id).bind(recipient_type).bind(recipient_id).execute(&mut *transaction).await?;
    let snapshot = snapshot(&mut transaction, resource_type, resource_id, recipient).await?;
    transaction
        .commit()
        .await
        .map_err(permissions::permission_database_error)?;
    permissions::invalidate_actor_roles(
        state,
        resource_type.as_ref(),
        &resource_id,
        recipient.into(),
    )
    .await?;
    Ok(snapshot)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::model_test_helpers::{
        get_random_organization_id, get_random_user_id, setup_default_app_and_session,
    };
    use crate::test_helpers::test_state;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn stale_save_is_atomic(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let caller_id = session.id.unwrap();
        let organization_id = get_random_organization_id(&app, &mut session).await?;
        let recipient = Recipient::User(get_random_user_id(&app, &mut session).await?);
        let state = Arc::new(test_state().db(pool).call()?);
        let first = save(
            &state,
            ResourceType::Organization,
            organization_id,
            caller_id,
            recipient,
            SaveAssignments {
                expected_version: 0,
                roles: vec!["organization_admin".into()],
                grant_reason: "Test".into(),
            },
        )
        .await?;
        assert!(first.version > 0);
        let stale = save(
            &state,
            ResourceType::Organization,
            organization_id,
            caller_id,
            recipient,
            SaveAssignments {
                expected_version: 0,
                roles: Vec::new(),
                grant_reason: "Stale".into(),
            },
        )
        .await;
        assert!(matches!(stale, Err(ComhairleError::Conflict(_))));
        let snapshot = load(
            &state,
            ResourceType::Organization,
            organization_id,
            caller_id,
            recipient,
        )
        .await?;
        assert_eq!(snapshot.version, first.version);
        assert_eq!(snapshot.roles, first.roles);
        Ok(())
    }
}
