use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ComhairleError;

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
#[serde(rename_all = "lowercase", tag = "type", content = "content")]
pub enum PageContent {
    Markdown(String),
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct LocalisedPage {
    pub lang: String,
    #[serde(flatten)]
    pub content: PageContent,
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub struct Page(pub Vec<LocalisedPage>);

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct LearnToolConfig {
    pub pages: Vec<Page>,
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct LearnToolSetup {
    pub pages: Vec<Page>,
}

pub async fn setup(setup_config: &LearnToolSetup) -> Result<LearnToolConfig, ComhairleError> {
    Ok(LearnToolConfig {
        pages: setup_config.pages.clone(),
    })
}
