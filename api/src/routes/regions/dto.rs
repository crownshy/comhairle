use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{
        pagination::PaginatedResults,
        region::{LocalizedRegion, Region, RegionType},
        translations::TextContentId,
    },
    schema_helpers::{example_localized_text, example_uuid},
};

/// Data transfer object (public API representation) for a Region.
///
/// This DTO is returned by region related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `updated_at`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct RegionDto {
    #[schemars(example = "example_uuid")]
    pub id: Uuid,
    #[schemars(example = "example_uuid")]
    pub name: TextContentId,
    #[schemars(example = "example_uuid")]
    pub description: TextContentId,
    pub region_type: RegionType,
    pub official_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Data transfer object (public API representation) for a LocalizedRegion.
///
/// This DTO is returned by region related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `updated_at`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LocalizedRegionDto {
    #[schemars(example = "example_uuid")]
    pub id: Uuid,
    #[schemars(example = "example_localized_text")]
    pub name: String,
    #[schemars(example = "example_localized_text")]
    pub description: String,
    pub region_type: RegionType,
    pub official_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Region> for RegionDto {
    fn from(o: Region) -> Self {
        Self {
            id: o.id,
            name: o.name,
            description: o.description,
            region_type: o.region_type,
            official_id: o.official_id,
            created_at: o.created_at,
        }
    }
}

impl From<LocalizedRegion> for LocalizedRegionDto {
    fn from(o: LocalizedRegion) -> Self {
        Self {
            id: o.id,
            name: o.name,
            description: o.description,
            region_type: o.region_type,
            official_id: o.official_id,
            created_at: o.created_at,
        }
    }
}

impl From<PaginatedResults<LocalizedRegion>> for PaginatedResults<LocalizedRegionDto> {
    fn from(r: PaginatedResults<LocalizedRegion>) -> Self {
        Self {
            total: r.total,
            records: r.records.into_iter().map(Into::into).collect(),
        }
    }
}
