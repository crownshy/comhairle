use std::sync::Arc;

use aide::axum::{
    routing::{get_with, post_with},
    ApiRouter,
};
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
use tracing::{info, instrument, warn};
use uuid::Uuid;

use crate::{error::ComhairleError, models, ComhairleState};

use super::{ToolConfig, ToolConfigSanitize, ToolImpl};

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
        state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        // Delegate to existing setup function
        polis_setup(setup, &state.config.polis_url).await
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
                "/polis/admin_login",
                post_with(admin_login, |op| {
                    op.id("PolisAdminLogin")
                        .tag("Tools")
                        .summary("Login as Polis admin and proxy cookie")
                        .description("Logs into Polis as admin and returns session cookie")
                }),
            )
            .api_route(
                "/polis/report_data",
                get_with(get_report_data, |op| {
                    op.id("PolisGetReportData")
                        .tag("Tools")
                        .summary("Get Polis report data for a workflow step")
                        .description("Fetches the polis data export for a given workflow step")
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

    #[error("Failed to get comments {0}")]
    FailedToGetComments(String),

    #[error("Failed to post seed comment {0}")]
    FailedToPostSeedComment(String),

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
    base_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PolisComment {
    pub tid: u32,
    pub txt: String,
    pub is_seed: bool,
    pub is_meta: bool,
    pub lang: Option<String>,
    pub pid: u32,
    pub quote_src_url: Option<String>,
    pub created: String,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct SetTopicRequest {
    pub topic: String,
    pub conversation_id: String,
}

impl PolisClient {
    pub fn new(base_url: &str) -> Self {
        let client = Client::builder().cookie_store(true).build().unwrap();
        Self {
            client,
            base_url: base_url.to_string(),
        }
    }

    pub async fn create_random_admin_user(&self) -> Result<(String, String), PolisError> {
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

    async fn login(&self, login: &PolisLogin) -> Result<String, PolisError> {
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

    pub async fn create_poll(&self) -> Result<String, PolisError> {
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

    pub async fn set_topic(&self, topic: SetTopicRequest) -> Result<(), PolisError> {
        let body = self
            .client
            .put(format!("{}/api/v3/conversations", self.base_url))
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

    #[instrument(err(Debug), skip(self))]
    pub async fn post_seed_comment(
        &self,
        comment: &str,
        poll_id: &str,
    ) -> Result<String, PolisError> {
        let post_json =
            json!({"txt":comment,"pid":"mypid","conversation_id":poll_id,"is_seed":true});

        let resp = self
            .client
            .post(format!("{}/api/v3/comments", self.base_url))
            .json(&post_json)
            .send()
            .await
            .map_err(|e| PolisError::FailedToPostSeedComment(e.to_string()))?
            .json::<PolisCommentCreateResponse>()
            .await
            .map_err(|e| PolisError::FailedToPostSeedComment(e.to_string()))?;

        Ok(resp.tid.to_string())
    }

    pub async fn get_comments(&self, poll_id: &str) -> Result<Vec<PolisComment>, PolisError> {
        let url = format!(
            "{}/api/v3/comments?conversation_id={poll_id}",
            self.base_url
        );
        let comments: Vec<PolisComment> =
            self.client
                .get(url)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .map_err(|e| PolisError::FailedToGetComments(e.to_string()))?;

        Ok(comments)
    }

    pub async fn get_math_pca(&self, poll_id: &str) -> Result<serde_json::Value, PolisError> {
        let url = format!(
            "https://{}/api/v3/math/pca2?conversation_id={}&lastVoteTimestamp=0",
            self.base_url, poll_id
        );

        info!("Getting PCA data from {url}");

        let response = self.client.get(&url).send().await.map_err(|e| {
            warn!("Failed to get PCA data: {e}");
            PolisError::FailedToGetComments(format!("Failed to get PCA data: {e}"))
        })?;

        info!("Response {response:#?}");
        let data = response.json::<serde_json::Value>().await.map_err(|e| {
            warn!("Failed to parse PCA data: {e:#?}");
            PolisError::FailedToGetComments(format!("Failed to parse PCA data: {e}"))
        })?;

        Ok(data)
    }

    pub async fn get_conversation(&self, poll_id: &str) -> Result<serde_json::Value, PolisError> {
        let url = format!(
            "https://{}/api/v3/conversations?conversation_id={}",
            self.base_url, poll_id
        );

        let response = self.client.get(&url).send().await.map_err(|e| {
            warn!("Failed to get conversation: {e}");
            PolisError::FailedToGetComments(format!("Failed to get conversation: {e}"))
        })?;

        let data = response.json::<serde_json::Value>().await.map_err(|e| {
            warn!("Failed to parse conversation: {e}");
            PolisError::FailedToGetComments(format!("Failed to parse conversation: {e}"))
        })?;

        Ok(data)
    }

    pub async fn get_comments_with_voting(
        &self,
        poll_id: &str,
    ) -> Result<serde_json::Value, PolisError> {
        let url = format!(
            "https://{}/api/v3/comments?conversation_id={}&moderation=true&include_voting_patterns=true",
            self.base_url, poll_id
        );

        let response = self.client.get(&url).send().await.map_err(|e| {
            warn!("Failed to get comments: {e}");
            PolisError::FailedToGetComments(format!("Failed to get comments: {e}"))
        })?;

        let data = response.json::<serde_json::Value>().await.map_err(|e| {
            warn!("Failed to parse comments: {e}");
            PolisError::FailedToGetComments(format!("Failed to parse comments: {e}"))
        })?;

        Ok(data)
    }

    pub async fn get_report_data(&self, poll_id: &str) -> Result<serde_json::Value, PolisError> {
        info!("Getting full report data for poll_id: {poll_id}");

        // Fetch all the data that powers the report page
        let math_pca = self.get_math_pca(poll_id).await?;
        let conversation = self.get_conversation(poll_id).await?;
        let comments = self.get_comments_with_voting(poll_id).await?;

        // Combine into a single response
        let report_data = json!({
            "math": math_pca,
            "conversation": conversation,
            "comments": comments
        });

        Ok(report_data)
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct AdminLoginQuery {
    pub workflow_step_id: Uuid,
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct ReportDataQuery {
    pub workflow_step_id: Uuid,
}

/// Gets the polis report data for a workflow step
async fn get_report_data(
    State(state): State<Arc<ComhairleState>>,
    Query(ReportDataQuery { workflow_step_id }): Query<ReportDataQuery>,
) -> Result<(StatusCode, Json<serde_json::Value>), ComhairleError> {
    let workflow_step = models::workflow_step::get_by_id(&state.db, &workflow_step_id).await?;

    if let ToolConfig::Polis(config) = workflow_step.preview_tool_config {
        let client = PolisClient::new(&config.server_url);

        // Login as admin to access the data export
        client
            .login(&PolisLogin {
                email: config.admin_user,
                password: config.admin_password,
            })
            .await?;

        // Get the report data
        let data = client.get_report_data(&config.poll_id).await?;

        Ok((StatusCode::OK, Json(data)))
    } else {
        Err(ComhairleError::WorkflowStepHasWrongType("Polis".into()))
    }
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
        let client = PolisClient::new(&config.server_url);
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

#[instrument(err(Debug))]
pub async fn launch(preview_config: &PolisToolConfig) -> Result<PolisToolConfig, ComhairleError> {
    let preview_client = PolisClient::new(&preview_config.server_url);
    let live_client = PolisClient::new(&preview_config.server_url);

    let live_poll_config = polis_setup(
        &PolisToolSetup { topic: "".into() },
        &preview_config.server_url,
    )
    .await?;

    preview_client
        .login(&PolisLogin {
            email: preview_config.admin_user.clone(),
            password: preview_config.admin_password.clone(),
        })
        .await?;

    live_client
        .login(&PolisLogin {
            email: live_poll_config.admin_user.clone(),
            password: live_poll_config.admin_password.clone(),
        })
        .await?;

    // Need to also migrate the setting for moderation
    let seed_statements = preview_client.get_comments(&preview_config.poll_id).await?;

    // TODO filter seed statements.

    for comment in seed_statements {
        let result = live_client
            .post_seed_comment(&comment.txt, &live_poll_config.poll_id)
            .await?;
    }

    let new_seed_comments = live_client.get_comments(&live_poll_config.poll_id).await?;

    Ok(live_poll_config)
}

async fn polis_setup(
    _setup: &PolisToolSetup,
    polis_url: &str,
) -> Result<PolisToolConfig, ComhairleError> {
    info!("Attempting to set up polis poll");
    let client = PolisClient::new(polis_url);
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
        server_url: polis_url.to_string(),
        poll_id,
        admin_user: email,
        admin_password: password,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn login() -> Result<(), Box<dyn std::error::Error>> {
        let client = PolisClient::new("polis.comhairle.scot");
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
        let client = PolisClient::new("polis.comhairle.scot");

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
        let client = PolisClient::new("polis.comhairle.scot");
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
        let client = PolisClient::new("polis.comhairle.scot");

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
