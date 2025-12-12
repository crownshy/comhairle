use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::client::RagflowClient;
use crate::error::Result;
use crate::{ChunkMethod, DeleteResources};

pub async fn create(
    client: &RagflowClient,
    name: String,
    description: Option<String>,
) -> Result<(StatusCode, Dataset)> {
    let path = "/datasets";
    let body = CreateDataset {
        name,
        description,
        permission: Some("team".to_string()),
    };
    let (status, value) = client.post(path, &body, None).await?;

    let json: CreateDatasetResponse = serde_json::from_value(value)?;

    Ok((status, json.data))
}

pub async fn delete(client: &RagflowClient, dataset_id: &str) -> Result<StatusCode> {
    let body = DeleteResources {
        ids: vec![dataset_id],
    };
    client.delete("/datasets", &body, None).await
}

#[derive(Serialize)]
pub struct CreateDataset {
    pub name: String,
    pub description: Option<String>,
    pub permission: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDatasetResponse {
    pub code: i32,
    pub data: Dataset,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Dataset {
    pub avatar: Option<String>,
    pub chunk_count: Option<i32>,
    pub chunk_method: Option<ChunkMethod>,
    pub create_date: String,
    pub create_time: i64,
    pub created_by: String,
    pub description: Option<String>,
    pub document_count: Option<i32>,
    pub embedding_model: Option<String>,
    pub id: String,
    pub language: Option<String>,
    pub name: String,
    pub pagerank: Option<i32>,
    pub permission: Option<String>,
    pub similarity_threshold: Option<f64>,
    pub status: Option<String>,
    pub tenant_id: Option<String>,
    pub token_num: Option<i32>,
    pub update_date: Option<String>,
    pub update_time: Option<i64>,
    pub vector_similarity_weight: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::result::Result;

    use crate::client::RagflowClient;
    use serde_json::json;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    #[tokio::test]
    async fn should_create_dataset() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        let dataset = Dataset {
            name: "test_dataset".to_string(),
            permission: Some("team".to_string()),
            ..Default::default()
        };
        let json = CreateDatasetResponse {
            code: 0,
            data: dataset,
        };
        Mock::given(method("POST"))
            .and(path(format!("{}/datasets", client.path_prefix)))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!(json)))
            .expect(1)
            .mount(&mock_server)
            .await;

        let (status, value) = create(
            &client,
            "test_dataset".to_string(),
            Some("a new test dataset".to_string()),
        )
        .await?;

        assert!(status.is_success(), "status code not success");
        assert_eq!(
            value.name,
            "test_dataset".to_string(),
            "response json incorrect"
        );
        assert_eq!(
            value.permission,
            Some("team".to_string()),
            "permission incorrect"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_delete_dataset() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("DELETE"))
            .and(path(format!("{}/datasets", client.path_prefix)))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "code": 0 })))
            .mount(&mock_server)
            .await;

        let status = delete(&client, "123").await?;

        assert!(status.is_success(), "dataset deletion status");

        Ok(())
    }
}
