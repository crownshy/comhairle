use crate::models::error::ModelError;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{
    CaseStatement, Expr, PostgresQueryBuilder, Query, SelectStatement, SimpleExpr, enum_def,
};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query_as_with};
use tracing::instrument;
use uuid::Uuid;

use crate::models::{SqlxResultExt, user_progress::UserProgressIden};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "thinking_space_summary")]
pub struct ThinkingSpaceSummary {
    pub id: Uuid,
    pub workflow_step_id: Uuid,
    pub user_id: Uuid,
    pub summary: String,
    pub ai_generated_summary: Option<String>,
    pub is_ai_generated: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [ThinkingSpaceSummaryIden; 8] = [
    ThinkingSpaceSummaryIden::Id,
    ThinkingSpaceSummaryIden::WorkflowStepId,
    ThinkingSpaceSummaryIden::UserId,
    ThinkingSpaceSummaryIden::Summary,
    ThinkingSpaceSummaryIden::IsAiGenerated,
    ThinkingSpaceSummaryIden::AiGeneratedSummary,
    ThinkingSpaceSummaryIden::CreatedAt,
    ThinkingSpaceSummaryIden::UpdatedAt,
];

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct CreateSummary {
    pub summary: String,
    pub is_ai_generated: Option<bool>,
}

impl CreateSummary {
    fn columns(&self) -> Vec<ThinkingSpaceSummaryIden> {
        let mut columns = vec![ThinkingSpaceSummaryIden::Summary];
        if self.is_ai_generated.is_some() {
            columns.push(ThinkingSpaceSummaryIden::IsAiGenerated);
        }

        columns
    }

    fn values(&self) -> Vec<SimpleExpr> {
        let mut values = vec![self.summary.clone().into()];
        if let Some(value) = &self.is_ai_generated {
            values.push((*value).into());
        }

        values
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn create(
    db: &PgPool,
    user_id: Uuid,
    workflow_step_id: Uuid,
    create_summary: &CreateSummary,
) -> Result<ThinkingSpaceSummary, ModelError> {
    let mut columns = create_summary.columns();
    let mut values = create_summary.values();

    columns.push(ThinkingSpaceSummaryIden::UserId);
    values.push(user_id.into());
    columns.push(ThinkingSpaceSummaryIden::WorkflowStepId);
    values.push(workflow_step_id.into());

    let (sql, values) = Query::insert()
        .into_table(ThinkingSpaceSummaryIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let summary = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(summary)
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct UpdateSummary {
    pub summary: String,
}

#[instrument(err(Debug), skip(db))]
pub async fn update(
    db: &PgPool,
    id: Uuid,
    update_summary: &UpdateSummary,
) -> Result<ThinkingSpaceSummary, ModelError> {
    // If is_ai_generated is currently true, snapshot the old `summary` into
    // ai_generated_summary; otherwise leave ai_generated_summary as-is.
    let ai_generated_summary_exp = CaseStatement::new()
        .case(
            Expr::col(ThinkingSpaceSummaryIden::IsAiGenerated).eq(true),
            Expr::col(ThinkingSpaceSummaryIden::Summary),
        )
        .finally(Expr::col(ThinkingSpaceSummaryIden::AiGeneratedSummary));

    let values = vec![
        (
            ThinkingSpaceSummaryIden::Summary,
            update_summary.summary.clone().into(),
        ),
        (
            ThinkingSpaceSummaryIden::AiGeneratedSummary,
            ai_generated_summary_exp.into(),
        ),
        (ThinkingSpaceSummaryIden::IsAiGenerated, false.into()),
    ];

    let (sql, values) = Query::update()
        .table(ThinkingSpaceSummaryIden::Table)
        .values(values)
        .and_where(Expr::col(ThinkingSpaceSummaryIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let summary = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(summary)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, id: Uuid) -> Result<ThinkingSpaceSummary, ModelError> {
    let (sql, values) = Query::select()
        .from(ThinkingSpaceSummaryIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(ThinkingSpaceSummaryIden::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    let summary = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Thinking Space Summary")?;

    Ok(summary)
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct ThinkingSpaceSummaryFilterOptions {
    // Server-set only — handlers must scope this to the authenticated user.
    // Hidden from the public schema/query string so callers can't spoof it.
    #[serde(skip_deserializing)]
    #[schemars(skip)]
    pub user_id: Option<Uuid>,
    pub is_ai_generated: Option<bool>,
    pub is_shared_with_organizer: Option<bool>,
}

impl ThinkingSpaceSummaryFilterOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(value) = self.user_id {
            query = query
                .and_where(
                    Expr::col((
                        ThinkingSpaceSummaryIden::Table,
                        ThinkingSpaceSummaryIden::UserId,
                    ))
                    .eq(value),
                )
                .to_owned();
        }
        if let Some(value) = &self.is_ai_generated {
            query = query
                .and_where(
                    Expr::col((
                        ThinkingSpaceSummaryIden::Table,
                        ThinkingSpaceSummaryIden::IsAiGenerated,
                    ))
                    .eq(*value),
                )
                .to_owned();
        }
        if let Some(value) = self.is_shared_with_organizer {
            let permission_check = Expr::exists(
                Query::select()
                    .expr(Expr::val(1))
                    .from(UserProgressIden::Table)
                    .and_where(
                        Expr::col((UserProgressIden::Table, UserProgressIden::WorkflowStepId))
                            .equals((
                                ThinkingSpaceSummaryIden::Table,
                                ThinkingSpaceSummaryIden::WorkflowStepId,
                            )),
                    )
                    .and_where(
                        Expr::col((UserProgressIden::Table, UserProgressIden::UserId)).equals((
                            ThinkingSpaceSummaryIden::Table,
                            ThinkingSpaceSummaryIden::UserId,
                        )),
                    )
                    .and_where(
                        Expr::col((
                            UserProgressIden::Table,
                            UserProgressIden::PermissionToShareWithOrganizers,
                        ))
                        .eq(true),
                    )
                    .to_owned(),
            );

            query = if value {
                query.and_where(permission_check).to_owned()
            } else {
                query.and_where(permission_check.not()).to_owned()
            };
        }

        query
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn list(
    db: &PgPool,
    workflow_step_id: &Uuid,
    filter_options: ThinkingSpaceSummaryFilterOptions,
) -> Result<Vec<ThinkingSpaceSummary>, ModelError> {
    let query = Query::select()
        .from(ThinkingSpaceSummaryIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (ThinkingSpaceSummaryIden::Table, col)))
        .and_where(
            Expr::col((
                ThinkingSpaceSummaryIden::Table,
                ThinkingSpaceSummaryIden::WorkflowStepId,
            ))
            .eq(workflow_step_id.to_owned()),
        )
        .to_owned();

    let query = filter_options.apply(query);

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let summaries = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(summaries)
}

#[instrument(err(Debug), skip(db))]
pub async fn delete(db: &PgPool, id: Uuid) -> Result<ThinkingSpaceSummary, ModelError> {
    let (sql, values) = Query::delete()
        .from_table(ThinkingSpaceSummaryIden::Table)
        .and_where(Expr::col(ThinkingSpaceSummaryIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let summary = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(summary)
}
