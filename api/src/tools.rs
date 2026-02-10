use std::sync::Arc;

use aide::axum::ApiRouter;
use async_trait::async_trait;
use comhairle_macros::DbJsonBEnum;
use enum_dispatch::enum_dispatch;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{error::ComhairleError, ComhairleState};

pub mod elicitation_bot;
pub mod heyform;
pub mod id;
pub mod learn;
pub mod polis;
pub mod stories;

use elicitation_bot::{ElicitationBotReport, ElicitationBotToolConfig, ElicitationBotToolSetup};
use heyform::{HeyFormReport, HeyFormToolConfig, HeyFormToolSetup};
use learn::{LearnReport, LearnToolConfig, LearnToolSetup};
use polis::{PolisReport, PolisToolConfig, PolisToolSetup};
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

    /// Create and configure a new tool instance
    async fn setup(
        setup: &Self::Setup,
        state: &Arc<ComhairleState>,
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
    ) -> Result<(), ComhairleError> {
        // Default: no-op
        let _ = (config, state);
        Ok(())
    }

    /// Register HTTP routes for this tool
    fn routes(state: &Arc<ComhairleState>) -> ApiRouter {
        // Default: no routes
        let _ = state;
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

    /// Sanitize config by removing sensitive data
    fn sanitize(config: Self::Config) -> Self::Config;
}

/// Trait for sync operations that can be dispatched via enum_dispatch
#[enum_dispatch]
pub trait ToolConfigSanitize {
    fn sanatize(&self) -> Self;
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, DbJsonBEnum, PartialEq)]
#[serde(rename_all = "lowercase", tag = "type")]
#[enum_dispatch(ToolConfigSanitize)]
pub enum ToolConfig {
    Polis(PolisToolConfig),
    Learn(LearnToolConfig),
    HeyForm(HeyFormToolConfig),
    Stories(StoriesToolConfig),
    ElicitationBot(ElicitationBotToolConfig),
}

impl ToolConfig {
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
        }
    }

    /// Clone tool to create new instance (used for launch)
    pub async fn clone_tool(&self, state: &Arc<ComhairleState>) -> Result<Self, ComhairleError> {
        match self {
            ToolConfig::Polis(config) => {
                Ok(ToolConfig::Polis(polis::PolisTool::clone_tool(config, state).await?))
            }
            ToolConfig::Learn(config) => {
                Ok(ToolConfig::Learn(learn::LearnTool::clone_tool(config, state).await?))
            }
            ToolConfig::HeyForm(config) => {
                Ok(ToolConfig::HeyForm(heyform::HeyFormTool::clone_tool(config, state).await?))
            }
            ToolConfig::Stories(config) => {
                Ok(ToolConfig::Stories(stories::StoriesTool::clone_tool(config, state).await?))
            }
            ToolConfig::ElicitationBot(config) => Ok(ToolConfig::ElicitationBot(
                elicitation_bot::ElicitationBotTool::clone_tool(config, state).await?,
            )),
        }
    }

    /// Delete tool and clean up resources
    pub async fn delete(&self, state: &Arc<ComhairleState>) -> Result<(), ComhairleError> {
        match self {
            ToolConfig::Polis(config) => polis::PolisTool::delete(config, state).await,
            ToolConfig::Learn(config) => learn::LearnTool::delete(config, state).await,
            ToolConfig::HeyForm(config) => heyform::HeyFormTool::delete(config, state).await,
            ToolConfig::Stories(config) => stories::StoriesTool::delete(config, state).await,
            ToolConfig::ElicitationBot(config) => {
                elicitation_bot::ElicitationBotTool::delete(config, state).await
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
}

impl ToolSetup {
    /// Setup a new tool from setup configuration
    pub async fn setup(&self, state: &Arc<ComhairleState>) -> Result<ToolConfig, ComhairleError> {
        match self {
            ToolSetup::Polis(setup) => {
                Ok(ToolConfig::Polis(polis::PolisTool::setup(setup, state).await?))
            }
            ToolSetup::Learn(setup) => {
                Ok(ToolConfig::Learn(learn::LearnTool::setup(setup, state).await?))
            }
            ToolSetup::HeyForm(setup) => {
                Ok(ToolConfig::HeyForm(heyform::HeyFormTool::setup(setup, state).await?))
            }
            ToolSetup::Stories(setup) => {
                Ok(ToolConfig::Stories(stories::StoriesTool::setup(setup, state).await?))
            }
            ToolSetup::ElicitationBot(setup) => Ok(ToolConfig::ElicitationBot(
                elicitation_bot::ElicitationBotTool::setup(setup, state).await?,
            )),
        }
    }
}

/// Register all tool routes
pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .merge(polis::PolisTool::routes(&state))
        .merge(learn::LearnTool::routes(&state))
        .merge(heyform::HeyFormTool::routes(&state))
        .merge(stories::StoriesTool::routes(&state))
        .merge(elicitation_bot::ElicitationBotTool::routes(&state))
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub enum ReportConfig {
    Polis(PolisReport),
    HeyForm(HeyFormReport),
    Learn(LearnReport),
    Stories(StoriesReport),
    ElicitationBot(ElicitationBotReport),
}
