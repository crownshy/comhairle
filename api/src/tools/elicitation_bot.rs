use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ComhairleError;

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct ElicitationBotToolConfig;

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct ElicitationBotToolSetup;

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct ElicitationBotReport;

pub async fn setup(
    _config: &ElicitationBotToolSetup,
) -> Result<ElicitationBotToolConfig, ComhairleError> {
    Ok(ElicitationBotToolConfig)
}
