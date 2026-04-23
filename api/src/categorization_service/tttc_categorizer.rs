use async_trait::async_trait;
use reqwest::Client;

use super::{error::Result, CategorizationService};

pub struct TttcCategorizer {
    client: Client,
    base_url: String,
    api_key: String,
}

impl TttcCategorizer {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        let client = Client::new();

        Self {
            client,
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
        }
    }
}

#[async_trait]
impl CategorizationService for TttcCategorizer {
    async fn queue_job(&self) -> Result<()> {
        Ok(())
    }
}
