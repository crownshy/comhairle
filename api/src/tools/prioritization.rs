use std::sync::Arc;

use aide::axum::ApiRouter;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    error::ComhairleError,
    tools::{ToolConfigSanitize, ToolImpl},
    ComhairleState,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
pub struct PrioritizationToolConfig {
    questions: Vec<Question>,
    randomize_order: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
pub struct Question {
    pub text: String,
    pub r#type: QuestionType,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
pub enum QuestionType {
    Text(String),
    LikertScale { categories: Vec<Category> },
    Continuous { label: String, sub_steps: i32 },
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
pub struct Category {
    value: f64,
    label: String,
}

impl ToolConfigSanitize for PrioritizationToolConfig {
    fn sanitize(&self) -> Self {
        Self {
            questions: self.questions.clone(),
            randomize_order: self.randomize_order,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct PrioritizationToolSetup {
    pub questions: Vec<Question>,
    pub randomize_order: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct PrioritizationReport;

pub struct PrioritizationTool;

#[async_trait]
impl ToolImpl for PrioritizationTool {
    type Config = PrioritizationToolConfig;
    type Setup = PrioritizationToolSetup;
    type Report = PrioritizationReport;

    async fn setup(
        setup: &Self::Setup,
        state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        todo!();
    }

    async fn clone_tool(
        config: &Self::Config,
        state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        todo!();
    }

    fn sanitize(config: Self::Config) -> Self::Config {
        config.sanitize()
    }

    fn routes(state: &Arc<ComhairleState>) -> ApiRouter {
        ApiRouter::new().with_state(state.clone())
    }
}
