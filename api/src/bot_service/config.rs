use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct BotServiceConfig {
    pub host: String,
    pub api_key: String,
    pub thinking_space_agent_id: String,
    pub thinking_space_summary_agent_id: String,
    pub elicitation_bot_agent_id: String,
    pub default_knowledge_base_id: String,
}
