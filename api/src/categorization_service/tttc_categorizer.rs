use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::categorization_service::{error::CategorizationServiceError, CreateAnalysisJobResponse};

use super::{error::Result, CategorizationService, Comment};

pub struct TttcCategorizer {
    client: Client,
    base_url: String,
    api_key: String,
    path_prefix: String,
}

impl TttcCategorizer {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        let client = Client::new();

        Self {
            client,
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            path_prefix: "/api/v1".to_string(),
        }
    }
}

#[async_trait]
impl CategorizationService for TttcCategorizer {
    async fn create_analysis_job(
        &self,
        comments: Vec<Comment>,
        webhook_url: String,
        webhook_signature: String,
    ) -> Result<CreateAnalysisJobResponse> {
        let url = format!("{}{}/jobs", self.base_url, self.path_prefix);

        let body = TttcCreateJob {
            data: comments.into_iter().map(Into::into).collect(),
            config: TttcConfig {
                title: "Comment analysis".to_string(),
                description: "Analysis of comments".to_string(),
                question: "What are the main themes?".to_string(),
                model: "gpt-4o".to_string(),
                instructions: TttcInstructions {
                    ..Default::default()
                },
                options: TttcOptions {
                    ..Default::default()
                },
            },
            webhook_url,
            webhook_secret: Some(webhook_signature),
        };

        let response = self
            .client
            .post(url)
            .header("X-API-Key", self.api_key.clone())
            .json(&body)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let text = response.text().await?;
            return Err(CategorizationServiceError::ExternalServiceFailure(text));
        }

        let json = response.json::<TttcCreateJobResponse>().await?;

        Ok(json.into())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TttcCreateJobResponse {
    job_id: String,
    status: String,
    created_at: String,
}

impl From<TttcCreateJobResponse> for CreateAnalysisJobResponse {
    fn from(r: TttcCreateJobResponse) -> Self {
        Self {
            id: r.job_id,
            status: r.status,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TttcCreateJob {
    data: Vec<TttcComment>,
    config: TttcConfig,
    webhook_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook_secret: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TttcComment {
    pub id: String,
    pub comment: String,
    pub interview: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TttcConfig {
    pub title: String,
    pub description: String,
    pub question: String,
    pub model: String,
    pub instructions: TttcInstructions,
    pub options: TttcOptions,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TttcInstructions {
    pub output_language: String,
}

impl Default for TttcInstructions {
    fn default() -> Self {
        Self {
            output_language: "en".to_string(),
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TttcOptions {
    pub cruxes: bool,
    pub sort_strategy: String,
}

impl Default for TttcOptions {
    fn default() -> Self {
        Self {
            cruxes: false,
            sort_strategy: "numPeople".to_string(),
        }
    }
}

impl From<Comment> for TttcComment {
    fn from(c: Comment) -> Self {
        Self {
            id: c.id,
            comment: c.comment,
            interview: c.speaker,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::test_state;

    use super::*;

    use sqlx::PgPool;
    use std::error::Error;

    #[sqlx::test]
    #[ignore]
    async fn should_create_new_tttc_job(pool: PgPool) -> std::result::Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;

        let comments = vec![
            Comment {
                id: "1".to_string(),
                comment: "I like dogs".to_string(),
                speaker: "speaker_1".to_string(),
            },
            Comment {
                id: "2".to_string(),
                comment: "I like cats".to_string(),
                speaker: "speaker_2".to_string(),
            },
        ];

        let config = state.config.categorization_service.unwrap();

        let tttc_client = TttcCategorizer::new(&config.server_url.clone(), &config.api_key.clone());

        let response = tttc_client
            .create_analysis_job(
                comments,
                "http://localhost:3000/api".to_string(),
                "signature".to_string(),
            )
            .await?;

        assert_eq!(response.status, "queued".to_string());

        Ok(())
    }
}
