use aide::openapi::StatusCode;
use async_trait::async_trait;
use mockall::automock;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use std::error::Error;

#[async_trait]
// #[cfg_attr(test, automock)]
pub trait ComhairleBotService: Send + Sync {
    type KnowledgeBase: Serialize + DeserializeOwned;
    type Error: std::error::Error + Send + Sync;

    async fn create_knowledge_base(
        &self,
        name: String,
        description: String,
    ) -> Result<(StatusCode, Self::KnowledgeBase), Self::Error>;
}

// create struct for knowledgebase
// struct {
//   author
//   created_at
//   conversation_id
// }
//
// create struct for document
// struct {
//   external_document_id
//   title
//   description
//   format
//
// }
//
// create knowledgebase for conversation
// get knowledgebase
//
// add document to knowledgebase
// view document
// delete document
