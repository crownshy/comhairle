pub mod config;
pub mod error;
pub mod tttc_categorizer;

use async_trait::async_trait;

use crate::categorization_service::error::Result;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CategorizationService: Sync + Send {
    async fn queue_job(&self) -> Result<()>;
}

#[cfg(test)]
impl MockCategorizationService {
    pub fn base() -> MockCategorizationService {
        let mut categorizer = MockCategorizationService::new();

        categorizer.expect_queue_job().returning(|| Ok(()));

        categorizer
    }
}
