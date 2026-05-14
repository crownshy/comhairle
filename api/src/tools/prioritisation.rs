use std::sync::Arc;

use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{error::ComhairleError, ComhairleState};

use super::{ToolConfigSanitize, ToolImpl};

/// Opaque blob config for the Prioritisation Tool prototype.
///
/// The prototype stores all poll state (proposals, questions, answers) in the
/// frontend's `localStorage`. The backend only needs to accept the step type
/// so the workflow step can be created. `data` is an arbitrary JSON value the
/// frontend can stash light metadata in if needed.
#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct PrioritisationToolConfig {
    #[serde(default)]
    pub data: serde_json::Value,
}

impl ToolConfigSanitize for PrioritisationToolConfig {
    fn sanitize(&self) -> Self {
        self.clone()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct PrioritisationReport;

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct PrioritisationToolSetup {
    #[serde(default)]
    pub data: serde_json::Value,
}

/// Zero-sized marker type for Prioritisation tool implementation.
pub struct PrioritisationTool;

#[async_trait]
impl ToolImpl for PrioritisationTool {
    type Config = PrioritisationToolConfig;
    type Setup = PrioritisationToolSetup;
    type Report = PrioritisationReport;

    async fn setup(
        setup: &Self::Setup,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        Ok(PrioritisationToolConfig {
            data: setup.data.clone(),
        })
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
}
