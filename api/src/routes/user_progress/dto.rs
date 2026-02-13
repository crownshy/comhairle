use schemars::JsonSchema;
use serde::Serialize;
use uuid::Uuid;

use crate::models::user_progress::{ProgressStatus, UserProgress};

/// Data transfer object (public API representation) for UserProgress
///
/// This DTO is returned by user progress related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `created_at`
/// * `updated_at`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserProgressDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub workflow_step_id: Uuid,
    pub status: ProgressStatus,
}

impl From<UserProgress> for UserProgressDto {
    fn from(p: UserProgress) -> Self {
        Self {
            id: p.id,
            user_id: p.user_id,
            workflow_step_id: p.workflow_step_id,
            status: p.status,
        }
    }
}
