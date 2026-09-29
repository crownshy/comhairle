use std::sync::Arc;

use aide::axum::ApiRouter;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Serialize, de::DeserializeOwned};
use uuid::Uuid;

use crate::{ComhairleState, error::ComhairleError};

pub mod elicitation_bot;
pub mod heyform;
pub use crate::models::id;
pub mod learn;
pub mod polis;
pub mod prioritization;
pub mod stories;
pub mod thinking_space;

pub use crate::models::tools::{
    LocalizedToolConfig, ToolConfig, ToolConfigSanitize, ToolConfigWithTranslations, ToolSetup,
};

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
        locale: &str,
    ) -> Result<Self::Config, ComhairleError>;

    /// Sync data from tool to common data pool. Not wired up to a caller yet;
    /// part of the tool contract so the allow keeps it until it is.
    #[allow(dead_code)]
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
    fn routes(state: &Arc<ComhairleState>) -> ApiRouter {
        // Default: no routes
        let _ = state;
        ApiRouter::new()
    }

    /// Register background workers/tasks. Not wired up to a caller yet either.
    #[allow(dead_code)]
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

/// State-taking operations on [`ToolConfig`].
///
/// An extension trait rather than an inherent impl because `ToolConfig` is model
/// data: once the model layer becomes its own crate, an inherent impl here would
/// be on a foreign type, which Rust does not allow.
#[async_trait]
pub trait ToolConfigExt: Sized {
    /// Sync data from tool to common data pool. No caller yet; kept as part of
    /// the tool contract.
    #[allow(dead_code)]
    async fn sync_data(&self, state: &Arc<ComhairleState>) -> Result<(), ComhairleError>;

    /// Clone tool to create new instance (used for launch)
    async fn clone_tool(&self, state: &Arc<ComhairleState>) -> Result<Self, ComhairleError>;

    /// Delete tool and clean up resources
    async fn delete(
        &self,
        state: &Arc<ComhairleState>,
        workflow_step_id: &Uuid,
    ) -> Result<(), ComhairleError>;

    /// Register background workers for this tool config. No caller yet; kept as
    /// part of the tool contract.
    #[allow(dead_code)]
    async fn register_workers(&self, state: &Arc<ComhairleState>) -> Result<(), ComhairleError>;
}

#[async_trait]
impl ToolConfigExt for ToolConfig {
    async fn sync_data(&self, state: &Arc<ComhairleState>) -> Result<(), ComhairleError> {
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

    async fn clone_tool(&self, state: &Arc<ComhairleState>) -> Result<Self, ComhairleError> {
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

    async fn delete(
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

    async fn register_workers(&self, state: &Arc<ComhairleState>) -> Result<(), ComhairleError> {
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
}

/// State-taking operations on [`ToolSetup`]; extension trait for the same
/// foreign-type reason as [`ToolConfigExt`].
#[async_trait]
pub trait ToolSetupExt {
    /// Setup a new tool from setup configuration
    async fn setup(
        &self,
        state: &Arc<ComhairleState>,
        locale: &str,
    ) -> Result<ToolConfig, ComhairleError>;
}

#[async_trait]
impl ToolSetupExt for ToolSetup {
    async fn setup(
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
pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .merge(polis::PolisTool::routes(&state))
        .merge(learn::LearnTool::routes(&state))
        .merge(heyform::HeyFormTool::routes(&state))
        .merge(stories::StoriesTool::routes(&state))
        .merge(elicitation_bot::ElicitationBotTool::routes(&state))
        .merge(prioritization::PrioritizationTool::routes(&state))
        .merge(thinking_space::ThinkingSpaceTool::routes(&state))
}
