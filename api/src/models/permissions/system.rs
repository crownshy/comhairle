use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

use super::{PermissionAction, PermissionRole, ResourceType};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, sqlx::FromRow)]
#[sea_query::enum_def(table_name = "system_user_permissions")]
pub struct UserPermission {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub role_name: String,
    pub granted_by: Option<uuid::Uuid>,
    pub grant_reason: String,
    pub granted_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, sqlx::FromRow)]
#[sea_query::enum_def(table_name = "system_group_permissions")]
pub struct GroupPermission {
    pub id: uuid::Uuid,
    pub group_id: uuid::Uuid,
    pub role_name: String,
    pub granted_by: Option<uuid::Uuid>,
    pub grant_reason: String,
    pub granted_at: chrono::DateTime<chrono::Utc>,
}

super::impl_permission_assignment!(
    UserPermission,
    UserPermissionIden,
    ResourceType::System,
    user_id,
    User,
    UserId,
    |_: &Self| super::SYSTEM_RESOURCE_ID
);
super::impl_permission_assignment!(
    GroupPermission,
    GroupPermissionIden,
    ResourceType::System,
    group_id,
    Group,
    GroupId,
    |_: &Self| super::SYSTEM_RESOURCE_ID
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
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Action {
    ListPermission,
    GrantPermission,
    RevokePermission,
    OrganizationCreate,
}

impl PermissionAction for Action {
    const RESOURCE_TYPE: ResourceType = ResourceType::System;
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
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Role {
    SuperAdmin,
    Admin,
}

impl PermissionRole for Role {
    type Action = Action;

    fn actions(self) -> &'static [Action] {
        match self {
            Self::SuperAdmin => &[
                Action::ListPermission,
                Action::GrantPermission,
                Action::RevokePermission,
                Action::OrganizationCreate,
            ],
            Self::Admin => &[],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_triplets_return_errors() {
        assert!(matches!(
            super::super::conversation::Role::ContentEditor.system_triplet(),
            Err(crate::error::ComhairleError::Unprocessable(_))
        ));
        let triplet = Role::SuperAdmin.system_triplet().unwrap();
        assert_eq!(triplet.0, "system");
        assert_eq!(*triplet.1, super::super::SYSTEM_RESOURCE_ID);
        assert_eq!(triplet.2, "super_admin");
    }

    #[test]
    fn preserves_system_role_names() {
        assert_eq!(Role::SuperAdmin.as_ref(), "super_admin");
        assert_eq!(Role::Admin.as_ref(), "admin");
        assert!(Role::SuperAdmin.allows(Action::GrantPermission));
        assert!(!Role::Admin.allows(Action::GrantPermission));
        assert_eq!(Action::RESOURCE_TYPE, ResourceType::System);
    }
}
