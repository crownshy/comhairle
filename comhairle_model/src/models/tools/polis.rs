//! Polis tool configuration.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::tools::ToolConfigSanitize;

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema, PartialEq)]
pub struct PolisToolConfig {
    pub server_url: String,
    pub poll_id: String,
    pub admin_user: String,
    pub admin_password: String,
    pub required_votes: Option<i32>,
    #[serde(default = "default_show_remaining_statements")]
    pub show_remaining_statements: bool,
    // Mirror of the Polis conversation config. These are written through to
    // Polis via PolisUpdateConfig AND stored here so the Setup tab can pre-fill
    // them (Polis exposes no read path for topic/description/is_active/
    // strict_moderation).
    #[serde(default)]
    pub topic: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: Option<bool>,
    #[serde(default)]
    pub strict_moderation: Option<bool>,
    // comhairle-only display flag (not sent to Polis): style seed statements
    // with a "conversation starter" label in the participant embed.
    #[serde(default)]
    pub label_seeds_as_conversation_starter: bool,
    // Reasons offered when rejecting a statement. None means the built-in
    // defaults in models::moderation_policy::DEFAULT_REASONS.
    #[serde(default)]
    pub moderation_policy_id: Option<Uuid>,
}

pub(crate) fn default_show_remaining_statements() -> bool {
    true
}

impl ToolConfigSanitize for PolisToolConfig {
    fn sanitize(&self) -> Self {
        Self {
            admin_user: "".into(),
            admin_password: "".into(),
            server_url: self.server_url.clone(),
            poll_id: self.poll_id.clone(),
            required_votes: self.required_votes,
            show_remaining_statements: self.show_remaining_statements,
            topic: self.topic.clone(),
            description: self.description.clone(),
            is_active: self.is_active,
            strict_moderation: self.strict_moderation,
            label_seeds_as_conversation_starter: self.label_seeds_as_conversation_starter,
            moderation_policy_id: self.moderation_policy_id,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct PolisToolSetup {
    pub topic: String,
    pub required_votes: Option<i32>,
    #[serde(default = "default_show_remaining_statements")]
    pub show_remaining_statements: bool,
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct PolisReport;
