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

// Report data structures
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct CommentReportData {
    pub tid: u32,
    pub text: String,
    pub overall_votes: VoteCounts,
    pub group_votes: Vec<GroupVoteCounts>,
    pub group_informed_consensus: Option<f64>,
    pub divisiveness: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct VoteCounts {
    pub agrees: u32,
    pub disagrees: u32,
    pub passes: u32,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct GroupVoteCounts {
    pub group_id: u32,
    pub agrees: u32,
    pub disagrees: u32,
    pub passes: u32,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct GroupReportData {
    pub group_id: u32,
    pub representative_comments: Vec<RepresentativeComment>,
    pub members: Vec<u32>, // pids
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct RepresentativeComment {
    pub tid: u32,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct ParticipantReportData {
    pub pid: u32,
    pub group_id: Option<u32>,
    pub pca_position: Option<PcaPosition>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct PcaPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct PolisReportResponse {
    pub comments: Vec<CommentReportData>,
    pub groups: Vec<GroupReportData>,
    pub participants: Vec<ParticipantReportData>,
}

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

    pub async fn get_report_data(&self, poll_id: &str) -> Result<PolisReportResponse, PolisError> {
        info!("Getting full report data for poll_id: {poll_id}");

        // Fetch all the data that powers the report page
        let math_pca = self.get_math_pca(poll_id).await?;
        let comments_data = self.get_comments_with_voting(poll_id).await?;

        info!("math pca {math_pca:#?}");
        info!("comments {comments_data:#?}");

        // Transform the raw data into structured report format
        self.transform_report_data(math_pca, comments_data)
    }

    fn transform_report_data(
        &self,
        math_pca: serde_json::Value,
        comments_data: serde_json::Value,
    ) -> Result<PolisReportResponse, PolisError> {
        // Extract comment texts from comments_data
        let comments_array = comments_data
            .as_array()
            .ok_or_else(|| PolisError::FailedToGetComments("Invalid comments format".into()))?;

        // Create a map of tid -> comment text
        let mut comment_texts = std::collections::HashMap::new();
        let comment_votes: HashMap<u32, VoteCounts> = std::collections::HashMap::new();

        for comment in comments_array {
            if let (Some(tid), Some(txt), agrees, disagrees, passes) = (
                comment.get("tid").and_then(|t| t.as_u64()),
                comment.get("txt").and_then(|t| t.as_str()),
                comment
                    .get("agree_count")
                    .and_then(|t| t.as_u64())
                    .unwrap()
                    .unwrap_or(0) as u32,
                comment
                    .get("disagree_count")
                    .and_then(|t| t.as_u64())
                    .unwrap()
                    .unwrap_or(0) as u32,
                comment
                    .get("pass_count")
                    .and_then(|t| t.as_u64())
                    .unwrap()
                    .unwrap_or(0) as u32,
            ) {
                comment_texts.insert(tid as u32, txt.to_string());
                comment_votes.insert(
                    tid as u32,
                    VoteCounts {
                        agrees,
                        disagrees,
                        passes,
                    },
                );
            }
        }

        // Extract data from math_pca
        let tids = math_pca["tids"]
            .as_array()
            .ok_or_else(|| PolisError::FailedToGetComments("No tids in math data".into()))?;

        let group_votes = &math_pca["group-votes"];
        let group_aware_consensus = &math_pca["group-aware-consensus"];
        let empty_vec = vec![];
        let comment_extremity = math_pca["pca"]["comment-extremity"]
            .as_array()
            .unwrap_or(&empty_vec);

        // Build comments with vote counts
        let mut comments_report = Vec::new();
        for (idx, tid_val) in tids.iter().enumerate() {
            let tid = tid_val.as_u64().unwrap_or(0) as u32;
            let text = comment_texts.get(&tid).cloned().unwrap_or_default();

            // Calculate overall votes from group votes
            let mut group_votes_list = Vec::new();

            if let Some(obj) = group_votes.as_object() {
                for (group_id_str, group_vote_data) in obj.iter() {
                    if let Ok(group_id) = group_id_str.parse::<u32>() {
                        if let Some(votes_for_tid) = group_vote_data.get(tid.to_string()) {
                            let agrees = votes_for_tid["A"].as_u64().unwrap_or(0) as u32;
                            let disagrees = votes_for_tid["D"].as_u64().unwrap_or(0) as u32;
                            let passes = votes_for_tid["S"].as_u64().unwrap_or(0) as u32;
                            group_votes_list.push(GroupVoteCounts {
                                group_id,
                                agrees,
                                disagrees,
                                passes,
                            });
                        }
                    }
                }
            }

            let consensus = group_aware_consensus
                .get(tid.to_string())
                .and_then(|v| v.as_f64());

            let divisiveness = comment_extremity.get(idx).and_then(|v| v.as_f64());

            comments_report.push(CommentReportData {
                tid,
                text,
                overall_votes: *comment_votes.get(&tid).unwrap().clone(),
                group_votes: group_votes_list,
                group_informed_consensus: consensus,
                divisiveness,
            });
        }

        // Extract group clusters and build groups
        let empty_clusters = vec![];
        let group_clusters = math_pca["group-clusters"]
            .as_array()
            .unwrap_or(&empty_clusters);

        let repness = &math_pca["repness"];

        let mut groups_report = Vec::new();
        for (idx, cluster) in group_clusters.iter().enumerate() {
            let group_id = idx as u32;
            let members: Vec<u32> = cluster["members"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_u64().map(|n| n as u32))
                        .collect()
                })
                .unwrap_or_default();

            // Get representative comments for this group
            let mut representative_comments = Vec::new();
            if let Some(rep_array) = repness.get(group_id.to_string()).and_then(|v| v.as_array()) {
                for rep in rep_array {
                    if let Some(tid) = rep.get("tid").and_then(|t| t.as_u64()) {
                        let tid = tid as u32;
                        let text = comment_texts.get(&tid).cloned().unwrap_or_default();
                        representative_comments.push(RepresentativeComment { tid, text });
                    }
                }
            }

            groups_report.push(GroupReportData {
                group_id,
                representative_comments,
                members,
            });
        }

        // Build participants list with group membership
        let bid_to_pid = &math_pca["bidToPid"];
        let pca_x = math_pca["pca"]["x"].as_array();
        let pca_y = math_pca["pca"]["y"].as_array();

        let mut participants_report = Vec::new();
        let mut pid_to_group: std::collections::HashMap<u32, u32> =
            std::collections::HashMap::new();

        // Build pid to group mapping from group clusters
        for (group_id, cluster) in group_clusters.iter().enumerate() {
            if let Some(members) = cluster["members"].as_array() {
                for member in members {
                    if let Some(pid) = member.as_u64() {
                        pid_to_group.insert(pid as u32, group_id as u32);
                    }
                }
            }
        }

        // If we have bidToPid mapping, use it to get all participants
        if let Some(bid_array) = bid_to_pid.as_array() {
            for (bid, pids_value) in bid_array.iter().enumerate() {
                if let Some(pids) = pids_value.as_array() {
                    for pid_val in pids {
                        if let Some(pid) = pid_val.as_u64() {
                            let pid = pid as u32;
                            let group_id = pid_to_group.get(&pid).copied();

                            // Get PCA position if available
                            let pca_position = if let (Some(x_arr), Some(y_arr)) = (pca_x, pca_y) {
                                x_arr.get(bid).and_then(|x| x.as_f64()).and_then(|x| {
                                    y_arr
                                        .get(bid)
                                        .and_then(|y| y.as_f64())
                                        .map(|y| PcaPosition { x, y })
                                })
                            } else {
                                None
                            };

                            participants_report.push(ParticipantReportData {
                                pid,
                                group_id,
                                pca_position,
                            });
                        }
                    }
                }
            }
        } else {
            // Fallback: just list participants from groups
            for (pid, group_id) in pid_to_group.iter() {
                participants_report.push(ParticipantReportData {
                    pid: *pid,
                    group_id: Some(*group_id),
                    pca_position: None,
                });
            }
        }

        Ok(PolisReportResponse {
            comments: comments_report,
            groups: groups_report,
            participants: participants_report,
        })
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
) -> Result<(StatusCode, Json<PolisReportResponse>), ComhairleError> {
    let workflow_step = models::workflow_step::get_by_id(&state.db, &workflow_step_id).await?;

    println!("{workflow_step:#?}");

    let config = match (workflow_step.tool_config, workflow_step.preview_tool_config) {
        (Some(ToolConfig::Polis(config)), _) => config,
        (None, ToolConfig::Polis(config)) => config,
        _ => return Err(ComhairleError::WorkflowStepHasWrongType("Polis".into())),
    };

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
    let live_auth_cookies = client
        .login(&WikiPollLogin {
            email: live_poll_config.admin_user.clone(),
            password: live_poll_config.admin_password.clone(),
        })
        .await?;

    // TODO: filter seed statements.

    for comment in seed_statements {
        client
            .post_seed_comment(&comment.txt, &live_poll_config.poll_id, &live_auth_cookies)
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
    let auth_cookies = client
        .login(&WikiPollLogin {
            email: email.clone(),
            password: password.clone(),
        })
        .await?;
    let poll_id = client.create_poll(&auth_cookies).await?;

    Ok(PolisToolConfig {
        server_url: polis_url.to_string(),
        poll_id,
        admin_user: email,
        admin_password: password,
    })
}
