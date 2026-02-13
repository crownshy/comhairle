use std::sync::Arc;

use aide::axum::ApiRouter;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{error::ComhairleError, ComhairleState};

use super::{ToolConfigSanitize, ToolImpl};

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct ElicitationBotToolConfig {
    pub topic: String,
}

impl ToolConfigSanitize for ElicitationBotToolConfig {
    fn sanatize(&self) -> Self {
        self.clone()
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct ElicitationBotToolSetup {
    pub topic: String,
    pub conversation_id: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct ElicitationBotReport;

async fn elicitation_bot_setup(
    config: &ElicitationBotToolSetup,
) -> Result<ElicitationBotToolConfig, ComhairleError> {
    Ok(ElicitationBotToolConfig {
        topic: config.topic.clone(),
    })
}

// Keep public function for backwards compatibility
pub async fn setup(
    config: &ElicitationBotToolSetup,
) -> Result<ElicitationBotToolConfig, ComhairleError> {
    elicitation_bot_setup(config).await
}

/// Zero-sized marker type for ElicitationBot tool implementation
pub struct ElicitationBotTool;

#[async_trait]
impl ToolImpl for ElicitationBotTool {
    type Config = ElicitationBotToolConfig;
    type Setup = ElicitationBotToolSetup;
    type Report = ElicitationBotReport;

    async fn setup(
        setup: &Self::Setup,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        elicitation_bot_setup(setup).await
    }

    async fn clone_tool(
        config: &Self::Config,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        // ElicitationBot tool is cloneable as-is
        Ok(config.clone())
    }

    fn sanitize(config: Self::Config) -> Self::Config {
        config.sanatize()
    }

    fn routes(_state: &Arc<ComhairleState>) -> ApiRouter {
        // ElicitationBot tool has no routes
        ApiRouter::new()
    }
}
