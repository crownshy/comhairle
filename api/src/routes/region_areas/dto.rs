use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::region_area::RegionArea;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegionAreaDto {
    pub id: Uuid,
    pub zip_prefix: String,
    pub created_at: DateTime<Utc>,
}

impl From<RegionArea> for RegionAreaDto {
    fn from(area: RegionArea) -> Self {
        Self {
            id: area.id,
            zip_prefix: area.zip_prefix,
            created_at: area.created_at,
        }
    }
}
