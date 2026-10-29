use std::sync::Arc;

use aide::axum::ApiRouter;
use async_trait::async_trait;
use comhairle_macros::{DbJsonBEnum, TranslatableJson};
use enum_dispatch::enum_dispatch;
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::models::translations::TextContentId;
use crate::tools::thinking_space::{
    ThinkingSpaceReport, ThinkingSpaceToolConfig, ThinkingSpaceToolSetup,
};

pub mod elicitation_bot;
pub mod heyform;
pub mod id;
pub mod learn;
pub mod polis;
pub mod prioritization;
pub mod stories;
pub mod thinking_space;

use elicitation_bot::{ElicitationBotReport, ElicitationBotToolConfig, ElicitationBotToolSetup};
use heyform::{HeyFormReport, HeyFormToolConfig, HeyFormToolSetup};
use learn::{LearnReport, LearnToolConfig, LearnToolSetup};
use polis::{PolisReport, PolisToolConfig, PolisToolSetup};
use prioritization::{PrioritizationReport, PrioritizationToolConfig, PrioritizationToolSetup};
use stories::{StoriesReport, StoriesToolConfig, StoriesToolSetup};

/// Core trait that all tools must implement.
///
/// Note: This trait is NOT object-safe due to associated types,
/// but we use enums with enum_dispatch for dynamic dispatch instead.
#[async_trait]
pub trait ToolImpl: Send + Sync + 'static {
    /// Tool-specific configuration type stored in database
    type Config: Clone + Serialize + DeserializeOwned + JsonSchema + Send + Sync + 'static;

    /// Tool-specific setup/creation parameters
    type Setup: Clone + Serialize + DeserializeOwned + JsonSchema + Send + Sync + 'static;

    /// Tool-specific report structure
    type Report: Serialize + DeserializeOwned + JsonSchema + Send + Sync + 'static;

    /// What a tool needs to recreate itself on import. Holds the content an admin built
    /// (e.g. survey questions) rather than references to remote resources, so an
    /// import creates new resources instead of sharing the original's.
    type ExportImport: Clone + Serialize + DeserializeOwned + JsonSchema + Send + Sync + 'static;

    /// Create and configure a new tool instance
    async fn setup(
        setup: &Self::Setup,
        state: &Arc<ComhairleState>,
        locale: &str,
    ) -> Result<Self::Config, ComhairleError>;

    /// Sync data from tool to common data pool
    async fn sync_data(
        config: &Self::Config,
        state: &Arc<ComhairleState>,
    ) -> Result<(), ComhairleError> {
        // Default: no-op
        let _ = (config, state);
        Ok(())
    }

    /// Clone tool to create new instance with same settings (used for launch)
    async fn clone_tool(
        config: &Self::Config,
        state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError>;

    /// Delete tool and clean up resources
    async fn delete(
        config: &Self::Config,
        state: &Arc<ComhairleState>,
        workflow_step_id: &Uuid,
    ) -> Result<(), ComhairleError> {
        // Default: no-op
        let _ = (config, state, workflow_step_id);
        Ok(())
    }

    /// Register HTTP routes for this tool
    fn routes() -> ApiRouter<Arc<ComhairleState>> {
        // Default: no routes
        ApiRouter::new()
    }

    /// Register background workers/tasks
    async fn register_workers(
        config: &Self::Config,
        state: &Arc<ComhairleState>,
    ) -> Result<(), ComhairleError> {
        // Default: no workers
        let _ = (config, state);
        Ok(())
    }

    /// Build the portable form of this tool for export. May read from the remote
    /// service, but must not create anything; new resources are created on import.
    async fn export_tool(config: &Self::Config) -> Result<Self::ExportImport, ComhairleError>;

    /// Sanitize config by removing sensitive data
    fn sanitize(config: Self::Config) -> Self::Config;
}

/// Trait for sync operations that can be dispatched via enum_dispatch
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

    /// Sync data from tool to common data pool
    pub async fn sync_data(&self, state: &Arc<ComhairleState>) -> Result<(), ComhairleError> {
        match self {
            ToolConfig::Polis(config) => polis::PolisTool::sync_data(config, state).await,
            ToolConfig::Learn(config) => learn::LearnTool::sync_data(config, state).await,
            ToolConfig::HeyForm(config) => heyform::HeyFormTool::sync_data(config, state).await,
            ToolConfig::Stories(config) => stories::StoriesTool::sync_data(config, state).await,
            ToolConfig::ElicitationBot(config) => {
                elicitation_bot::ElicitationBotTool::sync_data(config, state).await
            }
            ToolConfig::Prioritization(config) => {
                prioritization::PrioritizationTool::sync_data(config, state).await
            }
            ToolConfig::ThinkingSpace(config) => {
                thinking_space::ThinkingSpaceTool::sync_data(config, state).await
            }
        }
    }

    /// Clone tool to create new instance (used for launch)
    pub async fn clone_tool(&self, state: &Arc<ComhairleState>) -> Result<Self, ComhairleError> {
        match self {
            ToolConfig::Polis(config) => Ok(ToolConfig::Polis(
                polis::PolisTool::clone_tool(config, state).await?,
            )),
            ToolConfig::Learn(config) => Ok(ToolConfig::Learn(
                learn::LearnTool::clone_tool(config, state).await?,
            )),
            ToolConfig::HeyForm(config) => Ok(ToolConfig::HeyForm(
                heyform::HeyFormTool::clone_tool(config, state).await?,
            )),
            ToolConfig::Stories(config) => Ok(ToolConfig::Stories(
                stories::StoriesTool::clone_tool(config, state).await?,
            )),
            ToolConfig::ElicitationBot(config) => Ok(ToolConfig::ElicitationBot(
                elicitation_bot::ElicitationBotTool::clone_tool(config, state).await?,
            )),
            ToolConfig::Prioritization(config) => Ok(ToolConfig::Prioritization(
                prioritization::PrioritizationTool::clone_tool(config, state).await?,
            )),
            ToolConfig::ThinkingSpace(config) => Ok(ToolConfig::ThinkingSpace(
                thinking_space::ThinkingSpaceTool::clone_tool(config, state).await?,
            )),
        }
    }

    /// Delete tool and clean up resources
    pub async fn delete(
        &self,
        state: &Arc<ComhairleState>,
        workflow_step_id: &Uuid,
    ) -> Result<(), ComhairleError> {
        match self {
            ToolConfig::Polis(config) => {
                polis::PolisTool::delete(config, state, workflow_step_id).await
            }
            ToolConfig::Learn(config) => {
                learn::LearnTool::delete(config, state, workflow_step_id).await
            }
            ToolConfig::HeyForm(config) => {
                heyform::HeyFormTool::delete(config, state, workflow_step_id).await
            }
            ToolConfig::Stories(config) => {
                stories::StoriesTool::delete(config, state, workflow_step_id).await
            }
            ToolConfig::ElicitationBot(config) => {
                elicitation_bot::ElicitationBotTool::delete(config, state, workflow_step_id).await
            }
            ToolConfig::Prioritization(config) => {
                prioritization::PrioritizationTool::delete(config, state, workflow_step_id).await
            }
            ToolConfig::ThinkingSpace(config) => {
                thinking_space::ThinkingSpaceTool::delete(config, state, workflow_step_id).await
            }
        }
    }

    /// Register background workers for this tool config
    pub async fn register_workers(
        &self,
        state: &Arc<ComhairleState>,
    ) -> Result<(), ComhairleError> {
        match self {
            ToolConfig::Polis(config) => polis::PolisTool::register_workers(config, state).await,
            ToolConfig::Learn(config) => learn::LearnTool::register_workers(config, state).await,
            ToolConfig::HeyForm(config) => {
                heyform::HeyFormTool::register_workers(config, state).await
            }
            ToolConfig::Stories(config) => {
                stories::StoriesTool::register_workers(config, state).await
            }
            ToolConfig::ElicitationBot(config) => {
                elicitation_bot::ElicitationBotTool::register_workers(config, state).await
            }
            ToolConfig::Prioritization(config) => {
                prioritization::PrioritizationTool::register_workers(config, state).await
            }
            ToolConfig::ThinkingSpace(config) => {
                thinking_space::ThinkingSpaceTool::register_workers(config, state).await
            }
        }
    }

    pub async fn export_tool(&self) -> Result<ToolExIm, ComhairleError> {
        match self {
            ToolConfig::Polis(config) => Ok(ToolExIm::Polis(
                polis::PolisTool::export_tool(config).await?,
            )),
            ToolConfig::Learn(config) => Ok(ToolExIm::Learn(
                learn::LearnTool::export_tool(config).await?,
            )),
            ToolConfig::HeyForm(config) => Ok(ToolExIm::HeyForm(
                heyform::HeyFormTool::export_tool(config).await?,
            )),
            ToolConfig::Stories(config) => Ok(ToolExIm::Stories(
                stories::StoriesTool::export_tool(config).await?,
            )),
            ToolConfig::ElicitationBot(config) => Ok(ToolExIm::ElicitationBot(
                elicitation_bot::ElicitationBotTool::export_tool(config).await?,
            )),
            ToolConfig::Prioritization(config) => Ok(ToolExIm::Prioritization(
                prioritization::PrioritizationTool::export_tool(config).await?,
            )),
            ToolConfig::ThinkingSpace(config) => Ok(ToolExIm::ThinkingSpace(
                thinking_space::ThinkingSpaceTool::export_tool(config).await?,
            )),
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

impl ToolSetup {
    /// Setup a new tool from setup configuration
    pub async fn setup(
        &self,
        state: &Arc<ComhairleState>,
        locale: &str,
    ) -> Result<ToolConfig, ComhairleError> {
        match self {
            ToolSetup::Polis(setup) => Ok(ToolConfig::Polis(
                polis::PolisTool::setup(setup, state, locale).await?,
            )),
            ToolSetup::Learn(setup) => Ok(ToolConfig::Learn(
                learn::LearnTool::setup(setup, state, locale).await?,
            )),
            ToolSetup::HeyForm(setup) => Ok(ToolConfig::HeyForm(
                heyform::HeyFormTool::setup(setup, state, locale).await?,
            )),
            ToolSetup::Stories(setup) => Ok(ToolConfig::Stories(
                stories::StoriesTool::setup(setup, state, locale).await?,
            )),
            ToolSetup::ElicitationBot(setup) => Ok(ToolConfig::ElicitationBot(
                elicitation_bot::ElicitationBotTool::setup(setup, state, locale).await?,
            )),
            ToolSetup::Prioritization(setup) => Ok(ToolConfig::Prioritization(
                prioritization::PrioritizationTool::setup(setup, state, locale).await?,
            )),
            ToolSetup::ThinkingSpace(setup) => Ok(ToolConfig::ThinkingSpace(
                thinking_space::ThinkingSpaceTool::setup(setup, state, locale).await?,
            )),
        }
    }
}

/// Register all tool routes
pub fn router() -> ApiRouter<Arc<ComhairleState>> {
    ApiRouter::new()
        .merge(polis::PolisTool::routes())
        .merge(learn::LearnTool::routes())
        .merge(heyform::HeyFormTool::routes())
        .merge(stories::StoriesTool::routes())
        .merge(elicitation_bot::ElicitationBotTool::routes())
        .merge(prioritization::PrioritizationTool::routes())
        .merge(thinking_space::ThinkingSpaceTool::routes())
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

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum ToolExIm {
    Polis(<polis::PolisTool as ToolImpl>::ExportImport),
    HeyForm(<heyform::HeyFormTool as ToolImpl>::ExportImport),
    Learn(<learn::LearnTool as ToolImpl>::ExportImport),
    Stories(<stories::StoriesTool as ToolImpl>::ExportImport),
    ElicitationBot(<elicitation_bot::ElicitationBotTool as ToolImpl>::ExportImport),
    Prioritization(<prioritization::PrioritizationTool as ToolImpl>::ExportImport),
    ThinkingSpace(<thinking_space::ThinkingSpaceTool as ToolImpl>::ExportImport),
}
