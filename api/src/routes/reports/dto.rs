use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::Serialize;
use uuid::Uuid;

use crate::models::report::{Report, ReportSectionConfigs};

#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReportDto {
    id: Uuid,
    is_public: bool,
    conversation_id: Uuid,
    summary: String,
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
