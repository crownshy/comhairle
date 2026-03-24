pub mod error;
pub mod polis_service;

use crate::wiki_poll_service::error::WikiPollServiceError;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[cfg(test)]
use mockall::automock;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait WikiPollService: Send + Sync {
    async fn create_random_admin_user(&self) -> Result<(String, String), WikiPollServiceError>;

    async fn login(&self, login: &WikiPollLogin) -> Result<String, WikiPollServiceError>;

    async fn create_poll(&self) -> Result<String, WikiPollServiceError>;

    async fn post_seed_comment(
        &self,
        comment: &str,
        poll_id: &str,
    ) -> Result<String, WikiPollServiceError>;

    async fn get_comments(
        &self,
        poll_id: &str,
    ) -> Result<Vec<WikiPollComment>, WikiPollServiceError>;
}

#[derive(Deserialize, Serialize)]
pub struct WikiPollLogin {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WikiPollComment {
    pub tid: u32,
    pub txt: String,
    pub is_seed: bool,
    pub is_meta: bool,
    pub lang: Option<String>,
    pub pid: u32,
    pub quote_src_url: Option<String>,
    pub created: String,
}
