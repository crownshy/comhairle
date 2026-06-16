use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{
    enum_def, Expr, OnConflict, PostgresQueryBuilder, Query, SelectStatement, SimpleExpr,
};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, query_as_with, PgPool};
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;

#[derive(Debug, Deserialize, Serialize, PartialEq, sqlx::Type, Clone, JsonSchema, Default)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
pub enum ModerationStatus {
    #[sqlx(rename = "accepted")]
    Accepted,
    #[sqlx(rename = "rejected")]
    Rejected,
    #[sqlx(rename = "pending")]
    #[default]
    Pending,
}

impl std::fmt::Display for ModerationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ModerationStatus::Accepted => "accepted",
            ModerationStatus::Rejected => "rejected",
            ModerationStatus::Pending => "pending",
        };
        write!(f, "{}", value)
    }
}

impl From<ModerationStatus> for sea_query::Value {
    fn from(val: ModerationStatus) -> Self {
        sea_query::Value::String(Some(Box::new(val.to_string())))
    }
}

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "polis_statement_aux")]
pub struct PolisStatementAux {
    pub id: Uuid,
    pub workflow_step_id: Uuid,
    pub user_id: Option<Uuid>,
    pub zid: i32,
    pub polis_conversation_id: String,
    pub polis_statement_id: i32,
    pub statement_text: String,
    pub moderation_status: ModerationStatus,
    pub is_seed: bool,
    pub themes: Vec<String>,
    pub visible_statement_when_submitted: Option<String>,
    pub moderation_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [PolisStatementAuxIden; 14] = [
    PolisStatementAuxIden::Id,
    PolisStatementAuxIden::WorkflowStepId,
    PolisStatementAuxIden::UserId,
    PolisStatementAuxIden::Zid,
    PolisStatementAuxIden::PolisConversationId,
    PolisStatementAuxIden::PolisStatementId,
    PolisStatementAuxIden::StatementText,
    PolisStatementAuxIden::ModerationStatus,
    PolisStatementAuxIden::IsSeed,
    PolisStatementAuxIden::Themes,
    PolisStatementAuxIden::VisibleStatementWhenSubmitted,
    PolisStatementAuxIden::ModerationReason,
    PolisStatementAuxIden::CreatedAt,
    PolisStatementAuxIden::UpdatedAt,
];

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct CreatePolisStatementAux {
    pub workflow_step_id: Uuid,
    pub zid: i32,
    pub polis_conversation_id: String,
    pub polis_statement_id: i32,
    pub statement_text: String,
    #[serde(default)]
    pub moderation_status: ModerationStatus,
    pub is_seed: bool,
    pub themes: Vec<String>,
    pub visible_statement_when_submitted: Option<String>,
    pub moderation_reason: Option<String>,
}

impl CreatePolisStatementAux {
    fn columns(&self) -> Vec<PolisStatementAuxIden> {
        vec![
            PolisStatementAuxIden::WorkflowStepId,
            PolisStatementAuxIden::Zid,
            PolisStatementAuxIden::PolisConversationId,
            PolisStatementAuxIden::PolisStatementId,
            PolisStatementAuxIden::StatementText,
            PolisStatementAuxIden::ModerationStatus,
            PolisStatementAuxIden::IsSeed,
            PolisStatementAuxIden::Themes,
            PolisStatementAuxIden::VisibleStatementWhenSubmitted,
            PolisStatementAuxIden::ModerationReason,
        ]
    }

    fn values(&self) -> Vec<SimpleExpr> {
        vec![
            self.workflow_step_id.into(),
            self.zid.into(),
            self.polis_conversation_id.clone().into(),
            self.polis_statement_id.into(),
            self.statement_text.clone().into(),
            self.moderation_status.clone().into(),
            self.is_seed.into(),
            self.themes.clone().into(),
            self.visible_statement_when_submitted.clone().into(),
            self.moderation_reason.clone().into(),
        ]
    }
}

#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    user_id: Uuid,
    create_aux: &CreatePolisStatementAux,
) -> Result<PolisStatementAux, ComhairleError> {
    let mut columns = create_aux.columns();
    let mut values = create_aux.values();

    columns.push(PolisStatementAuxIden::UserId);
    values.push(user_id.into());

    let (sql, values) = Query::insert()
        .into_table(PolisStatementAuxIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let aux = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(aux)
}

#[derive(Debug)]
pub struct UpsertFromPolis {
    pub workflow_step_id: Uuid,
    pub user_id: Option<Uuid>,
    pub zid: i32,
    pub polis_conversation_id: String,
    pub polis_statement_id: i32,
    pub statement_text: String,
    pub is_seed: bool,
}

/// Upsert a row from polis. On conflict (workflow_step_id, polis_statement_id),
/// only `statement_text` and `is_seed` are updated — `moderation_reason`,
/// `themes`, `visible_statement_when_submitted`, `moderation_status` and
/// `user_id` are preserved.
#[instrument(err(Debug), skip(db))]
pub async fn upsert_from_polis(
    db: &PgPool,
    record: &UpsertFromPolis,
) -> Result<PolisStatementAux, ComhairleError> {
    let columns = [
        PolisStatementAuxIden::WorkflowStepId,
        PolisStatementAuxIden::UserId,
        PolisStatementAuxIden::Zid,
        PolisStatementAuxIden::PolisConversationId,
        PolisStatementAuxIden::PolisStatementId,
        PolisStatementAuxIden::StatementText,
        PolisStatementAuxIden::IsSeed,
    ];
    let values: Vec<SimpleExpr> = vec![
        record.workflow_step_id.into(),
        record.user_id.into(),
        record.zid.into(),
        record.polis_conversation_id.clone().into(),
        record.polis_statement_id.into(),
        record.statement_text.clone().into(),
        record.is_seed.into(),
    ];

    let (sql, values) = Query::insert()
        .into_table(PolisStatementAuxIden::Table)
        .columns(columns)
        .values(values)?
        .on_conflict(
            OnConflict::columns([
                PolisStatementAuxIden::WorkflowStepId,
                PolisStatementAuxIden::PolisStatementId,
            ])
            .update_columns([
                PolisStatementAuxIden::StatementText,
                PolisStatementAuxIden::IsSeed,
            ])
            .value(PolisStatementAuxIden::UpdatedAt, Expr::current_timestamp())
            .to_owned(),
        )
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let aux = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(aux)
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct UpdatePolisStatementAux {
    pub statement_text: Option<String>,
    pub moderation_status: Option<ModerationStatus>,
    pub themes: Option<Vec<String>>,
    pub visible_statement_when_submitted: Option<String>,
    pub moderation_reason: Option<String>,
}

#[instrument(err(Debug))]
pub async fn update(
    db: &PgPool,
    id: Uuid,
    update_aux: &UpdatePolisStatementAux,
) -> Result<PolisStatementAux, ComhairleError> {
    let mut values: Vec<(PolisStatementAuxIden, SimpleExpr)> = vec![];

    if let Some(text) = &update_aux.statement_text {
        values.push((PolisStatementAuxIden::StatementText, text.clone().into()));
    }
    if let Some(status) = &update_aux.moderation_status {
        values.push((
            PolisStatementAuxIden::ModerationStatus,
            status.clone().into(),
        ));
    }
    if let Some(themes) = &update_aux.themes {
        values.push((PolisStatementAuxIden::Themes, themes.clone().into()));
    }
    if let Some(visible) = &update_aux.visible_statement_when_submitted {
        values.push((
            PolisStatementAuxIden::VisibleStatementWhenSubmitted,
            visible.clone().into(),
        ));
    }
    if let Some(reason) = &update_aux.moderation_reason {
        values.push((
            PolisStatementAuxIden::ModerationReason,
            reason.clone().into(),
        ));
    }

    let (sql, values) = Query::update()
        .table(PolisStatementAuxIden::Table)
        .values(values)
        .and_where(Expr::col(PolisStatementAuxIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let aux = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(aux)
}

#[instrument(err(Debug))]
pub async fn get_by_id(db: &PgPool, id: Uuid) -> Result<PolisStatementAux, ComhairleError> {
    let (sql, values) = Query::select()
        .from(PolisStatementAuxIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(PolisStatementAuxIden::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    let aux = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                ComhairleError::ResourceNotFound("Polis statement aux".into())
            }
            other => ComhairleError::DatabaseError(other),
        })?;

    Ok(aux)
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct PolisStatementAuxFilterOptions {
    user_id: Option<Uuid>,
    polis_conversation_id: Option<String>,
    polis_statement_id: Option<i32>,
}

impl PolisStatementAuxFilterOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(value) = self.user_id {
            query = query
                .and_where(
                    Expr::col((PolisStatementAuxIden::Table, PolisStatementAuxIden::UserId))
                        .eq(value),
                )
                .to_owned();
        }
        if let Some(value) = &self.polis_conversation_id {
            query = query
                .and_where(
                    Expr::col((
                        PolisStatementAuxIden::Table,
                        PolisStatementAuxIden::PolisConversationId,
                    ))
                    .eq(value.clone()),
                )
                .to_owned();
        }
        if let Some(value) = self.polis_statement_id {
            query = query
                .and_where(
                    Expr::col((
                        PolisStatementAuxIden::Table,
                        PolisStatementAuxIden::PolisStatementId,
                    ))
                    .eq(value),
                )
                .to_owned();
        }

        query
    }
}

#[instrument(err(Debug))]
pub async fn list(
    db: &PgPool,
    workflow_step_id: &Uuid,
    filter_options: PolisStatementAuxFilterOptions,
) -> Result<Vec<PolisStatementAux>, ComhairleError> {
    let query = Query::select()
        .from(PolisStatementAuxIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (PolisStatementAuxIden::Table, col)))
        .and_where(
            Expr::col((
                PolisStatementAuxIden::Table,
                PolisStatementAuxIden::WorkflowStepId,
            ))
            .eq(workflow_step_id.to_owned()),
        )
        .to_owned();

    let query = filter_options.apply(query);

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let aux = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(aux)
}

#[instrument(err(Debug))]
pub async fn list_filtered(
    db: &PgPool,
    workflow_step_id: Option<Uuid>,
    polis_conversation_id: Option<String>,
) -> Result<Vec<PolisStatementAux>, ComhairleError> {
    let mut query = Query::select()
        .from(PolisStatementAuxIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (PolisStatementAuxIden::Table, col)))
        .to_owned();

    if let Some(id) = workflow_step_id {
        query = query
            .and_where(
                Expr::col((
                    PolisStatementAuxIden::Table,
                    PolisStatementAuxIden::WorkflowStepId,
                ))
                .eq(id),
            )
            .to_owned();
    }
    if let Some(conversation_id) = polis_conversation_id {
        query = query
            .and_where(
                Expr::col((
                    PolisStatementAuxIden::Table,
                    PolisStatementAuxIden::PolisConversationId,
                ))
                .eq(conversation_id),
            )
            .to_owned();
    }

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let aux = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(aux)
}

#[derive(Debug, Serialize, FromRow, JsonSchema)]
pub struct ThemeStatistic {
    pub theme: String,
    pub count: i64,
}

#[instrument(err(Debug))]
pub async fn theme_stats(
    db: &PgPool,
    workflow_step_id: Option<Uuid>,
    polis_conversation_id: Option<String>,
) -> Result<Vec<ThemeStatistic>, ComhairleError> {
    let mut builder = sqlx::QueryBuilder::new(
        "SELECT theme, COUNT(*)::BIGINT AS count \
         FROM polis_statement_aux, UNNEST(themes) AS theme \
         WHERE TRUE",
    );

    if let Some(id) = workflow_step_id {
        builder.push(" AND workflow_step_id = ").push_bind(id);
    }
    if let Some(conversation_id) = polis_conversation_id {
        builder
            .push(" AND polis_conversation_id = ")
            .push_bind(conversation_id);
    }

    builder.push(" GROUP BY theme ORDER BY count DESC");

    let stats = builder
        .build_query_as::<ThemeStatistic>()
        .fetch_all(db)
        .await?;

    Ok(stats)
}

#[instrument(err(Debug))]
pub async fn delete(db: &PgPool, id: Uuid) -> Result<PolisStatementAux, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(PolisStatementAuxIden::Table)
        .and_where(Expr::col(PolisStatementAuxIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let aux = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(aux)
}
