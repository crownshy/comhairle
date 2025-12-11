use crate::error::{RagflowError, Result};

use reqwest::{
    Client as HttpClient, StatusCode,
    header::{HeaderName, HeaderValue},
    multipart::Form,
};
use serde::Serialize;
use serde_json::Value;

#[derive(Clone)]
pub struct RagflowClient {
    pub base_url: String,
    pub path_prefix: String,
    api_key: String,
    pub http: HttpClient,
}

impl RagflowClient {
    pub fn new<S: Into<String>>(base_url: S, api_key: S) -> Self {
        let client = HttpClient::new();
        let path_prefix = "/api/v1".to_string();

        RagflowClient {
            base_url: format!("{}{}", base_url.into(), path_prefix),
            path_prefix,
            api_key: api_key.into(),
            http: client,
        }
    }

    pub fn auth_header(&self) -> String {
        format!("Bearer {}", self.api_key)
    }

    /// Method required to validate a-typical error handling from ragflow api.
    /// All Ragflow endpoints return 200 code and a json response body. The json
    /// includes a `code` field, which is 0 if successful and 10* if unsuccessful.
    fn validate_ragflow_response(&self, _status: StatusCode, json: &Value) -> Result<()> {
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
                // status code from ragflow unhelpful as always 200
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: message.to_string(),
            });
        }

        Ok(())
    }

    pub async fn get<Q>(
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
            .http
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

        if !status.is_success() {
            let text = response.text().await?;
            return Err(RagflowError::Api { status, body: text });
        }

        let json: serde_json::Value = response.json().await?;
        self.validate_ragflow_response(status, &json)?;
        Ok((status, json))
    }

    pub async fn post<B: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &B,
        headers: Option<&[(HeaderName, HeaderValue)]>,
    ) -> Result<(StatusCode, Value)> {
        let url = format!("{}{}", self.base_url, path);

        let mut request = self
            .http
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

        if !status.is_success() {
            let text = response.text().await?;
            return Err(RagflowError::Api { status, body: text });
        }

        let json = response.json().await?;
        self.validate_ragflow_response(status, &json)?;
        Ok((status, json))
    }

    pub async fn post_multipart(
        &self,
        path: &str,
        form: Form,
        headers: Option<&[(HeaderName, HeaderValue)]>,
    ) -> Result<(StatusCode, Value)> {
        let url = format!("{}{}", self.base_url, path);

        let mut request = self
            .http
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
        self.validate_ragflow_response(status, &json)?;
        Ok((status, json))
    }

    pub async fn put<B: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &B,
        headers: Option<&[(HeaderName, HeaderValue)]>,
    ) -> Result<(StatusCode, Value)> {
        let url = format!("{}{}", self.base_url, path);

        let mut request = self
            .http
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
        self.validate_ragflow_response(status, &json)?;
        Ok((status, json))
    }

    pub async fn delete<B: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &B,
        headers: Option<&[(HeaderName, HeaderValue)]>,
    ) -> Result<StatusCode> {
        let url = format!("{}{}", self.base_url, path);

        let mut request = self
            .http
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

        self.validate_ragflow_response(status, &json)?;

        Ok(status)
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::{client::RagflowClient, error::RagflowError};
    use reqwest::{StatusCode, multipart::Form};
    use serde_json::json;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    #[tokio::test]
    async fn sends_get_request_to_base_url() -> Result<(), Box<dyn Error>> {
        let api_key = "test_key";
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), api_key.to_string());

        Mock::given(method("GET"))
            .and(path(format!("{}/", client.path_prefix)))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"code": 0})))
            .expect(1)
            .mount(&mock_server)
            .await;

        let (status, value) = client.get::<()>("/", None, None).await?;

        assert_eq!(status, StatusCode::OK, "success status from get request");
        assert_eq!(
            value.get("code").and_then(|v| v.as_i64()).unwrap(),
            0,
            "incorrect json response"
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
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({ "code": 0 })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let (status, value) = client.post("/test-post", &json!({}), None).await?;

        assert_eq!(status, StatusCode::CREATED, "success status from post");
        assert_eq!(
            value.get("code").and_then(|v| v.as_i64()).unwrap(),
            0,
            "incorrect json response"
        );

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
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "code": 0 })))
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
        assert_eq!(
            value.get("code").and_then(|v| v.as_i64()).unwrap(),
            0,
            "incorrect json response"
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
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "code": 0 })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let (status, value) = client
            .put("/test-update", &json!({ "email": "foo@bar.com"}), None)
            .await?;

        assert_eq!(status, StatusCode::OK, "success from update");
        assert_eq!(
            value.get("code").and_then(|v| v.as_i64()).unwrap(),
            0,
            "incorrect json response"
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
}
