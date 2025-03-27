use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Page {
    Markdown(String),
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct LearnToolConfig {
    pub pages: Vec<Page>,
}
