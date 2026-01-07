use reqwest::{
    StatusCode,
    multipart::{Form, Part},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::DeleteResources;
use crate::client::RagflowClient;
use crate::error::Result;
use crate::{ChunkMethod, GetQueryParams};

pub async fn list(
    client: &RagflowClient,
    dataset_id: &str,
    query: Option<GetQueryParams>,
) -> Result<(StatusCode, Vec<Document>)> {
    let path = format!("/datasets/{dataset_id}/documents");
    let (status, value) = client.get(&path, query.as_ref(), None).await?;

    let json: GetDocumentsResponse = serde_json::from_value(value)?;

    Ok((status, json.data.docs))
}

pub async fn download(
    client: &RagflowClient,
    document_id: &str,
    dataset_id: &str,
) -> reqwest::Result<reqwest::Response> {
    let url = format!(
        "{}/datasets/{}/documents/{}",
        client.base_url, dataset_id, document_id
    );

    // Returning direct reqwest response so that large file contents are streamed
    client
        .http
        .get(url)
        .header("Authorization", client.auth_header())
        .send()
        .await
}

pub async fn upload(
    client: &RagflowClient,
    dataset_id: &str,
    files: Vec<UploadFile>,
) -> Result<(StatusCode, Vec<Document>)> {
    let path = format!("/datasets/{dataset_id}/documents");
    let mut form = Form::new();

    for file in files {
        let part = Part::bytes(file.bytes).file_name(file.filename);
        form = form.part("file", part);
    }

    let (_status, value) = client.post_multipart(&path, form, None).await?;

    let json: UploadDocumentsResponse = serde_json::from_value(value)?;
    let document_ids: Vec<&str> = json.data.iter().map(|doc| doc.id.as_ref()).collect();

    // Start parsing documents after upload
    // Can't be done in Ragflow as a single request
    // TODO: clean up if parsing fails?
    let parse_params = ParseDocuments { document_ids };
    let (status, _value) = parse(client, dataset_id, parse_params).await?;

    Ok((status, json.data))
}

pub async fn update(
    client: &RagflowClient,
    document_id: &str,
    dataset_id: &str,
    body: UpdateDocument,
) -> Result<(StatusCode, Value)> {
    let path = format!("/datasets/{dataset_id}/documents/{document_id}");
    client.put(&path, &body, None).await
}

pub async fn delete(
    client: &RagflowClient,
    document_id: &str,
    dataset_id: &str,
) -> Result<StatusCode> {
    let body = DeleteResources {
        ids: vec![document_id],
    };
    let path = format!("/datasets/{dataset_id}/documents");
    client.delete(&path, &body, None).await
}

pub async fn parse(
    client: &RagflowClient,
    dataset_id: &str,
    body: ParseDocuments<'_>,
) -> Result<(StatusCode, Value)> {
    let path = format!("/datasets/{dataset_id}/chunks");
    client.post(&path, &body, None).await
}

pub async fn stop_parse(
    client: &RagflowClient,
    dataset_id: &str,
    body: ParseDocuments<'_>,
) -> Result<StatusCode> {
    let path = format!("/datasets/{dataset_id}/chunks");
    client.delete(&path, &body, None).await
}

#[derive(Serialize, Deserialize)]
pub struct GetDocumentsResponse {
    pub code: i32,
    pub data: DocumentList,
}

#[derive(Serialize, Deserialize)]
pub struct DocumentList {
    pub docs: Vec<Document>,
    pub total: Option<i32>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Document {
    pub chunk_count: Option<i32>,
    pub create_date: Option<String>,
    pub create_time: Option<i64>,
    pub created_by: Option<String>,
    pub id: String,
    pub knowledgebase_id: Option<String>,
    pub location: Option<String>,
    pub name: String,
    pub chunk_method: Option<ChunkMethod>,
    pub process_begin_at: Option<String>,
    pub process_duration: Option<f64>,
    pub progress: Option<f64>,
    pub progress_message: Option<String>,
    pub run: Option<String>,
    pub size: i64,
    pub source_type: Option<String>,
    pub status: Option<String>,
    pub suffix: Option<String>,
    pub thumbnail: Option<String>,
    pub token_count: Option<i32>,
    pub r#type: Option<String>,
    pub update_date: Option<String>,
    pub update_time: Option<i64>,
}

#[derive(Deserialize)]
pub struct UploadDocumentsResponse {
    pub code: i32,
    pub data: Vec<Document>,
}

#[derive(Serialize)]
pub struct UploadFile {
    pub filename: String,
    pub bytes: Vec<u8>,
}

#[derive(Serialize, Default)]
pub struct UpdateDocument {
    pub name: Option<String>,
    pub chunk_method: Option<ChunkMethod>,
    pub parser_config: Option<ParserConfig>,
}

#[derive(Serialize)]
pub enum ParserConfig {
    Naive(NaiveParserConfig),
    RaptorOnly(RaptorOnlyParserConfig),
    Empty(EmptyParserConfig),
}

#[derive(Serialize)]
pub struct NaiveParserConfig {
    chunk_token_num: Option<u32>,
    layout_recognize: Option<bool>,
    html4excel: Option<bool>,
    delimiter: Option<String>,
    task_page_size: Option<u32>,
    raptor: Option<RaptorSettings>,
}

#[derive(Serialize)]
pub struct RaptorOnlyParserConfig {
    raptor: RaptorSettings,
}

#[derive(Serialize)]
pub struct EmptyParserConfig;

#[derive(Serialize)]
pub struct RaptorSettings {
    use_raptor: bool,
}

#[derive(Serialize)]
pub struct ParseDocuments<'a> {
    pub document_ids: Vec<&'a str>,
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
        matchers::{method, path, query_param},
    };

    #[tokio::test]
    async fn should_get_documents_from_dataset() -> Result<(), Box<dyn Error>> {
        let api_key = "test_key";
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), api_key.to_string());
        let dataset_id = "123";
        let req_path = format!("/datasets/{dataset_id}/documents");

        let document = Document {
            name: "test_doc".to_string(),
            ..Default::default()
        };
        let mock_documents = vec![document];
        Mock::given(method("GET"))
            .and(path(format!("{}{}", client.path_prefix, req_path)))
            .and(query_param("page", "1"))
            .and(query_param("page_size", "12"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({ "code": 0, "data": { "docs": mock_documents } })),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let query = GetQueryParams {
            page: Some(1),
            page_size: Some(12),
            ..Default::default()
        };
        let (status, documents) = list(&client, "123", Some(query)).await?;

        assert!(status.is_success(), "success from get documents");
        assert_eq!(
            documents[0].name,
            "test_doc".to_string(),
            "incorrect json response"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_download_document_as_streamed_bytes() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("GET"))
            .and(path(format!(
                "{}/datasets/123/documents/456",
                client.path_prefix
            )))
            .respond_with(ResponseTemplate::new(200).set_body_string("mock file content"))
            .mount(&mock_server)
            .await;

        let response = download(&client, "456", "123").await?;
        let status = response.status();
        let text = response.text().await.unwrap();

        assert_eq!(status, StatusCode::OK, "success from file download");
        assert_eq!(text, "mock file content", "file body matches");

        Ok(())
    }

    #[tokio::test]
    async fn should_upload_documents_to_dataset() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("POST"))
            .and(path(format!(
                "{}/datasets/123/documents",
                client.path_prefix
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "code": 0 })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let file = UploadFile {
            filename: "foo".to_string(),
            bytes: "bar".as_bytes().into(),
        };
        let files = vec![file];
        let (status, value) = upload(&client, "123", files).await?;

        assert_eq!(status, StatusCode::OK, "success from doc upload");
        assert_eq!(
            value.get("code").and_then(|v| v.as_i64()).unwrap(),
            0,
            "incorrect json response"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_update_document_in_dataset() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("PUT"))
            .and(path(format!(
                "{}/datasets/123/documents/456",
                client.path_prefix
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "code": 0 })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let update_doc = UpdateDocument {
            name: Some("foo".to_string()),
            chunk_method: Some(ChunkMethod::Table),
            parser_config: Some(ParserConfig::Empty(EmptyParserConfig)),
        };
        let (status, value) = update(&client, "456", "123", update_doc).await?;

        assert_eq!(status, StatusCode::OK, "success from document update");
        assert_eq!(
            value.get("code").and_then(|v| v.as_i64()).unwrap(),
            0,
            "incorrect json response"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_delete_document() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("DELETE"))
            .and(path(format!(
                "{}/datasets/123/documents",
                client.path_prefix
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "code": 0 })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let status = delete(&client, "456", "123").await?;

        assert_eq!(status, StatusCode::OK, "success from document delete");

        Ok(())
    }

    #[tokio::test]
    async fn should_parse_documents() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("POST"))
            .and(path(format!("{}/datasets/123/chunks", client.path_prefix)))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "code": 0 })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let body = ParseDocuments {
            document_ids: vec!["456", "789"],
        };
        let (status, value) = parse(&client, "123", body).await?;

        assert_eq!(status, StatusCode::OK, "success from document parsing");
        assert_eq!(
            value.get("code").and_then(|v| v.as_i64()).unwrap(),
            0,
            "valid json response"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_top_parsing_documents() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("DELETE"))
            .and(path(format!("{}/datasets/123/chunks", client.path_prefix)))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "code": 0 })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let body = ParseDocuments {
            document_ids: vec!["456", "789"],
        };
        let status = stop_parse(&client, "123", body).await?;

        assert_eq!(
            status,
            StatusCode::OK,
            "success from stopping document parsing"
        );

        Ok(())
    }
}
