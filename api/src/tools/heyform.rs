use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct HeyFormToolConfig {
    pub survey_id: String,
    pub survey_url: String,
}
