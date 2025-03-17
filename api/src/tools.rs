use polis::PolisToolConfig;
use serde::{Deserialize, Serialize};

use crate::error::ComhairleError;

pub mod polis;

pub trait Tool {
    fn setup(&self) -> Result<(), ComhairleError>;
    fn drop(&self) -> Result<(), ComhairleError>;
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct LearnToolConfig {
    pub markdown: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ToolConfig {
    Polis(PolisToolConfig),
    Learn(LearnToolConfig),
}
