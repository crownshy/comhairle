use std::{
    collections::{HashMap, HashSet},
    pin::Pin,
    sync::Arc,
};

use async_trait::async_trait;
use axum::body::Bytes;
use futures::stream::{self, Stream, StreamExt};
use ragflow::{
    ConvoQuestion, DeleteResources, GetQueryParams, Input, MessageReference, RagflowError,
    SessionMessage,
    agent::{session::*, *},
    chat::{session::*, *},
    client::RagflowClient,
    dataset::*,
    document::*,
};
use reqwest::StatusCode;
use serde_json::{Value, from_str};
use tracing::instrument;

use crate::{
    bot_service::{
        AgentConversationRequest, BotServiceSseEvent, ChatConversationRequest, ComhairleAgent,
        ComhairleAgentSession, ComhairleBotService, ComhairleChat, ComhairleChatSession,
        ComhairleDocument, ComhairleKnowledgeBase, ComhairleLlm, ComhairleMessageReference,
        ComhairlePrompt, ComhairleSessionMessage, CreateAgentRequest, CreateChatRequest,
        CreateChatSessionRequest, GetQueryParams as ApiGetQueryParams, UpdateAgentRequest,
        UpdateChatRequest, UpdateChatSessionRequest, UpdateDocumentRequest,
        UpdateKnowledgeBaseRequest, UploadFileRequest,
    },
    error::ComhairleError,
};

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

    /// Re-attach passage highlight `positions` to a reloaded session's
    /// references.
    ///
    /// RAGFlow's stored session history omits `positions`, but its chunk store
    /// keeps them: they describe where a chunk sits in the document, not the
    /// query, so the same chunk id always maps to the same boxes. We fetch each
    /// cited document's chunks once and fill in positions by chunk id, which
    /// fixes reloaded answers (old and new) the same way live answers already
    /// work. Best-effort: any RAGFlow failure just leaves those references
    /// without positions (no highlight), never a failed history load. See issue
    /// #783.
    async fn enrich_reference_positions(&self, session: &mut ComhairleChatSession) {
        // Chunk-list pagination bounds. A page size this large usually means one
        // request per document; the page cap is a runaway guard.
        const CHUNK_PAGE_SIZE: i32 = 512;
        const MAX_CHUNK_PAGES: i32 = 20;

        let mut documents: HashSet<(String, String)> = HashSet::new();
        for message in &session.messages {
            let Some(refs) = &message.reference else {
                continue;
            };
            for reference in refs {
                if reference.positions.is_none() {
                    documents.insert((reference.dataset_id.clone(), reference.document_id.clone()));
                }
            }
        }
        if documents.is_empty() {
            return;
        }

        let mut positions_by_chunk: HashMap<String, Vec<Vec<f64>>> = HashMap::new();
        for (dataset_id, document_id) in documents {
            for page in 1..=MAX_CHUNK_PAGES {
                let query = GetQueryParams {
                    page: Some(page),
                    page_size: Some(CHUNK_PAGE_SIZE),
                    ..Default::default()
                };
                let chunks = match ragflow::document::list_chunks(
                    &self.client,
                    &dataset_id,
                    &document_id,
                    Some(query),
                )
                .await
                {
                    Ok((_, chunk_list)) => chunk_list.chunks,
                    Err(_) => break,
                };
                let page_len = chunks.len();
                for chunk in chunks {
                    if let Some(positions) = chunk.positions {
                        positions_by_chunk.insert(chunk.id, positions);
                    }
                }
                if page_len < CHUNK_PAGE_SIZE as usize {
                    break;
                }
            }
        }

        for message in &mut session.messages {
            let Some(refs) = &mut message.reference else {
                continue;
            };
            for reference in refs {
                if reference.positions.is_none() {
                    if let Some(positions) = positions_by_chunk.get(&reference.id) {
                        reference.positions = Some(positions.clone());
                    }
                }
            }
        }
    }
}

#[async_trait]
impl ComhairleBotService for ComhairleRagBotService {
    #[instrument(err(Debug))]
    async fn get_knowledge_base(
        &self,
        knowledge_base_id: &str,
    ) -> Result<(StatusCode, ComhairleKnowledgeBase), ComhairleError> {
        let params = GetQueryParams {
            id: Some(knowledge_base_id.to_string()),
            ..Default::default()
        };

        let (status, knowledge_bases) = ragflow::dataset::list(&self.client, Some(params)).await?;

        let knowledge_base: ComhairleKnowledgeBase = (&knowledge_bases[0]).into();

        Ok((status, knowledge_base))
    }

    #[instrument(err(Debug))]
    async fn list_knowledge_bases(
        &self,
        params: Option<ApiGetQueryParams>,
    ) -> Result<(StatusCode, Vec<ComhairleKnowledgeBase>), ComhairleError> {
        let params: Option<GetQueryParams> = params.map(|p| p.into());

        let (status, knowledge_bases) = ragflow::dataset::list(&self.client, params).await?;

        let knowledge_bases: Vec<ComhairleKnowledgeBase> =
            knowledge_bases.into_iter().map(Into::into).collect();

        Ok((status, knowledge_bases))
    }

    #[instrument(err(Debug))]
    async fn create_knowledge_base(
        &self,
        name: String,
        description: Option<String>,
    ) -> Result<(StatusCode, ComhairleKnowledgeBase), ComhairleError> {
        let (status, knowledge_base) =
            ragflow::dataset::create(&self.client, name, description).await?;

        let knowledge_base: ComhairleKnowledgeBase = knowledge_base.into();

        Ok((status, knowledge_base))
    }

    #[instrument(err(Debug))]
    async fn update_knowledge_base(
        &self,
        knowledge_base_id: &str,
        body: UpdateKnowledgeBaseRequest,
    ) -> Result<(StatusCode, ComhairleKnowledgeBase), ComhairleError> {
        let body: UpdateDataset = body.into();

        let status = ragflow::dataset::update(&self.client, knowledge_base_id, body).await?;

        let params = GetQueryParams {
            id: Some(knowledge_base_id.to_string()),
            ..Default::default()
        };

        let (_, knowledge_bases) = ragflow::dataset::list(&self.client, Some(params)).await?;

        if knowledge_bases.is_empty() || knowledge_bases.len() > 1 {
            return Err(ComhairleError::RagflowError(RagflowError::Api {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: "error retrieving knowledge base after update".to_string(),
            }));
        }

        let knowledge_base: ComhairleKnowledgeBase = (&knowledge_bases[0]).into();

        Ok((status, knowledge_base))
    }

    #[instrument(err(Debug))]
    async fn delete_knowledge_base(
        &self,
        knowledge_base_id: &str,
    ) -> Result<StatusCode, ComhairleError> {
        let body = DeleteResources {
            ids: vec![knowledge_base_id],
        };

        let status = ragflow::dataset::delete(&self.client, body).await?;

        Ok(status)
    }

    #[instrument(err(Debug))]
    async fn list_documents(
        &self,
        knowledge_base_id: &str,
        params: Option<ApiGetQueryParams>,
    ) -> Result<(StatusCode, Vec<ComhairleDocument>), ComhairleError> {
        let params: Option<GetQueryParams> = params.map(|p| p.into());

        let (status, documents) =
            ragflow::document::list(&self.client, knowledge_base_id, params).await?;

        let documents: Vec<ComhairleDocument> = documents.into_iter().map(Into::into).collect();

        Ok((status, documents))
    }

    #[instrument(err(Debug))]
    async fn get_document(
        &self,
        document_id: &str,
        knowledge_base_id: &str,
    ) -> Result<(StatusCode, ComhairleDocument), ComhairleError> {
        let params = GetQueryParams {
            id: Some(document_id.to_string()),
            ..Default::default()
        };

        let (status, documents) =
            ragflow::document::list(&self.client, knowledge_base_id, Some(params)).await?;

        let document: ComhairleDocument = (&documents[0]).into();

        Ok((status, document))
    }

    #[instrument(err(Debug))]
    async fn upload_document(
        &self,
        knowledge_base_id: &str,
        file: UploadFileRequest,
    ) -> Result<(StatusCode, ComhairleDocument), ComhairleError> {
        let file: UploadFile = file.into();

        let (status, documents) =
            ragflow::document::upload(&self.client, knowledge_base_id, vec![file]).await?;

        let document: ComhairleDocument = (&documents[0]).into();

        Ok((status, document))
    }

    #[instrument(err(Debug))]
    async fn update_document(
        &self,
        document_id: &str,
        knowledge_base_id: &str,
        body: UpdateDocumentRequest,
    ) -> Result<(StatusCode, ComhairleDocument), ComhairleError> {
        let body: UpdateDocument = body.into();

        let (status, _) =
            ragflow::document::update(&self.client, document_id, knowledge_base_id, body).await?;

        let params = GetQueryParams {
            id: Some(document_id.to_string()),
            ..Default::default()
        };

        let (_, documents) =
            ragflow::document::list(&self.client, knowledge_base_id, Some(params)).await?;

        if documents.is_empty() || documents.len() > 1 {
            return Err(ComhairleError::RagflowError(ragflow::RagflowError::Api {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: "error retrieving document after update".to_string(),
            }));
        }

        let document: ComhairleDocument = (&documents[0]).into();

        Ok((status, document))
    }

    #[instrument(err(Debug))]
    async fn delete_document(
        &self,
        document_id: String,
        knowledge_base_id: String,
    ) -> Result<StatusCode, ComhairleError> {
        let status =
            ragflow::document::delete(&self.client, &document_id, &knowledge_base_id).await?;

        Ok(status)
    }

    async fn parse_document(
        &self,
        document_id: String,
        knowledge_base_id: String,
    ) -> Result<StatusCode, ComhairleError> {
        let body = ParseDocuments {
            document_ids: vec![&document_id],
        };
        let (status, _) = ragflow::document::parse(&self.client, &knowledge_base_id, body).await?;

        Ok(status)
    }

    async fn stop_parsing_document(
        &self,
        document_id: String,
        knowledge_base_id: String,
    ) -> Result<StatusCode, ComhairleError> {
        let body = ParseDocuments {
            document_ids: vec![&document_id],
        };
        let status = ragflow::document::stop_parse(&self.client, &knowledge_base_id, body).await?;

        Ok(status)
    }

    async fn download_document(
        &self,
        document_id: &str,
        knowledge_base_id: String,
    ) -> Result<reqwest::Response, ComhairleError> {
        let response = ragflow::document::download(&self.client, document_id, &knowledge_base_id)
            .await
            .map_err(RagflowError::from)?;

        Ok(response)
    }

    #[instrument(err(Debug))]
    async fn get_chat(&self, chat_id: &str) -> Result<(StatusCode, ComhairleChat), ComhairleError> {
        let params = GetQueryParams {
            id: Some(chat_id.to_string()),
            ..Default::default()
        };

        let (status, chats) = ragflow::chat::list(&self.client, Some(params)).await?;

        let chat: ComhairleChat = (&chats[0]).into();

        Ok((status, chat))
    }

    #[instrument(err(Debug))]
    async fn list_chats(
        &self,
        params: Option<ApiGetQueryParams>,
    ) -> Result<(StatusCode, Vec<ComhairleChat>), ComhairleError> {
        let params: Option<GetQueryParams> = params.map(|p| p.into());

        let (status, chats) = ragflow::chat::list(&self.client, params).await?;

        let chats: Vec<ComhairleChat> = chats.into_iter().map(Into::into).collect();

        Ok((status, chats))
    }

    #[instrument(err(Debug))]
    async fn create_chat(
        &self,
        body: CreateChatRequest,
    ) -> Result<(StatusCode, ComhairleChat), ComhairleError> {
        let body: CreateChat = body.into();

        let (status, chat) = ragflow::chat::create(&self.client, body).await?;

        let chat: ComhairleChat = chat.into();

        Ok((status, chat))
    }

    #[instrument(err(Debug))]
    async fn update_chat(
        &self,
        chat_id: &str,
        body: UpdateChatRequest,
    ) -> Result<(StatusCode, ComhairleChat), ComhairleError> {
        let body: UpdateChat = body.into();

        let status = ragflow::chat::update(&self.client, chat_id, body).await?;

        let params = GetQueryParams {
            id: Some(chat_id.to_string()),
            ..Default::default()
        };

        let (_, chats) = ragflow::chat::list(&self.client, Some(params)).await?;

        if chats.is_empty() || chats.len() > 1 {
            return Err(ComhairleError::RagflowError(RagflowError::Api {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: "error retrieving chat after update".to_string(),
            }));
        }

        let chat: ComhairleChat = (&chats[0]).into();

        Ok((status, chat))
    }

    #[instrument(err(Debug))]
    async fn delete_chat(&self, chat_id: &str) -> Result<StatusCode, ComhairleError> {
        let body = DeleteResources { ids: vec![chat_id] };

        let status = ragflow::chat::delete(&self.client, body).await?;

        Ok(status)
    }

    #[instrument(err(Debug))]
    async fn get_chat_session(
        &self,
        session_id: &str,
        chat_id: &str,
    ) -> Result<(StatusCode, ComhairleChatSession), ComhairleError> {
        let params = GetQueryParams {
            id: Some(session_id.to_string()),
            ..Default::default()
        };

        let (status, chat_sessions) =
            ragflow::chat::session::list(&self.client, chat_id, Some(params)).await?;

        let mut chat_session: ComhairleChatSession = (&chat_sessions[0]).into();
        self.enrich_reference_positions(&mut chat_session).await;

        Ok((status, chat_session))
    }

    #[instrument(err(Debug))]
    async fn list_chat_sessions(
        &self,
        chat_id: &str,
        params: Option<ApiGetQueryParams>,
    ) -> Result<(StatusCode, Vec<ComhairleChatSession>), ComhairleError> {
        let params: Option<GetQueryParams> = params.map(|p| p.into());

        let (status, chat_sessions) =
            ragflow::chat::session::list(&self.client, chat_id, params).await?;

        let chat_sessions: Vec<ComhairleChatSession> =
            chat_sessions.into_iter().map(Into::into).collect();

        Ok((status, chat_sessions))
    }

    #[instrument(err(Debug))]
    async fn create_chat_session(
        &self,
        chat_id: &str,
        body: CreateChatSessionRequest,
    ) -> Result<(StatusCode, ComhairleChatSession), ComhairleError> {
        let body: CreateChatSession = body.into();

        let (status, chat_session) =
            ragflow::chat::session::create(&self.client, chat_id, body).await?;

        let chat_session: ComhairleChatSession = chat_session.into();

        Ok((status, chat_session))
    }

    #[instrument(err(Debug))]
    async fn update_chat_session(
        &self,
        session_id: &str,
        chat_id: &str,
        body: UpdateChatSessionRequest,
    ) -> Result<(StatusCode, ComhairleChatSession), ComhairleError> {
        let body: UpdateChatSession = body.into();

        let status =
            ragflow::chat::session::update(&self.client, session_id, chat_id, body).await?;

        let params = GetQueryParams {
            id: Some(session_id.to_string()),
            ..Default::default()
        };
        let (_, chat_sessions) =
            ragflow::chat::session::list(&self.client, chat_id, Some(params)).await?;

        if chat_sessions.is_empty() || chat_sessions.len() > 1 {
            return Err(ComhairleError::RagflowError(ragflow::RagflowError::Api {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: "error retrieving session after update".to_string(),
            }));
        }

        let chat_session: ComhairleChatSession = (&chat_sessions[0]).into();

        Ok((status, chat_session))
    }

    #[instrument(err(Debug))]
    async fn delete_chat_session(
        &self,
        session_id: &str,
        chat_id: &str,
    ) -> Result<StatusCode, ComhairleError> {
        let body = DeleteResources {
            ids: vec![&session_id],
        };

        let status = ragflow::chat::session::delete(&self.client, chat_id, body).await?;

        Ok(status)
    }

    #[instrument(err(Debug))]
    async fn converse_with_chat(
        &self,
        session_id: &str,
        chat_id: &str,
        body: ChatConversationRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<Bytes, ComhairleError>> + Send + 'static>>,
        ComhairleError,
    > {
        let mut body: ConvoQuestion = body.into();
        body.session_id = Some(session_id.to_string());

        let stream =
            ragflow::chat::session::stream_chat_conversation(&self.client, chat_id, body).await?;

        let mapped_stream = Box::pin(stream.map(|item| item.map_err(ComhairleError::from)));

        intercept_ragflow_stream(mapped_stream).await
    }

    async fn get_agent(
        &self,
        agent_id: &str,
    ) -> Result<(StatusCode, ComhairleAgent), ComhairleError> {
        let params = GetQueryParams {
            id: Some(agent_id.to_string()),
            ..Default::default()
        };

        let (status, agents) = ragflow::agent::list(&self.client, Some(params)).await?;

        let agent: ComhairleAgent = (&agents[0]).into();

        Ok((status, agent))
    }

    async fn list_agents(
        &self,
        params: Option<ApiGetQueryParams>,
    ) -> Result<(StatusCode, Vec<ComhairleAgent>), ComhairleError> {
        let params: Option<GetQueryParams> = params.map(|p| p.into());

        let (status, agents) = ragflow::agent::list(&self.client, params).await?;

        let agents: Vec<ComhairleAgent> = agents.into_iter().map(Into::into).collect();

        Ok((status, agents))
    }

    async fn create_agent(
        &self,
        body: CreateAgentRequest,
    ) -> Result<(StatusCode, ComhairleAgent), ComhairleError> {
        let mut body: CreateAgent = body.into();
        let title = body.title.clone();

        let dsl = build_agent_dsl()?;
        body.dsl = dsl;

        let (status, json) = ragflow::agent::create(&self.client, body).await?;

        if !json.data {
            return Err(ComhairleError::RagflowError(ragflow::RagflowError::Api {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: "Error creating agent".to_string(),
            }));
        }

        let params = GetQueryParams {
            title: Some(title),
            ..Default::default()
        };
        let (_, agents) = ragflow::agent::list(&self.client, Some(params)).await?;

        if agents.is_empty() || agents.len() > 1 {
            return Err(ComhairleError::RagflowError(ragflow::RagflowError::Api {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: "Error retrieving agent after creation".to_string(),
            }));
        }

        let agent: ComhairleAgent = (&agents[0]).into();

        Ok((status, agent))
    }

    async fn update_agent(
        &self,
        agent_id: &str,
        body: UpdateAgentRequest,
    ) -> Result<(StatusCode, ComhairleAgent), ComhairleError> {
        let body: UpdateAgent = body.into();

        let (status, _) = ragflow::agent::update(&self.client, agent_id, body).await?;

        let params = GetQueryParams {
            id: Some(agent_id.to_string()),
            ..Default::default()
        };
        let (_, agents) = ragflow::agent::list(&self.client, Some(params)).await?;

        if agents.is_empty() || agents.len() > 1 {
            return Err(ComhairleError::RagflowError(ragflow::RagflowError::Api {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: "error retrieving agent after update".to_string(),
            }));
        }

        let agent: ComhairleAgent = (&agents[0]).into();

        Ok((status, agent))
    }

    async fn delete_agent(&self, agent_id: &str) -> Result<StatusCode, ComhairleError> {
        let status = ragflow::agent::delete(&self.client, agent_id).await?;

        Ok(status)
    }

    async fn get_agent_session(
        &self,
        session_id: &str,
        agent_id: &str,
    ) -> Result<(StatusCode, ComhairleAgentSession), ComhairleError> {
        let params = GetQueryParams {
            id: Some(session_id.to_string()),
            ..Default::default()
        };

        let (status, agent_sessions) =
            ragflow::agent::session::list(&self.client, agent_id, Some(params)).await?;

        let agent_session: ComhairleAgentSession = (&agent_sessions[0]).into();

        Ok((status, agent_session))
    }

    async fn list_agent_sessions(
        &self,
        agent_id: &str,
        params: Option<ApiGetQueryParams>,
    ) -> Result<(StatusCode, Vec<ComhairleAgentSession>), ComhairleError> {
        let params: Option<GetQueryParams> = params.map(|p| p.into());

        let (status, agent_sessions) =
            ragflow::agent::session::list(&self.client, agent_id, params).await?;

        let agent_sessions: Vec<ComhairleAgentSession> =
            agent_sessions.into_iter().map(Into::into).collect();

        Ok((status, agent_sessions))
    }

    async fn create_agent_session(
        &self,
        agent_id: &str,
    ) -> Result<(StatusCode, ComhairleAgentSession), ComhairleError> {
        let (status, agent_session) =
            ragflow::agent::session::create(&self.client, agent_id).await?;

        let agent_session: ComhairleAgentSession = agent_session.into();

        Ok((status, agent_session))
    }

    async fn delete_agent_session(
        &self,
        session_id: &str,
        agent_id: &str,
    ) -> Result<StatusCode, ComhairleError> {
        let body = DeleteResources {
            ids: vec![&session_id],
        };

        let status = ragflow::agent::session::delete(&self.client, agent_id, body).await?;

        Ok(status)
    }

    async fn converse_with_agent(
        &self,
        session_id: Option<&str>,
        agent_id: &str,
        body: AgentConversationRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<Bytes, ComhairleError>> + Send + 'static>>,
        ComhairleError,
    > {
        let mut body: ConvoQuestion = body.into();
        body.session_id = session_id.map(|id| id.to_string());

        let stream =
            ragflow::agent::session::stream_agent_conversation(&self.client, agent_id, body)
                .await?;

        let mapped_stream = Box::pin(stream.map(|item| item.map_err(ComhairleError::from)));

        intercept_ragflow_stream(mapped_stream).await
    }

    async fn parse_sse_stream_to_events(
        &self,
        stream: Pin<Box<dyn Stream<Item = Result<Bytes, ComhairleError>> + Send + 'static>>,
    ) -> Result<Vec<BotServiceSseEvent>, ComhairleError> {
        let events = parse_sse_stream(stream)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(events)
    }
}

/// Peek at first stream chunks to check for `**ERROR**:` answer chunks. Prevent
/// such chunks reaching frontend UI by returning [`ComhairleError`] if such
/// chunks are detected.
async fn intercept_ragflow_stream(
    mut stream: Pin<Box<dyn Stream<Item = Result<Bytes, ComhairleError>> + Send + 'static>>,
) -> Result<
    Pin<Box<dyn Stream<Item = Result<Bytes, ComhairleError>> + Send + 'static>>,
    ComhairleError,
> {
    if let Some(first) = stream.next().await {
        let first = first?;

        if extract_ragflow_stream_error(&first).is_some() {
            return Err(ComhairleError::StreamChunkError(
                "Chunk contains ragflow '**ERROR**:' message. Aborting.".to_string(),
            ));
        }

        // No error - put the chunk back on the front of the stream.
        let head = stream::once(async move { Ok(first) });
        return Ok(Box::pin(head.chain(stream)));
    }

    Ok(stream)
}

fn extract_ragflow_stream_error(bytes: &Bytes) -> Option<String> {
    let text = std::str::from_utf8(bytes).ok()?;

    for line in text.lines() {
        let line = line.trim();
        let Some(payload) = line.strip_prefix("data:") else {
            continue;
        };

        let Ok(json) = serde_json::from_str::<serde_json::Value>(payload.trim()) else {
            continue;
        };

        if let Some(answer) = json.pointer("/data/answer").and_then(|v| v.as_str())
            && answer.trim_start().starts_with("**ERROR**:")
        {
            return Some(answer.to_string());
        }
    }

    None
}

fn build_agent_dsl() -> Result<serde_json::Value, ComhairleError> {
    let graph_json: Value = from_str(include_str!(
        "../agent_templates/ragflow-elicitation-bot.json"
    ))?;
    let mut dsl: Value = from_str(include_str!(
        "../agent_templates/ragflow-agent-static-dsl-content.json"
    ))?;
    dsl.as_object_mut()
        .ok_or(ComhairleError::CorruptedData(
            "json template must be an object".to_string(),
        ))?
        .insert("graph".to_string(), graph_json.clone());

    Ok(dsl)
}

pub async fn parse_sse_stream(
    mut stream: Pin<Box<dyn Stream<Item = Result<Bytes, ComhairleError>> + Send + 'static>>,
) -> Result<Vec<SseEvent>, ComhairleError> {
    let mut raw_bytes = vec![];
    while let Some(chunk) = stream.next().await {
        let bytes = chunk?;
        raw_bytes.extend_from_slice(&bytes);
    }

    let raw_str = String::from_utf8(raw_bytes).map_err(|_| {
        ComhairleError::CorruptedData("Invalid UTF-8 in bot service response".to_string())
    })?;

    Ok(parse_sse_str(&raw_str))
}

pub fn parse_sse_str(sse_str: &str) -> Vec<SseEvent> {
    sse_str
        .lines()
        .filter(|line| line.starts_with("data:") && !line.ends_with("[DONE]"))
        .map(|line| line.trim_start_matches("data:").trim())
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .filter_map(|value| serde_json::from_value::<SseEvent>(value).ok())
        .collect()
}

//
// From conversions
//

impl From<ApiGetQueryParams> for GetQueryParams {
    fn from(params: ApiGetQueryParams) -> Self {
        Self {
            page: params.page,
            page_size: params.page_size,
            orderby: params.order_by,
            name: params.name,
            id: None,
            desc: None,
            title: params.title,
        }
    }
}

impl From<Dataset> for ComhairleKnowledgeBase {
    fn from(input: Dataset) -> Self {
        Self {
            id: input.id,
            name: input.name,
        }
    }
}

impl From<&Dataset> for ComhairleKnowledgeBase {
    fn from(input: &Dataset) -> Self {
        Self {
            id: input.id.clone(),
            name: input.name.clone(),
        }
    }
}

impl From<UpdateKnowledgeBaseRequest> for UpdateDataset {
    fn from(input: UpdateKnowledgeBaseRequest) -> Self {
        Self {
            name: input.name,
            description: None,
        }
    }
}

impl From<Document> for ComhairleDocument {
    fn from(input: Document) -> Self {
        Self {
            id: input.id,
            name: input.name,
            parse_progress: input.progress.unwrap_or(0.0),
            parse_status: input.run.unwrap_or("RUNNING".to_string()),
            size: input.size,
        }
    }
}

impl From<&Document> for ComhairleDocument {
    fn from(input: &Document) -> Self {
        Self {
            id: input.id.clone(),
            name: input.name.clone(),
            parse_progress: input.progress.unwrap_or(0.0),
            parse_status: input.run.clone().unwrap_or("RUNNING".to_string()),
            size: input.size,
        }
    }
}

impl From<UploadFileRequest> for UploadFile {
    fn from(input: UploadFileRequest) -> Self {
        Self {
            filename: input.filename,
            bytes: input.bytes,
        }
    }
}

impl From<UpdateDocumentRequest> for UpdateDocument {
    fn from(input: UpdateDocumentRequest) -> Self {
        Self {
            name: input.name,
            ..Default::default()
        }
    }
}

impl From<&Chat> for ComhairleChat {
    fn from(chat: &Chat) -> Self {
        Self {
            id: chat.id.clone(),
            name: chat.name.clone(),
            llm_model: chat.llm.as_ref().map(Into::into),
            prompt: chat.prompt.as_ref().map(Into::into),
            knowledge_base_ids: chat
                .datasets
                .iter()
                .flat_map(|v| v.iter())
                .map(|d| d.id.clone())
                .collect(),
        }
    }
}

impl From<Chat> for ComhairleChat {
    fn from(chat: Chat) -> Self {
        Self {
            id: chat.id,
            name: chat.name,
            llm_model: chat.llm.map(Into::into),
            prompt: chat.prompt.map(Into::into),
            knowledge_base_ids: chat
                .datasets
                .unwrap_or_default()
                .iter()
                .map(|d| d.id.clone())
                .collect(),
        }
    }
}

impl From<Llm> for ComhairleLlm {
    fn from(input: Llm) -> Self {
        Self {
            model_name: input.model_name,
        }
    }
}

impl From<&Llm> for ComhairleLlm {
    fn from(input: &Llm) -> Self {
        Self {
            model_name: input.model_name.clone(),
        }
    }
}

impl From<Prompt> for ComhairlePrompt {
    fn from(input: Prompt) -> Self {
        Self {
            llm_prompt: input.prompt,
            opener: input.opener,
            empty_response: input.empty_response,
            cross_languages: input.cross_languages,
        }
    }
}

impl From<&Prompt> for ComhairlePrompt {
    fn from(input: &Prompt) -> Self {
        Self {
            llm_prompt: input.prompt.clone(),
            opener: input.opener.clone(),
            empty_response: input.empty_response.clone(),
            cross_languages: input.cross_languages.clone(),
        }
    }
}

impl From<ComhairlePrompt> for Prompt {
    fn from(input: ComhairlePrompt) -> Self {
        Self {
            prompt: input.llm_prompt,
            opener: input.opener,
            empty_response: input.empty_response,
            cross_languages: input.cross_languages,
            ..Default::default()
        }
    }
}

impl From<CreateChatRequest> for CreateChat {
    fn from(input: CreateChatRequest) -> Self {
        Self {
            name: input.name,
            avatar: None,
            dataset_ids: input.knowledge_base_ids.unwrap_or_default(),
            llm: input.llm_model.map(|model| Llm {
                model_name: model.model_name,
            }),
            prompt: input.prompt.map(|prompt| Prompt {
                variables: Some(vec![Variable {
                    key: "knowledge".to_string(),
                    optional: false,
                }]),
                ..prompt.into()
            }),
        }
    }
}

impl From<UpdateChatRequest> for UpdateChat {
    fn from(input: UpdateChatRequest) -> Self {
        Self {
            name: input.name,
            dataset_ids: input.knowledge_base_ids,
            llm: input.llm_model.map(|model| Llm {
                model_name: model.model_name,
            }),
            prompt: input.prompt.map(|prompt| Prompt { ..prompt.into() }),
            ..Default::default()
        }
    }
}

impl From<ChatSession> for ComhairleChatSession {
    fn from(session: ChatSession) -> Self {
        Self {
            id: session.id,
            chat_id: session.chat_id,
            name: session.name,
            messages: reassociate_message_references(
                session.messages.into_iter().map(Into::into).collect(),
            ),
        }
    }
}

impl From<&ChatSession> for ComhairleChatSession {
    fn from(session: &ChatSession) -> Self {
        Self {
            id: session.id.clone(),
            chat_id: session.chat_id.clone(),
            name: session.name.clone(),
            messages: reassociate_message_references(
                session.messages.iter().map(Into::into).collect(),
            ),
        }
    }
}

/// Fix RAGFlow's off-by-one association between an answer and its retrieval
/// references in stored session history
///
/// The session always opens with a canned greeting assistant message that never
/// ran a retrieval, but it still occupies the first assistant slot. RAGFlow's
/// reference list has one entry per *answered* question, and the history payload
/// zips those entries onto assistant messages starting from that opener. The net
/// effect is a one-turn shift: the opener carries answer 1's chunks, answer 1
/// carries answer 2's chunks, and the most recent answer comes back with no
/// reference at all. The `[ID:N]` markers in each answer index positionally into
/// its reference list, so this shift makes citations resolve against the wrong
/// chunks (or, for the latest answer, against nothing).
///
/// We undo it by shifting references forward one assistant turn: each assistant
/// message takes the reference that was parked on the previous assistant
/// message, and the opener is left with none. This is only valid when the
/// session actually starts with that opener, which is why we bail out unless the
/// first message is an assistant one.
fn reassociate_message_references(
    mut messages: Vec<ComhairleSessionMessage>,
) -> Vec<ComhairleSessionMessage> {
    if messages
        .first()
        .map(|m| m.role != "assistant")
        .unwrap_or(true)
    {
        return messages;
    }

    let mut carry: Option<Vec<ComhairleMessageReference>> = None;
    for message in messages.iter_mut() {
        if message.role != "assistant" {
            continue;
        }
        let stored = message.reference.take();
        message.reference = carry;
        carry = stored;
    }
    messages
}

impl From<SessionMessage> for ComhairleSessionMessage {
    fn from(message: SessionMessage) -> Self {
        Self {
            id: message.id.unwrap_or("".to_string()),
            content: message.content,
            role: message.role,
            reference: message
                .reference
                .map(|refs| refs.into_iter().map(Into::into).collect()),
        }
    }
}

impl From<&SessionMessage> for ComhairleSessionMessage {
    fn from(message: &SessionMessage) -> Self {
        Self {
            id: message.id.clone().unwrap_or("".to_string()),
            content: message.content.clone(),
            role: message.role.clone(),
            reference: message
                .reference
                .clone()
                .map(|refs| refs.into_iter().map(Into::into).collect()),
        }
    }
}

impl From<MessageReference> for ComhairleMessageReference {
    fn from(r: MessageReference) -> Self {
        Self {
            id: r.id,
            content: r.content,
            dataset_id: r.dataset_id,
            document_id: r.document_id,
            document_name: r.document_name,
            // History omits positions; enriched from the chunk store on reload.
            positions: None,
        }
    }
}

impl From<&MessageReference> for ComhairleMessageReference {
    fn from(r: &MessageReference) -> Self {
        Self {
            id: r.id.clone(),
            content: r.content.clone(),
            dataset_id: r.dataset_id.clone(),
            document_id: r.document_id.clone(),
            document_name: r.document_name.clone(),
            positions: None,
        }
    }
}

impl From<CreateChatSessionRequest> for CreateChatSession {
    fn from(input: CreateChatSessionRequest) -> Self {
        Self {
            name: input.name,
            user_id: None,
        }
    }
}

impl From<UpdateChatSessionRequest> for UpdateChatSession {
    fn from(input: UpdateChatSessionRequest) -> Self {
        Self {
            name: input.name,
            user_id: input.user_id,
        }
    }
}

impl From<ChatConversationRequest> for ConvoQuestion {
    fn from(input: ChatConversationRequest) -> Self {
        Self {
            question: input.question,
            session_id: None,
            user_id: None,
            stream: Some(true),
            inputs: None,
        }
    }
}

impl From<Agent> for ComhairleAgent {
    fn from(input: Agent) -> Self {
        Self {
            id: input.id,
            name: input.title.unwrap_or_default(),
            configuration: input.dsl,
        }
    }
}

impl From<&Agent> for ComhairleAgent {
    fn from(input: &Agent) -> Self {
        Self {
            id: input.id.clone(),
            name: input.title.clone().unwrap_or_default(),
            configuration: input.dsl.clone(),
        }
    }
}

impl From<UpdateAgentRequest> for UpdateAgent {
    fn from(input: UpdateAgentRequest) -> Self {
        Self {
            title: input.name,
            dsl: None,
        }
    }
}

impl From<CreateAgentRequest> for CreateAgent {
    fn from(input: CreateAgentRequest) -> Self {
        Self {
            title: input.name,
            dsl: serde_json::json!({}),
        }
    }
}

impl From<AgentSession> for ComhairleAgentSession {
    fn from(input: AgentSession) -> Self {
        Self {
            id: input.id,
            agent_id: input.agent_id,
            configuration: input.dsl,
            messages: input
                .messages
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<&AgentSession> for ComhairleAgentSession {
    fn from(input: &AgentSession) -> Self {
        Self {
            id: input.id.clone(),
            agent_id: input.agent_id.clone(),
            configuration: input.dsl.clone(),
            messages: input
                .messages
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<AgentConversationRequest> for ConvoQuestion {
    fn from(a: AgentConversationRequest) -> Self {
        let mut inputs = HashMap::new();
        if let Some(topic) = a.topic {
            inputs.insert(
                "topic".to_string(),
                Input {
                    r#type: "line".to_string(),
                    value: topic,
                },
            );
        }
        if let Some(history) = a.history {
            inputs.insert(
                "history".to_string(),
                Input {
                    r#type: "line".to_string(),
                    value: history,
                },
            );
        }
        if let Some(starting_question) = a.starting_question {
            inputs.insert(
                "starting_question".to_string(),
                Input {
                    r#type: "line".to_string(),
                    value: starting_question,
                },
            );
        }
        if let Some(question_intent) = a.question_intent {
            inputs.insert(
                "question_intent".to_string(),
                Input {
                    r#type: "line".to_string(),
                    value: question_intent,
                },
            );
        }
        if let Some(survey_responses) = a.survey_responses {
            inputs.insert(
                "survey_responses".to_string(),
                Input {
                    r#type: "line".to_string(),
                    value: survey_responses,
                },
            );
        }
        Self {
            question: a.question,
            session_id: None,
            user_id: None,
            stream: Some(true),
            inputs: Some(inputs),
        }
    }
}

impl From<SseEvent> for BotServiceSseEvent {
    fn from(e: SseEvent) -> Self {
        Self {
            event: e.event,
            component_type: e.data.component_type,
            content: e
                .data
                .content
                .or_else(|| e.data.outputs.and_then(|o| o.content)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;

    #[tokio::test]
    async fn should_parse_sse_event_str_to_json() -> Result<(), Box<dyn Error>> {
        let test_sse_str = include_str!("../../../fixtures/bot-service-sse-events.txt");

        let _results = parse_sse_str(test_sse_str);

        Ok(())
    }

    #[tokio::test]
    async fn should_parse_sse_event_str_to_comhairle_type() -> Result<(), Box<dyn Error>> {
        let test_sse_str = include_str!("../../../fixtures/bot-service-sse-events.txt");
        let results = parse_sse_str(test_sse_str);

        let _results: Vec<BotServiceSseEvent> = results.into_iter().map(Into::into).collect();

        Ok(())
    }

    fn msg(role: &str, id: &str, ref_ids: Option<Vec<&str>>) -> ComhairleSessionMessage {
        ComhairleSessionMessage {
            id: id.to_string(),
            content: String::new(),
            role: role.to_string(),
            reference: ref_ids.map(|ids| {
                ids.into_iter()
                    .map(|rid| ComhairleMessageReference {
                        id: rid.to_string(),
                        ..Default::default()
                    })
                    .collect()
            }),
        }
    }

    fn reference_ids(message: &ComhairleSessionMessage) -> Option<Vec<String>> {
        message
            .reference
            .as_ref()
            .map(|refs| refs.iter().map(|r| r.id.clone()).collect())
    }

    #[test]
    fn reassociates_shifted_history_references() {
        // Mirrors RAGFlow's stored session (issue #791): the opener carries
        // answer 1's chunks, answer 1 carries answer 2's chunks, and the latest
        // answer comes back with none.
        let messages = vec![
            msg("assistant", "", Some(vec!["r1"])),
            msg("user", "q1", None),
            msg("assistant", "a1", Some(vec!["r2"])),
            msg("user", "q2", None),
            msg("assistant", "a2", None),
        ];

        let fixed = reassociate_message_references(messages);

        // Opener drops its (bogus) reference; each answer recovers its own.
        assert!(fixed[0].reference.is_none());
        assert_eq!(reference_ids(&fixed[2]), Some(vec!["r1".to_string()]));
        assert_eq!(reference_ids(&fixed[4]), Some(vec!["r2".to_string()]));
        // User turns are never touched.
        assert!(fixed[1].reference.is_none());
        assert!(fixed[3].reference.is_none());
    }

    #[test]
    fn leaves_references_untouched_without_an_opener() {
        // No leading assistant opener means no shift to undo; passing through
        // avoids corrupting a correctly-aligned history.
        let messages = vec![
            msg("user", "q1", None),
            msg("assistant", "a1", Some(vec!["r1"])),
        ];

        let fixed = reassociate_message_references(messages);

        assert_eq!(reference_ids(&fixed[1]), Some(vec!["r1".to_string()]));
    }

    fn sse_stream_chunk(answer: &str) -> Bytes {
        Bytes::from(format!(r#"data:{{"data": {{"answer": "{answer}"}}}}"#))
    }

    #[test]
    fn returns_none_for_normal_answer_chunk() {
        let chunk = sse_stream_chunk("Some test chunk");
        assert_eq!(extract_ragflow_stream_error(&chunk), None);
    }

    #[test]
    fn detects_error_in_answer() {
        let chunk = sse_stream_chunk("**ERROR**: (Test error)");
        let result = extract_ragflow_stream_error(&chunk);
        assert!(result.is_some(), "Error chunk not detected");
        assert!(
            result.unwrap().starts_with("**ERROR**:"),
            "Error chunk data incorrectly passed through"
        );
    }

    #[test]
    fn returns_none_when_answer_missing() {
        let chunk = Bytes::from(r#"data:{"code": 0, "data": {"other_field": "Some test chunk"}}"#);
        assert_eq!(
            extract_ragflow_stream_error(&chunk),
            None,
            "ERROR chunk detected"
        );
    }

    #[tokio::test]
    async fn errors_out_when_first_chunk_is_error() -> Result<(), Box<dyn Error>> {
        let chunks = stream::iter(vec![
            Ok(sse_stream_chunk("**ERROR**: (Some bad text)")),
            Ok(sse_stream_chunk("Some safe text")),
        ]);
        let boxed: Pin<Box<dyn Stream<Item = _> + Send>> = Box::pin(chunks);

        let result = intercept_ragflow_stream(boxed).await;
        assert!(
            matches!(result, Err(ComhairleError::StreamChunkError(_))),
            "Error chunk not detected"
        );

        Ok(())
    }

    #[tokio::test]
    async fn passes_through_all_chunks_when_no_error() -> Result<(), Box<dyn Error>> {
        let chunks = stream::iter(vec![
            Ok(sse_stream_chunk("Some safe text")),
            Ok(sse_stream_chunk("Some more safe text")),
        ]);
        let boxed: Pin<Box<dyn Stream<Item = _> + Send>> = Box::pin(chunks);

        let result = intercept_ragflow_stream(boxed).await?;
        let chunks: Vec<_> = result.collect().await;

        assert_eq!(chunks.len(), 2);
        let first_chunk = std::str::from_utf8(chunks[0].as_ref().unwrap()).unwrap();
        let second_chunk = std::str::from_utf8(chunks[1].as_ref().unwrap()).unwrap();

        assert!(
            first_chunk.contains("Some safe text"),
            "First chunk incorrect data"
        );
        assert!(
            second_chunk.contains("Some more safe text"),
            "Second chunk incorrect data"
        );

        Ok(())
    }

    #[tokio::test]
    async fn empty_stream_returns_empty_stream() -> Result<(), Box<dyn Error>> {
        let chunks = stream::iter(Vec::<Result<Bytes, ComhairleError>>::new());
        let boxed: Pin<Box<dyn Stream<Item = _> + Send>> = Box::pin(chunks);

        let result = intercept_ragflow_stream(boxed).await?;
        let chunks: Vec<_> = result.collect().await;

        assert!(chunks.is_empty(), "Stream chunks not empty");

        Ok(())
    }

    #[tokio::test]
    async fn comhairle_error_propogated_as_is() -> Result<(), Box<dyn Error>> {
        let chunks = stream::iter(vec![Err(ComhairleError::BadRequest(
            "missing param".to_string(),
        ))]);
        let boxed: Pin<Box<dyn Stream<Item = _> + Send>> = Box::pin(chunks);

        let result = intercept_ragflow_stream(boxed).await;

        assert!(
            matches!(result, Err(ComhairleError::BadRequest(_))),
            "Comhairle error not propogated"
        );

        Ok(())
    }

    #[tokio::test]
    async fn error_chunks_after_first_are_passed_through() -> Result<(), Box<dyn Error>> {
        let chunks = stream::iter(vec![
            Ok(sse_stream_chunk("Some safe text")),
            Ok(sse_stream_chunk("**ERROR**: (something bad)")),
        ]);
        let boxed: Pin<Box<dyn Stream<Item = _> + Send>> = Box::pin(chunks);

        let result = intercept_ragflow_stream(boxed).await?;
        let chunks: Vec<_> = result.collect().await;

        let first_chunk = std::str::from_utf8(chunks[0].as_ref().unwrap()).unwrap();
        let second_chunk = std::str::from_utf8(chunks[1].as_ref().unwrap()).unwrap();

        assert_eq!(chunks.len(), 2);
        assert!(
            first_chunk.contains("Some safe text"),
            "First chunk incorrect data"
        );
        assert!(
            second_chunk.contains("**ERROR**: (something bad)"),
            "Second chunk incorrect data"
        );

        Ok(())
    }
}
