use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::ToolConfigSanitize;
use crate::error::ComhairleError;

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub enum RankingType {
    Basic,
    Quadratic(QuadraticVoting),
    Comparative,
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct RankOption {
    pub id: Uuid,
    pub title: String,
    pub details: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct CreateRankOptionDTO {
    pub title: String,
    pub details: Option<String>,
}

impl From<CreateRankOptionDTO> for RankOption {}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct RankingToolConfig {
    pub prompt: String,
    pub options: Vec<RankOption>,
    pub ranking_type: RankingType,
}

impl ToolConfigSanitize for LearnToolConfig {
    fn sanatize(&self) -> Self {
        self.clone()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct RankingReport;

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct RankingToolSetup {
    pub prompt: String,
    pub options: Vec<String>,
    pub ranking_type: RankingType,
}

pub async fn setup(setup_config: &RankingToolSetup) -> Result<RankingToolSetup, ComhairleError> {
    Ok(RankingToolSetup {
        prompt: setup_config.prompt,
        ranking_type: setup_config.ranking_type,
        options: setup_config
            .options
            .into_iter()
            .map(|o| RankOption::from(o))
            .collect(),
    })
}
