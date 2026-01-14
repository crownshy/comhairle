use std::{pin::Pin, sync::Arc};

use async_trait::async_trait;
use axum::body::Bytes;
use futures::Stream;
use ragflow::client::RagflowClient;
use reqwest::StatusCode;

#[cfg(test)]
use mockall::{automock, predicate::*};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    error::ComhairleError,
    routes::{
        bot::{
            chats::{CreateChatRequest, UpdateChatRequest},
            documents::UpdateDocumentRequest,
            knowledge_bases::UpdateKnowledgeBaseRequest,
            sessions::{
                ChatConversationRequest, CreateChatSessionRequest, UpdateChatSessionRequest,
            },
            GetQueryParams,
        },
        conversations::UploadFileRequest,
    },
};

pub mod ragflow_bot;

#[derive(Debug)]
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
    async fn get_knowledge_base(
        &self,
        knowledge_base_id: &str,
    ) -> Result<(StatusCode, ComhairleKnowledgeBase), ComhairleError>;

    async fn list_knowledge_bases(
        &self,
        params: Option<GetQueryParams>,
    ) -> Result<(StatusCode, Vec<ComhairleKnowledgeBase>), ComhairleError>;

    async fn create_knowledge_base(
        &self,
        name: String,
        description: Option<String>,
    ) -> Result<(StatusCode, ComhairleKnowledgeBase), ComhairleError>;

    async fn update_knowledge_base(
        &self,
        knowledge_base_id: &str,
        body: UpdateKnowledgeBaseRequest,
    ) -> Result<(StatusCode, ComhairleKnowledgeBase), ComhairleError>;

    async fn delete_knowledge_base(
        &self,
        knowledge_base_id: String,
    ) -> Result<StatusCode, ComhairleError>;

    async fn list_documents(
        &self,
        knowledge_base_id: &str,
        params: Option<GetQueryParams>,
    ) -> Result<(StatusCode, Vec<ComhairleDocument>), ComhairleError>;

    async fn get_document(
        &self,
        document_id: &str,
        knowledge_base_id: &str,
    ) -> Result<(StatusCode, ComhairleDocument), ComhairleError>;

    async fn upload_documents(
        &self,
        knowledge_base_id: &str,
        files: Vec<UploadFileRequest>,
    ) -> Result<(StatusCode, Vec<ComhairleDocument>), ComhairleError>;

    async fn update_document(
        &self,
        document_id: &str,
        knowledge_base_id: &str,
        body: UpdateDocumentRequest,
    ) -> Result<(StatusCode, ComhairleDocument), ComhairleError>;

    async fn delete_document(
        &self,
        document_id: String,
        knowledge_base_id: String,
    ) -> Result<StatusCode, ComhairleError>;

    async fn get_chat(&self, chat_id: &str) -> Result<(StatusCode, ComhairleChat), ComhairleError>;

    async fn list_chats(
        &self,
        params: Option<GetQueryParams>,
    ) -> Result<(StatusCode, Vec<ComhairleChat>), ComhairleError>;

    async fn create_chat(
        &self,
        body: CreateChatRequest,
    ) -> Result<(StatusCode, ComhairleChat), ComhairleError>;

    async fn update_chat(
        &self,
        chat_id: &str,
        body: UpdateChatRequest,
    ) -> Result<(StatusCode, ComhairleChat), ComhairleError>;

    async fn delete_chat(&self, chat_id: &str) -> Result<StatusCode, ComhairleError>;

    async fn get_chat_session(
        &self,
        session_id: &str,
        chat_id: &str,
    ) -> Result<(StatusCode, ComhairleChatSession), ComhairleError>;

    async fn list_chat_sessions(
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
    ) -> Result<(StatusCode, ComhairleChatSession), ComhairleError>;

    async fn delete_chat_session(
        &self,
        session_id: &str,
        chat_id: &str,
    ) -> Result<StatusCode, ComhairleError>;

    async fn converse_with_chat(
        &self,
        session_id: &str,
        chat_id: &str,
        body: ChatConversationRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<Bytes, ComhairleError>> + Send + 'static>>,
        ComhairleError,
    >;
}

#[derive(Serialize, Deserialize, JsonSchema, Default, Debug, Clone)]
pub struct ComhairleKnowledgeBase {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Default, Debug, Clone)]
pub struct ComhairleDocument {
    pub id: String,
    pub name: String,
    pub parse_status: String,
    pub parse_progress: f64,
    pub size: i64,
    // TODO: figure out what fields we require
}

#[derive(Serialize, Deserialize, JsonSchema, Default, Debug, Clone)]
pub struct ComhairleChat {
    pub id: String,
    pub name: String,
    pub llm_model: Option<ComhairleLlm>,
    pub prompt: Option<ComhairlePrompt>,
    pub knowledge_base_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Default, Debug, Clone, PartialEq)]
pub struct ComhairlePrompt {
    pub llm_prompt: Option<String>,
    pub opener: Option<String>,
    pub empty_response: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Default, Debug, Clone, PartialEq)]
pub struct ComhairleLlm {
    pub model_name: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Default, Debug, Clone)]
pub struct ComhairleChatSession {
    pub id: String,
    pub chat_id: String,
    pub name: Option<String>,
    pub messages: Vec<ComhairleSessionMessage>,
}

#[derive(Serialize, Deserialize, JsonSchema, Default, Debug, Clone)]
pub struct ComhairleSessionMessage {
    content: String,
    id: String,
    role: String,
    reference: Option<Vec<ComhairleMessageReference>>,
}

#[derive(Serialize, Deserialize, JsonSchema, Default, Debug, Clone)]
pub struct ComhairleMessageReference {
    pub id: String,
    pub content: String,
    pub dataset_id: String,
    pub document_id: String,
    pub document_name: String,
}

#[cfg(test)]
impl MockComhairleBotService {
    pub fn base() -> MockComhairleBotService {
        let mut bot_service = MockComhairleBotService::new();

        bot_service
            .expect_list_knowledge_bases()
            .returning(|_| Box::pin(async move { Ok((StatusCode::OK, Vec::new())) }));
        bot_service.expect_get_knowledge_base().returning(|_| {
            Box::pin(async move {
                Ok((
                    StatusCode::OK,
                    ComhairleKnowledgeBase {
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service
            .expect_create_knowledge_base()
            .returning(|_, _| {
                Box::pin(async move {
                    Ok((
                        StatusCode::OK,
                        ComhairleKnowledgeBase {
                            ..Default::default()
                        },
                    ))
                })
            });
        bot_service
            .expect_update_knowledge_base()
            .returning(|_, _| {
                Box::pin(async move {
                    Ok((
                        StatusCode::OK,
                        ComhairleKnowledgeBase {
                            ..Default::default()
                        },
                    ))
                })
            });
        bot_service
            .expect_delete_knowledge_base()
            .returning(|_| Box::pin(async move { Ok(StatusCode::OK) }));
        bot_service
            .expect_delete_knowledge_base()
            .returning(|_| Box::pin(async move { Ok(StatusCode::OK) }));
        bot_service
            .expect_list_documents()
            .returning(|_, _| Box::pin(async move { Ok((StatusCode::OK, Vec::new())) }));
        bot_service
            .expect_delete_document()
            .returning(|_, _| Box::pin(async move { Ok(StatusCode::OK) }));
        bot_service.expect_get_chat().returning(|_| {
            Box::pin(async move {
                Ok((
                    StatusCode::OK,
                    ComhairleChat {
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service
            .expect_list_chats()
            .returning(|_| Box::pin(async move { Ok((StatusCode::OK, Vec::new())) }));
        bot_service.expect_create_chat().returning(|_| {
            Box::pin(async move {
                Ok((
                    StatusCode::OK,
                    ComhairleChat {
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service.expect_update_chat().returning(|_, _| {
            Box::pin(async move {
                Ok((
                    StatusCode::OK,
                    ComhairleChat {
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service
            .expect_delete_chat()
            .returning(|_| Box::pin(async move { Ok(StatusCode::OK) }));
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
        bot_service.expect_get_chat_session().returning(|_, _| {
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
            .expect_list_chat_sessions()
            .returning(|_, _| Box::pin(async move { Ok((StatusCode::OK, Vec::new())) }));
        bot_service
            .expect_update_chat_session()
            .returning(|_, _, _| {
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
            .expect_delete_chat_session()
            .returning(|_, _| Box::pin(async move { Ok(StatusCode::OK) }));
        bot_service
            .expect_converse_with_chat()
            .returning(|_, _, _| {
                Box::pin(async move {
                    let stream: Pin<Box<dyn Stream<Item = Result<Bytes, ComhairleError>> + Send>> =
                        Box::pin(futures::stream::empty());

                    Ok(stream)
                })
            });

        bot_service
    }
}
