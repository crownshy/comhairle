use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct PolisToolConfig {
    pub server_url: String,
    pub poll_id: String,

    pub admin_user: String,
    pub admin_password: String,
}
