use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PolisToolConfig {
    pub server_url: String,
    pub poll_id: String,
}
