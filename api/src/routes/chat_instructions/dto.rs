use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::chat_instructions::ChatInstructions;

/// Data transfer object (public API representation) for a ChatInstructions record.
///
/// This DTO is returned by chat_instructions related endpoints and is safe to expose
/// to clients.
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatInstructionsDto {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub target_reading_age: Option<i32>,
    pub max_length: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ChatInstructions> for ChatInstructionsDto {
    fn from(ci: ChatInstructions) -> Self {
        Self {
            id: ci.id,
            conversation_id: ci.conversation_id,
            target_reading_age: ci.target_reading_age,
            max_length: ci.max_length,
            created_at: ci.created_at,
            updated_at: ci.updated_at,
        }
    }
}
