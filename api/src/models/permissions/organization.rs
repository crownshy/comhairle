use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

use super::{PermissionAction, PermissionRole, ResourceType};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, sqlx::FromRow)]
#[sea_query::enum_def(table_name = "organization_user_permissions")]
pub struct UserPermission {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub resource_id: uuid::Uuid,
    pub role_name: String,
    pub granted_by: Option<uuid::Uuid>,
    pub grant_reason: String,
    pub granted_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, sqlx::FromRow)]
#[sea_query::enum_def(table_name = "organization_group_permissions")]
pub struct GroupPermission {
    pub id: uuid::Uuid,
    pub group_id: uuid::Uuid,
    pub resource_id: uuid::Uuid,
    pub role_name: String,
    pub granted_by: Option<uuid::Uuid>,
    pub grant_reason: String,
    pub granted_at: chrono::DateTime<chrono::Utc>,
}

super::impl_permission_assignment!(
    UserPermission,
    UserPermissionIden,
    ResourceType::Organization,
    user_id,
    User,
    UserId
);
super::impl_permission_assignment!(
    GroupPermission,
    GroupPermissionIden,
    ResourceType::Organization,
    group_id,
    Group,
    GroupId
);

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    AsRefStr,
    Display,
    EnumString,
    EnumIter,
)]
pub enum Action {
    #[serde(rename = "organization_read")]
    #[strum(serialize = "organization_read")]
    Read,
    #[serde(rename = "organization_update")]
    #[strum(serialize = "organization_update")]
    Update,
    #[serde(rename = "organization_delete")]
    #[strum(serialize = "organization_delete")]
    Delete,
    #[serde(rename = "list_permission")]
    #[strum(serialize = "list_permission")]
    ListPermission,
    #[serde(rename = "grant_permission")]
    #[strum(serialize = "grant_permission")]
    GrantPermission,
    #[serde(rename = "revoke_permission")]
    #[strum(serialize = "revoke_permission")]
    RevokePermission,
    #[serde(rename = "organization_add_member")]
    #[strum(serialize = "organization_add_member")]
    AddMember,
    #[serde(rename = "organization_remove_member")]
    #[strum(serialize = "organization_remove_member")]
    RemoveMember,
}

impl PermissionAction for Action {
    const RESOURCE_TYPE: ResourceType = ResourceType::Organization;
    type Role = Role;
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    AsRefStr,
    Display,
    EnumString,
    EnumIter,
    IntoStaticStr,
)]
pub enum Role {
    #[serde(rename = "organization_admin")]
    #[strum(serialize = "organization_admin")]
    Admin,
}

impl PermissionRole for Role {
    type Action = Action;

    fn actions(self) -> &'static [Action] {
        static ALL_ACTIONS: std::sync::LazyLock<Vec<Action>> =
            std::sync::LazyLock::new(|| Action::iter().collect());

        match self {
            Self::Admin => ALL_ACTIONS.as_slice(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn administrators_manage_organization_permissions() {
        assert_eq!(Role::Admin.as_ref(), "organization_admin");
        assert!(Role::Admin.allows(Action::GrantPermission));
        assert!(Role::Admin.allows(Action::RevokePermission));
        assert!(Role::Admin.allows(Action::RemoveMember));
        assert_eq!(Action::RESOURCE_TYPE, ResourceType::Organization);
    }
}
