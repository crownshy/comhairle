use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase", tag = "type", content = "content")]
pub enum PageContent {
    Markdown(String),
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct LocalisedPage {
    pub lang: String,
    #[serde(flatten)]
    pub content: PageContent,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct Page(pub Vec<LocalisedPage>);

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct LearnToolConfig {
    pub pages: Vec<Page>,
}
