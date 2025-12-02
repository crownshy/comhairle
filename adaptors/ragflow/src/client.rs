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
            base_url: base_url.into(),
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

        if !status.is_success() {
            let text = response.text().await?;
            return Err(RagflowError::Api { status, body: text });
        }

        let json: serde_json::Value = response.json().await?;
        Ok((status, json))
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::client::RagflowClient;
    use reqwest::StatusCode;
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
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"ok": true})))
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
}
