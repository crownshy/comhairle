use bytes::Bytes;
use futures::{Stream, TryStreamExt};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::client::RagflowClient;
use crate::error::Result;
use crate::{DeleteResources, GetQueryParams, RagflowError};

pub async fn create(
    client: &RagflowClient,
    chat_id: &str,
    body: CreateChatSession,
) -> Result<(StatusCode, ChatSession)> {
    let path = format!("/chats/{chat_id}/sessions");
    let (status, value) = client.post(&path, &body, None).await?;

    let json: CreateChatSessionResponse = serde_json::from_value(value)?;

    Ok((status, json.data))
}

pub async fn update(
    client: &RagflowClient,
    session_id: &str,
    chat_id: &str,
    body: UpdateChatSession,
) -> Result<StatusCode> {
    let path = format!("/chats/{chat_id}/sessions/{session_id}");

    let (status, _) = client.put(&path, &body, None).await?;

    Ok(status)
}

pub async fn delete(
    client: &RagflowClient,
    chat_id: &str,
    body: DeleteResources<'_>,
) -> Result<StatusCode> {
    let path = format!("/chats/{chat_id}/sessions");
    let status = client.delete(&path, &body, None).await?;

    Ok(status)
}

pub async fn list(
    client: &RagflowClient,
    chat_id: &str,
    params: Option<GetQueryParams>,
) -> Result<(StatusCode, Vec<ChatSession>)> {
    let path = format!("/chats/{chat_id}/sessions");
    let (status, value) = client.get(&path, params.as_ref(), None).await?;

    let json: GetChatSessionResponse = serde_json::from_value(value)?;

    Ok((status, json.data))
}

pub async fn stream_chat_conversation(
    client: &RagflowClient,
    chat_id: &str,
    body: ConvoQuestion,
) -> Result<impl Stream<Item = Result<Bytes>> + use<>> {
    let url = format!("{}/chats/{chat_id}/completions", client.base_url);

    let response = client
        .http
        .post(&url)
        .header("Authorization", client.auth_header())
        .json(&body)
        .send()
        .await?;

    Ok(response.bytes_stream().map_err(RagflowError::from))
}

#[derive(Serialize, Default)]
pub struct CreateChatSession {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Serialize, Default)]
pub struct UpdateChatSession {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ChatSession {
    pub chat_id: String,
    pub create_date: String,
    pub create_time: i64,
    pub id: String,
    pub name: Option<String>,
    pub update_date: String,
    pub update_time: i64,
    pub messages: Vec<ChatSessionMessage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatSessionMessage {
    pub content: String,
    pub id: Option<String>,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<Vec<MessageReference>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageReference {
    pub id: String,
    pub content: String,
    pub dataset_id: String,
    pub document_id: String,
    pub document_name: String,
}

#[derive(Deserialize)]
pub struct CreateChatSessionResponse {
    code: i32,
    pub data: ChatSession,
}

#[derive(Deserialize)]
pub struct GetChatSessionResponse {
    code: i32,
    pub data: Vec<ChatSession>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ConvoQuestion {
    pub question: String,
    pub stream: Option<bool>,
    pub session_id: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Reference {
    pub chunks: Vec<Chunk>,
}

#[derive(Serialize, Deserialize)]
pub struct Chunk;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::RagflowClient;
    use std::error::Error;
    use std::result::Result;

    use serde_json::json;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    #[tokio::test]
    async fn should_create_chat_session() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        let chat_session = ChatSession {
            name: Some("test_session".to_string()),
            ..Default::default()
        };
        Mock::given(method("POST"))
            .and(path(format!("{}/chats/123/sessions", client.path_prefix)))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({ "code": 0, "data": chat_session })),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let create_session = CreateChatSession {
            name: "test_session".to_string(),
            user_id: None,
        };
        let (status, value) = create(&client, "123", create_session).await?;

        assert!(status.is_success(), "error status from request");
        assert_eq!(
            value.name,
            Some("test_session".to_string()),
            "incorrect json response"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_update_chat_session() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_server".to_string());

        Mock::given(method("PUT"))
            .and(path(format!(
                "{}/chats/123/sessions/456",
                client.path_prefix
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "code": 0 })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let update_chat = UpdateChatSession {
            name: Some("something_new".to_string()),
            user_id: None,
        };
        let status = update(&client, "456", "123", update_chat).await?;

        assert!(status.is_success(), "error status from request");

        Ok(())
    }

    #[tokio::test]
    async fn should_delete_chat_sessions() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("DELETE"))
            .and(path(format!("{}/chats/123/sessions", client.path_prefix)))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "code": 0 })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let delete_resources = DeleteResources { ids: vec!["456"] };
        let status = delete(&client, "123", delete_resources).await?;

        assert!(status.is_success(), "error status from request");

        Ok(())
    }

    #[tokio::test]
    async fn should_return_list_of_chat_sessions() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        let chat_session = ChatSession {
            name: Some("test_chat_session".to_string()),
            ..Default::default()
        };

        Mock::given(method("GET"))
            .and(path(format!("{}/chats/123/sessions", client.path_prefix)))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({ "code": 0, "data": vec![chat_session] })),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let (status, chat_sessions) = list(&client, "123", None).await?;

        assert!(status.is_success(), "error status from request");
        assert_eq!(
            chat_sessions[0].name,
            Some("test_chat_session".to_string()),
            "incorrect json response"
        );

        Ok(())
    }
}
