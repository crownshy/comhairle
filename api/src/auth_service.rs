pub mod config;
pub mod error;
pub mod keycloak;

use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait AuthService: Send + Sync {
    // TODO:
    async fn get_users(&self) -> ();
}
