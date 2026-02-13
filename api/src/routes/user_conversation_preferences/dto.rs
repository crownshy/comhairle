use schemars::JsonSchema;
use serde::Serialize;
use uuid::Uuid;

use crate::models::user_conversation_preferences::UserConversationPreferences;

/// Data transfer object (public API representation) for UserConversationPreferences.
///
/// This DTO is returned by user conversation preferences routes.
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserConversationPreferencesDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub conversation_id: Uuid,
    pub receive_updates_by_notification: bool,
    pub receive_updates_by_email: bool,
    pub receive_similar_conversation_updates_by_email: bool,
    pub receive_similar_conversation_updates_by_notification: bool,
}

impl From<UserConversationPreferences> for UserConversationPreferencesDto {
    fn from(p: UserConversationPreferences) -> Self {
        Self {
            id: p.id,
            user_id: p.user_id,
            conversation_id: p.conversation_id,
            receive_updates_by_notification: p.receive_updates_by_notification,
            receive_updates_by_email: p.receive_updates_by_email,
            receive_similar_conversation_updates_by_email: p
                .receive_similar_conversation_updates_by_email,
            receive_similar_conversation_updates_by_notification: p
                .receive_similar_conversation_updates_by_notification,
        }
    }
}
