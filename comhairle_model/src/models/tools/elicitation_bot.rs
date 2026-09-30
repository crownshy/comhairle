//! Elicitation bot tool configuration.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::models::tools::ToolConfigSanitize;

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct ElicitationBotToolConfig {
    pub topic: String,
}

impl ToolConfigSanitize for ElicitationBotToolConfig {
    fn sanitize(&self) -> Self {
        self.clone()
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct ElicitationBotToolSetup {
    pub topic: String,
}

#[derive(PartialEq, Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct ElicitationBotReport;
