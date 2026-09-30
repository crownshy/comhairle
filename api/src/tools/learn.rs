use std::sync::Arc;

use aide::axum::ApiRouter;
use async_trait::async_trait;

use crate::{ComhairleState, error::ComhairleError};

use super::{ToolConfigSanitize, ToolImpl};

pub use crate::models::tools::learn::{
    LearnPageEntry, LearnReport, LearnToolConfig, LearnToolSetup, PageContent,
};

async fn learn_setup(setup_config: &LearnToolSetup) -> Result<LearnToolConfig, ComhairleError> {
    Ok(LearnToolConfig {
        pages: setup_config.pages.clone(),
    })
}

// Keep public function for backwards compatibility
pub async fn setup(setup_config: &LearnToolSetup) -> Result<LearnToolConfig, ComhairleError> {
    learn_setup(setup_config).await
}

/// Zero-sized marker type for Learn tool implementation
pub struct LearnTool;

#[async_trait]
impl ToolImpl for LearnTool {
    type Config = LearnToolConfig;
    type Setup = LearnToolSetup;
    type Report = LearnReport;

    async fn setup(
        setup: &Self::Setup,
        _state: &Arc<ComhairleState>,
        _locale: &str,
    ) -> Result<Self::Config, ComhairleError> {
        learn_setup(setup).await
    }

    async fn clone_tool(
        config: &Self::Config,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        // Learn tool is cloneable as-is (static content)
        Ok(config.clone())
    }

    fn sanitize(config: Self::Config) -> Self::Config {
        config.sanitize()
    }

    fn routes(_state: &Arc<ComhairleState>) -> ApiRouter {
        // Learn tool has no routes
        ApiRouter::new()
    }
}
