use std::sync::Arc;

use aide::axum::ApiRouter;
use comhairle_macros::DbJsonBEnum;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{encode::IsNull, prelude::Type, Decode, Encode, Postgres};
use sqlx_postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef};

use crate::{error::ComhairleError, ComhairleState};

pub mod elicitation_bot;
pub mod heyform;
pub mod id;
pub mod learn;
pub mod polis;
pub mod stories;

use elicitation_bot::{ElicitationBotReport, ElicitationBotToolConfig, ElicitationBotToolSetup};
use heyform::{HeyFormReport, HeyFormToolConfig, HeyFormToolSetup};
use learn::{LearnReport, LearnToolConfig, LearnToolSetup};
use polis::{PolisReport, PolisToolConfig, PolisToolSetup};
use stories::{StoriesReport, StoriesToolConfig, StoriesToolSetup};

pub trait Tool {
    fn setup(&self) -> Result<(), ComhairleError>;
    fn drop(&self) -> Result<(), ComhairleError>;
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum ToolSetup {
    Polis(PolisToolSetup),
    Learn(LearnToolSetup),
    HeyForm(HeyFormToolSetup),
    Stories(StoriesToolSetup),
    ElicitationBot(ElicitationBotToolSetup),
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, DbJsonBEnum)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum ToolConfig {
    Polis(PolisToolConfig),
    Learn(LearnToolConfig),
    HeyForm(HeyFormToolConfig),
    Stories(StoriesToolConfig),
    ElicitationBot(ElicitationBotToolConfig),
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .nest_api_service("/polis", polis::router(state.clone()))
        .with_state(state)
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub enum ReportConfig {
    Polis(PolisReport),
    HeyForm(HeyFormReport),
    Learn(LearnReport),
    Stories(StoriesReport),
    ElicitationBot(ElicitationBotReport),
}
