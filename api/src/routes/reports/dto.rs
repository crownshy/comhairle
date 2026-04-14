use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{
    report::{LocalizedReport, Report, ReportSectionConfigs},
    translations::TextContentId,
};

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
    id: Uuid,
    is_public: bool,
    conversation_id: Uuid,
    summary: TextContentId,
    section_configs: ReportSectionConfigs,
    created_at: DateTime<Utc>,
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

/// Data transfer object (public API representation) for a LocalizedReport.
///
/// This DTO is returned by report related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `updated_at`
///
/// It includes localized `String` values for translatable fields:
///
/// * `summary`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalizedReportDto {
    id: Uuid,
    is_public: bool,
    conversation_id: Uuid,
    summary: String,
    section_configs: ReportSectionConfigs,
    created_at: DateTime<Utc>,
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
