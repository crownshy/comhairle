use serde::{Deserialize, Serialize};

use crate::error::ComhairleError;

pub mod heyform;
pub mod learn;
pub mod polis;
pub mod id;

use heyform::HeyFormToolConfig;
use learn::LearnToolConfig;
use polis::PolisToolConfig;

pub trait Tool {
    fn setup(&self) -> Result<(), ComhairleError>;
    fn drop(&self) -> Result<(), ComhairleError>;
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ToolConfig {
    Polis(PolisToolConfig),
    Learn(LearnToolConfig),
    HeyForm(HeyFormToolConfig),
}
