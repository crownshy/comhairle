use crate::{
    error::{RagflowError, Result},
    types::{DeleteDocument, GetDocumentsQueryParams, ParseDocuments, UpdateDocument, UploadFile},
};
use reqwest::{
    Client as HttpClient, StatusCode,
    header::{HeaderName, HeaderValue},
    multipart::{Form, Part},
};
use serde::Serialize;
use serde_json::Value;

#[derive(Clone)]
pub struct RagflowClient {
    base_url: String,
    path_prefix: String,
    api_key: String,
    http_client: HttpClient,
}

impl RagflowClient {
    pub fn new<S: Into<String>>(base_url: S, api_key: S) -> Self {
        let client = HttpClient::new();
        let path_prefix = "/api/v1".to_string();

        RagflowClient {
            base_url: format!("{}{}", base_url.into(), path_prefix),
            path_prefix,
            api_key: api_key.into(),
            http_client: client,
        }
    }

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.api_key)
    }

    async fn get<Q>(
        &self,
        path: &str,
        query: Option<&Q>,
        headers: Option<&[(HeaderName, HeaderValue)]>,
    ) -> Result<(StatusCode, Value)>
    where
        Q: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.base_url, path);

        let mut request = self
            .http_client
            .get(&url)
            .header("Authorization", self.auth_header());

        if let Some(q) = query {
            request = request.query(q);
        }

        if let Some(headers) = headers {
            for (name, value) in headers {
                request = request.header(name, value);
            }
        }

        let response = request.send().await?;
        let status = response.status();

        // TODO: look at if there is a better way to handle this
        if !status.is_success() {
            let text = response.text().await?;
            return Err(RagflowError::Api { status, body: text });
        }
        // TODO: will need another way to handle 200 responses that are still errors

        let json: serde_json::Value = response.json().await?;
        Ok((status, json))
    }

    async fn post<B: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &B,
        headers: Option<&[(HeaderName, HeaderValue)]>,
    ) -> Result<(StatusCode, Value)> {
        let url = format!("{}{}", self.base_url, path);

        let mut request = self
            .http_client
            .post(&url)
            .header("Authorization", self.auth_header())
            .json(body);

        if let Some(headers) = headers {
            for (name, value) in headers {
                request = request.header(name, value);
            }
        }

        let response = request.send().await?;
        let status = response.status();

        // TODO: not convinced by this
        if !status.is_success() {
            let text = response.text().await?;
            return Err(RagflowError::Api { status, body: text });
        }

        let json = response.json().await?;
        Ok((status, json))
    }

    async fn post_multipart(
        &self,
        path: &str,
        form: Form,
        headers: Option<&[(HeaderName, HeaderValue)]>,
    ) -> Result<(StatusCode, Value)> {
        let url = format!("{}{}", self.base_url, path);

        let mut request = self
            .http_client
            .post(&url)
            .header("Authorization", self.auth_header())
            .multipart(form);

        if let Some(headers) = headers {
            for (name, value) in headers {
                request = request.header(name, value);
            }
        }

        let response = request.send().await?;
        let status = response.status();

        if !status.is_success() {
            let text = response.text().await?;
            return Err(RagflowError::Api { status, body: text });
        }

        let json = response.json().await?;
        Ok((status, json))
    }

    async fn put<B: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &B,
        headers: Option<&[(HeaderName, HeaderValue)]>,
    ) -> Result<(StatusCode, Value)> {
        let url = format!("{}{}", self.base_url, path);

        let mut request = self
            .http_client
            .put(&url)
            .header("Authorization", self.auth_header())
            .json(body);

        if let Some(headers) = headers {
            for (name, value) in headers {
                request = request.header(name, value);
            }
        }

        let response = request.send().await?;
        let status = response.status();

        if !status.is_success() {
            let text = response.text().await?;
            return Err(RagflowError::Api { status, body: text });
        }

        let json = response.json().await?;
        Ok((status, json))
    }

    async fn delete<B: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &B,
        headers: Option<&[(HeaderName, HeaderValue)]>,
    ) -> Result<StatusCode> {
        let url = format!("{}{}", self.base_url, path);

        let mut request = self
            .http_client
            .delete(&url)
            .header("Authorization", self.auth_header())
            .json(body);

        if let Some(headers) = headers {
            for (name, value) in headers {
                request = request.header(name, value);
            }
        }

        let response = request.send().await?;
        let status = response.status();

        if !status.is_success() {
            let text = response.text().await?;
            return Err(RagflowError::Api { status, body: text });
        }

        let json = response.json::<Value>().await?;

        // TODO: extract and add to each crud method
        // Required error handling for Ragflow
        let code = json
            .get("code")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| RagflowError::Api {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: "Missing or invalid 'code' field in response".into(),
            })?;

        if code != 0 {
            let message = json
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("Something went wrong");
            return Err(RagflowError::Api {
                status, // TODO: what should the status be here?
                body: message.to_string(),
            });
        }

        Ok(status)
    }

    pub async fn get_documents(
        &self,
        dataset_id: &str,
        query: Option<&GetDocumentsQueryParams>,
    ) -> Result<(StatusCode, Value)> {
        let path = format!("/datasets/{dataset_id}/documents");
        self.get(&path, query, None).await
    }

    pub async fn download_document(
        &self,
        document_id: &str,
        dataset_id: &str,
    ) -> reqwest::Result<reqwest::Response> {
        let url = format!(
            "{}/datasets/{}/documents/{}",
            self.base_url, dataset_id, document_id
        );

        // Returning direct reqwest response so that large file contents are streamed
        self.http_client
            .get(url)
            .header("Authorization", self.auth_header())
            .send()
            .await
    }

    pub async fn upload_documents(
        &self,
        dataset_id: &str,
        files: Vec<UploadFile>,
    ) -> Result<(StatusCode, Value)> {
        let path = format!("/datasets/{dataset_id}/documents");
        let mut form = Form::new();

        for file in files {
            let part = Part::bytes(file.bytes).file_name(file.filename);
            form = form.part("file", part);
        }

        self.post_multipart(&path, form, None).await
    }

    pub async fn update_document(
        &self,
        document_id: &str,
        dataset_id: &str,
        body: UpdateDocument,
    ) -> Result<(StatusCode, Value)> {
        let path = format!("/datasets/{dataset_id}/documents/{document_id}");
        self.put(&path, &body, None).await
    }

    pub async fn delete_document(
        &self,
        document_id: &str,
        dataset_id: &str,
        body: DeleteDocument<'_>,
    ) -> Result<StatusCode> {
        let path = format!("/datasets/{dataset_id}/documents/{document_id}");
        self.delete(&path, &body, None).await
    }

    pub async fn parse_documents(
        &self,
        dataset_id: &str,
        body: ParseDocuments<'_>,
    ) -> Result<(StatusCode, Value)> {
        let path = format!("/datasets/{dataset_id}/chunks");
        self.post(&path, &body, None).await
    }

    pub async fn stop_parsing_documents(
        &self,
        dataset_id: &str,
        body: ParseDocuments<'_>,
    ) -> Result<StatusCode> {
        let path = format!("/datasets/{dataset_id}/chunks");
        self.delete(&path, &body, None).await
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::{
        client::RagflowClient,
        error::RagflowError,
        types::{
            ChunkMethod, DeleteDocument, EmptyParserConfig, GetDocumentsQueryParams,
            ParseDocuments, ParserConfig, UpdateDocument, UploadFile,
        },
    };
    use reqwest::{StatusCode, multipart::Form};
    use serde_json::json;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path, query_param},
    };

    #[tokio::test]
    async fn sends_get_request_to_base_url() -> Result<(), Box<dyn Error>> {
        let api_key = "test_key";
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), api_key.to_string());

        Mock::given(method("GET"))
            .and(path(format!("{}/", client.path_prefix)))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
            .expect(1)
            .mount(&mock_server)
            .await;

        let (status, value) = client.get::<()>("/", None, None).await?;

        assert_eq!(status, StatusCode::OK, "success status from get request");
        assert!(
            value.get("ok").and_then(|v| v.as_bool()).unwrap(),
            "response body should contain ok field"
        );

        Ok(())
    }

    #[tokio::test]
    async fn get_returns_api_error() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("GET"))
            .and(path(format!("{}/test", client.path_prefix)))
            .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
            .mount(&mock_server)
            .await;

        let err = client.get::<()>("/test", None, None).await.unwrap_err();

        match err {
            RagflowError::Api { status, body: _ } => {
                assert_eq!(status, StatusCode::NOT_FOUND, "get returns 404 status");
            }
            _ => panic!("Expected RagflowError::Api"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn get_returns_reqwest_error() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("GET"))
            .and(path(format!("{}/test", client.path_prefix)))
            .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
            .mount(&mock_server)
            .await;

        let err = client.get::<()>("/test", None, None).await.unwrap_err();

        match err {
            RagflowError::Http(e) => {
                assert!(e.is_decode());
            }
            _ => panic!("Expected RagflowError::Http"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn sends_successful_post() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("POST"))
            .and(path(format!("{}/test-post", client.path_prefix)))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({ "success": true })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let (status, value) = client.post("/test-post", &json!({}), None).await?;

        let success_field = value.get("success").and_then(|v| v.as_bool()).unwrap();
        assert_eq!(status, StatusCode::CREATED, "success status from post");
        assert!(success_field, "json response is valid");

        Ok(())
    }

    #[tokio::test]
    async fn post_returns_api_error() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("POST"))
            .and(path(format!("{}/test-post-error", client.path_prefix)))
            .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
            .mount(&mock_server)
            .await;

        let err = client
            .post("/test-post-error", &json!({}), None)
            .await
            .unwrap_err();

        match err {
            RagflowError::Api { status, body: _ } => {
                assert_eq!(status, StatusCode::NOT_FOUND, "post returns 404 status");
            }
            _ => panic!("Expected RagflowError::Api"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn post_returns_reqwest_error() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("POST"))
            .and(path(format!("{}/test-post-error", client.path_prefix)))
            .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
            .mount(&mock_server)
            .await;

        let err = client
            .post("/test-post-error", &json!({}), None)
            .await
            .unwrap_err();

        match err {
            RagflowError::Http(e) => {
                assert!(e.is_decode());
            }
            _ => panic!("Expected RagflowError::Http"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn sends_multipart_post_request() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("POST"))
            .and(path(format!("{}/test-multipart-post", client.path_prefix)))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "success": true })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let form = Form::new()
            .text("email", "foo@bar.com")
            .text("phone", "70123456789");
        let (status, value) = client
            .post_multipart("/test-multipart-post", form, None)
            .await?;

        assert_eq!(status, StatusCode::OK, "success from multipart post");
        assert!(
            value.get("success").and_then(|v| v.as_bool()).unwrap(),
            "valid json response"
        );

        Ok(())
    }

    #[tokio::test]
    async fn post_multipart_returns_api_error() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("POST"))
            .and(path(format!(
                "{}/test-multipart-api-error",
                client.path_prefix
            )))
            .respond_with(ResponseTemplate::new(404).set_body_string("api error"))
            .mount(&mock_server)
            .await;

        let form = Form::new().text("test", "error");
        let err = client
            .post_multipart("/test-multipart-api-error", form, None)
            .await
            .unwrap_err();

        match err {
            RagflowError::Api { status, body: _ } => {
                assert_eq!(status, StatusCode::NOT_FOUND, "Error from api");
            }
            _ => panic!("Expected RagflowError::Api"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn post_multipart_returns_reqwest_error() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("POST"))
            .and(path(format!(
                "{}/test-multipart-reqwest-error",
                client.path_prefix
            )))
            .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
            .mount(&mock_server)
            .await;

        let form = Form::new().text("email", "foo@bar.com");
        let err = client
            .post_multipart("/test-multipart-reqwest-error", form, None)
            .await
            .unwrap_err();

        match err {
            RagflowError::Http(e) => {
                assert!(e.is_decode())
            }
            _ => panic!("Expected RagflowError::Http"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn should_update_document() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("PUT"))
            .and(path(format!("{}/test-update", client.path_prefix)))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "success": true })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let (status, value) = client
            .put("/test-update", &json!({ "email": "foo@bar.com"}), None)
            .await?;

        assert_eq!(status, StatusCode::OK, "success from update");
        assert!(
            value.get("success").and_then(|v| v.as_bool()).unwrap(),
            "valid json response"
        );

        Ok(())
    }

    #[tokio::test]
    async fn put_returns_api_error() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("PUT"))
            .and(path(format!("{}/test-post-error", client.path_prefix)))
            .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
            .mount(&mock_server)
            .await;

        let err = client
            .put("/test-put-error", &json!({}), None)
            .await
            .unwrap_err();

        match err {
            RagflowError::Api { status, body: _ } => {
                assert_eq!(status, StatusCode::NOT_FOUND, "put returns 404 status");
            }
            _ => panic!("Expected RagflowError::Api"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn put_returns_reqwest_error() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("PUT"))
            .and(path(format!("{}/test-put-error", client.path_prefix)))
            .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
            .mount(&mock_server)
            .await;

        let err = client
            .put("/test-put-error", &json!({}), None)
            .await
            .unwrap_err();

        match err {
            RagflowError::Http(e) => {
                assert!(e.is_decode());
            }
            _ => panic!("Expected RagflowError::Http"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn delete_ragflow_resources() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("DELETE"))
            .and(path(format!("{}/test-delete-success", client.path_prefix)))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({ "code": 0,  "success": true })),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let body = json!({ "ids": vec!["123", "456"]});
        let status = client.delete("/test-delete-success", &body, None).await?;

        assert_eq!(status, StatusCode::OK, "success from delete method");

        Ok(())
    }

    #[tokio::test]
    async fn delete_error_from_api() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("DELETE"))
            .and(path(format!(
                "{}/test-delete-api-error",
                client.path_prefix
            )))
            .respond_with(ResponseTemplate::new(404).set_body_string("error from api"))
            .mount(&mock_server)
            .await;

        let err = client
            .delete("/test-delete-api-error", &json!({}), None)
            .await
            .unwrap_err();

        match err {
            RagflowError::Api { status, body: _ } => {
                assert_eq!(status, StatusCode::NOT_FOUND, "delete returns 404 status");
            }
            _ => panic!("Expected RagflowError::Api"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn delete_returns_reqwest_error() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("DELETE"))
            .and(path(format!("{}/test-delete-error", client.path_prefix)))
            .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
            .mount(&mock_server)
            .await;

        let err = client
            .delete("/test-delete-error", &json!({}), None)
            .await
            .unwrap_err();

        match err {
            RagflowError::Http(e) => {
                assert!(e.is_decode());
            }
            _ => panic!("Expected RagflowError::Http"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn delete_returns_error_from_ragflow_code() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("DELETE"))
            .and(path(format!(
                "{}/test-delete-from-code",
                client.path_prefix
            )))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({ "code": 102, "message": "You do not own the dataset"})),
            )
            .mount(&mock_server)
            .await;

        let err = client
            .delete("/test-delete-from-code", &json!({}), None)
            .await
            .unwrap_err();

        match err {
            RagflowError::Api { status: _, body } => {
                assert_eq!(
                    body, "You do not own the dataset",
                    "error json from response"
                );
            }
            _ => panic!("Expected RagflowError::Api"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn should_get_documents_from_dataset() -> Result<(), Box<dyn Error>> {
        let api_key = "test_key";
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), api_key.to_string());
        let dataset_id = "123";
        let req_path = format!("/datasets/{dataset_id}/documents");

        Mock::given(method("GET"))
            .and(path(format!("{}{}", client.path_prefix, req_path)))
            .and(query_param("page", "1"))
            .and(query_param("page_size", "12"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .expect(1)
            .mount(&mock_server)
            .await;

        let query = GetDocumentsQueryParams {
            page: Some(1),
            page_size: Some(12),
            ..Default::default()
        };
        let (status, _) = client.get(&req_path, Some(&query), None).await?;

        assert_eq!(status, StatusCode::OK, "success from get documents");

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

        let response = client.download_document("456", "123").await?;
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
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "success": true })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let file = UploadFile {
            filename: "foo".to_string(),
            bytes: "bar".as_bytes().into(),
        };
        let files = vec![file];
        let (status, value) = client.upload_documents("123", files).await?;

        assert_eq!(status, StatusCode::OK, "success from doc upload");
        assert!(
            value.get("success").and_then(|v| v.as_bool()).unwrap(),
            "valid response json"
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
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "success": true })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let update_doc = UpdateDocument {
            name: Some("foo".to_string()),
            enabled: None,
            meta_fields: None,
            chunk_method: Some(ChunkMethod::Table),
            parser_config: Some(ParserConfig::Empty(EmptyParserConfig)),
        };
        let (status, value) = client.update_document("456", "123", update_doc).await?;

        assert_eq!(status, StatusCode::OK, "success from document update");
        assert!(
            value.get("success").and_then(|v| v.as_bool()).unwrap(),
            "valid json response"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_delete_document() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("DELETE"))
            .and(path(format!(
                "{}/datasets/123/documents/456",
                client.path_prefix
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "code": 0 })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let body = DeleteDocument { ids: vec!["123"] };
        let status = client.delete_document("456", "123", body).await?;

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
        let (status, value) = client.parse_documents("123", body).await?;

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
        let status = client.stop_parsing_documents("123", body).await?;

        assert_eq!(
            status,
            StatusCode::OK,
            "success from stopping document parsing"
        );

        Ok(())
    }
}
