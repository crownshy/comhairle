use std::{pin::Pin, sync::Arc};

use async_trait::async_trait;
use axum::body::Bytes;
use futures::{Stream, StreamExt};
use ragflow::{
    client::RagflowClient, Chat, ChatSession, ConvoQuestion, CreateChat, CreateUpdateChatSession,
    Dataset, DeleteResources, Document, GetQueryParams, UpdateChat, UploadFile,
};
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
        query: Option<GetQueryParams>,
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

    async fn create_chat(&self, body: CreateChat) -> Result<(StatusCode, Chat), ComhairleError>;

    async fn update_chat(&self, id: &str, body: UpdateChat) -> Result<StatusCode, ComhairleError>;

    async fn delete_chats(&self, body: DeleteResources<'_>) -> Result<StatusCode, ComhairleError>;

    async fn get_chats(
        &self,
        params: Option<GetQueryParams>,
    ) -> Result<(StatusCode, Vec<Chat>), ComhairleError>;

    async fn create_chat_session(
        &self,
        chat_id: &str,
        body: CreateUpdateChatSession,
    ) -> Result<(StatusCode, ChatSession), ComhairleError>;

    async fn update_chat_session(
        &self,
        session_id: &str,
        chat_id: &str,
        body: CreateUpdateChatSession,
    ) -> Result<StatusCode, ComhairleError>;

    async fn delete_chat_sessions(
        &self,
        chat_id: &str,
        body: DeleteResources<'_>,
    ) -> Result<StatusCode, ComhairleError>;

    async fn get_chat_sessions(
        &self,
        chat_id: &str,
        params: Option<GetQueryParams>,
    ) -> Result<(StatusCode, Vec<ChatSession>), ComhairleError>;

    async fn converse_with_chat(
        &self,
        chat_id: &str,
        body: ConvoQuestion,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<Bytes, ComhairleError>> + Send + 'static>>,
        ComhairleError,
    >;
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
        query: Option<GetQueryParams>,
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

    async fn create_chat(&self, body: CreateChat) -> Result<(StatusCode, Chat), ComhairleError> {
        let (status, chat) = self.client.create_chat(body).await?;
        Ok((status, chat))
    }

    async fn update_chat(&self, id: &str, body: UpdateChat) -> Result<StatusCode, ComhairleError> {
        let status = self.client.update_chat(id, body).await?;
        Ok(status)
    }

    async fn delete_chats(&self, body: DeleteResources<'_>) -> Result<StatusCode, ComhairleError> {
        let status = self.client.delete_chats(body).await?;
        Ok(status)
    }

    async fn get_chats(
        &self,
        params: Option<GetQueryParams>,
    ) -> Result<(StatusCode, Vec<Chat>), ComhairleError> {
        let (status, chats) = self.client.get_chats(params).await?;
        Ok((status, chats))
    }

    async fn create_chat_session(
        &self,
        chat_id: &str,
        body: CreateUpdateChatSession,
    ) -> Result<(StatusCode, ChatSession), ComhairleError> {
        let (status, chat_session) = self.client.create_chat_session(chat_id, body).await?;
        Ok((status, chat_session))
    }

    async fn update_chat_session(
        &self,
        session_id: &str,
        chat_id: &str,
        body: CreateUpdateChatSession,
    ) -> Result<StatusCode, ComhairleError> {
        let status = self
            .client
            .update_chat_session(session_id, chat_id, body)
            .await?;
        Ok(status)
    }

    async fn delete_chat_sessions(
        &self,
        chat_id: &str,
        body: DeleteResources<'_>,
    ) -> Result<StatusCode, ComhairleError> {
        let status = self.client.delete_chat_sessions(chat_id, body).await?;
        Ok(status)
    }

    async fn get_chat_sessions(
        &self,
        chat_id: &str,
        params: Option<GetQueryParams>,
    ) -> Result<(StatusCode, Vec<ChatSession>), ComhairleError> {
        let (status, chat_sessions) = self.client.get_chat_sessions(chat_id, params).await?;
        Ok((status, chat_sessions))
    }

    async fn converse_with_chat(
        &self,
        chat_id: &str,
        body: ConvoQuestion,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<Bytes, ComhairleError>> + Send + 'static>>,
        ComhairleError,
    > {
        let stream = self.client.stream_chat_conversation(chat_id, body).await?;

        let mapped_stream = stream.map(|item| item.map_err(ComhairleError::from));

        Ok(Box::pin(mapped_stream))
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
        bot_service.expect_create_chat().returning(|_| {
            Box::pin(async move {
                Ok((
                    StatusCode::OK,
                    Chat {
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service
            .expect_update_chat()
            .returning(|_, _| Box::pin(async move { Ok(StatusCode::OK) }));
        bot_service
            .expect_delete_chats()
            .returning(|_| Box::pin(async move { Ok(StatusCode::OK) }));
        bot_service
            .expect_get_chats()
            .returning(|_| Box::pin(async move { Ok((StatusCode::OK, Vec::new())) }));
        bot_service.expect_create_chat_session().returning(|_, _| {
            Box::pin(async move {
                Ok((
                    StatusCode::OK,
                    ChatSession {
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service
            .expect_update_chat_session()
            .returning(|_, _, _| Box::pin(async move { Ok(StatusCode::OK) }));
        bot_service
            .expect_delete_chat_sessions()
            .returning(|_, _| Box::pin(async move { Ok(StatusCode::OK) }));
        bot_service
            .expect_get_chat_sessions()
            .returning(|_, _| Box::pin(async move { Ok((StatusCode::OK, Vec::new())) }));
        bot_service.expect_converse_with_chat().returning(|_, _| {
            Box::pin(async move {
                let stream: Pin<Box<dyn Stream<Item = Result<Bytes, ComhairleError>> + Send>> =
                    Box::pin(futures::stream::empty());

                Ok(stream)
            })
        });

        bot_service
    }
}
