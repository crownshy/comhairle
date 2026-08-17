use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{
        translations::{
            LocalizeTranslations, ResolveWithTranslations, TextContentId, TranslationDto,
        },
        user_progress::ProgressStatus,
        workflow_step::{
            ActivationRule, LocalizedWorkflowStep, LocalizedWorkflowStepWithProgress, WorkflowStep,
            WorkflowStepTranslations, WorkflowStepWithTranslations,
        },
    },
    schema_helpers::{example_localized_text, example_uuid},
    tools::{LocalizedToolConfig, ToolConfig, ToolConfigWithTranslations},
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
    pub tool_config: Option<LocalizedToolConfig>,
    pub preview_tool_config: LocalizedToolConfig,
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
    pub tool_config: Option<LocalizedToolConfig>,
    pub preview_tool_config: LocalizedToolConfig,
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

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStepWithTranslationsDto {
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
    pub request_user_share_permission: bool,
    pub translations: WorkflowStepTranslations,
    pub tool_config: Option<ToolConfigWithTranslations>,
    pub preview_tool_config: ToolConfigWithTranslations,
}

impl WorkflowStepWithTranslations {
    pub fn into_dto(
        self,
        tool_config_translations: &HashMap<TextContentId, TranslationDto>,
        locale: &str,
    ) -> WorkflowStepWithTranslationsDto {
        WorkflowStepWithTranslationsDto {
            id: self.id,
            workflow_id: self.workflow_id,
            name: self.name,
            step_order: self.step_order,
            activation_rule: self.activation_rule,
            description: self.description,
            is_offline: self.is_offline,
            required: self.required,
            can_revisit: self.can_revisit,
            request_user_share_permission: self.request_user_share_permission,
            translations: self.translations,
            tool_config: self
                .tool_config
                .map(|tc| tc.resolve_with_translations(tool_config_translations, locale)),
            preview_tool_config: self
                .preview_tool_config
                .resolve_with_translations(tool_config_translations, locale),
        }
    }
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

impl LocalizedWorkflowStep {
    pub fn into_dto(
        self,
        translations_map: &HashMap<TextContentId, String>,
    ) -> LocalizedWorkflowStepDto {
        LocalizedWorkflowStepDto {
            id: self.id,
            workflow_id: self.workflow_id,
            name: self.name,
            step_order: self.step_order,
            activation_rule: self.activation_rule,
            description: self.description,
            is_offline: self.is_offline,
            required: self.required,
            can_revisit: self.can_revisit,
            tool_config: self.tool_config.map(|tc| tc.localize(translations_map)),
            preview_tool_config: self.preview_tool_config.localize(translations_map),
            request_user_share_permission: self.request_user_share_permission,
        }
    }
}

impl LocalizedWorkflowStepWithProgress {
    /// `sealed` is taken as an argument rather than derived here: it is a property of the
    /// participant's relationship to the whole workflow, not of this step, so only the caller
    /// (which has the user and the db) can work it out. Making it a required parameter rather
    /// than defaulting it means a caller cannot build one of these without considering the
    /// seal.
    pub fn into_dto(
        self,
        translations_map: &HashMap<TextContentId, String>,
        sealed: bool,
    ) -> LocalizedWorkflowStepWithProgressDto {
        LocalizedWorkflowStepWithProgressDto {
            sealed,
            id: self.step.id,
            workflow_id: self.step.workflow_id,
            name: self.step.name,
            step_order: self.step.step_order,
            activation_rule: self.step.activation_rule,
            description: self.step.description,
            is_offline: self.step.is_offline,
            required: self.step.required,
            can_revisit: self.step.can_revisit,
            tool_config: self
                .step
                .tool_config
                .map(|tc| tc.localize(translations_map)),
            preview_tool_config: self.step.preview_tool_config.localize(translations_map),
            request_user_share_permission: self.step.request_user_share_permission,
            progress_status: self.status,
        }
    }
}
