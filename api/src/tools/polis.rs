use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PolisToolConfig {
    pub server_url: String,
    pub poll_id: String,
    pub admin_user: String,
    pub admin_password: String,
}
