use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{client::RagflowClient, error::Result, GetQueryParams};

pub mod session;

pub async fn list(
    client: &RagflowClient,
    params: Option<GetQueryParams>,
) -> Result<(StatusCode, Vec<Agent>)> {
    let (status, value) = client.get("/agents", params.as_ref(), None).await?;

    let json: GetAgentResponse = serde_json::from_value(value)?;

    Ok((status, json.data))
}

pub async fn create(
    client: &RagflowClient,
    body: CreateAgent,
) -> Result<(StatusCode, CreateAgentResponse)> {
    let (status, value) = client.post("/agents", &body, None).await?;

    let json: CreateAgentResponse = serde_json::from_value(value)?;

    Ok((status, json))
}

pub async fn update(
    client: &RagflowClient,
    agent_id: &str,
    body: UpdateAgent,
) -> Result<(StatusCode, UpdateAgentResponse)> {
    let (status, value) = client
        .put(&format!("/agents/{agent_id}"), &body, None)
        .await?;

    let json: UpdateAgentResponse = serde_json::from_value(value)?;

    Ok((status, json))
}

pub async fn delete(client: &RagflowClient, agent_id: &str) -> Result<StatusCode> {
    client
        .delete::<()>(&format!("/agents/{agent_id}"), None, None)
        .await
}

#[derive(Serialize, Deserialize)]
struct GetAgentResponse {
    code: i32,
    data: Vec<Agent>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateAgentResponse {
    pub code: i32,
    pub data: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateAgentResponse {
    pub code: i32,
    pub data: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAgent {
    pub title: String,
    pub dsl: serde_json::Value, // TODO:
}

#[derive(Serialize, Deserialize, Default)]
pub struct UpdateAgent {
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Agent {
    pub id: String,
    pub title: Option<String>,
    pub dsl: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{error::Error, result::Result};

    use crate::client::RagflowClient;
    use serde_json::json;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    #[tokio::test]
    async fn should_get_list_of_agents() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        let agent = Agent {
            title: Some("Test agent".to_string()),
            ..Default::default()
        };
        let response = GetAgentResponse {
            code: 0,
            data: vec![agent],
        };
        Mock::given(method("GET"))
            .and(path(format!("{}/agents", client.path_prefix)))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!(response)))
            .expect(1)
            .mount(&mock_server)
            .await;

        let (status, agents) = list(&client, None).await?;

        assert!(status.is_success(), "error status code");
        assert_eq!(
            agents[0].title.clone().unwrap(),
            "Test agent".to_string(),
            "incorrect json response"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_create_agent() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        let create_agent = CreateAgent {
            title: "Test agent".to_string(),
            dsl: json!({}),
        };
        let response = CreateAgentResponse {
            code: 0,
            data: true,
            message: "success".to_string(),
        };
        Mock::given(method("POST"))
            .and(path(format!("{}/agents", client.path_prefix)))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!(response)))
            .expect(1)
            .mount(&mock_server)
            .await;

        let (status, json) = create(&client, create_agent).await?;

        assert!(status.is_success(), "error status code");
        assert_eq!(
            json.message,
            "success".to_string(),
            "incorrect json response"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_update_agent() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        let update_agent = UpdateAgent {
            title: Some("updated".to_string()),
            ..Default::default()
        };
        let response = UpdateAgentResponse {
            code: 0,
            data: true,
            message: "success".to_string(),
        };
        Mock::given(method("PUT"))
            .and(path(format!("{}/agents/123", client.path_prefix)))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!(response)))
            .expect(1)
            .mount(&mock_server)
            .await;

        let (status, json) = update(&client, "123", update_agent).await?;

        assert!(status.is_success(), "error status code");
        assert_eq!(
            json.message,
            "success".to_string(),
            "incorrect json response"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_delete_agent() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("DELETE"))
            .and(path(format!("{}/agents/123", client.path_prefix)))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "code": 0 })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let status = delete(&client, "123").await?;

        assert!(status.is_success(), "error status code");

        Ok(())
    }
}
