use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ComhairleError;

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct HeyFormToolConfig {
    pub survey_id: String,
    pub survey_url: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct HeyFormToolSetup;

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct HeyFormReport;

pub async fn setup(setup_config: &HeyFormToolSetup) -> Result<HeyFormToolConfig, ComhairleError> {
    Ok(HeyFormToolConfig {
        survey_id: "".into(),
        survey_url: "".into(),
    })
}
