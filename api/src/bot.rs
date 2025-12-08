use std::sync::Arc;

use async_trait::async_trait;
use ragflow::{client::RagflowClient, Dataset, Document, GetDocumentsQueryParams, UploadFile};
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
    async fn create_knowledgebase(
        &self,
        name: String,
        description: Option<String>,
    ) -> Result<(StatusCode, Dataset), ComhairleError>;

    async fn delete_knowledgebase(&self, id: String) -> Result<StatusCode, ComhairleError>;

    async fn get_documents(
        &self,
        knowledgebase_id: String,
        query: Option<GetDocumentsQueryParams>,
    ) -> Result<(StatusCode, Vec<Document>), ComhairleError>;

    async fn delete_document(
        &self,
        id: String,
        knowledgebase_id: String,
    ) -> Result<StatusCode, ComhairleError>;

    async fn upload_documents(
        &self,
        knowledgebase_id: &str,
        files: Vec<UploadFile>,
    ) -> Result<StatusCode, ComhairleError>;
}

pub struct ComhairleRagBotService {
    client: Arc<RagflowClient>,
}

#[async_trait]
impl ComhairleBotService for ComhairleRagBotService {
    async fn create_knowledgebase(
        &self,
        name: String,
        description: Option<String>,
    ) -> Result<(StatusCode, Dataset), ComhairleError> {
        let (status, knowledgebase) = self.client.create_dataset(name, description).await?;
        Ok((status, knowledgebase))
    }

    async fn delete_knowledgebase(&self, id: String) -> Result<StatusCode, ComhairleError> {
        let status = self.client.delete_dataset(&id).await?;
        Ok(status)
    }

    async fn get_documents(
        &self,
        knowledgebase_id: String,
        query: Option<GetDocumentsQueryParams>,
    ) -> Result<(StatusCode, Vec<Document>), ComhairleError> {
        let (status, documents) = self.client.get_documents(&knowledgebase_id, query).await?;
        Ok((status, documents))
    }

    async fn delete_document(
        &self,
        id: String,
        knowledgebase_id: String,
    ) -> Result<StatusCode, ComhairleError> {
        let status = self.client.delete_document(&id, &knowledgebase_id).await?;
        Ok(status)
    }

    async fn upload_documents(
        &self,
        knowledgebase_id: &str,
        files: Vec<UploadFile>,
    ) -> Result<StatusCode, ComhairleError> {
        let (status, _) = self
            .client
            .upload_documents(knowledgebase_id, files)
            .await?;
        Ok(status)
    }
}

#[cfg(test)]
impl MockComhairleBotService {
    pub fn base() -> MockComhairleBotService {
        let mut bot_service = MockComhairleBotService::new();

        bot_service.expect_create_knowledgebase().returning(|_, _| {
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
            .expect_delete_knowledgebase()
            .returning(|_| Box::pin(async move { Ok(StatusCode::OK) }));
        bot_service
            .expect_get_documents()
            .returning(|_, _| Box::pin(async move { Ok((StatusCode::OK, Vec::new())) }));
        bot_service
            .expect_delete_document()
            .returning(|_, _| Box::pin(async move { Ok(StatusCode::OK) }));

        bot_service
    }
}
