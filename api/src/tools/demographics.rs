use std::sync::Arc;

use aide::axum::ApiRouter;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{ComhairleState, error::ComhairleError};

use super::{ToolConfigSanitize, ToolImpl};

/// Which version of a demographic question or form a step asks.
#[derive(Clone, Copy, Deserialize, Serialize, Debug, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum VersionPolicy {
    /// The latest published version at the time the step is shown.
    Latest,
    /// A specific published version that never changes.
    Pinned { version_id: Uuid },
}

/// Overrides one question of a form for this step. Questions without an override use the
/// form's own settings.
#[derive(Clone, Copy, Deserialize, Serialize, Debug, JsonSchema, PartialEq, Eq)]
pub struct QuestionOverride {
    pub question_id: Uuid,
    pub enabled: bool,
    pub required: bool,
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq, Eq)]
pub struct DemographicQuestionToolConfig {
    pub question_id: Uuid,
    pub version: VersionPolicy,
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq, Eq)]
pub struct DemographicFormToolConfig {
    pub form_id: Uuid,
    pub version: VersionPolicy,
    #[serde(default)]
    pub questions: Vec<QuestionOverride>,
}

impl ToolConfigSanitize for DemographicQuestionToolConfig {
    fn sanitize(&self) -> Self {
        self.clone()
    }
}

impl ToolConfigSanitize for DemographicFormToolConfig {
    fn sanitize(&self) -> Self {
        self.clone()
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct DemographicReport;

pub type DemographicQuestionToolSetup = DemographicQuestionToolConfig;
pub type DemographicFormToolSetup = DemographicFormToolConfig;

pub struct DemographicQuestionTool;

#[async_trait]
impl ToolImpl for DemographicQuestionTool {
    type Config = DemographicQuestionToolConfig;
    type Setup = DemographicQuestionToolSetup;
    type Report = DemographicReport;

    async fn setup(
        setup: &Self::Setup,
        _state: &Arc<ComhairleState>,
        _locale: &str,
    ) -> Result<Self::Config, ComhairleError> {
        Ok(setup.clone())
    }

    async fn clone_tool(
        config: &Self::Config,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        Ok(config.clone())
    }

    fn sanitize(config: Self::Config) -> Self::Config {
        config.sanitize()
    }

    fn routes(_state: &Arc<ComhairleState>) -> ApiRouter {
        ApiRouter::new()
    }
}

pub struct DemographicFormTool;

#[async_trait]
impl ToolImpl for DemographicFormTool {
    type Config = DemographicFormToolConfig;
    type Setup = DemographicFormToolSetup;
    type Report = DemographicReport;

    async fn setup(
        setup: &Self::Setup,
        _state: &Arc<ComhairleState>,
        _locale: &str,
    ) -> Result<Self::Config, ComhairleError> {
        Ok(setup.clone())
    }

    async fn clone_tool(
        config: &Self::Config,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        Ok(config.clone())
    }

    fn sanitize(config: Self::Config) -> Self::Config {
        config.sanitize()
    }

    fn routes(_state: &Arc<ComhairleState>) -> ApiRouter {
        ApiRouter::new()
    }
}
