pub mod error;
pub mod polis_service;

use crate::wiki_poll_service::{error::WikiPollServiceError, polis_service::WikiPollReport};

use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, sqlx::Type, Clone, JsonSchema, Default)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
pub enum ModerationStatus {
    #[sqlx(rename = "accepted")]
    Accepted,
    #[sqlx(rename = "rejected")]
    Rejected,
    #[sqlx(rename = "pending")]
    #[default]
    Pending,
}

impl std::fmt::Display for ModerationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ModerationStatus::Accepted => "accepted",
            ModerationStatus::Rejected => "rejected",
            ModerationStatus::Pending => "pending",
        };
        write!(f, "{}", value)
    }
}

impl TryFrom<i32> for ModerationStatus {
    type Error = WikiPollServiceError;
    fn try_from(value: i32) -> Result<Self, WikiPollServiceError> {
        match value {
            -1 => Ok(ModerationStatus::Rejected),
            1 => Ok(ModerationStatus::Accepted),
            0 => Ok(ModerationStatus::Pending),
            _ => Err(WikiPollServiceError::UnknownModerationStatus),
        }
    }
}

#[cfg(test)]
use mockall::automock;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait WikiPollService: Send + Sync {
    async fn create_random_admin_user(&self) -> Result<(String, String), WikiPollServiceError>;

    async fn login(&self, login: &WikiPollLogin) -> Result<String, WikiPollServiceError>;

    async fn create_poll(&self, auth_cookies: &str) -> Result<String, WikiPollServiceError>;

    async fn post_seed_comment(
        &self,
        comment: &str,
        poll_id: &str,
        auth_cookies: &str,
    ) -> Result<String, WikiPollServiceError>;

    async fn get_comments(
        &self,
        poll_id: &str,
    ) -> Result<Vec<WikiPollComment>, WikiPollServiceError>;

    async fn get_xids(
        &self,
        poll_id: &str,
        auth_cookies: &str,
    ) -> Result<Vec<WikiPollXid>, WikiPollServiceError>;

    async fn get_report_data(&self, poll_id: &str) -> Result<WikiPollReport, WikiPollServiceError>;

    async fn moderate_comment(
        &self,
        poll_id: &str,
        tid: i32,
        decision: ModerationStatus,
        auth_cookies: &str,
    ) -> Result<(), WikiPollServiceError>;
}

impl ModerationStatus {
    pub fn mod_value(self) -> i32 {
        match self {
            ModerationStatus::Accepted => 1,
            ModerationStatus::Rejected => -1,
            ModerationStatus::Pending => 0,
        }
    }

    pub fn active(self) -> bool {
        matches!(self, ModerationStatus::Accepted)
    }
}

#[derive(Deserialize, Serialize)]
pub struct WikiPollLogin {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct WikiPollComment {
    pub tid: u32,
    pub txt: String,
    pub is_seed: bool,
    pub is_meta: bool,
    pub lang: Option<String>,
    pub pid: u32,
    pub quote_src_url: Option<String>,
    pub created: String,
    #[serde(alias = "mod")]
    pub moderation: i32,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct WikiPollXid {
    pub pid: u32,
    pub xid: String,
}

#[cfg(test)]
impl MockWikiPollService {
    pub fn base() -> MockWikiPollService {
        let mut wiki_poll_service = MockWikiPollService::new();

        wiki_poll_service
            .expect_create_random_admin_user()
            .returning(|| {
                Box::pin(async move {
                    Ok(("test_poll_user@mock.com".to_string(), "test_pw".to_string()))
                })
            });
        wiki_poll_service
            .expect_login()
            .returning(|_| Box::pin(async move { Ok("wiki_poll_auth_cookie".to_string()) }));
        wiki_poll_service
            .expect_create_poll()
            .returning(|_| Box::pin(async move { Ok("test_poll_id".to_string()) }));
        wiki_poll_service.expect_get_comments().returning(|_| {
            Box::pin(async move {
                Ok(vec![WikiPollComment {
                    ..Default::default()
                }])
            })
        });
        wiki_poll_service
            .expect_get_xids()
            .returning(|_, _| Box::pin(async move { Ok(vec![]) }));
        wiki_poll_service
            .expect_moderate_comment()
            .returning(|_, _, _, _| Box::pin(async move { Ok(()) }));

        wiki_poll_service
    }
}
