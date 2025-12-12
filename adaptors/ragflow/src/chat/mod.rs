use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::client::RagflowClient;
use crate::dataset::Dataset;
use crate::error::Result;
use crate::{DeleteResources, GetQueryParams};

pub mod session;

pub async fn create(client: &RagflowClient, body: CreateChat) -> Result<(StatusCode, Chat)> {
    let (status, value) = client.post("/chats", &body, None).await?;

    let json: CreateChatResponse = serde_json::from_value(value)?;

    Ok((status, json.data))
}

pub async fn update(client: &RagflowClient, id: &str, body: UpdateChat) -> Result<StatusCode> {
    let path = format!("/chats/{id}");
    let (status, _) = client.put(&path, &body, None).await?;

    Ok(status)
}

pub async fn delete(client: &RagflowClient, body: DeleteResources<'_>) -> Result<StatusCode> {
    let status = client.delete("/chats", &body, None).await?;

    Ok(status)
}

pub async fn list(
    client: &RagflowClient,
    params: Option<GetQueryParams>,
) -> Result<(StatusCode, Vec<Chat>)> {
    let (status, value) = client.get("/chats", params.as_ref(), None).await?;

    let json: GetChatResponse = serde_json::from_value(value)?;

    Ok((status, json.data))
}

#[derive(Serialize, Deserialize, Default)]
pub struct CreateChat {
    pub name: String,
    pub avatar: Option<String>,
    pub dataset_ids: Vec<String>,
    pub llm: Option<Llm>,
    pub prompt: Option<Prompt>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Llm {
    pub model_name: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Prompt {
    pub opener: Option<String>,
    pub empty_response: Option<String>,
    pub prompt: Option<String>,
    pub keywords_similarity_weight: Option<f64>,
    pub rerank_model: Option<String>,
    pub similarity_threshold: Option<f64>,
    pub top_n: i32,
    pub variables: Option<Vec<Variable>>,
}

#[derive(Deserialize)]
pub struct CreateChatResponse {
    code: i32,
    pub data: Chat,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Chat {
    pub name: String,
    pub avatar: Option<String>,
    pub create_date: String,
    pub create_time: i64,
    pub datasets: Option<Vec<Dataset>>,
    pub dataset_ids: Option<Vec<String>>,
    pub description: Option<String>,
    pub do_refer: Option<String>,
    pub id: String,
    pub language: Option<String>,
    pub llm: Llm,
    pub prompt: Prompt,
}

#[derive(Serialize, Default)]
pub struct UpdateChat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm: Option<Llm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Prompt>,
}

#[derive(Deserialize)]
pub struct GetChatResponse {
    code: i32,
    pub data: Vec<Chat>,
}

#[derive(Serialize, Deserialize)]
pub struct Variable {
    key: String,
    optional: bool,
}

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
    async fn should_create_chat() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        let chat = Chat {
            name: "new chat".to_string(),
            ..Default::default()
        };

        Mock::given(method("POST"))
            .and(path(format!("{}/chats", client.path_prefix)))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({ "code": 0, "data": chat })),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let chat = CreateChat {
            name: "new chat".to_string(),
            llm: Some(Llm {
                model_name: Some("gtp-4@OpenAI".to_string()),
            }),
            ..Default::default()
        };
        let (status, value) = create(&client, chat).await?;

        assert!(status.is_success(), "error status from request");
        assert_eq!(
            value.name,
            "new chat".to_string(),
            "incorrect json response"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_update_chat() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_server".to_string());

        Mock::given(method("PUT"))
            .and(path(format!("{}/chats/123", client.path_prefix)))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "code": 0 })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let update_chat = UpdateChat {
            name: Some("something_new".to_string()),
            ..Default::default()
        };
        let status = update(&client, "123", update_chat).await?;

        assert!(status.is_success(), "error status from request");

        Ok(())
    }

    #[tokio::test]
    async fn should_delete_chat() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("DELETE"))
            .and(path(format!("{}/chats", client.path_prefix)))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "code": 0 })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let delete_resources = DeleteResources { ids: vec!["123"] };
        let status = delete(&client, delete_resources).await?;

        assert!(status.is_success(), "error status from request");

        Ok(())
    }

    #[tokio::test]
    async fn should_return_list_of_chats() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        let chat = Chat {
            name: "test_chat".to_string(),
            ..Default::default()
        };

        Mock::given(method("GET"))
            .and(path(format!("{}/chats", client.path_prefix)))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({ "code": 0, "data": vec![chat] })),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let (status, chats) = list(&client, None).await?;

        assert!(status.is_success(), "error status from request");
        assert_eq!(
            chats[0].name,
            "test_chat".to_string(),
            "incorrect json response"
        );

        Ok(())
    }
}
