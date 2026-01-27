use std::pin::Pin;

use async_trait::async_trait;
use axum::body::Bytes;
use futures::Stream;
use reqwest::StatusCode;

#[cfg(test)]
use mockall::{automock, predicate::*};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    error::ComhairleError,
    routes::{
        bot::{
            agents::{CreateAgentRequest, UpdateAgentRequest},
            chat_sessions::{
                ChatConversationRequest, CreateChatSessionRequest, UpdateChatSessionRequest,
            },
            chats::{CreateChatRequest, UpdateChatRequest},
            documents::UpdateDocumentRequest,
            knowledge_bases::UpdateKnowledgeBaseRequest,
            GetQueryParams,
        },
        conversations::UploadFileRequest,
        workflow_steps::AgentConversationRequestExt,
    },
};

pub mod ragflow_bot;

pub use ragflow_bot::ComhairleRagBotService;

pub const DEFAULT_CHAT_OPENER: &str =
    "Hey I am here to help you better understand this consultation. Ask me anything";
pub const DEFAULT_CHAT_NOT_FOUND_RESPONSE: &str =
    "Sorry I couldn't find an answer to that question.";
/// Default system prompt for conversation Q&A chatbots
pub const DEFAULT_CHAT_PROMPT: &str = r#"You are a helpful assistant for a participatory democracy platform.

Your task is to answer the user's question using ONLY the information in the knowledge base below.

Write your answer for a general public audience:
- Use clear, simple language
- Avoid technical terms, academic phrasing, and jargon
- Use short sentences and plain explanations
- Explain ideas as if speaking to an interested citizen with no prior expertise

Structure your answer as follows:
1. A short, direct answer (2–4 sentences)
2. A clear explanation in bullet points or short paragraphs
3. If helpful, include simple examples

If multiple viewpoints or pieces of information appear in the dataset, summarize them in a balanced and neutral way.

If ALL of the dataset content is irrelevant to the question, include this exact sentence:
"The answer you are looking for is not found in the dataset!"

Take prior chat history into account when answering.

Here is the knowledge base:
{knowledge}"#;

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
        knowledge_base_id: &str,
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

    async fn upload_document(
        &self,
        knowledge_base_id: &str,
        file: UploadFileRequest,
    ) -> Result<(StatusCode, ComhairleDocument), ComhairleError>;

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

    async fn parse_document(
        &self,
        document_id: String,
        knowledge_base_id: String,
    ) -> Result<StatusCode, ComhairleError>;

    async fn stop_parsing_document(
        &self,
        document_id: String,
        knowledge_base_id: String,
    ) -> Result<StatusCode, ComhairleError>;

    async fn download_document(
        &self,
        document_id: String,
        knowledge_base_id: String,
    ) -> Result<reqwest::Response, ComhairleError>;

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

    async fn get_agent(
        &self,
        agent_id: &str,
    ) -> Result<(StatusCode, ComhairleAgent), ComhairleError>;

    async fn list_agents(
        &self,
        params: Option<GetQueryParams>,
    ) -> Result<(StatusCode, Vec<ComhairleAgent>), ComhairleError>;

    async fn create_agent(
        &self,
        body: CreateAgentRequest,
    ) -> Result<(StatusCode, ComhairleAgent), ComhairleError>;

    async fn update_agent(
        &self,
        agent_id: &str,
        body: UpdateAgentRequest,
    ) -> Result<(StatusCode, ComhairleAgent), ComhairleError>;

    async fn delete_agent(&self, agent_id: &str) -> Result<StatusCode, ComhairleError>;

    async fn get_agent_session(
        &self,
        session_id: &str,
        agent_id: &str,
    ) -> Result<(StatusCode, ComhairleAgentSession), ComhairleError>;

    async fn list_agent_sessions(
        &self,
        agent_id: &str,
        params: Option<GetQueryParams>,
    ) -> Result<(StatusCode, Vec<ComhairleAgentSession>), ComhairleError>;

    async fn create_agent_session(
        &self,
        agent_id: &str,
    ) -> Result<(StatusCode, ComhairleAgentSession), ComhairleError>;

    async fn delete_agent_session(
        &self,
        session_id: &str,
        agent_id: &str,
    ) -> Result<StatusCode, ComhairleError>;

    async fn converse_with_agent(
        &self,
        session_id: &str,
        agent_id: &str,
        body: AgentConversationRequestExt,
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

#[derive(Serialize, Deserialize, JsonSchema, Debug, Default, Clone)]
pub struct ComhairleAgent {
    pub id: String,
    pub name: String,
    pub configuration: serde_json::Value,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Default, Clone)]
pub struct ComhairleAgentSession {
    pub id: String,
    pub agent_id: String,
    pub configuration: serde_json::Value,
    pub messages: Vec<ComhairleSessionMessage>,
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
        bot_service.expect_get_document().returning(|_, _| {
            Box::pin(async move {
                Ok((
                    StatusCode::OK,
                    ComhairleDocument {
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service.expect_upload_document().returning(|_, _| {
            Box::pin(async move {
                Ok((
                    StatusCode::OK,
                    ComhairleDocument {
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service.expect_update_document().returning(|_, _, _| {
            Box::pin(async move {
                Ok((
                    StatusCode::OK,
                    ComhairleDocument {
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service
            .expect_delete_document()
            .returning(|_, _| Box::pin(async move { Ok(StatusCode::OK) }));
        bot_service
            .expect_parse_document()
            .returning(|_, _| Box::pin(async move { Ok(StatusCode::OK) }));
        bot_service
            .expect_stop_parsing_document()
            .returning(|_, _| Box::pin(async move { Ok(StatusCode::OK) }));
        bot_service.expect_download_document().returning(|_, _| {
            Box::pin(async move {
                use axum::http;
                Ok(reqwest::Response::from(
                    http::Response::builder()
                        .status(200)
                        .body(reqwest::Body::from(Vec::<u8>::new()))
                        .unwrap(),
                ))
            })
        });
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
        bot_service.expect_get_agent().returning(|_| {
            Box::pin(async move {
                Ok((
                    StatusCode::OK,
                    ComhairleAgent {
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service
            .expect_list_agents()
            .returning(|_| Box::pin(async move { Ok((StatusCode::OK, Vec::new())) }));
        bot_service.expect_create_agent().returning(|_| {
            Box::pin(async move {
                Ok((
                    StatusCode::OK,
                    ComhairleAgent {
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service.expect_update_agent().returning(|_, _| {
            Box::pin(async move {
                Ok((
                    StatusCode::OK,
                    ComhairleAgent {
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service
            .expect_delete_agent()
            .returning(|_| Box::pin(async move { Ok(StatusCode::NO_CONTENT) }));
        bot_service.expect_get_agent_session().returning(|_, _| {
            Box::pin(async move {
                Ok((
                    StatusCode::OK,
                    ComhairleAgentSession {
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service
            .expect_list_agent_sessions()
            .returning(|_, _| Box::pin(async move { Ok((StatusCode::OK, vec![])) }));
        bot_service.expect_create_agent_session().returning(|_| {
            Box::pin(async move {
                Ok((
                    StatusCode::CREATED,
                    ComhairleAgentSession {
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service
            .expect_delete_agent_session()
            .returning(|_, _| Box::pin(async move { Ok(StatusCode::NO_CONTENT) }));
        bot_service
            .expect_converse_with_agent()
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
