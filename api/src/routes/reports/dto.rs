use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::report::{LocalizedReport, Report, ReportSectionConfigs};
use crate::models::translations::TextContentId;

/// Data transfer object (public API representation) for a Report.
///
/// This DTO is returned by report related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `updated_at`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReportDto {
    pub id: Uuid,
    pub is_public: bool,
    pub conversation_id: Uuid,
    pub summary: TextContentId,
    pub section_configs: ReportSectionConfigs,
    pub created_at: DateTime<Utc>,
}

impl From<Report> for ReportDto {
    fn from(r: Report) -> Self {
        Self {
            id: r.id,
            is_public: r.is_public,
            conversation_id: r.conversation_id,
            summary: r.summary,
            section_configs: r.section_configs,
            created_at: r.created_at,
        }
    }
}

/// Data transfer object (public API representation) for a Report.
///
/// This DTO is returned by report related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `updated_at`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalizedReportDto {
    pub id: Uuid,
    pub is_public: bool,
    pub conversation_id: Uuid,
    pub summary: String,
    pub section_configs: ReportSectionConfigs,
    pub created_at: DateTime<Utc>,
}

impl From<LocalizedReport> for LocalizedReportDto {
    fn from(r: LocalizedReport) -> Self {
        Self {
            id: r.id,
            is_public: r.is_public,
            conversation_id: r.conversation_id,
            summary: r.summary,
            section_configs: r.section_configs,
            created_at: r.created_at,
        }
    }
}
