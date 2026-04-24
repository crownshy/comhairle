use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CategorizationConfig {
    pub server_url: String,
    pub api_key: String,
}
