//! Tool configuration data.
//!
//! Every tool has three persisted shapes: its `Config` (stored on
//! `workflow_step.tool_config`), its `Setup` (the creation payload), and its
//! `Report`. Those are plain serde types and live here, in the model layer.
//!
//! The *behaviour* — `ToolImpl`, route registration, and the state-taking
//! operations on these enums — stays in [`crate::tools`]. Note that the
//! operations there are extension traits rather than inherent impls: once this
//! module becomes its own crate, `impl ToolConfig { .. }` in `api` would be an
//! inherent impl on a foreign type, which Rust does not allow.

pub mod elicitation_bot;
pub mod heyform;
pub mod learn;
pub mod polis;
pub mod prioritization;
pub mod stories;
pub mod thinking_space;

use comhairle_macros::{DbJsonBEnum, TranslatableJson};
use enum_dispatch::enum_dispatch;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::translations::TextContentId;

use elicitation_bot::{ElicitationBotReport, ElicitationBotToolConfig, ElicitationBotToolSetup};
use heyform::{HeyFormReport, HeyFormToolConfig, HeyFormToolSetup};
use learn::{LearnReport, LearnToolConfig, LearnToolSetup};
use polis::{PolisReport, PolisToolConfig, PolisToolSetup};
use prioritization::{PrioritizationReport, PrioritizationToolConfig, PrioritizationToolSetup};
use stories::{StoriesReport, StoriesToolConfig, StoriesToolSetup};
use thinking_space::{ThinkingSpaceReport, ThinkingSpaceToolConfig, ThinkingSpaceToolSetup};

/// Strips secrets out of a tool config before it is handed back over the API.
#[enum_dispatch]
pub trait ToolConfigSanitize {
    fn sanitize(&self) -> Self;
}

#[derive(
    Clone, Deserialize, Serialize, Debug, JsonSchema, DbJsonBEnum, PartialEq, TranslatableJson,
)]
#[serde(rename_all = "lowercase", tag = "type")]
#[enum_dispatch(ToolConfigSanitize)]
pub enum ToolConfig {
    Polis(PolisToolConfig),
    Learn(LearnToolConfig),
    HeyForm(HeyFormToolConfig),
    Stories(StoriesToolConfig),
    ElicitationBot(ElicitationBotToolConfig),
    #[translatable]
    Prioritization(PrioritizationToolConfig),
    #[translatable]
    ThinkingSpace(ThinkingSpaceToolConfig),
}

impl ToolConfig {
    /// The moderation policy the step points at. Only Polis moderates statements today.
    pub fn moderation_policy_id(&self) -> Option<Uuid> {
        match self {
            ToolConfig::Polis(config) => config.moderation_policy_id,
            _ => None,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum ToolSetup {
    Polis(PolisToolSetup),
    Learn(LearnToolSetup),
    HeyForm(HeyFormToolSetup),
    Stories(StoriesToolSetup),
    ElicitationBot(ElicitationBotToolSetup),
    Prioritization(PrioritizationToolSetup),
    ThinkingSpace(ThinkingSpaceToolSetup),
}

#[derive(PartialEq, Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub enum ReportConfig {
    Polis(PolisReport),
    HeyForm(HeyFormReport),
    Learn(LearnReport),
    Stories(StoriesReport),
    ElicitationBot(ElicitationBotReport),
    Prioritization(PrioritizationReport),
    ThinkingSpace(ThinkingSpaceReport),
}
