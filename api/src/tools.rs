use std::sync::Arc;

use aide::axum::ApiRouter;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{encode::IsNull, prelude::Type, Decode, Encode, Postgres};
use sqlx_postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef};

use crate::{error::ComhairleError, ComhairleState};

pub mod heyform;
pub mod id;
pub mod learn;
pub mod polis;
pub mod stories;

use heyform::{HeyFormToolConfig, HeyFormToolSetup};
use learn::{LearnToolConfig, LearnToolSetup};
use polis::{PolisToolConfig, PolisToolSetup};
use stories::{StoriesToolConfig, StoriesToolSetup};

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
    Stoies(StoriesToolSetup),
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum ToolConfig {
    Polis(PolisToolConfig),
    Learn(LearnToolConfig),
    HeyForm(HeyFormToolConfig),
    Stories(StoriesToolConfig),
}
impl Type<Postgres> for ToolConfig {
    fn type_info() -> PgTypeInfo {
        <serde_json::Value as Type<Postgres>>::type_info()
    }
}

impl PgHasArrayType for ToolConfig {
    fn array_type_info() -> PgTypeInfo {
        <serde_json::Value as PgHasArrayType>::array_type_info()
    }
}

impl<'q> Encode<'q, Postgres> for ToolConfig {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<(dyn std::error::Error + Send + Sync + 'static)>> {
        let json = serde_json::to_value(self).unwrap();
        <serde_json::Value as Encode<Postgres>>::encode(json, buf)
    }

    fn size_hint(&self) -> usize {
        let json = serde_json::to_value(self).unwrap();
        <serde_json::Value as Encode<Postgres>>::size_hint(&json)
    }
}

impl<'r> Decode<'r, Postgres> for ToolConfig {
    fn decode(
        value: PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let json: serde_json::Value = Decode::<Postgres>::decode(value)?;
        Ok(serde_json::from_value(json)?)
    }
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .nest_api_service("/polis", polis::router(state.clone()))
        .with_state(state)
}
