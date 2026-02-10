use std::sync::Arc;

use aide::axum::{routing::post_with, ApiRouter};
use async_trait::async_trait;
use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use rand::{distributions::Alphanumeric, Rng};
use reqwest::{header::SET_COOKIE, Client};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{error::ComhairleError, models, ComhairleState};

use super::{ToolConfig, ToolConfigSanitize, ToolImpl};

pub const POLIS_BASE_URL: &str = "https://polis.comhairle.scot";

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema, PartialEq)]
pub struct PolisToolConfig {
    pub server_url: String,
    pub poll_id: String,

    pub admin_user: String,
    pub admin_password: String,
}

impl ToolConfigSanitize for PolisToolConfig {
    fn sanatize(&self) -> Self {
        Self {
            admin_user: "".into(),
            admin_password: "".into(),
            server_url: self.server_url.clone(),
            poll_id: self.poll_id.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct PolisToolSetup {
    pub topic: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct PolisReport;

/// Zero-sized marker type for Polis tool implementation
pub struct PolisTool;

#[async_trait]
impl ToolImpl for PolisTool {
    type Config = PolisToolConfig;
    type Setup = PolisToolSetup;
    type Report = PolisReport;

    async fn setup(
        setup: &Self::Setup,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        // Delegate to existing setup function
        polis_setup(setup).await
    }

    async fn clone_tool(
        config: &Self::Config,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        // Delegate to existing launch function
        launch(config).await
    }

    fn sanitize(config: Self::Config) -> Self::Config {
        config.sanatize()
    }

    fn routes(state: &Arc<ComhairleState>) -> ApiRouter {
        ApiRouter::new()
            .api_route(
                "/tools/polis/admin_login",
                post_with(admin_login, |op| {
                    op.id("PolisAdminLogin")
                        .summary("Login as Polis admin and proxy cookie")
                        .description("Logs into Polis as admin and returns session cookie")
                }),
            )
            .with_state(state.clone())
    }
}

#[derive(Error, Debug)]
pub enum PolisError {
    #[error("Failed to create new admin user")]
    FailedToCreateNewAdminUser,

    #[error("Failed to login")]
    FailedToLogin,

    #[error("Failed to create new poll")]
    FailedToCreateNewPoll,

    #[error("Failed to proxy route {from} : {to}")]
    ProxyError { from: String, to: String },
}

#[derive(Deserialize, Serialize)]
struct NewAdminUser {
    pub hname: String,
    pub password: String,
    pub password2: String,
    pub email: String,
    pub gatekeeperTosPrivacy: bool,
}

#[derive(Deserialize, Serialize)]
struct PolisLogin {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct NewUserResp {
    pub uid: u32,
    pub hname: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct NewPollResp {
    conversation_id: String,
}

pub struct PolisClient {
    client: reqwest::Client,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PolisComment {
    pub tid: u32,
    pub txt: String,
    pub is_seed: bool,
    pub is_meta: bool,
    pub lang: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginResp {
    pub uid: u32,
    pub email: String,
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetTopicRequest {
    pub topic: String,
    pub conversation_id: String,
}

impl PolisClient {
    pub fn new() -> Self {
        let client = Client::builder().cookie_store(true).build().unwrap();
        Self { client }
    }

    pub async fn create_random_admin_user(&self) -> Result<(String, String), PolisError> {
        info!("Creating a random admin user");
        let username: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();

        let email = format!("{username}@comhairle.com");

        let password: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
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
            .post(format!("{POLIS_BASE_URL}/api/v3/auth/new"))
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

    async fn login(&self, login: &PolisLogin) -> Result<String, PolisError> {
        info!("Logging in to polis");
        let url = format!("{POLIS_BASE_URL}/api/v3/auth/login");
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

    pub async fn create_poll(&self) -> Result<String, PolisError> {
        info!("Attepting to create a new poll");
        let new_poll = self
            .client
            .post(format!("{POLIS_BASE_URL}/api/v3/conversations"))
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

    pub async fn set_topic(&self, topic: SetTopicRequest) -> Result<(), PolisError> {
        let body = self
            .client
            .put(format!("{POLIS_BASE_URL}/api/v3/conversations"))
            .json(&topic)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("{body}");
        Ok(())
    }

    pub async fn post_seed_comment(
        &self,
        comment: &str,
        poll_id: &str,
    ) -> Result<String, PolisError> {
        let _body = self
            .client
            .post(format!("{POLIS_BASE_URL}/api/v3/comments"))
            .json(
                &json!(
                    {"txt":comment,"pid":"mypid","conversation_id":poll_id,"is_seed":true}
                )
                .to_string(),
            )
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        Ok("test".into())
    }

    pub async fn get_comments(&self, poll_id: &str) -> Result<Vec<PolisComment>, PolisError> {
        let comments: Vec<PolisComment> = self
            .client
            .get(format!(
                "{POLIS_BASE_URL}/api/v3/comments?conversation_id=${poll_id}"
            ))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        Ok(comments)
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct AdminLoginQuery {
    pub workflow_step_id: Uuid,
}

/// Logs a user into polis and proxies the cookie
/// to the frontend
async fn admin_login(
    State(state): State<Arc<ComhairleState>>,
    Query(AdminLoginQuery { workflow_step_id }): Query<AdminLoginQuery>,
    cookies: CookieJar,
) -> Result<(CookieJar, (StatusCode, Json<String>)), ComhairleError> {
    let workflow_step = models::workflow_step::get_by_id(&state.db, &workflow_step_id).await?;

    if let ToolConfig::Polis(config) = workflow_step.preview_tool_config {
        let client = PolisClient::new();
        let cookie = client
            .login(&PolisLogin {
                email: config.admin_user,
                password: config.admin_password,
            })
            .await?;
        let mut parsed_cookie = Cookie::parse(cookie).map_err(|_| PolisError::FailedToLogin)?;
        parsed_cookie.set_domain("comhairle.scot");

        let new_cookies = cookies.add(parsed_cookie);

        Ok((new_cookies, (StatusCode::OK, Json("logged in".into()))))
    } else {
        Err(ComhairleError::WorkflowStepHasWrongType("Polis".into()))
    }
}

pub async fn launch(preview_config: &PolisToolConfig) -> Result<PolisToolConfig, ComhairleError> {
    let client = PolisClient::new();
    client
        .login(&PolisLogin {
            email: preview_config.admin_user.clone(),
            password: preview_config.admin_password.clone(),
        })
        .await?;

    let poll_id = client.create_poll().await?;

    let seed_statements = client.get_comments(&preview_config.poll_id).await?;

    for comment in seed_statements {
        client.post_seed_comment(&comment.txt, &poll_id).await?;
    }

    let mut new_config = preview_config.clone();
    new_config.poll_id = poll_id;

    Ok(new_config)
}

async fn polis_setup(_setup: &PolisToolSetup) -> Result<PolisToolConfig, ComhairleError> {
    info!("Attempting to set up polis poll");
    let client = PolisClient::new();
    let (email, password) = client.create_random_admin_user().await?;
    client
        .login(&PolisLogin {
            email: email.clone(),
            password: password.clone(),
        })
        .await?;
    // sleep(Duration::from_millis(1)).await;
    let poll_id = client.create_poll().await?;

    Ok(PolisToolConfig {
        server_url: POLIS_BASE_URL.into(),
        poll_id,
        admin_user: email,
        admin_password: password,
    })
}

// Keep public function for backwards compatibility
pub async fn setup(setup: &PolisToolSetup) -> Result<PolisToolConfig, ComhairleError> {
    polis_setup(setup).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn login() -> Result<(), Box<dyn std::error::Error>> {
        let client = PolisClient::new();
        let login = PolisLogin {
            email: "xVHTX2@comhairle.com".into(),
            password: "GNgTWJ".into(),
        };
        client.login(&login).await?;
        Ok(())
    }

    // #[tokio::test]
    // async fn signup() -> Result<(), Box<dyn std::error::Error>> {
    //     let client = PolisClient::new();
    //     let user = client.create_random_admin_user().await?;
    //     Ok(())
    // }

    #[tokio::test]
    #[ignore]
    async fn create_poll() -> Result<(), Box<dyn std::error::Error>> {
        let client = PolisClient::new();

        // let login = PolisLogin {
        //     email: "xVHTX2@comhairle.com".into(),
        //     password: "GNgTWJ".into(),
        // };

        let login = PolisLogin {
            email: "LtILIo@comhairle.com".into(),
            password: "sa1d3v".into(),
        };
        client.login(&login).await?;

        client.create_poll().await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn sign_up_and_create_poll() -> Result<(), Box<dyn std::error::Error>> {
        let client = PolisClient::new();
        let (email, password) = client.create_random_admin_user().await?;
        println!("{email} {password}");

        let login = PolisLogin { email, password };

        client.login(&login).await?;

        let resp = client.create_poll().await?;
        println!("{resp:#?}");

        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn set_topic() -> Result<(), Box<dyn std::error::Error>> {
        let client = PolisClient::new();

        let _login = PolisLogin {
            email: "xVHTX2@comhairle.com".into(),
            password: "GNgTWJ".into(),
        };

        let poll_id = "6f5faeb96f";
        client
            .set_topic(SetTopicRequest {
                topic: "New Topic".into(),
                conversation_id: poll_id.into(),
            })
            .await?;
        Ok(())
    }
}
