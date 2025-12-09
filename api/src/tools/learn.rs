use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ComhairleError;

use super::ToolConfigSanitize;

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
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Page(pub Vec<LocalisedPage>);

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct LearnToolConfig {
    pub pages: Vec<Page>,
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
    pub pages: Vec<Page>,
}

pub async fn setup(setup_config: &LearnToolSetup) -> Result<LearnToolConfig, ComhairleError> {
    Ok(LearnToolConfig {
        pages: setup_config.pages.clone(),
    })
}
