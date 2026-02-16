use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::workflow::Workflow;

/// Data transfer object (public API representation) for a Workflow.
///
/// This DTO is returned by user worflow related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `owner_id`
/// * `updated_at`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowDto {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub name: String,
    pub description: String,
    pub is_active: bool,
    pub is_public: bool,
    pub auto_login: bool,
    pub created_at: DateTime<Utc>,
}

impl From<Workflow> for WorkflowDto {
    fn from(w: Workflow) -> Self {
        Self {
            id: w.id,
            conversation_id: w.conversation_id,
            name: w.name,
            description: w.description,
            is_active: w.is_active,
            is_public: w.is_public,
            auto_login: w.auto_login,
            created_at: w.created_at,
        }
    }
}
