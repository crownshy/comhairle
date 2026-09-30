//! Stories tool configuration.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::models::tools::ToolConfigSanitize;

#[derive(Debug, Default, JsonSchema, Serialize, Deserialize, Clone, PartialEq)]
pub struct StoriesToolConfig {
    pub max_time: i32,
    pub to_see: i32,
}

#[derive(PartialEq, Debug, Default, JsonSchema, Serialize, Deserialize, Clone)]
pub struct StoriesReport;

#[derive(Debug, Default, JsonSchema, Serialize, Deserialize, Clone)]
pub struct StoriesToolSetup {
    pub max_time: i32,
    pub to_see: i32,
}

impl ToolConfigSanitize for StoriesToolConfig {
    fn sanitize(&self) -> Self {
        self.clone()
    }
}
