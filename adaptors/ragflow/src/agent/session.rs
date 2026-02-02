use bytes::Bytes;
use futures::{Stream, TryStreamExt};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::RagflowClient;
use crate::error::Result;
use crate::{ConvoQuestion, DeleteResources, GetQueryParams, RagflowError, SessionMessage};

pub async fn create(client: &RagflowClient, agent_id: &str) -> Result<(StatusCode, AgentSession)> {
    let path = format!("/agents/{agent_id}/sessions");
    let (status, value) = client.post(&path, &(), None).await?;

    let json: CreateAgentSessionResponse = serde_json::from_value(value)?;

    Ok((status, json.data))
}

pub async fn delete(
    client: &RagflowClient,
    agent_id: &str,
    body: DeleteResources<'_>,
) -> Result<StatusCode> {
    let path = format!("/agents/{agent_id}/sessions");
    let status = client.delete(&path, Some(&body), None).await?;

    Ok(status)
}

pub async fn list(
    client: &RagflowClient,
    agent_id: &str,
    params: Option<GetQueryParams>,
) -> Result<(StatusCode, Vec<AgentSession>)> {
    let path = format!("/agents/{agent_id}/sessions");
    let (status, value) = client.get(&path, params.as_ref(), None).await?;

    let json: GetAgentSessionResponse = serde_json::from_value(value)?;

    Ok((status, json.data))
}

pub async fn stream_agent_conversation(
    client: &RagflowClient,
    agent_id: &str,
    body: ConvoQuestion,
) -> Result<impl Stream<Item = Result<Bytes>> + use<>> {
    let url = format!("{}/agents/{agent_id}/completions", client.base_url);

    let response = client
        .http
        .post(&url)
        .header("Authorization", client.auth_header())
        .json(&body)
        .send()
        .await?;

    Ok(response.bytes_stream().map_err(RagflowError::from))
}

#[derive(Serialize, Deserialize, Default)]
pub struct AgentSession {
    pub id: String,
    pub agent_id: String,
    pub dsl: Value, // TODO:
    pub messages: Option<Vec<SessionMessage>>,
    pub duration: Option<f64>,
    pub round: Option<i32>,
    pub source: Option<String>,
    pub thumbs_up: Option<i32>,
    pub tokens: Option<i32>,
    pub update_date: Option<String>,
    pub update_time: Option<i64>,
    pub user_id: Option<String>,
}

#[derive(Serialize, Default)]
pub struct CreateAgentSession;

#[derive(Serialize, Deserialize, Default)]
pub struct CreateAgentSessionResponse {
    data: AgentSession,
}

#[derive(Serialize, Deserialize, Default)]
pub struct GetAgentSessionResponse {
    data: Vec<AgentSession>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{error::Error, result::Result};

    use crate::client::RagflowClient;
    use futures::StreamExt;
    use serde_json::json;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    #[tokio::test]
    async fn should_get_list_of_agent_sessions() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        let session = AgentSession {
            agent_id: "123".to_string(),
            ..Default::default()
        };
        Mock::given(method("GET"))
            .and(path(format!("{}/agents/123/sessions", client.path_prefix)))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({ "code": 0, "data": vec![session] })),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let (status, sessions) = list(&client, "123", None).await?;

        assert!(status.is_success(), "error status code");
        assert_eq!(
            sessions[0].agent_id,
            "123".to_string(),
            "incorrect json response"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_create_agent_session() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        let session = AgentSession {
            agent_id: "123".to_string(),
            ..Default::default()
        };
        Mock::given(method("POST"))
            .and(path(format!("{}/agents/123/sessions", client.path_prefix)))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({ "code": 0, "data": session })),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let (status, session) = create(&client, "123").await?;

        assert!(status.is_success(), "error status code");
        assert_eq!(
            session.agent_id,
            "123".to_string(),
            "incorrect json response"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_delete_agent_session() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("DELETE"))
            .and(path(format!("{}/agents/123/sessions", client.path_prefix)))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({ "code": 0, "message": "success" })),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let body = DeleteResources { ids: vec!["456"] };
        let status = delete(&client, "123", body).await?;

        assert!(status.is_success(), "error status code");

        Ok(())
    }

    #[tokio::test]
    async fn should_stream_agent_conversation() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        let streamed_body = "chunk1\nchunk2\nchunk3";
        Mock::given(method("POST"))
            .and(path(format!(
                "{}/agents/123/completions",
                client.path_prefix
            )))
            .respond_with(ResponseTemplate::new(200).set_body_raw(streamed_body, "test/plain"))
            .expect(1)
            .mount(&mock_server)
            .await;

        let body = ConvoQuestion {
            question: "hello".to_string(),
            ..Default::default()
        };
        let stream = stream_agent_conversation(&client, "123", body).await?;
        let bytes: Vec<u8> = stream
            .map(|chunk| chunk.unwrap())
            .fold(Vec::new(), |mut acc, bytes| async move {
                acc.extend_from_slice(&bytes);
                acc
            })
            .await;

        let result = String::from_utf8(bytes)?;

        assert_eq!(result, streamed_body);

        Ok(())
    }
}
