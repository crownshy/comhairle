use crate::models::{self, users::User};
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

use crate::{error::ComhairleError, wiki_poll_service::ModerationStatus};

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
    pub moderation_status: ModerationStatus,
}

/// Upsert a row from polis. On conflict (workflow_step_id, polis_statement_id),
/// `statement_text`, `is_seed`, and `moderation_status` are refreshed from
/// polis — `moderation_reason`, `themes`, `visible_statement_when_submitted`,
/// and `user_id` are preserved (these are comhairle-side state polis doesn't know about).
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
        PolisStatementAuxIden::ModerationStatus,
    ];
    let values: Vec<SimpleExpr> = vec![
        record.workflow_step_id.into(),
        record.user_id.into(),
        record.zid.into(),
        record.polis_conversation_id.clone().into(),
        record.polis_statement_id.into(),
        record.statement_text.clone().into(),
        record.is_seed.into(),
        record.moderation_status.clone().into(),
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
                PolisStatementAuxIden::ModerationStatus,
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
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<PolisStatementAux, ComhairleError> {
    let (sql, values) = Query::select()
        .from(PolisStatementAuxIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(PolisStatementAuxIden::Id).eq(*id))
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
pub async fn check_is_commentor(
    db: &PgPool,
    aux_id: &Uuid,
    user_id: &Uuid,
) -> Result<(), ComhairleError> {
    let aux = get_by_id(db, aux_id).await?;

    if aux.user_id != Some(*user_id) {
        return Err(ComhairleError::UserNotAuthorized);
    }

    Ok(())
}

#[instrument(err(Debug))]
pub async fn check_can_moderate(
    db: &PgPool,
    user: &User,
    workflow_step_id: &Uuid,
) -> Result<(), ComhairleError> {
    let workflow_step = models::workflow_step::get_by_id(db, workflow_step_id).await?;

    let workflow = models::workflow::get_by_id(db, &workflow_step.workflow_id).await?;
    let conversation_id = workflow.conversation_id.ok_or(ComhairleError::BadRequest(
        "workflow is not attached to a conversation".into(),
    ))?;
    let conversation = models::conversation::get_by_id(&db, &conversation_id).await?;
    if conversation.owner_id != user.id {
        return Err(ComhairleError::UserNotAuthorized);
    }
    Ok(())
}

#[instrument(err(Debug))]
pub async fn list(
    db: &PgPool,
    workflow_step_id: Option<Uuid>,
    polis_conversation_id: Option<String>,
    filter_options: PolisStatementAuxFilterOptions,
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

    let query = filter_options.apply(query);

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

#[instrument(err(Debug), skip(db))]
pub async fn add_theme(
    db: &PgPool,
    id: Uuid,
    theme: &str,
) -> Result<PolisStatementAux, ComhairleError> {
    let aux = sqlx::query_as::<_, PolisStatementAux>(
        "UPDATE polis_statement_aux \
         SET themes = CASE WHEN $2 = ANY(themes) THEN themes ELSE array_append(themes, $2) END, \
             updated_at = NOW() \
         WHERE id = $1 \
         RETURNING *",
    )
    .bind(id)
    .bind(theme)
    .fetch_one(db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => ComhairleError::ResourceNotFound("Polis statement aux".into()),
        other => ComhairleError::DatabaseError(other),
    })?;

    Ok(aux)
}

#[instrument(err(Debug), skip(db))]
pub async fn remove_theme(
    db: &PgPool,
    id: Uuid,
    theme: &str,
) -> Result<PolisStatementAux, ComhairleError> {
    let aux = sqlx::query_as::<_, PolisStatementAux>(
        "UPDATE polis_statement_aux \
         SET themes = array_remove(themes, $2), updated_at = NOW() \
         WHERE id = $1 \
         RETURNING *",
    )
    .bind(id)
    .bind(theme)
    .fetch_one(db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => ComhairleError::ResourceNotFound("Polis statement aux".into()),
        other => ComhairleError::DatabaseError(other),
    })?;

    Ok(aux)
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

#[cfg(test)]
mod tests {
    use std::error::Error;

    use serde_json::json;
    use sqlx::PgPool;

    use crate::{
        models::model_test_helpers::setup_default_app_and_session,
        test_helpers::{extract, polis_tool_config},
    };

    use super::*;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn upsert_from_polis_refreshes_moderation_status(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;
        let conversation_id: String = extract("id", &conversation);

        let (_, workflow, _) = session
            .create_random_workflow(&app, &conversation_id)
            .await?;
        let workflow_id: String = extract("id", &workflow);

        let (_, workflow_step, _) = session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
                json!({
                    "name": "Polis step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "polis step",
                    "is_offline": false,
                    "required": true,
                    "tool_setup": polis_tool_config(),
                })
                .to_string()
                .into(),
            )
            .await?;
        let workflow_step_id: String = extract("id", &workflow_step);
        let workflow_step_id = Uuid::parse_str(&workflow_step_id)?;

        let record = UpsertFromPolis {
            workflow_step_id,
            user_id: None,
            zid: 7,
            polis_conversation_id: "test-poll".into(),
            polis_statement_id: 42,
            statement_text: "first text".into(),
            is_seed: false,
            moderation_status: ModerationStatus::Pending,
        };

        let initial = upsert_from_polis(&pool, &record).await?;
        assert_eq!(initial.moderation_status, ModerationStatus::Pending);

        let accepted = upsert_from_polis(
            &pool,
            &UpsertFromPolis {
                statement_text: "updated text".into(),
                moderation_status: ModerationStatus::Accepted,
                ..record
            },
        )
        .await?;

        assert_eq!(
            accepted.id, initial.id,
            "should have hit ON CONFLICT, not inserted a new row"
        );
        assert_eq!(
            accepted.moderation_status,
            ModerationStatus::Accepted,
            "moderation_status should refresh from polis on re-sync"
        );
        assert_eq!(accepted.statement_text, "updated text");
        Ok(())
    }
}
