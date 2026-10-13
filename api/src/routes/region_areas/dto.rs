use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::region_area::RegionArea;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegionAreaDto {
    pub id: Uuid,
    pub zip_prefix: Option<String>,
    pub name: Option<String>,
    pub tags: Vec<String>,
    pub area_geometry: Option<serde_json::Value>,
    /// Geodesic area of the boundary in square metres.
    pub area_sqm: Option<f64>,
    pub bbox_min_lng: Option<f64>,
    pub bbox_min_lat: Option<f64>,
    pub bbox_max_lng: Option<f64>,
    pub bbox_max_lat: Option<f64>,
    pub created_at: DateTime<Utc>,
}

impl From<RegionArea> for RegionAreaDto {
    fn from(area: RegionArea) -> Self {
        Self {
            id: area.id,
            zip_prefix: area.zip_prefix,
            name: area.name,
            tags: area.tags,
            area_geometry: area.area_geometry,
            area_sqm: area.area_sqm,
            bbox_min_lng: area.bbox_min_lng,
            bbox_min_lat: area.bbox_min_lat,
            bbox_max_lng: area.bbox_max_lng,
            bbox_max_lat: area.bbox_max_lat,
            created_at: area.created_at,
        }
    }
}
