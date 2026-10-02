use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
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
    #[serde(rename = "conversation_launch")]
    #[strum(serialize = "conversation_launch")]
    Launch,
    #[serde(rename = "conversation_delete")]
    #[strum(serialize = "conversation_delete")]
    Delete,
    #[serde(rename = "conversation_moderate")]
    #[strum(serialize = "conversation_moderate")]
    Moderate,
    #[serde(rename = "conversation_translate")]
    #[strum(serialize = "conversation_translate")]
    Translate,
    #[serde(rename = "conversation_export")]
    #[strum(serialize = "conversation_export")]
    Export,
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
    #[serde(rename = "admin")]
    #[strum(serialize = "admin")]
    Admin,
    #[serde(rename = "observer")]
    #[strum(serialize = "observer")]
    Observer,
    #[serde(rename = "content_editor")]
    #[strum(serialize = "content_editor")]
    ContentEditor,
    #[serde(rename = "moderator")]
    #[strum(serialize = "moderator")]
    Moderator,
    #[serde(rename = "translator")]
    #[strum(serialize = "translator")]
    Translator,
    #[serde(rename = "data_access")]
    #[strum(serialize = "data_access")]
    DataAccess,
}

impl PermissionRole for Role {
    type Action = Action;

    fn actions(self) -> &'static [Action] {
        static ALL_ACTIONS: std::sync::LazyLock<Vec<Action>> =
            std::sync::LazyLock::new(|| Action::iter().collect());

        match self {
            Self::Admin => ALL_ACTIONS.as_slice(),
            Self::ContentEditor => &[Action::Read, Action::Update],
            Self::Observer => &[Action::Read],
            Self::Moderator => &[Action::Read, Action::Moderate],
            Self::Translator => &[Action::Read, Action::Translate],
            Self::DataAccess => &[Action::Read, Action::Export],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_conversation_policy() {
        assert_eq!(Role::ContentEditor.as_ref(), "content_editor");
        assert_eq!(Role::Observer.as_ref(), "observer");
        assert!(Role::ContentEditor.allows(Action::Read));
        assert!(Role::ContentEditor.allows(Action::Update));
        assert!(!Role::ContentEditor.allows(Action::GrantPermission));
        assert!(Role::Observer.allows(Action::Read));
        assert!(!Role::Observer.allows(Action::Update));
        assert_eq!(Action::RESOURCE_TYPE, ResourceType::Conversation);
    }

    #[test]
    fn conversation_roles_have_separate_capabilities() {
        use strum::IntoEnumIterator;

        for role in Role::iter() {
            for action in Action::iter() {
                let expected = match role {
                    Role::Admin => true,
                    Role::Observer => action == Action::Read,
                    Role::ContentEditor => matches!(action, Action::Read | Action::Update),
                    Role::Moderator => matches!(action, Action::Read | Action::Moderate),
                    Role::Translator => matches!(action, Action::Read | Action::Translate),
                    Role::DataAccess => matches!(action, Action::Read | Action::Export),
                };
                assert_eq!(role.allows(action), expected, "{role}: {action}");
            }
        }
    }
}
