use bytes::Bytes;
use futures::{Stream, TryStreamExt};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::RagflowClient;
use crate::error::Result;
use crate::{ConvoQuestion, DeleteResources, GetQueryParams, RagflowError};

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
    pub agent_id: String,
    pub dsl: Value, // TODO:
}

#[derive(Serialize, Default)]
pub struct CreateAgentSession;

#[derive(Deserialize, Default)]
pub struct CreateAgentSessionResponse {
    code: i32,
    data: AgentSession,
}

#[derive(Deserialize, Default)]
pub struct GetAgentSessionResponse {
    code: i32,
    data: Vec<AgentSession>,
}

// TODO: Tests
