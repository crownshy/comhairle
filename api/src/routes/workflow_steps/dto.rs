use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{
        translations::TextContentId,
        workflow_step::{ActivationRule, LocalizedWorkflowStep, WorkflowStep},
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
        }
    }
}
