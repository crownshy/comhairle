//! Append-only log of moderation decisions on Polis statements. Rows are only ever
//! inserted; an undo is recorded as a new event.

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, Order, PostgresQueryBuilder, Query, SimpleExpr, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::{PgPool, query_as_with};
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::polis_statement_aux::PolisStatementAux;
use crate::wiki_poll_service::ModerationStatus;

/// What kind of moderation step an event records.
#[derive(Debug, Deserialize, Serialize, PartialEq, sqlx::Type, Clone, Copy, JsonSchema)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ModerationAction {
    /// Accept, reject or move back to waiting.
    Decision,
    /// The statement was replaced by a split or reword, or was posted as a replacement.
    Split,
    /// A split was undone.
    UndoSplit,
}

impl ModerationAction {
    fn as_str(self) -> &'static str {
        match self {
            ModerationAction::Decision => "decision",
            ModerationAction::Split => "split",
            ModerationAction::UndoSplit => "undo_split",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "polis_moderation_event")]
pub struct PolisModerationEvent {
    pub id: Uuid,
    pub polis_statement_aux_id: Uuid,
    pub workflow_step_id: Uuid,
    /// Who made the decision. Empty if their account has since been deleted.
    pub actor_id: Option<Uuid>,
    pub action: ModerationAction,
    pub from_status: ModerationStatus,
    pub to_status: ModerationStatus,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [PolisModerationEventIden; 9] = [
    PolisModerationEventIden::Id,
    PolisModerationEventIden::PolisStatementAuxId,
    PolisModerationEventIden::WorkflowStepId,
    PolisModerationEventIden::ActorId,
    PolisModerationEventIden::Action,
    PolisModerationEventIden::FromStatus,
    PolisModerationEventIden::ToStatus,
    PolisModerationEventIden::Reason,
    PolisModerationEventIden::CreatedAt,
];

/// One change to record: a statement as it was before and as it is now.
pub struct StatusChange<'a> {
    pub before: &'a PolisStatementAux,
    pub after: &'a PolisStatementAux,
}

/// Record one event per change. A change whose status and reason didn't move is skipped,
/// so retrying a decision doesn't log it twice.
#[instrument(err(Debug), skip(db, changes))]
pub async fn record(
    db: &PgPool,
    actor_id: Uuid,
    action: ModerationAction,
    changes: &[StatusChange<'_>],
) -> Result<(), ComhairleError> {
    let changed: Vec<&StatusChange> = changes
        .iter()
        .filter(|change| {
            change.before.moderation_status != change.after.moderation_status
                || change.before.moderation_reason != change.after.moderation_reason
        })
        .collect();
    if changed.is_empty() {
        return Ok(());
    }

    let mut insert = Query::insert()
        .into_table(PolisModerationEventIden::Table)
        .columns([
            PolisModerationEventIden::PolisStatementAuxId,
            PolisModerationEventIden::WorkflowStepId,
            PolisModerationEventIden::ActorId,
            PolisModerationEventIden::Action,
            PolisModerationEventIden::FromStatus,
            PolisModerationEventIden::ToStatus,
            PolisModerationEventIden::Reason,
        ])
        .to_owned();
    for change in changed {
        let values: [SimpleExpr; 7] = [
            change.after.id.into(),
            change.after.workflow_step_id.into(),
            actor_id.into(),
            action.as_str().into(),
            change.before.moderation_status.to_string().into(),
            change.after.moderation_status.to_string().into(),
            change.after.moderation_reason.clone().into(),
        ];
        insert.values(values)?;
    }

    let (sql, values) = insert.build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(db).await?;
    Ok(())
}

/// Every event for a step, newest first.
#[instrument(err(Debug), skip(db))]
pub async fn list_by_workflow_step_id(
    db: &PgPool,
    workflow_step_id: Uuid,
) -> Result<Vec<PolisModerationEvent>, ComhairleError> {
    let (sql, values) = Query::select()
        .from(PolisModerationEventIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(PolisModerationEventIden::WorkflowStepId).eq(workflow_step_id))
        .order_by(PolisModerationEventIden::CreatedAt, Order::Desc)
        .order_by(PolisModerationEventIden::Id, Order::Desc)
        .build_sqlx(PostgresQueryBuilder);

    let events = query_as_with(&sql, values).fetch_all(db).await?;
    Ok(events)
}
