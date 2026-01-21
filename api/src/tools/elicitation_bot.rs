use std::sync::Arc;

use minijinja::context;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    bot_service::ComhairleBotService, error::ComhairleError,
    routes::bot::agents::CreateAgentRequest,
};

use super::ToolConfigSanitize;

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct ElicitationBotToolConfig {
    pub bot_id: String,
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
    bot_service: &Arc<dyn ComhairleBotService>,
) -> Result<ElicitationBotToolConfig, ComhairleError> {
    let create_agent = CreateAgentRequest {
        name: config.conversation_id.clone(),
    };
    let (_, bot) = bot_service
        .create_agent(create_agent, context! { topic => &config.topic })
        .await?;

    Ok(ElicitationBotToolConfig {
        bot_id: bot.id,
        topic: config.topic.clone(),
    })
}
