use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::report_impact::ReportImpact;

/// Data transfer object (public API representation) for a Report Impact.
///
/// This DTO is returned by report related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `updated_at`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReportImpactDto {
    id: Uuid,
    created_by: Uuid,
    report_id: Uuid,
    details: String,
    kind: String,
    title: String,
    created_at: DateTime<Utc>,
}

impl From<ReportImpact> for ReportImpactDto {
    fn from(r: ReportImpact) -> Self {
        Self {
            id: r.id,
            created_by: r.created_by,
            report_id: r.report_id,
            details: r.details,
            kind: r.kind,
            title: r.title,
            created_at: r.created_at,
        }
    }
}
