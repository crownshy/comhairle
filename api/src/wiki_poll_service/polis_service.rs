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

// Raw Polis API response structures

#[derive(Deserialize, Debug)]
struct PolisCommentWithVoting {
    tid: u32,
    txt: String,
    #[serde(default)]
    agree_count: u32,
    #[serde(default)]
    disagree_count: u32,
    #[serde(default)]
    pass_count: u32,
    is_seed: bool,
    #[serde(rename = "mod")]
    moderation: f64,
}

#[derive(Deserialize, Debug)]
struct PolisMathPca {
    tids: Vec<u32>,
    #[serde(rename = "group-votes")]
    group_votes: HashMap<String, GroupVoteData>,
    #[serde(rename = "group-aware-consensus")]
    group_aware_consensus: HashMap<String, f64>,
    pca: PcaData,
    #[serde(rename = "group-clusters")]
    group_clusters: Vec<GroupCluster>,
    repness: HashMap<String, Vec<RepnessEntry>>,
    #[serde(rename = "base-clusters")]
    base_clusters: BaseClusters,
}

#[derive(Deserialize, Debug)]
struct GroupVoteData {
    #[serde(rename = "n-members")]
    n_members: u64,
    votes: HashMap<String, VoteBreakdown>,
}

#[derive(Deserialize, Debug)]
struct VoteBreakdown {
    #[serde(rename = "A")]
    agrees: u32,
    #[serde(rename = "D")]
    disagrees: u32,
    #[serde(rename = "S")]
    saw: u32,
}

#[derive(Deserialize, Debug)]
struct PcaData {
    #[serde(rename = "comment-extremity")]
    comment_extremity: Vec<f64>,
}

#[derive(Deserialize, Debug)]
struct GroupCluster {
    members: Vec<u32>,
}

#[derive(Deserialize, Debug)]
struct RepnessEntry {
    tid: u32,
}

#[derive(Deserialize, Debug)]
struct BaseClusters {
    members: Vec<Vec<u32>>,
    x: Vec<f64>,
    y: Vec<f64>,
}

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

    async fn get_comments_with_voting(
        &self,
        poll_id: &str,
    ) -> Result<Vec<PolisCommentWithVoting>, PolisError> {
        let url = format!(
            "https://{}/api/v3/comments?conversation_id={}&moderation=true&include_voting_patterns=true",
            self.base_url, poll_id
        );

        let response = self.client.get(&url).send().await.map_err(|e| {
            warn!("Failed to get comments: {e}");
            PolisError::FailedToGetComments(format!("Failed to get comments: {e}"))
        })?;

        let data = response.json::<Vec<PolisCommentWithVoting>>().await.map_err(|e| {
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

    async fn get_math_pca(&self, poll_id: &str) -> Result<PolisMathPca, PolisError> {
        let url = format!(
            "https://{}/api/v3/math/pca2?conversation_id={}&lastVoteTimestamp=0",
            self.base_url, poll_id
        );

        let response = self.client.get(&url).send().await.map_err(|e| {
            warn!("Failed to get PCA data: {e}");
            PolisError::FailedToGetComments(format!("Failed to get PCA data: {e}"))
        })?;

        let data = response.json::<PolisMathPca>().await.map_err(|e| {
            warn!("Failed to parse PCA data: {e:#?}");
            PolisError::FailedToGetComments(format!("Failed to parse PCA data: {e}"))
        })?;

        Ok(data)
    }

    fn transform_report_data(
        &self,
        math_pca: PolisMathPca,
        comments_data: Vec<PolisCommentWithVoting>,
    ) -> Result<WikiPollReport, WikiPollServiceError> {
        // Create maps for easy lookup
        let mut comment_texts = HashMap::new();
        let mut comment_votes = HashMap::new();
        let mut comment_is_seed = HashMap::new();

        for comment in comments_data.iter() {
            comment_texts.insert(comment.tid, comment.txt.clone());
            if comment.moderation > 0.0 {
                comment_is_seed.insert(comment.tid, comment.is_seed);
                comment_votes.insert(
                    comment.tid,
                    VoteCounts {
                        agrees: comment.agree_count,
                        disagrees: comment.disagree_count,
                        passes: comment.pass_count,
                    },
                );
            }
        }

        // Build comments with vote counts
        let mut comments_report = Vec::new();
        for (idx, &tid) in math_pca.tids.iter().enumerate() {
            let text = comment_texts.get(&tid).cloned().unwrap_or_default();

            // Calculate group votes for this comment
            let mut group_votes_list = Vec::new();
            for (group_id_str, group_data) in math_pca.group_votes.iter() {
                if let Ok(group_id) = group_id_str.parse::<u32>() {
                    if let Some(vote_breakdown) = group_data.votes.get(&tid.to_string()) {
                        let passes = vote_breakdown
                            .saw
                            .saturating_sub(vote_breakdown.agrees)
                            .saturating_sub(vote_breakdown.disagrees);

                        group_votes_list.push(GroupVoteCounts {
                            group_id,
                            agrees: vote_breakdown.agrees,
                            disagrees: vote_breakdown.disagrees,
                            passes,
                        });
                    }
                }
            }

            let consensus = math_pca.group_aware_consensus.get(&tid.to_string()).copied();
            let divisiveness = math_pca.pca.comment_extremity.get(idx).copied();

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
                    is_seed: *is_seed,
                });
            }
        }

        // Build groups report
        let group_sizes: Vec<u64> = math_pca
            .group_votes
            .values()
            .map(|g| g.n_members)
            .collect();

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
        for (idx, cluster) in math_pca.group_clusters.iter().enumerate() {
            let group_id = idx as u32;

            // Get representative comments for this group
            let representative_comments = math_pca
                .repness
                .get(&group_id.to_string())
                .map(|rep_entries| {
                    rep_entries
                        .iter()
                        .map(|entry| {
                            let text = comment_texts.get(&entry.tid).cloned().unwrap_or_default();
                            RepresentativeComment {
                                tid: entry.tid,
                                text,
                            }
                        })
                        .collect()
                })
                .unwrap_or_default();

            groups_report.push(GroupReportData {
                group_id,
                representative_comments,
                members: cluster.members.clone(),
                total_members: *group_sizes.get(idx).unwrap_or(&0),
            });
        }

        // Build participants list with group membership
        let mut pid_to_group = HashMap::new();
        for (group_id, cluster) in math_pca.group_clusters.iter().enumerate() {
            for &pid in cluster.members.iter() {
                pid_to_group.insert(pid, group_id as u32);
            }
        }

        // Build pid to PCA position mapping from base-clusters
        let mut pid_to_pca = HashMap::new();
        for (idx, member_cluster) in math_pca.base_clusters.members.iter().enumerate() {
            for &pid in member_cluster.iter() {
                if let (Some(&x), Some(&y)) = (
                    math_pca.base_clusters.x.get(idx),
                    math_pca.base_clusters.y.get(idx),
                ) {
                    pid_to_pca.insert(pid, PcaPosition { x, y });
                }
            }
        }

        // Create participant report entries
        let participants_report = pid_to_group
            .iter()
            .map(|(&pid, &group_id)| {
                let pca_position = pid_to_pca.get(&pid).cloned();
                ParticipantReportData {
                    pid,
                    group_id: Some(group_id),
                    pca_position,
                }
            })
            .collect();

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
