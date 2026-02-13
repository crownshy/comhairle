use chrono::{DateTime, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{
    encode::IsNull,
    prelude::{FromRow, Type},
    Decode, Encode, PgPool, Postgres,
};
use sqlx_postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    tools::{
        elicitation_bot::ElicitationBotReport, heyform::HeyFormReport, learn::LearnReport,
        polis::PolisReport, stories::StoriesReport, ReportConfig, ToolConfig,
    },
};

use super::{
    feedback::{self, Feedback},
    report_impact::{self, ReportImpact},
    workflow, workflow_step,
};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct FullReportDTO {
    #[serde(flatten)]
    pub report: Report,
    pub facilitator_feedback: Vec<Feedback>,
    pub participant_feedback: Vec<Feedback>,
    pub impacts: Vec<ReportImpact>,
}

impl FullReportDTO {
    pub async fn from_report(db: &PgPool, report: Report) -> Result<FullReportDTO, ComhairleError> {
        let feedback = feedback::list_for_conversation(db, &report.conversation_id).await?;
        let impacts = report_impact::get_for_report(db, &report.id).await?;
        Ok(FullReportDTO {
            report,
            impacts,
            facilitator_feedback: feedback,
            participant_feedback: vec![],
        })
    }
}

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "report")]
#[partially(derive(Deserialize, Debug, JsonSchema))]
pub struct Report {
    #[partially(omit)]
    pub id: Uuid,
    pub is_public: bool,
    pub conversation_id: Uuid,
    pub summary: String,
    pub section_configs: ReportSectionConfigs,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [ReportIden; 7] = [
    ReportIden::Id,
    ReportIden::IsPublic,
    ReportIden::ConversationId,
    ReportIden::Summary,
    ReportIden::SectionConfigs,
    ReportIden::CreatedAt,
    ReportIden::UpdatedAt,
];

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
pub struct ReportSectionConfigs(pub Vec<ReportSectionConfig>);

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "lowercase", tag = "type")]
pub struct ReportSectionConfig {
    workflow_step_id: Uuid,
    config: ReportConfig,
    ai_generated: bool,
    verified: bool,
}

impl Type<Postgres> for ReportSectionConfigs {
    fn type_info() -> PgTypeInfo {
        <serde_json::Value as Type<Postgres>>::type_info()
    }
}

impl PgHasArrayType for ReportSectionConfigs {
    fn array_type_info() -> PgTypeInfo {
        <serde_json::Value as PgHasArrayType>::array_type_info()
    }
}
impl<'q> Encode<'q, Postgres> for ReportSectionConfigs {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let json = serde_json::to_value(self).unwrap();
        <serde_json::Value as Encode<Postgres>>::encode(json, buf)
    }

    fn size_hint(&self) -> usize {
        let json = serde_json::to_value(self).unwrap();
        <serde_json::Value as Encode<Postgres>>::size_hint(&json)
    }
}

impl<'r> Decode<'r, Postgres> for ReportSectionConfigs {
    fn decode(
        value: PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let json: serde_json::Value = Decode::<Postgres>::decode(value)?;
        Ok(serde_json::from_value(json)?)
    }
}

impl PartialReport {
    pub fn to_values(&self) -> Vec<(ReportIden, sea_query::SimpleExpr)> {
        let mut values = vec![];
        if let Some(value) = self.is_public {
            values.push((ReportIden::IsPublic, value.into()));
        }
        if let Some(value) = &self.summary {
            values.push((ReportIden::Summary, value.into()));
        }
        if let Some(value) = &self.section_configs {
            values.push((
                ReportIden::SectionConfigs,
                serde_json::to_string_pretty(value).unwrap().into(),
            ));
        }
        values
    }
}

pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<Report, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(ReportIden::Table)
        .and_where(Expr::col(ReportIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let conversation = sqlx::query_as_with::<_, Report, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("Conversation".into()))?;

    Ok(conversation)
}

pub async fn update(
    db: &PgPool,
    conversation_id: Uuid,
    update: PartialReport,
) -> Result<Report, ComhairleError> {
    let values = update.to_values();
    let (sql, values) = Query::update()
        .table(ReportIden::Table)
        .values(values)
        .and_where(Expr::col(ReportIden::ConversationId).eq(conversation_id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with::<_, Report, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::FailedToUpdateReport)
}

#[instrument(err(Debug))]
pub async fn create_for_conversation(
    db: &PgPool,
    conversation_id: Uuid,
) -> Result<Report, ComhairleError> {
    let workflows = workflow::list(db, conversation_id).await?;
    let workflow_steps = workflow_step::list(db, &workflows[0].id).await?;

    let section_configs: Result<Vec<ReportSectionConfig>, ComhairleError> = workflow_steps
        .iter()
        .map(|step| {
            if let Some(tool_config) = &step.tool_config {
                let config = match tool_config {
                    ToolConfig::Polis(_) => ReportConfig::Polis(PolisReport),
                    ToolConfig::Learn(_) => ReportConfig::Learn(LearnReport),
                    ToolConfig::HeyForm(_) => ReportConfig::HeyForm(HeyFormReport),
                    ToolConfig::Stories(_) => ReportConfig::Stories(StoriesReport),
                    ToolConfig::ElicitationBot(_) => {
                        ReportConfig::ElicitationBot(ElicitationBotReport)
                    }
                };

                Ok(ReportSectionConfig {
                    workflow_step_id: step.id,
                    config,
                    ai_generated: false,
                    verified: false,
                })
            } else {
                Err(ComhairleError::ToolConfigMismatch)
            }
        })
        .collect();

    let section_configs = ReportSectionConfigs(section_configs?);

    let values: Vec<sea_query::SimpleExpr> = vec![
        false.into(),
        conversation_id.into(),
        "Summary to be filled out by facilitator".into(),
        serde_json::to_value(&section_configs).unwrap().into(),
    ];

    let (sql, values) = Query::insert()
        .into_table(ReportIden::Table)
        .columns(vec![
            ReportIden::IsPublic,
            ReportIden::ConversationId,
            ReportIden::Summary,
            ReportIden::SectionConfigs,
        ])
        .values(values)
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let report = sqlx::query_as_with::<_, Report, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| ComhairleError::FailedToCreateResource {
            resource_type: "Report".into(),
            error: e,
        })?;

    Ok(report)
}

pub async fn get_for_conversation(
    db: &PgPool,
    conversation_id: &Uuid,
) -> Result<Report, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(ReportIden::Table)
        .and_where(Expr::col(ReportIden::ConversationId).eq(conversation_id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let report = sqlx::query_as_with::<_, Report, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("Report".into()))?;

    Ok(report)
}
