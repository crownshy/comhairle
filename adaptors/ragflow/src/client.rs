use crate::error::{RagflowError, Result};
use reqwest::{
    Client as HttpClient, StatusCode,
    header::{HeaderName, HeaderValue},
};
use serde::Serialize;
use serde_json::Value;

#[derive(Clone)]
pub struct RagflowClient {
    base_url: String,
    api_key: String,
    http_client: HttpClient,
}

impl RagflowClient {
    pub fn new<S: Into<String>>(base_url: S, api_key: S) -> Self {
        let client = HttpClient::new();
        RagflowClient {
            base_url: format!("{}/api/v1", base_url.into()),
            api_key: api_key.into(),
            http_client: client,
        }
    }

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.api_key)
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

    pub async fn get_documents(
        &self,
        dataset_id: &str,
        query: Option<&GetDocumentsQueryParams>,
    ) -> Result<(StatusCode, Value)> {
        let path = format!("/datasets/{dataset_id}/documents");
        self.get(&path, query, None).await
    }
}

#[derive(Serialize, Default)]
pub struct GetDocumentsQueryParams {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub orderby: Option<String>,
    pub desc: Option<bool>,
    pub id: Option<String>,
    pub create_time_from: Option<i32>,
    // keywords: Option<String>, // TODO: find way to implement these
    // suffix: Option<Vec<String>>,
    // run: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::{
        client::{GetDocumentsQueryParams, RagflowClient},
        error::RagflowError,
    };
    use reqwest::StatusCode;
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
            .and(path("/"))
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
    async fn get_returns_not_found() -> Result<(), Box<dyn Error>> {
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), "test_key".to_string());

        Mock::given(method("GET"))
            .and(path("/test"))
            .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
            // .expect(1)
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
            .and(path("/api/v1/test"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
            .mount(&mock_server)
            .await;

        let err = client.get::<()>("/test", None, None).await.unwrap_err();

        match err {
            RagflowError::Http(e) => {
                assert!(e.is_decode());
            }
            _ => panic!("Expected reqwest error"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn retrieves_documents_from_dataset() -> Result<(), Box<dyn Error>> {
        let api_key = "test_key";
        let mock_server = MockServer::start().await;
        let client = RagflowClient::new(mock_server.uri(), api_key.to_string());
        let dataset_id = "123";
        let req_path = format!("/datasets/{dataset_id}/documents");

        Mock::given(method("GET"))
            .and(path(format!("/api/v1{req_path}")))
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

        assert_eq!(status, StatusCode::OK);

        Ok(())
    }
}
