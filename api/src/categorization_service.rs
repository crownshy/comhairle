pub mod config;
pub mod error;
pub mod tttc_categorizer;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::categorization_service::error::Result;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CategorizationService: Sync + Send {
    /// Submits a batch of comments for analysis, returning a handle to the created job.
    ///
    /// The service will process the provided comments asynchronously, categorizing
    /// opinions and generating summaries or reports from the free-text input.
    ///
    /// # Arguments
    ///
    /// * `comments` — The free-text comments to be analyzed and categorized.
    /// * `webhook_url` — An optional URL to be called by the service when the analysis
    ///   job completes. If provided, the service will POST the finished report to this
    ///   endpoint. Not all implementations support webhooks; those that don't may
    ///   ignore this parameter.
    ///
    /// # Errors
    ///
    /// Returns an error if the job could not be created, for example due to an
    /// invalid request or a failure communicating with the underlying service.
    async fn create_analysis_job(
        &self,
        comments: Vec<Comment>,
        webhook_url: Option<String>,
    ) -> Result<CreateAnalysisJobResponse>;
}

#[cfg(test)]
impl MockCategorizationService {
    pub fn base() -> MockCategorizationService {
        let mut categorizer = MockCategorizationService::new();

        categorizer.expect_create_analysis_job().returning(|_, _| {
            Ok(CreateAnalysisJobResponse {
                ..Default::default()
            })
        });

        categorizer
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CreateAnalysisJobResponse {
    pub id: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    pub id: String,
    pub comment: String,
    pub speaker: String,
}
