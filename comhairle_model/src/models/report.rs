use crate::models::error::ModelError;
use chrono::{DateTime, Utc};
use comhairle_macros::Translatable;
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{
    Decode, Encode, PgPool, Postgres,
    encode::IsNull,
    prelude::{FromRow, Type},
};
use sqlx_postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef};
use tracing::instrument;
use uuid::Uuid;

use crate::models::SqlxResultExt;
use crate::models::error::DataError;
use crate::models::error::ReportError;
use crate::models::error::WorkflowError;
use crate::models::tools::elicitation_bot::ElicitationBotReport;
use crate::models::tools::heyform::HeyFormReport;
use crate::models::tools::learn::LearnReport;
use crate::models::tools::polis::PolisReport;
use crate::models::tools::prioritization::PrioritizationReport;
use crate::models::tools::stories::StoriesReport;
use crate::models::tools::thinking_space::ThinkingSpaceReport;
use crate::models::tools::{ReportConfig, ToolConfig};
use crate::models::translations::{TextContentId, TextFormat, new_translation};

use super::{workflow, workflow_step};

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema, Translatable)]
#[enum_def(table_name = "report")]
#[partially(derive(Deserialize, Debug, JsonSchema))]
pub struct Report {
    #[partially(omit)]
    pub id: Uuid,
    pub is_public: bool,
    pub conversation_id: Uuid,
    #[partially(omit)]
    pub summary: TextContentId,
    #[partially(transparent)]
    pub body: Option<TextContentId>,
    pub section_configs: ReportSectionConfigs,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [ReportIden; 8] = [
    ReportIden::Id,
    ReportIden::IsPublic,
    ReportIden::ConversationId,
    ReportIden::Summary,
    ReportIden::Body,
    ReportIden::SectionConfigs,
    ReportIden::CreatedAt,
    ReportIden::UpdatedAt,
];

#[derive(PartialEq, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
pub struct ReportSectionConfigs(pub Vec<ReportSectionConfig>);

#[derive(PartialEq, Debug, Deserialize, Serialize, Clone, JsonSchema)]
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
        if let Some(value) = &self.section_configs {
            values.push((
                ReportIden::SectionConfigs,
                serde_json::to_string_pretty(value).unwrap().into(),
            ));
        }
        values
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, id: Uuid) -> Result<Report, ModelError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(ReportIden::Table)
        .and_where(Expr::col(ReportIden::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    let conversation = sqlx::query_as_with::<_, Report, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Report")?;

    Ok(conversation)
}

#[instrument(err(Debug), skip(db))]
pub async fn update(
    db: &PgPool,
    conversation_id: Uuid,
    update: PartialReport,
) -> Result<Report, ModelError> {
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
        .map_err(|_| ReportError::FailedToUpdateReport.into())
}

#[instrument(err(Debug), skip(db))]
pub async fn create_for_conversation(
    db: &PgPool,
    conversation_id: Uuid,
    locale: &str,
) -> Result<Report, ModelError> {
    let workflows = workflow::list(db, conversation_id, None).await?;
    let workflow_steps = workflow_step::list(db, &workflows[0].id).await?;

    let section_configs: Result<Vec<ReportSectionConfig>, ModelError> = workflow_steps
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
                    ToolConfig::Prioritization(_) => {
                        ReportConfig::Prioritization(PrioritizationReport)
                    }
                    ToolConfig::ThinkingSpace(_) => {
                        ReportConfig::ThinkingSpace(ThinkingSpaceReport)
                    }
                };

                Ok(ReportSectionConfig {
                    workflow_step_id: step.id,
                    config,
                    ai_generated: false,
                    verified: false,
                })
            } else {
                Err(ModelError::Workflow(WorkflowError::ToolConfigMismatch))
            }
        })
        .collect();

    let section_configs = ReportSectionConfigs(section_configs?);

    let mut values: Vec<sea_query::SimpleExpr> = vec![
        false.into(),
        conversation_id.into(),
        serde_json::to_value(&section_configs).unwrap().into(),
    ];

    let mut columns = vec![
        ReportIden::IsPublic,
        ReportIden::ConversationId,
        ReportIden::SectionConfigs,
    ];

    let summary = new_translation(
        db,
        locale,
        "Summary to be filled out by facilitator",
        TextFormat::Rich,
    )
    .await?;
    let body = new_translation(
        db,
        locale,
        "Body to be filled out by facilitator",
        TextFormat::Rich,
    )
    .await?;

    columns.push(ReportIden::Summary);
    values.push(summary.id.into());

    columns.push(ReportIden::Body);
    values.push(body.id.into());

    let (sql, values) = Query::insert()
        .into_table(ReportIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let report = sqlx::query_as_with::<_, Report, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| DataError::FailedToCreateResource {
            resource_type: "Report".into(),
            error: e,
        })?;

    Ok(report)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_for_conversation(
    db: &PgPool,
    conversation_id: Uuid,
) -> Result<Report, ModelError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(ReportIden::Table)
        .and_where(Expr::col(ReportIden::ConversationId).eq(conversation_id))
        .build_sqlx(PostgresQueryBuilder);

    let report = sqlx::query_as_with::<_, Report, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Report")?;

    Ok(report)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_localized_for_conversation(
    db: &PgPool,
    conversation_id: Uuid,
    locale: &str,
) -> Result<LocalizedReport, ModelError> {
    let query = Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (ReportIden::Table, col)))
        .from(ReportIden::Table)
        .and_where(Expr::col((ReportIden::Table, ReportIden::ConversationId)).eq(conversation_id))
        .to_owned();

    let (sql, values) =
        LocalizedReport::query_to_localisation(query, locale).build_sqlx(PostgresQueryBuilder);

    let report = sqlx::query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Report")?;

    Ok(report)
}
