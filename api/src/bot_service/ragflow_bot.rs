use std::pin::Pin;

use async_trait::async_trait;
use axum::body::Bytes;
use futures::{Stream, StreamExt};
use ragflow::{
    chat::session::*, chat::*, dataset::*, document::*, DeleteResources, GetQueryParams,
};
use reqwest::StatusCode;

use crate::{
    bot_service::{ComhairleBotService, ComhairleChatSession, ComhairleRagBotService},
    error::ComhairleError,
    routes::bot::{
        sessions::{
            ChatConversationRequest, CreateChatSessionRequest as ApiCreateChatSessionRequest,
            UpdateChatSessionRequest as ApiUpdateChatSessionRequest,
        },
        GetQueryParams as ApiGetQueryParams,
    },
};

#[async_trait]
impl ComhairleBotService for ComhairleRagBotService {
    async fn create_knowledge_base(
        &self,
        name: String,
        description: Option<String>,
    ) -> Result<(StatusCode, Dataset), ComhairleError> {
        let (status, knowledge_base) =
            ragflow::dataset::create(&self.client, name, description).await?;
        Ok((status, knowledge_base))
    }

    async fn delete_knowledge_base(&self, id: String) -> Result<StatusCode, ComhairleError> {
        let status = ragflow::dataset::delete(&self.client, &id).await?;
        Ok(status)
    }

    async fn get_documents(
        &self,
        knowledge_base_id: String,
        params: Option<ApiGetQueryParams>,
    ) -> Result<(StatusCode, Vec<Document>), ComhairleError> {
        let params: Option<GetQueryParams> = params.map(|p| p.into());

        let (status, documents) =
            ragflow::document::list(&self.client, &knowledge_base_id, params).await?;

        Ok((status, documents))
    }

    async fn delete_document(
        &self,
        id: String,
        knowledge_base_id: String,
    ) -> Result<StatusCode, ComhairleError> {
        let status = ragflow::document::delete(&self.client, &id, &knowledge_base_id).await?;
        Ok(status)
    }

    async fn upload_documents(
        &self,
        knowledge_base_id: &str,
        files: Vec<UploadFile>,
    ) -> Result<StatusCode, ComhairleError> {
        let (status, _) = ragflow::document::upload(&self.client, knowledge_base_id, files).await?;
        Ok(status)
    }

    async fn create_chat(&self, body: CreateChat) -> Result<(StatusCode, Chat), ComhairleError> {
        let (status, chat) = ragflow::chat::create(&self.client, body).await?;
        Ok((status, chat))
    }

    async fn update_chat(&self, id: &str, body: UpdateChat) -> Result<StatusCode, ComhairleError> {
        let status = ragflow::chat::update(&self.client, id, body).await?;
        Ok(status)
    }

    async fn delete_chats(&self, body: DeleteResources<'_>) -> Result<StatusCode, ComhairleError> {
        let status = ragflow::chat::delete(&self.client, body).await?;
        Ok(status)
    }

    async fn get_chats(
        &self,
        params: Option<ApiGetQueryParams>,
    ) -> Result<(StatusCode, Vec<Chat>), ComhairleError> {
        let params: Option<GetQueryParams> = params.map(|p| p.into());

        let (status, chats) = ragflow::chat::list(&self.client, params).await?;

        Ok((status, chats))
    }

    async fn get_chat_sessions(
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

    async fn update_chat_session(
        &self,
        session_id: &str,
        chat_id: &str,
        body: ApiUpdateChatSessionRequest,
    ) -> Result<StatusCode, ComhairleError> {
        let body: UpdateChatSession = body.into();

        let status =
            ragflow::chat::session::update(&self.client, session_id, chat_id, body).await?;

        Ok(status)
    }

    async fn delete_chat_sessions(
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

    async fn converse_with_chat(
        &self,
        chat_id: &str,
        body: ChatConversationRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<Bytes, ComhairleError>> + Send + 'static>>,
        ComhairleError,
    > {
        let body: ConvoQuestion = body.into();

        let stream =
            ragflow::chat::session::stream_chat_conversation(&self.client, chat_id, body).await?;

        let mapped_stream = stream.map(|item| item.map_err(ComhairleError::from));

        Ok(Box::pin(mapped_stream))
    }
}

impl From<ApiGetQueryParams> for GetQueryParams {
    fn from(params: ApiGetQueryParams) -> Self {
        Self {
            page: params.page,
            page_size: params.page_size,
            orderby: params.order_by,
            name: params.name,
            id: params.id,
            desc: None, // TODO:
        }
    }
}

impl From<ChatSession> for ComhairleChatSession {
    fn from(session: ChatSession) -> Self {
        Self {
            id: session.id,
            chat_id: session.chat_id,
            name: session.name,
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
            session_id: input.session_id,
            user_id: input.user_id,
            stream: Some(true),
        }
    }
}
