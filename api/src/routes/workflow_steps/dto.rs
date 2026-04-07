use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{
        translations::TextContentId,
        user_progress::ProgressStatus,
        workflow_step::{
            ActivationRule, CreateWorkflowStep, LocalizedWorkflowStep,
            LocalizedWorkflowStepWithProgress, WorkflowStep,
        },
    },
    schema_helpers::{example_localized_text, example_uuid},
    tools::{
        elicitation_bot::{ElicitationBotToolConfig, ElicitationBotToolSetup},
        heyform::HeyFormToolSetup,
        learn::{LearnToolConfig, LearnToolSetup},
        polis::PolisToolSetup,
        stories::StoriesToolSetup,
        ToolConfig, ToolSetup,
    },
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
}

impl From<LocalizedWorkflowStepWithProgress> for LocalizedWorkflowStepWithProgressDto {
    fn from(w: LocalizedWorkflowStepWithProgress) -> Self {
        Self {
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
            progress_status: w.status,
        }
    }
}

/// Data transfer object (public API representation) for importing / exporting
/// of a workflow_step.
///
/// This DTO is returned by workflow step related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `id`
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
pub struct ImportExportWorkflowStepDto {
    #[schemars(example = "example_localized_text")]
    pub name: String,
    pub step_order: i32,
    pub activation_rule: ActivationRule,
    #[schemars(example = "example_localized_text")]
    pub description: String,
    pub is_offline: bool,
    pub required: bool,
    pub can_revisit: bool,
    pub preview_tool_config: ImportExportToolConfig,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ImportExportToolConfig {
    Polis,
    Learn(LearnToolConfig),
    HeyForm,
    Stories,
    ElicitationBot(ElicitationBotToolConfig),
}

impl From<ImportExportToolConfig> for ToolSetup {
    fn from(c: ImportExportToolConfig) -> Self {
        match c {
            ImportExportToolConfig::Learn(config) => Self::Learn(LearnToolSetup {
                pages: config.pages,
            }),
            ImportExportToolConfig::Polis => Self::Polis(PolisToolSetup {
                topic: "".to_string(),
                required_votes: None,
            }),
            ImportExportToolConfig::HeyForm => Self::HeyForm(HeyFormToolSetup {
                server_url: "forms.comhairle.scot".to_string(), // TODO:
            }),
            ImportExportToolConfig::ElicitationBot(config) => {
                Self::ElicitationBot(ElicitationBotToolSetup {
                    topic: config.topic,
                })
            }
            ImportExportToolConfig::Stories => Self::Stories(StoriesToolSetup {
                max_time: 10,
                to_see: 3,
            }),
        }
    }
}

impl From<ImportExportWorkflowStepDto> for CreateWorkflowStep {
    fn from(s: ImportExportWorkflowStepDto) -> Self {
        Self {
            name: s.name,
            step_order: s.step_order,
            activation_rule: s.activation_rule,
            description: s.description,
            is_offline: s.is_offline,
            required: s.required,
            can_revisit: s.can_revisit,
            tool_setup: s.preview_tool_config.into(),
        }
    }
}
