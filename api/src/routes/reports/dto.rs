use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::report::{
    LocalizedReport, Report, ReportSectionConfigs, ReportWithTranslations,
};
use crate::models::translations::TextContentId;
use crate::models::{feedback, report_impact};
use crate::routes::feedback::dto::FeedbackDto;
use crate::routes::report_impacts::dto::ReportImpactDto;

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
    pub body: Option<TextContentId>,
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
            body: r.body,
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
    pub body: Option<String>,
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
            body: r.body,
            section_configs: r.section_configs,
            created_at: r.created_at,
        }
    }
}

/// A report in one of its two API shapes: the admin view with raw translation
/// ids, or the participant view localized to one locale.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(untagged)]
pub enum ReportView {
    WithTranslations(ReportWithTranslations),
    Localized(LocalizedReportDto),
}

/// The full report response: the report in either view, plus the feedback and
/// impacts attached to it.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FullReportDto {
    #[serde(flatten)]
    pub report: ReportView,
    pub facilitator_feedback: Vec<FeedbackDto>,
    pub participant_feedback: Vec<FeedbackDto>,
    pub impacts: Vec<ReportImpactDto>,
}

impl FullReportDto {
    pub async fn from_report(
        db: &PgPool,
        report: ReportView,
    ) -> Result<FullReportDto, ComhairleError> {
        let (report_id, conversation_id) = match &report {
            ReportView::WithTranslations(report) => (report.id, report.conversation_id),
            ReportView::Localized(report) => (report.id, report.conversation_id),
        };
        let feedback = feedback::list_for_conversation(db, &conversation_id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();
        let impacts = report_impact::get_for_report(db, &report_id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();
        Ok(FullReportDto {
            report,
            impacts,
            facilitator_feedback: feedback,
            participant_feedback: vec![],
        })
    }
}
