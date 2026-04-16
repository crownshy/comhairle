use std::collections::HashMap;

use async_trait::async_trait;
use cookie::Cookie;
use rand::{distributions::Alphanumeric, Rng};
use reqwest::{
    header::{COOKIE, SET_COOKIE},
    Client,
};
use schemars::JsonSchema;
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

// Report data structures
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct CommentReportData {
    pub tid: u32,
    pub text: String,
    pub overall_votes: VoteCounts,
    pub group_votes: Vec<GroupVoteCounts>,
    pub group_informed_consensus: Option<f64>,
    pub divisiveness: Option<f64>,
    pub is_seed: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
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
    pub total_members: u64,
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

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct PcaPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct WikiPollReport {
    pub comments: Vec<CommentReportData>,
    pub groups: Vec<GroupReportData>,
    pub participants: Vec<ParticipantReportData>,
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

impl PolisClient {
    pub fn new(base_url: &str) -> Self {
        let client = Client::new();
        Self {
            client,
            base_url: base_url.to_string(),
        }
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

        let response = self.client.get(&url).send().await.map_err(|e| {
            warn!("Failed to get PCA data: {e}");
            PolisError::FailedToGetComments(format!("Failed to get PCA data: {e}"))
        })?;

        let data = response.json::<serde_json::Value>().await.map_err(|e| {
            warn!("Failed to parse PCA data: {e:#?}");
            PolisError::FailedToGetComments(format!("Failed to parse PCA data: {e}"))
        })?;

        Ok(data)
    }

    pub fn transform_report_data(
        &self,
        math_pca: serde_json::Value,
        comments_data: serde_json::Value,
    ) -> Result<WikiPollReport, WikiPollServiceError> {
        // Extract comment texts from comments_data
        let comments_array = comments_data
            .as_array()
            .ok_or_else(|| PolisError::FailedToGetComments("Invalid comments format".into()))?;

        // Create a map of tid -> comment text
        let mut comment_texts = std::collections::HashMap::new();
        let mut comment_votes: HashMap<u32, VoteCounts> = std::collections::HashMap::new();
        let mut comment_is_seed: HashMap<u32, bool> = std::collections::HashMap::new();

        for comment in comments_array {
            if let (Some(tid), Some(txt), agrees, disagrees, passes, is_seed, is_moderated) = (
                comment.get("tid").and_then(|t| t.as_u64()),
                comment.get("txt").and_then(|t| t.as_str()),
                comment
                    .get("agree_count")
                    .and_then(|t| t.as_u64())
                    .unwrap_or(0) as u32,
                comment
                    .get("disagree_count")
                    .and_then(|t| t.as_u64())
                    .unwrap_or(0) as u32,
                comment
                    .get("pass_count")
                    .and_then(|t| t.as_u64())
                    .unwrap_or(0) as u32,
                comment.get("is_seed").and_then(|t| t.as_bool()).unwrap(),
                comment.get("mod").and_then(|t| t.as_f64()).unwrap(),
            ) {
                comment_texts.insert(tid as u32, txt.to_string());
                if is_moderated > 0.0 {
                    comment_is_seed.insert(tid as u32, is_seed);
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
                for (group_id_str, group_data) in obj.iter() {
                    if let Ok(group_id) = group_id_str.parse::<u32>() {
                        // Access the "votes" object within each group
                        if let Some(votes_obj) = group_data.get("votes").and_then(|v| v.as_object())
                        {
                            // Get the votes for this specific comment (tid)
                            if let Some(votes_for_tid) = votes_obj.get(&tid.to_string()) {
                                let agrees = votes_for_tid["A"].as_u64().unwrap_or(0) as u32;
                                let disagrees = votes_for_tid["D"].as_u64().unwrap_or(0) as u32;
                                let saw = votes_for_tid["S"].as_u64().unwrap_or(0) as u32;
                                // S represents "saw" not "passes". Passes = saw - agrees - disagrees
                                let passes = saw.saturating_sub(agrees).saturating_sub(disagrees);
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
            }

            let consensus = group_aware_consensus
                .get(tid.to_string())
                .and_then(|v| v.as_f64());

            let divisiveness = comment_extremity.get(idx).and_then(|v| v.as_f64());

            if let (Some(overall_votes), Some(is_seed)) =
                (comment_votes.get(&tid), comment_is_seed.get(&tid))
            {
                comments_report.push(CommentReportData {
                    tid,
                    text,
                    overall_votes: overall_votes.clone(),
                    group_votes: group_votes_list,
                    group_informed_consensus: consensus,
                    divisiveness,
                    is_seed: is_seed.clone(),
                });
            }
        }

        // Extract group clusters and build groups
        let empty_clusters = vec![];
        let group_clusters = math_pca["group-clusters"]
            .as_array()
            .unwrap_or(&empty_clusters);

        let repness = &math_pca["repness"];

        let mut group_sizes: Vec<u64> = vec![];

        println!("{group_votes:#?}");

        for g in group_votes
            .as_object()
            .unwrap()
            .values()
        {
            let members = g.get("n-members").and_then(|v| v.as_u64()).unwrap() as u64;
            group_sizes.push(members);
        }

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
                total_members: group_sizes.get(idx).unwrap().clone(),
            });
        }

        // Build participants list with group membership
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

        // Build pid to PCA position mapping from base-clusters
        let mut pid_to_pca: std::collections::HashMap<u32, PcaPosition> =
            std::collections::HashMap::new();

        if let Some(base_clusters) = math_pca.get("base-clusters") {
            let base_members = base_clusters["members"].as_array();
            let base_x = base_clusters["x"].as_array();
            let base_y = base_clusters["y"].as_array();

            if let (Some(members_arr), Some(x_arr), Some(y_arr)) = (base_members, base_x, base_y) {
                for (idx, member_cluster) in members_arr.iter().enumerate() {
                    if let Some(member_arr) = member_cluster.as_array() {
                        // Each base cluster typically has one member (participant)
                        for pid_val in member_arr {
                            if let Some(pid) = pid_val.as_u64() {
                                let pid = pid as u32;
                                if let (Some(x), Some(y)) = (
                                    x_arr.get(idx).and_then(|v| v.as_f64()),
                                    y_arr.get(idx).and_then(|v| v.as_f64()),
                                ) {
                                    pid_to_pca.insert(pid, PcaPosition { x, y });
                                }
                            }
                        }
                    }
                }
            }
        }

        // Create participant report entries
        // Get all participants from group clusters
        for (pid, group_id) in pid_to_group.iter() {
            let pca_position = pid_to_pca.get(pid).cloned();
            participants_report.push(ParticipantReportData {
                pid: *pid,
                group_id: Some(*group_id),
                pca_position,
            });
        }

        Ok(WikiPollReport {
            comments: comments_report,
            groups: groups_report,
            participants: participants_report,
        })
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

    async fn get_report_data(&self, poll_id: &str) -> Result<WikiPollReport, WikiPollServiceError> {
        // Fetch all the data that powers the report page
        let math_pca = self.get_math_pca(poll_id).await?;
        let comments_data = self.get_comments_with_voting(poll_id).await?;

        // Transform the raw data into structured report format
        self.transform_report_data(math_pca, comments_data)
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

        let login = WikiPollLogin { email, password };

        let cookies = client.login(&login).await?;

        let resp = client.create_poll(&cookies).await?;

        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn post_seed_comment() -> Result<(), Box<dyn std::error::Error>> {
        let client = PolisClient::new("polis.comhairle.scot");
        let (email, password) = client.create_random_admin_user().await?;

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
