use std::sync::Arc;

use aide::axum::ApiRouter;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{error::ComhairleError, ComhairleState};
use crate::models::translations::TextContentId;

use super::{ToolConfigSanitize, ToolImpl};

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct LearnPage {
    pub text_content_id: TextContentId,
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase", tag = "type", content = "content")]
pub enum PageContent {
    Markdown(String),
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct LocalisedPage {
    pub lang: String,
    #[serde(flatten)]
    pub content: PageContent,
    #[serde(default = "default_requires_validation")]
    pub requires_validation: bool,
}

fn default_requires_validation() -> bool {
    true
}

pub type Page = Vec<LocalisedPage>;

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum LearnPageEntry {
    TextContent(LearnPage),
    Legacy(Page),
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct LearnToolConfig {
    pub pages: Vec<LearnPageEntry>,
}

impl ToolConfigSanitize for LearnToolConfig {
    fn sanatize(&self) -> Self {
        self.clone()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct LearnReport;

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct LearnToolSetup {
    pub pages: Vec<LearnPageEntry>,
}

async fn learn_setup(setup_config: &LearnToolSetup) -> Result<LearnToolConfig, ComhairleError> {
    Ok(LearnToolConfig {
        pages: setup_config.pages.clone(),
    })
}

// Keep public function for backwards compatibility
pub async fn setup(setup_config: &LearnToolSetup) -> Result<LearnToolConfig, ComhairleError> {
    learn_setup(setup_config).await
}

/// Zero-sized marker type for Learn tool implementation
pub struct LearnTool;

#[async_trait]
impl ToolImpl for LearnTool {
    type Config = LearnToolConfig;
    type Setup = LearnToolSetup;
    type Report = LearnReport;

    async fn setup(
        setup: &Self::Setup,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        learn_setup(setup).await
    }

    async fn clone_tool(
        config: &Self::Config,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        // Learn tool is cloneable as-is (static content)
        Ok(config.clone())
    }

    fn sanitize(config: Self::Config) -> Self::Config {
        config.sanatize()
    }

    fn routes(_state: &Arc<ComhairleState>) -> ApiRouter {
        // Learn tool has no routes
        ApiRouter::new()
    }
}
