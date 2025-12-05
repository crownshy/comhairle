use std::sync::Arc;

use async_trait::async_trait;
use ragflow::{client::RagflowClient, Dataset};
use reqwest::StatusCode;

#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::error::ComhairleError;

impl ComhairleRagBotService {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        ComhairleRagBotService {
            client: Arc::new(RagflowClient::new(
                base_url.to_string(),
                api_key.to_string(),
            )),
        }
    }
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait ComhairleBotService: Send + Sync {
    async fn create_knowledge_base(
        &self,
        name: String,
        description: String,
    ) -> Result<(StatusCode, Dataset), ComhairleError>;

    async fn delete_knowledge_base(&self, id: String) -> Result<StatusCode, ComhairleError>;
}

pub struct ComhairleRagBotService {
    client: Arc<RagflowClient>,
}

#[async_trait]
impl ComhairleBotService for ComhairleRagBotService {
    async fn create_knowledge_base(
        &self,
        name: String,
        description: String,
    ) -> Result<(StatusCode, Dataset), ComhairleError> {
        let (status, knowledge_base) = self.client.create_dataset(name, description).await?;

        Ok((status, knowledge_base))
    }

    async fn delete_knowledge_base(&self, id: String) -> Result<StatusCode, ComhairleError> {
        let status = self.client.delete_dataset(&id).await?;
        Ok(status)
    }
}

#[cfg(test)]
impl MockComhairleBotService {
    pub fn base() -> MockComhairleBotService {
        let mut bot_service = MockComhairleBotService::new();

        bot_service
            .expect_create_knowledge_base()
            .returning(|_, _| {
                Box::pin(async move {
                    Ok((
                        StatusCode::OK,
                        Dataset {
                            ..Default::default()
                        },
                    ))
                })
            });
        bot_service
            .expect_delete_knowledge_base()
            .returning(|_| Box::pin(async move { Ok(StatusCode::OK) }));

        bot_service
    }
}
