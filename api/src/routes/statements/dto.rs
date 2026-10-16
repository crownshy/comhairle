use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::statement::Statement;

/// Data transfer object (public API representation) for a statement.
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatementDto {
    pub id: Uuid,
    pub workflow_step_id: Uuid,
    pub user_id: Option<Uuid>,
    pub content: String,
    pub ai_generated: bool,
    pub context: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Statement> for StatementDto {
    fn from(statement: Statement) -> Self {
        Self {
            id: statement.id,
            workflow_step_id: statement.workflow_step_id,
            user_id: statement.user_id,
            content: statement.content,
            ai_generated: statement.ai_generated,
            context: statement.context,
            created_at: statement.created_at,
            updated_at: statement.updated_at,
        }
    }
}
