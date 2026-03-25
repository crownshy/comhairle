use std::sync::Arc;

use aide::axum::{routing::post_with, ApiRouter};
use async_trait::async_trait;
use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models,
    wiki_poll_service::{WikiPollLogin, WikiPollService},
    ComhairleState,
};

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
        polis_setup(setup, &state.config.polis_url, &state.wiki_poll_service).await
    }

    async fn clone_tool(
        config: &Self::Config,
        state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        // Delegate to existing launch function
        launch(config, &state.wiki_poll_service).await
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
            .with_state(state.clone())
    }
}

#[derive(Error, Debug)]
pub enum PolisError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

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
        let client = &state.wiki_poll_service;
        let cookie = client
            .login(&WikiPollLogin {
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

#[instrument(err(Debug), skip(client))]
pub async fn launch(
    preview_config: &PolisToolConfig,
    client: &Arc<dyn WikiPollService>,
) -> Result<PolisToolConfig, ComhairleError> {
    // Login as preview config user
    client
        .login(&WikiPollLogin {
            email: preview_config.admin_user.clone(),
            password: preview_config.admin_password.clone(),
        })
        .await?;

    // Need to also migrate the setting for moderation
    let seed_statements = client.get_comments(&preview_config.poll_id).await?;

    let live_poll_config = polis_setup(
        &PolisToolSetup { topic: "".into() },
        &preview_config.server_url,
        client,
    )
    .await?;

    // Login as live config user
    client
        .login(&WikiPollLogin {
            email: live_poll_config.admin_user.clone(),
            password: live_poll_config.admin_password.clone(),
        })
        .await?;

    // TODO: filter seed statements.

    for comment in seed_statements {
        client
            .post_seed_comment(&comment.txt, &live_poll_config.poll_id)
            .await?;
    }

    Ok(live_poll_config)
}

async fn polis_setup(
    _setup: &PolisToolSetup,
    polis_url: &str,
    client: &Arc<dyn WikiPollService>,
) -> Result<PolisToolConfig, ComhairleError> {
    let (email, password) = client.create_random_admin_user().await?;
    client
        .login(&WikiPollLogin {
            email: email.clone(),
            password: password.clone(),
        })
        .await?;
    let poll_id = client.create_poll().await?;

    Ok(PolisToolConfig {
        server_url: polis_url.to_string(),
        poll_id,
        admin_user: email,
        admin_password: password,
    })
}
