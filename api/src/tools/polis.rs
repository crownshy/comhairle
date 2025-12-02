use std::sync::Arc;

use aide::axum::{routing::get_with, ApiRouter};
use axum::{
    extract::{Query, Request, State},
    http::StatusCode,
    response::Response,
    Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use fancy_regex::Regex;
use rand::{distributions::Alphanumeric, Rng};
use reqwest::{header::SET_COOKIE, Client};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{error::ComhairleError, models, ComhairleState};

use super::{ToolConfig, ToolConfigSanitize};

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

        let res = self
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

    async fn login(&self, login: &PolisLogin) -> Result<(String), PolisError> {
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

    pub async fn post_seed_comment(&self) -> Result<String, PolisError> {
        let body = self
            .client
            .post(format!("{POLIS_BASE_URL}/api/v3/comments"))
            .json("")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        Ok("test".into())
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

    if let ToolConfig::Polis(config) = workflow_step.tool_config {
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

fn rewrite_local_urls(input: &str, prefix: &str) -> String {
    // Match string literals that start with a single slash ("/...") inside either single or double quotes
    // This helps avoid JS regexes (which are not inside quotes)
    let re = Regex::new(r#"(?P<quote>["'])(?P<slash>/)(?P<path>(?!/)[^"']*)"#).unwrap();

    re.replace_all(input, |caps: &fancy_regex::Captures| {
        let quote = &caps["quote"];
        let path = &caps["path"];
        let prefix = prefix.trim_end_matches('/');

        let result = format!("{quote}{}/{path}", prefix);
        info!("{result}");
        result
    })
    .into_owned()
}

pub async fn proxy(req: Request) -> Result<Response, ComhairleError> {
    let client = Client::new();
    let (parts, body) = req.into_parts();
    let method = parts.method;
    let path = parts.uri.path();
    let headers = parts.headers;

    let path_query = parts
        .uri
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let new_uri = format!(
        "http://poliscommunity.crown-shy.com{}",
        // "http://example.com{}",
        path_query.replace("/proxy", "")
    );

    info!("Proxying to {new_uri}");

    let mut new_req = Request::builder().method(method).uri(new_uri.clone());

    // Copy headers
    for (k, v) in headers.iter() {
        if k != "host" && k != "content-length" {
            new_req = new_req.header(k, v.clone());
        }
    }

    // Finalize request
    let new_req = match new_req.body(body) {
        Ok(req) => req,
        Err(_) => {
            return Err(PolisError::ProxyError {
                from: path_query.to_owned(),
                to: new_uri.clone().to_string(),
            }
            .into())
        }
    };

    let method = new_req.method().clone();
    let url = new_req.uri().clone().to_string();
    let headers = new_req.headers().clone();

    let body_bytes = axum::body::to_bytes(new_req.into_body(), usize::MAX)
        .await
        .map_err(|e| {
            warn!("Proxy error {e}");
            PolisError::ProxyError {
                from: path_query.to_owned(),
                to: new_uri.clone().to_string(),
            }
        })?;

    // let response = client
    //     .request(method, url)
    //     .headers(headers)
    //     .body(reqwest::Body::from(body_bytes))
    //     .send()
    //     .await
    //     .map_err(|err| {
    //         warn!("Polis Proxy error: {err:#?}");
    //         PolisError::ProxyError {
    //             from: path_query.to_owned(),
    //             to: new_uri.to_string(),
    //         }
    //     })?;

    let response = client
        .get(new_uri.clone())
        // .headers(headers)
        // .body(reqwest::Body::from(body_bytes))
        .send()
        .await
        .map_err(|err| {
            warn!("Polis Proxy error: {err:#?}");
            PolisError::ProxyError {
                from: path_query.to_owned(),
                to: new_uri.to_string(),
            }
        })?;

    info!("Reponse {response:#?}");

    let status = response.status();

    let mut response_builder = Response::builder().status(status);

    for (key, value) in response.headers() {
        info!("{key} : {value:#?}");
        if key == "content-type" {
            response_builder = response_builder.header(key, value);
        }
    }

    let body = response.text().await.map_err(|e| {
        warn!("{e}");

        PolisError::ProxyError {
            from: path_query.to_owned(),
            to: new_uri.to_string(),
        }
    })?;

    // let replaced = rewrite_local_urls(&body, "/tools/polis/proxy/");
    // info!("{replaced}");

    response_builder.body(body.into()).map_err(|e| {
        warn!("Proxy error {e:#?}");
        PolisError::ProxyError {
            from: path_query.to_owned(),
            to: new_uri.to_string(),
        }
        .into()
    })
    // Ok(Json(json!({"works":"ok"})).into_response())
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    // let polis_host = axum_proxy::builder_https("poliscommunity.crown-shy.com").unwrap();
    // let host1 = axum_proxy::builder_http("example.com").unwrap();
    ApiRouter::new()
        .route_service("/proxy{*rest}", axum::routing::any(proxy))
        .api_route(
            "/admin_login",
            get_with(admin_login, |op| {
                op.id("PolisAdminLogin".into())
                    .description(
                        "Used to login the current user to the specified workflow id polis",
                    )
                    .response::<200, Json<String>>()
            }),
        )
        .with_state(state)
}

pub async fn setup(_setup: &PolisToolSetup) -> Result<PolisToolConfig, ComhairleError> {
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

        let login = PolisLogin {
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
