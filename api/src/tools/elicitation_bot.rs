use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ComhairleError;

use super::ToolConfigSanitize;

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

pub async fn setup(
    config: &ElicitationBotToolSetup,
) -> Result<ElicitationBotToolConfig, ComhairleError> {
    Ok(ElicitationBotToolConfig {
        topic: config.topic.clone(),
    })
}
