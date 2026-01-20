use std::{pin::Pin, sync::Arc};

use async_trait::async_trait;
use axum::body::Bytes;
use futures::{Stream, StreamExt};
use minijinja::{context, Environment, Value};
use minijinja_embed::load_templates;
use ragflow::client::RagflowClient;
use ragflow::{
    agent::{session::*, *},
    chat::{session::*, *},
    dataset::*,
    document::*,
    ConvoQuestion, DeleteResources, GetQueryParams, RagflowError,
};
use reqwest::StatusCode;
use tracing::instrument;

use crate::{
    bot_service::{
        ComhairleAgent, ComhairleAgentSession, ComhairleBotService, ComhairleChat,
        ComhairleChatSession, ComhairleDocument, ComhairleKnowledgeBase, ComhairleLlm,
        ComhairleMessageReference, ComhairlePrompt, ComhairleSessionMessage,
    },
    error::ComhairleError,
    routes::{
        bot::{
            agent_sessions::{
                AgentConversationRequest, CreateAgentSessionRequest, UpdateAgentSessionRequest,
            },
            agents::{CreateAgentRequest, UpdateAgentRequest},
            chat_sessions::{
                ChatConversationRequest, CreateChatSessionRequest as ApiCreateChatSessionRequest,
                UpdateChatSessionRequest as ApiUpdateChatSessionRequest,
            },
            chats::{CreateChatRequest, UpdateChatRequest},
            documents::UpdateDocumentRequest,
            knowledge_bases::UpdateKnowledgeBaseRequest,
            GetQueryParams as ApiGetQueryParams,
        },
        conversations::UploadFileRequest,
    },
};

#[derive(Debug)]
pub struct ComhairleRagBotService {
    client: Arc<RagflowClient>,
    template_engine: Environment<'static>,
}

impl ComhairleRagBotService {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        let mut env = Environment::new();
        load_templates!(&mut env);
        ComhairleRagBotService {
            client: Arc::new(RagflowClient::new(
                base_url.to_string(),
                api_key.to_string(),
            )),
            template_engine: env,
        }
    }
}

#[async_trait]
impl ComhairleBotService for ComhairleRagBotService {
    fn render_from_template(
        &self,
        template: &str,
        context: Value,
    ) -> Result<String, ComhairleError> {
        let template = self.template_engine.get_template(template)?;
        let content = template.render(context)?;

        Ok(content)
    }

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
        knowledge_base_id: String,
    ) -> Result<StatusCode, ComhairleError> {
        let body = DeleteResources {
            ids: vec![&knowledge_base_id],
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
        document_id: String,
        knowledge_base_id: String,
    ) -> Result<reqwest::Response, ComhairleError> {
        let response = ragflow::document::download(&self.client, &document_id, &knowledge_base_id)
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

        let chat_session: ComhairleChatSession = (&chat_sessions[0]).into();

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
        body: ApiCreateChatSessionRequest,
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
        body: ApiUpdateChatSessionRequest,
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

        let mapped_stream = stream.map(|item| item.map_err(ComhairleError::from));

        Ok(Box::pin(mapped_stream))
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

        let content = self.render_from_template(
            "ragflow-elicitation-bot.json",
            context! { foo => "foo", bar => "bar" },
        )?;
        let graph_json: serde_json::Value = serde_json::from_str(&content).map_err(|_| {
            ComhairleError::CorruptedData("Unable to parse json from agent template".to_string())
        })?;

        // Additional json that isn't included in json exported from working agent
        // but is required to create an agent which allows sessions to be created
        // without any errors
        // TODO: clean up
        let globals = self.render_from_template("ragflow-globals.json", context! {})?;
        let globals: serde_json::Value = serde_json::from_str(&globals)?;
        let components = self.render_from_template("ragflow-components.json", context! {})?;
        let components: serde_json::Value = serde_json::from_str(&components)?;
        // TODO: currently hard coded to match exported json template
        // May need to be dynamically calculated from the template
        let path = self.render_from_template("ragflow-path.json", context! {})?;
        let path: serde_json::Value = serde_json::from_str(&path)?;
        let retrieval = self.render_from_template("ragflow-retrieval.json", context! {})?;
        let retrieval: serde_json::Value = serde_json::from_str(&retrieval)?;

        let dsl_json = serde_json::json!({ "graph": graph_json, "components": components, "globals": globals, "path": path, "history": [], "retrieval": retrieval });
        body.dsl = dsl_json;

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

    async fn list_agent_session(
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

    // Not supported by ragflow
    // Endpoint currently commented out
    async fn update_agent_session(
        &self,
        session_id: &str,
        agent_id: &str,
        body: UpdateAgentSessionRequest,
    ) -> Result<(StatusCode, ComhairleAgentSession), ComhairleError> {
        Ok((
            StatusCode::OK,
            ComhairleAgentSession {
                ..Default::default()
            },
        ))
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
        session_id: &str,
        agent_id: &str,
        body: AgentConversationRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<Bytes, ComhairleError>> + Send + 'static>>,
        ComhairleError,
    > {
        let mut body: ConvoQuestion = body.into();
        body.session_id = Some(session_id.to_string());

        let stream =
            ragflow::agent::session::stream_agent_conversation(&self.client, agent_id, body)
                .await?;

        let mapped_stream = stream.map(|item| item.map_err(ComhairleError::from));

        Ok(Box::pin(mapped_stream))
    }
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
        }
    }
}

impl From<&Prompt> for ComhairlePrompt {
    fn from(input: &Prompt) -> Self {
        Self {
            llm_prompt: input.prompt.clone(),
            opener: input.opener.clone(),
            empty_response: input.empty_response.clone(),
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
                prompt: prompt.llm_prompt,
                ..Default::default()
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
            prompt: input.prompt.map(|prompt| Prompt {
                prompt: prompt.llm_prompt,
                ..Default::default()
            }),
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
            messages: session.messages.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<&ChatSession> for ComhairleChatSession {
    fn from(session: &ChatSession) -> Self {
        Self {
            id: session.id.clone(),
            chat_id: session.chat_id.clone(),
            name: session.name.clone(),
            messages: session
                .messages
                .clone()
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<ChatSessionMessage> for ComhairleSessionMessage {
    fn from(message: ChatSessionMessage) -> Self {
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

impl From<&ChatSessionMessage> for ComhairleSessionMessage {
    fn from(message: &ChatSessionMessage) -> Self {
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
        }
    }
}

impl From<ApiCreateChatSessionRequest> for CreateChatSession {
    fn from(input: ApiCreateChatSessionRequest) -> Self {
        Self {
            name: input.name,
            user_id: input.user_id,
        }
    }
}

impl From<ApiUpdateChatSessionRequest> for UpdateChatSession {
    fn from(input: ApiUpdateChatSessionRequest) -> Self {
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
            user_id: input.user_id,
            stream: Some(true),
        }
    }
}

impl From<Agent> for ComhairleAgent {
    fn from(input: Agent) -> Self {
        Self {
            name: input.title.unwrap_or_default(),
            configuration: input.dsl,
        }
    }
}

impl From<&Agent> for ComhairleAgent {
    fn from(input: &Agent) -> Self {
        Self {
            name: input.title.clone().unwrap_or_default(),
            configuration: input.dsl.clone(),
        }
    }
}

impl From<UpdateAgentRequest> for UpdateAgent {
    fn from(input: UpdateAgentRequest) -> Self {
        Self { title: input.title }
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
            agent_id: input.agent_id,
            dsl: input.dsl,
        }
    }
}

impl From<&AgentSession> for ComhairleAgentSession {
    fn from(input: &AgentSession) -> Self {
        Self {
            agent_id: input.agent_id.clone(),
            dsl: input.dsl.clone(),
        }
    }
}

impl From<CreateAgentSessionRequest> for CreateAgentSession {
    fn from(input: CreateAgentSessionRequest) -> Self {
        Self {}
    }
}

impl From<AgentConversationRequest> for ConvoQuestion {
    fn from(input: AgentConversationRequest) -> Self {
        Self {
            question: input.question,
            session_id: None,
            user_id: None,
            stream: Some(true),
        }
    }
}
