//! HeyForm tool configuration.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::models::tools::ToolConfigSanitize;

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct HeyFormToolConfig {
    pub survey_id: String,
    pub survey_url: String,
    pub admin_user: String,
    pub admin_password: String,
    pub workspace_id: String,
    pub project_id: String,
    #[serde(default = "default_server_url")]
    pub server_url: String,
}

pub(crate) fn default_server_url() -> String {
    "forms.comhairle.scot".to_string()
}

impl ToolConfigSanitize for HeyFormToolConfig {
    fn sanitize(&self) -> Self {
        Self {
            survey_id: self.survey_id.clone(),
            survey_url: self.survey_url.clone(),
            admin_user: "".into(),
            admin_password: "".into(),
            workspace_id: self.workspace_id.clone(),
            project_id: self.project_id.clone(),
            server_url: self.server_url.clone(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct HeyFormToolSetup {
    #[serde(default = "default_server_url")]
    pub server_url: String,
}

#[derive(PartialEq, Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct HeyFormReport;
