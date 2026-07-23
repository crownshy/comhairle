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

use crate::models::translations::{TextContentId, TextFormat, new_translation};
use crate::{
    error::ComhairleError,
    models::SqlxResultExt,
    routes::{
        feedback::dto::FeedbackDto, report_impacts::dto::ReportImpactDto, reports::dto::ReportDto,
    },
    tools::{
        ReportConfig, ToolConfig, elicitation_bot::ElicitationBotReport, heyform::HeyFormReport,
        learn::LearnReport, polis::PolisReport, prioritization::PrioritizationReport,
        stories::StoriesReport, thinking_space::ThinkingSpaceReport,
    },
};

use super::{
    feedback::{self},
    report_impact::{self},
    workflow, workflow_step,
};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FullReportDto {
    #[serde(flatten)]
    pub report: ReportDto,
    pub facilitator_feedback: Vec<FeedbackDto>,
    pub participant_feedback: Vec<FeedbackDto>,
    pub impacts: Vec<ReportImpactDto>,
}

impl FullReportDto {
    pub async fn from_report(db: &PgPool, report: Report) -> Result<FullReportDto, ComhairleError> {
        let feedback = feedback::list_for_conversation(db, &report.conversation_id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();
        let impacts = report_impact::get_for_report(db, &report.id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();
        Ok(FullReportDto {
            report: report.into(),
            impacts,
            facilitator_feedback: feedback,
            participant_feedback: vec![],
        })
    }
}

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

#[instrument(err(Debug))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<Report, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(ReportIden::Table)
        .and_where(Expr::col(ReportIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let conversation = sqlx::query_as_with::<_, Report, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Report")?;

    Ok(conversation)
}

#[instrument(err(Debug))]
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
    locale: &str,
) -> Result<Report, ComhairleError> {
    let workflows = workflow::list(db, conversation_id, None).await?;
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
                Err(ComhairleError::ToolConfigMismatch)
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

    columns.push(ReportIden::Summary);
    values.push(summary.id.into());

    let (sql, values) = Query::insert()
        .into_table(ReportIden::Table)
        .columns(columns)
        .values(values)?
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

#[instrument(err(Debug))]
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
        .resolve_db_err("Report")?;

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;

    use sqlx::PgPool;

    use crate::models::model_test_helpers::{
        get_random_conversation_id, setup_default_app_and_session,
    };
    use crate::models::translations::get_text_translation_by_content_and_locale;
    use crate::routes::workflows::dto::WorkflowDto;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_report_for_conversation_with_translation(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, value, _) = session
            .create_random_workflow(&app, &conversation_id.to_string())
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(value)?;
        session
            .create_random_workflow_steps(
                &app,
                &conversation_id.to_string(),
                &workflow.id.to_string(),
                2,
            )
            .await?;

        let report = create_for_conversation(&pool, conversation_id, "en").await?;

        let summary_translation =
            get_text_translation_by_content_and_locale(&pool, &report.summary, "en").await?;

        assert_eq!(
            summary_translation.content, "Summary to be filled out by facilitator",
            "incorrect summary translation text"
        );

        Ok(())
    }
}
