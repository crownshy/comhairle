use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::workflow::{CreateWorkflow, Workflow},
    routes::workflow_steps::dto::ImexWorkflowStepDto,
};

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

/// Data transfer object (public API representation) for importing / exporting
/// of a workflow.
///
/// This DTO is returned by user worflow related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `id`
/// * `conversation_id`
/// * `event_id`
/// * `owner_id`
/// * `updated_at`
/// * `created_at`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImexWorkflowDto {
    pub name: String,
    pub description: String,
    pub is_active: bool,
    pub is_public: bool,
    pub auto_login: bool,
    pub region_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImexWorkflowWithStepsDto {
    #[serde(flatten)]
    pub workflow: ImexWorkflowDto,
    pub workflow_steps: Vec<ImexWorkflowStepDto>,
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

impl From<Workflow> for ImexWorkflowDto {
    fn from(w: Workflow) -> Self {
        Self {
            name: w.name,
            description: w.description,
            is_active: w.is_active,
            is_public: w.is_public,
            auto_login: w.auto_login,
            region_id: w.region_id,
        }
    }
}

impl From<ImexWorkflowDto> for CreateWorkflow {
    fn from(w: ImexWorkflowDto) -> Self {
        Self {
            name: w.name,
            description: w.description,
            is_active: w.is_active,
            is_public: w.is_public,
            auto_login: w.auto_login,
            region_id: w.region_id,
        }
    }
}
