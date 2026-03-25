use async_trait::async_trait;
use cookie::Cookie;
use rand::{distributions::Alphanumeric, Rng};
use reqwest::{
    header::{COOKIE, SET_COOKIE},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{instrument, warn};

use crate::{
    tools::polis::PolisError,
    wiki_poll_service::{
        error::WikiPollServiceError, WikiPollComment, WikiPollLogin, WikiPollService,
    },
};

pub struct PolisClient {
    client: reqwest::Client,
    base_url: String,
}

impl PolisClient {
    pub fn new(base_url: &str) -> Self {
        let client = Client::new();
        Self {
            client,
            base_url: base_url.to_string(),
        }
    }
}

#[async_trait]
impl WikiPollService for PolisClient {
    async fn create_random_admin_user(&self) -> Result<(String, String), WikiPollServiceError> {
        let username: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

        let email = format!("{username}@comhairle.com");

        let password: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

        let new_user = NewAdminUser {
            hname: username.clone(),
            password: password.clone(),
            password2: password,
            email,
            gatekeeperTosPrivacy: true,
        };

        let _res = self
            .client
            .post(format!("https://{}/api/v3/auth/new", self.base_url))
            .json(&new_user)
            .send()
            .await
            .map_err(|e| {
                warn!("{e}");
                PolisError::FailedToCreateNewAdminUser
            })?
            .text()
            .await
            .map_err(|e| {
                warn!("{e}");
                PolisError::FailedToCreateNewAdminUser
            })?;

        Ok((new_user.email, new_user.password))
    }

    /// Authenticates with the Polis API and returns the session cookies required
    /// for subsequent admin requests.
    ///
    /// The `Set-Cookie` headers returned by the login response are parsed and
    /// reformatted into a single `Cookie` header value, stripping attributes
    /// such as `Max-Age`, `Domain`, and `Expires` which are only valid in
    /// server responses.
    ///
    /// # Example
    ///
    /// Given login response headers:
    /// ```text
    /// set-cookie: token2=abcd; Max-Age=31536000; Domain=...
    /// set-cookie: uid2=abcd; Max-Age=31536000; Domain=...
    /// set-cookie: e=1; Max-Age=31536000; Domain=...
    /// ```
    ///
    /// The returned string will be:
    /// ```text
    /// token2=abcd; uid2=abcd; e=1
    /// ```
    async fn login(&self, login: &WikiPollLogin) -> Result<String, WikiPollServiceError> {
        let url = format!("https://{}/api/v3/auth/login", self.base_url);
        let resp = self
            .client
            .post(url)
            .json(&login)
            .send()
            .await
            .map_err(|_| PolisError::FailedToLogin)?;

        let cookies = resp
            .headers()
            .get_all(SET_COOKIE)
            .iter()
            .filter_map(|v| v.to_str().ok())
            .filter_map(|v| Cookie::parse(v).ok())
            .map(|c| format!("{}={}", c.name(), c.value()))
            .collect::<Vec<_>>()
            .join("; ");

        let _ = resp
            .json::<LoginResp>()
            .await
            .map_err(|_| PolisError::FailedToLogin)?;

        Ok(cookies)
    }

    async fn create_poll(&self, auth_cookies: &str) -> Result<String, WikiPollServiceError> {
        let new_poll = self
            .client
            .post(format!("https://{}/api/v3/conversations", self.base_url))
            .header(COOKIE, auth_cookies)
            .send()
            .await
            .map_err(|e| {
                warn!("Failed to create new poll: {e:#?}");
                PolisError::FailedToCreateNewPoll
            })?
            .json::<NewPollResp>()
            .await
            .map_err(|e| {
                warn!("Failed to create new poll: {e:#?}");
                PolisError::FailedToCreateNewPoll
            })?;
        Ok(new_poll.conversation_id.to_owned())
    }

    #[instrument(err(Debug), skip(self))]
    async fn post_seed_comment(
        &self,
        comment: &str,
        poll_id: &str,
        auth_cookies: &str,
    ) -> Result<String, WikiPollServiceError> {
        let post_json = json!({
            "txt": comment,
            "pid": "mypid",
            "conversation_id": poll_id,
            "is_seed": true
        });

        let resp = self
            .client
            .post(format!("https://{}/api/v3/comments", self.base_url))
            .header(COOKIE, auth_cookies)
            .json(&post_json)
            .send()
            .await
            .map_err(|e| PolisError::FailedToPostSeedComment(e.to_string()))?
            .json::<PolisCommentCreateResponse>()
            .await
            .map_err(|e| PolisError::FailedToPostSeedComment(e.to_string()))?;

        Ok(resp.tid.to_string())
    }

    async fn get_comments(
        &self,
        poll_id: &str,
    ) -> Result<Vec<WikiPollComment>, WikiPollServiceError> {
        let url = format!(
            "https://{}/api/v3/comments?conversation_id={poll_id}",
            self.base_url
        );
        let comments: Vec<WikiPollComment> = self
            .client
            .get(url)
            .send()
            .await?
            .json()
            .await
            .map_err(|e| PolisError::FailedToGetComments(e.to_string()))?;

        Ok(comments)
    }
}

#[derive(Deserialize, Serialize)]
struct NewAdminUser {
    pub hname: String,
    pub password: String,
    pub password2: String,
    pub email: String,
    pub gatekeeperTosPrivacy: bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct NewPollResp {
    conversation_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PolisCommentCreateResponse {
    tid: u8,
    current_pid: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginResp {
    pub uid: u32,
    pub email: String,
    pub token: String,
}

#[cfg(test)]
mod tests {
    use crate::wiki_poll_service::polis_service::PolisClient;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn login() -> Result<(), Box<dyn std::error::Error>> {
        let client = PolisClient::new("polis.comhairle.scot");
        let login = WikiPollLogin {
            email: "cJIc2EPhHL@comhairle.com".into(),
            password: "f8QYSX9U9x".into(),
        };
        client.login(&login).await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn create_poll() -> Result<(), Box<dyn std::error::Error>> {
        let client = PolisClient::new("polis.comhairle.scot");

        let login = WikiPollLogin {
            email: "cJIc2EPhHL@comhairle.com".into(),
            password: "f8QYSX9U9x".into(),
        };
        let cookies = client.login(&login).await?;

        let _result = client.create_poll(&cookies).await?;

        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn sign_up_and_create_poll() -> Result<(), Box<dyn std::error::Error>> {
        let client = PolisClient::new("polis.comhairle.scot");
        let (email, password) = client.create_random_admin_user().await?;
        println!("{email} {password}");

        let login = WikiPollLogin { email, password };

        let cookies = client.login(&login).await?;

        let resp = client.create_poll(&cookies).await?;
        println!("{resp:#?}");

        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn post_seed_comment() -> Result<(), Box<dyn std::error::Error>> {
        let client = PolisClient::new("polis.comhairle.scot");
        let (email, password) = client.create_random_admin_user().await?;
        println!("{email} {password}");

        let login = WikiPollLogin { email, password };

        let cookies = client.login(&login).await?;

        let poll_id = client.create_poll(&cookies).await?;

        let _response = client
            .post_seed_comment("test_seed_comment", &poll_id, &cookies)
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn get_comments() -> Result<(), Box<dyn std::error::Error>> {
        let client = PolisClient::new("polis.comhairle.scot");
        let (email, password) = client.create_random_admin_user().await?;
        println!("{email} {password}");

        let login = WikiPollLogin { email, password };

        let cookies = client.login(&login).await?;

        let poll_id = client.create_poll(&cookies).await?;

        client
            .post_seed_comment("test_seed_comment_1", &poll_id, &cookies)
            .await?;
        client
            .post_seed_comment("test_seed_comment_2", &poll_id, &cookies)
            .await?;

        let _comments = client.get_comments(&poll_id).await?;

        Ok(())
    }
}
