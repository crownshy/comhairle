use async_trait::async_trait;
use rand::{distributions::Alphanumeric, Rng};
use reqwest::{header::SET_COOKIE, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{info, instrument, warn};

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
        let client = Client::builder().cookie_store(true).build().unwrap();
        Self {
            client,
            base_url: base_url.to_string(),
        }
    }
}

#[async_trait]
impl WikiPollService for PolisClient {
    async fn create_random_admin_user(&self) -> Result<(String, String), WikiPollServiceError> {
        info!("Creating a random admin user");
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
            // .json::<NewUserResp>()
            .await
            .map_err(|e| {
                warn!("{e}");
                PolisError::FailedToCreateNewAdminUser
            })?;

        Ok((new_user.email, new_user.password))
    }

    async fn login(&self, login: &WikiPollLogin) -> Result<String, WikiPollServiceError> {
        info!("Logging in to polis");
        let url = format!("https://{}/api/v3/auth/login", self.base_url);
        println!("format {url}");
        let resp = self
            .client
            .post(url)
            .json(&login)
            .send()
            .await
            .map_err(|e| {
                println!("First bit {e}");
                PolisError::FailedToLogin
            })?;

        let cookie = resp
            .headers()
            .get(SET_COOKIE)
            .ok_or(PolisError::FailedToLogin)?
            .to_str()
            .map_err(|_| PolisError::FailedToLogin)?
            .to_owned();

        let login_resp = resp
            .json::<LoginResp>()
            // .text()
            .await
            .map_err(|e| {
                println!("{e}");
                PolisError::FailedToLogin
            })?;

        info!("Logged user into polis {login_resp:#?}");

        Ok(cookie)
    }

    async fn create_poll(&self) -> Result<String, WikiPollServiceError> {
        info!("Attempting to create a new poll");
        let new_poll = self
            .client
            .post(format!("https://{}/api/v3/conversations", self.base_url))
            .send()
            .await
            .map_err(|e| {
                warn!("Failed to create new poll: {e:#?}");
                PolisError::FailedToCreateNewPoll
            })?
            // .text()
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
    ) -> Result<String, WikiPollServiceError> {
        let post_json =
            json!({"txt":comment,"pid":"mypid","conversation_id":poll_id,"is_seed":true});

        let resp = self
            .client
            .post(format!("https://{}/api/v3/comments", self.base_url))
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
struct NewUserResp {
    pub uid: u32,
    pub hname: String,
    pub email: String,
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
