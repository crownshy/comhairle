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
            .returning(|| Box::pin(async move { Ok("test_poll_id".to_string()) }));
        wiki_poll_service.expect_get_comments().returning(|_| {
            Box::pin(async move {
                Ok(vec![WikiPollComment {
                    ..Default::default()
                }])
            })
        });

        wiki_poll_service
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::test_helpers::test_state;

    use super::*;

    #[sqlx::test]
    #[ignore]
    async fn login(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let state = test_state().db(pool).call()?;
        let login = WikiPollLogin {
            email: "xVHTX2@comhairle.com".into(),
            password: "GNgTWJ".into(),
        };
        state.wiki_poll_service.login(&login).await?;
        Ok(())
    }

    #[sqlx::test]
    #[ignore]
    async fn create_poll(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let state = test_state().db(pool).call()?;

        let login = WikiPollLogin {
            email: "LtILIo@comhairle.com".into(),
            password: "sa1d3v".into(),
        };
        state.wiki_poll_service.login(&login).await?;

        state.wiki_poll_service.create_poll().await?;
        Ok(())
    }

    #[sqlx::test]
    #[ignore]
    async fn sign_up_and_create_poll(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let state = test_state().db(pool).call()?;
        let (email, password) = state.wiki_poll_service.create_random_admin_user().await?;

        let login = WikiPollLogin { email, password };

        state.wiki_poll_service.login(&login).await?;

        let _ = state.wiki_poll_service.create_poll().await?;

        Ok(())
    }
}
