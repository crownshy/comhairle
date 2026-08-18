use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{user_participation::UserParticipation, workflow::Workflow};

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
    pub conversation_id: Option<Uuid>,
    pub event_id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub is_active: bool,
    pub is_public: bool,
    pub auto_login: bool,
    pub region_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// A participant's standing on a workflow: their participation row, plus whether they are
/// sealed out of it.
///
/// `sealed` lives here rather than on [`WorkflowDto`] because it describes a person's
/// relationship to a workflow, not the workflow itself. `WorkflowDto` is also what the admin
/// create, get, update and delete handlers return, and those have no participant to evaluate
/// it for. This endpoint is already keyed by user and workflow, so it needs no extra context.
/// See ADR-0016.
///
/// Field names mirror [`UserParticipation`] rather than switching to camelCase: the register
/// and leave endpoints still return the model directly, and renaming here would make the
/// three disagree.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct UserParticipationDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub workflow_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Has this participant finished every step of a workflow whose conversation has
    /// `allow_revisit_after_finishing` off? Sealed participants are redirected to the thank
    /// you page and their step contribution writes are rejected.
    pub sealed: bool,
}

impl UserParticipation {
    /// `sealed` is taken as an argument rather than derived here, so that the one definition
    /// of the seal stays in `user_progress::is_sealed` alongside the write gates that enforce
    /// it. A required parameter also means a caller cannot build one of these without
    /// deciding what the seal is.
    pub fn into_dto(self, sealed: bool) -> UserParticipationDto {
        UserParticipationDto {
            id: self.id,
            user_id: self.user_id,
            workflow_id: self.workflow_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            sealed,
        }
    }
}

impl From<Workflow> for WorkflowDto {
    fn from(w: Workflow) -> Self {
        Self {
            id: w.id,
            conversation_id: w.conversation_id,
            event_id: w.event_id,
            name: w.name,
            description: w.description,
            is_active: w.is_active,
            is_public: w.is_public,
            auto_login: w.auto_login,
            region_id: w.region_id,
            created_at: w.created_at,
        }
    }
}
