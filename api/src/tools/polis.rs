use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct PolisToolConfig {
    pub server_url: String,
    pub poll_id: String,
    #[serde(skip_serializing)]
    pub admin_user: String,
    #[serde(skip_serializing)]
    pub admin_password: String,
}
