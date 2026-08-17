use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{
        translations::TextContentId,
        user_progress::ProgressStatus,
        workflow_step::{
            ActivationRule, LocalizedWorkflowStep, LocalizedWorkflowStepWithProgress, WorkflowStep,
        },
    },
    schema_helpers::{example_localized_text, example_uuid},
    tools::ToolConfig,
};

/// Data transfer object (public API representation) for a WorkflowStep.
///
/// This DTO is returned by workflow step related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `created_at`
/// * `updated_at`
///
/// It includes raw `uuid` values for translatable fields:
///
/// * `name`
/// * `description`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStepDto {
    #[schemars(example = "example_uuid")]
    pub id: Uuid,
    #[schemars(example = "example_uuid")]
    pub workflow_id: Uuid,
    #[schemars(example = "example_uuid")]
    pub name: TextContentId,
    pub step_order: i32,
    pub activation_rule: ActivationRule,
    #[schemars(example = "example_uuid")]
    pub description: TextContentId,
    pub is_offline: bool,
    pub required: bool,
    pub can_revisit: bool,
    pub tool_config: Option<ToolConfig>,
    pub preview_tool_config: ToolConfig,
    pub request_user_share_permission: bool,
}

/// Data transfer object (public API representation) for a LocalizedWorkflowStep.
///
/// This DTO is returned by workflow step related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `created_at`
/// * `updated_at`
///
/// It includes localized `String` values for translatable fields:
///
/// * `name`
/// * `description`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalizedWorkflowStepDto {
    #[schemars(example = "example_uuid")]
    pub id: Uuid,
    #[schemars(example = "example_uuid")]
    pub workflow_id: Uuid,
    #[schemars(example = "example_localized_text")]
    pub name: String,
    pub step_order: i32,
    pub activation_rule: ActivationRule,
    #[schemars(example = "example_localized_text")]
    pub description: String,
    pub is_offline: bool,
    pub required: bool,
    pub can_revisit: bool,
    pub tool_config: Option<ToolConfig>,
    pub preview_tool_config: ToolConfig,
    pub request_user_share_permission: bool,
}

/// Data transfer object (public API representation) for a LocalizedWorkflowStepWithProgress.
/// It represents a `workflow_step` row with localized fields and additionally includes
/// the active user's progress status for the step for convenience on the frontend.
///
/// This DTO is returned by workflow step related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `created_at`
/// * `updated_at`
///
/// It includes localized `String` values for translatable fields:
///
/// * `name`
/// * `description`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalizedWorkflowStepWithProgressDto {
    #[schemars(example = "example_uuid")]
    pub id: Uuid,
    #[schemars(example = "example_uuid")]
    pub workflow_id: Uuid,
    #[schemars(example = "example_localized_text")]
    pub name: String,
    pub step_order: i32,
    pub activation_rule: ActivationRule,
    #[schemars(example = "example_localized_text")]
    pub description: String,
    pub is_offline: bool,
    pub required: bool,
    pub can_revisit: bool,
    pub tool_config: Option<ToolConfig>,
    pub preview_tool_config: ToolConfig,
    pub progress_status: ProgressStatus,
    pub request_user_share_permission: bool,
    /// Is this participant sealed out of the workflow: have they finished (every step done)
    /// in a conversation whose `allow_revisit_after_finishing` is off? See ADR-0016.
    ///
    /// A property of the participant's relationship to the *workflow*, so it carries the same
    /// value on every step in the response. It rides here rather than on `WorkflowDto` because
    /// the workflow list route has no authenticated user to compute it for, and here rather
    /// than being derived in the frontend so that the seal has exactly one definition, shared
    /// with the write gates that enforce it.
    pub sealed: bool,
}

impl From<WorkflowStep> for WorkflowStepDto {
    fn from(w: WorkflowStep) -> Self {
        Self {
            id: w.id,
            workflow_id: w.workflow_id,
            name: w.name,
            step_order: w.step_order,
            activation_rule: w.activation_rule,
            description: w.description,
            is_offline: w.is_offline,
            required: w.required,
            can_revisit: w.can_revisit,
            tool_config: w.tool_config,
            preview_tool_config: w.preview_tool_config,
            request_user_share_permission: w.request_user_share_permission,
        }
    }
}

impl From<LocalizedWorkflowStep> for LocalizedWorkflowStepDto {
    fn from(w: LocalizedWorkflowStep) -> Self {
        Self {
            id: w.id,
            workflow_id: w.workflow_id,
            name: w.name,
            step_order: w.step_order,
            activation_rule: w.activation_rule,
            description: w.description,
            is_offline: w.is_offline,
            required: w.required,
            can_revisit: w.can_revisit,
            tool_config: w.tool_config,
            preview_tool_config: w.preview_tool_config,
            request_user_share_permission: w.request_user_share_permission,
        }
    }
}

impl LocalizedWorkflowStepWithProgressDto {
    /// Deliberately a named constructor rather than a `From` impl: `sealed` cannot be derived
    /// from the step alone, and a `From` would let a caller build one of these without ever
    /// considering the seal.
    pub fn from_with_seal(w: LocalizedWorkflowStepWithProgress, sealed: bool) -> Self {
        Self {
            sealed,
            id: w.step.id,
            workflow_id: w.step.workflow_id,
            name: w.step.name,
            step_order: w.step.step_order,
            activation_rule: w.step.activation_rule,
            description: w.step.description,
            is_offline: w.step.is_offline,
            required: w.step.required,
            can_revisit: w.step.can_revisit,
            tool_config: w.step.tool_config,
            preview_tool_config: w.step.preview_tool_config,
            request_user_share_permission: w.step.request_user_share_permission,
            progress_status: w.status,
        }
    }
}
