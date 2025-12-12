use std::{pin::Pin, sync::Arc};

use async_trait::async_trait;
use axum::body::Bytes;
use futures::Stream;
use ragflow::{
    chat::session::*, chat::*, client::RagflowClient, dataset::*, document::*, DeleteResources,
};
use reqwest::StatusCode;

#[cfg(test)]
use mockall::{automock, predicate::*};
use schemars::JsonSchema;
use serde::Serialize;

use crate::{
    error::ComhairleError,
    routes::bot::{
        sessions::{ChatConversationRequest, CreateChatSessionRequest, UpdateChatSessionRequest},
        GetQueryParams,
    },
};

pub mod ragflow_bot;

pub struct ComhairleRagBotService {
    client: Arc<RagflowClient>,
}

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
        description: Option<String>,
    ) -> Result<(StatusCode, Dataset), ComhairleError>;

    async fn delete_knowledge_base(&self, id: String) -> Result<StatusCode, ComhairleError>;

    async fn get_documents(
        &self,
        knowledge_base_id: String,
        params: Option<GetQueryParams>,
    ) -> Result<(StatusCode, Vec<Document>), ComhairleError>;

    async fn delete_document(
        &self,
        id: String,
        knowledge_base_id: String,
    ) -> Result<StatusCode, ComhairleError>;

    async fn upload_documents(
        &self,
        knowledge_base_id: &str,
        files: Vec<UploadFile>,
    ) -> Result<StatusCode, ComhairleError>;

    async fn create_chat(&self, body: CreateChat) -> Result<(StatusCode, Chat), ComhairleError>;

    async fn update_chat(&self, id: &str, body: UpdateChat) -> Result<StatusCode, ComhairleError>;

    async fn delete_chats(&self, body: DeleteResources<'_>) -> Result<StatusCode, ComhairleError>;

    async fn get_chats(
        &self,
        params: Option<GetQueryParams>,
    ) -> Result<(StatusCode, Vec<Chat>), ComhairleError>;

    async fn get_chat_sessions(
        &self,
        chat_id: &str,
        params: Option<GetQueryParams>,
    ) -> Result<(StatusCode, Vec<ComhairleChatSession>), ComhairleError>;

    async fn create_chat_session(
        &self,
        chat_id: &str,
        body: CreateChatSessionRequest,
    ) -> Result<(StatusCode, ComhairleChatSession), ComhairleError>;

    async fn update_chat_session(
        &self,
        session_id: &str,
        chat_id: &str,
        body: UpdateChatSessionRequest,
    ) -> Result<StatusCode, ComhairleError>;

    async fn delete_chat_sessions(
        &self,
        session_id: &str,
        chat_id: &str,
    ) -> Result<StatusCode, ComhairleError>;

    async fn converse_with_chat(
        &self,
        chat_id: &str,
        body: ChatConversationRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<Bytes, ComhairleError>> + Send + 'static>>,
        ComhairleError,
    >;
}

/// Comhairle specific chat session type
#[derive(Serialize, JsonSchema, Default)]
pub struct ComhairleChatSession {
    pub id: String,
    pub chat_id: String,
    pub name: Option<String>,
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
                    ComhairleChatSession {
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
