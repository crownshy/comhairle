use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

use super::{PermissionAction, PermissionRole, ResourceType};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, sqlx::FromRow)]
#[sea_query::enum_def(table_name = "conversation_user_permissions")]
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
#[sea_query::enum_def(table_name = "conversation_group_permissions")]
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
    ResourceType::Conversation,
    user_id,
    User,
    UserId
);
super::impl_permission_assignment!(
    GroupPermission,
    GroupPermissionIden,
    ResourceType::Conversation,
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
    #[serde(rename = "conversation_read")]
    #[strum(serialize = "conversation_read")]
    Read,
    #[serde(rename = "conversation_update")]
    #[strum(serialize = "conversation_update")]
    Update,
    #[serde(rename = "conversation_admin")]
    #[strum(serialize = "conversation_admin")]
    Admin,
    #[serde(rename = "list_permission")]
    #[strum(serialize = "list_permission")]
    ListPermission,
    #[serde(rename = "grant_permission")]
    #[strum(serialize = "grant_permission")]
    GrantPermission,
    #[serde(rename = "revoke_permission")]
    #[strum(serialize = "revoke_permission")]
    RevokePermission,
}

impl PermissionAction for Action {
    const RESOURCE_TYPE: ResourceType = ResourceType::Conversation;
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
    #[serde(rename = "content_editor")]
    #[strum(serialize = "content_editor")]
    ContentEditor,
    #[serde(rename = "conversation_co_host")]
    #[strum(serialize = "conversation_co_host")]
    CoHost,
}

impl PermissionRole for Role {
    type Action = Action;

    fn actions(self) -> &'static [Action] {
        match self {
            Self::ContentEditor => &[Action::Read, Action::Update],
            Self::CoHost => &[Action::Read],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_conversation_policy() {
        assert_eq!(Role::ContentEditor.as_ref(), "content_editor");
        assert_eq!(Role::CoHost.as_ref(), "conversation_co_host");
        assert!(Role::ContentEditor.allows(Action::Read));
        assert!(Role::ContentEditor.allows(Action::Update));
        assert!(!Role::ContentEditor.allows(Action::GrantPermission));
        assert!(Role::CoHost.allows(Action::Read));
        assert!(!Role::CoHost.allows(Action::Update));
        assert_eq!(Action::RESOURCE_TYPE, ResourceType::Conversation);
    }
}
