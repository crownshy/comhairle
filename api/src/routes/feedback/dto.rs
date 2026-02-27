use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::feedback::Feedback;

/// Data transfer object (public API representation) for a Feedback instance.
///
/// This DTO is returned by feedback related endpoints and is safe to expose to
/// clients. It intentionally omits fields such as:
///
/// * `created_by`
/// * `created_at`
/// * `updated_at`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackDto {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
}

impl From<Feedback> for FeedbackDto {
    fn from(f: Feedback) -> Self {
        Self {
            id: f.id,
            conversation_id: f.conversation_id,
            content: f.content,
        }
    }
}
