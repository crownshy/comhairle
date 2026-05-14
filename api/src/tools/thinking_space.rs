use std::sync::Arc;

use aide::axum::ApiRouter;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{error::ComhairleError, ComhairleState};

use super::{ToolConfigSanitize, ToolImpl};

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct ThinkingSpaceQuestion {
    pub id: String,
    pub text: String,
}

fn default_questions() -> Vec<ThinkingSpaceQuestion> {
    vec![]
}

fn default_follow_up_count() -> u8 {
    2
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct ThinkingSpaceToolConfig {
    pub topic: String,
    #[serde(default = "default_questions")]
    pub questions: Vec<ThinkingSpaceQuestion>,
    #[serde(default = "default_follow_up_count")]
    pub follow_up_count: u8,
}

impl ToolConfigSanitize for ThinkingSpaceToolConfig {
    fn sanitize(&self) -> Self {
        self.clone()
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct ThinkingSpaceToolSetup {
    pub topic: String,
    #[serde(default = "default_questions")]
    pub questions: Vec<ThinkingSpaceQuestion>,
    #[serde(default = "default_follow_up_count")]
    pub follow_up_count: u8,
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct ThinkingSpaceReport;

async fn thinking_space_setup(
    config: &ThinkingSpaceToolSetup,
) -> Result<ThinkingSpaceToolConfig, ComhairleError> {
    Ok(ThinkingSpaceToolConfig {
        topic: config.topic.clone(),
        questions: config.questions.clone(),
        follow_up_count: config.follow_up_count,
    })
}

// Keep public function for backwards compatibility
pub async fn setup(
    config: &ThinkingSpaceToolSetup,
) -> Result<ThinkingSpaceToolConfig, ComhairleError> {
    thinking_space_setup(config).await
}

/// Zero-sized marker type for ThinkingSpace tool implementation
pub struct ThinkingSpaceTool;

#[async_trait]
impl ToolImpl for ThinkingSpaceTool {
    type Config = ThinkingSpaceToolConfig;
    type Setup = ThinkingSpaceToolSetup;
    type Report = ThinkingSpaceReport;

    async fn setup(
        setup: &Self::Setup,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        thinking_space_setup(setup).await
    }

    async fn clone_tool(
        config: &Self::Config,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        // ThinkingSpace tool is cloneable as-is
        Ok(config.clone())
    }

    fn sanitize(config: Self::Config) -> Self::Config {
        config.sanitize()
    }

    fn routes(_state: &Arc<ComhairleState>) -> ApiRouter {
        // ThinkingSpace tool has no dedicated routes yet
        ApiRouter::new()
    }
}
