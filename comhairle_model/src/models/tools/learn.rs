//! Learn tool configuration.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::models::tools::ToolConfigSanitize;
use crate::models::translations::TextContentId;

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
pub struct LocalizedPage {
    pub lang: String,
    #[serde(flatten)]
    pub content: PageContent,
    #[serde(default = "default_requires_validation")]
    pub requires_validation: bool,
}

fn default_requires_validation() -> bool {
    true
}

pub type Page = Vec<LocalizedPage>;

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
    fn sanitize(&self) -> Self {
        self.clone()
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct LearnReport;

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct LearnToolSetup {
    pub pages: Vec<LearnPageEntry>,
}
