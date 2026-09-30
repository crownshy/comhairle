//! Moderation policies: the reasons a moderator can pick when rejecting a statement.
//!
//! A policy belongs to a conversation and holds an ordered list of reasons. A Polis step
//! points at one through `PolisToolConfig::moderation_policy_id`, so steps in the same
//! conversation can share a policy. A step without one uses [`DEFAULT_REASONS`].
//!
//! The picked label is still stored as free text in `polis_statement_aux.moderation_reason`
//! (ADR-0015), so editing a policy never rewrites reasons already recorded.

use crate::models::error::ModelError;
use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, LockType, Order, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, PgPool, prelude::FromRow, query_as_with};
use tracing::instrument;
use uuid::Uuid;

use crate::models::SqlxResultExt;
use crate::models::error::DataError;
use crate::models::error::ValidationError;

/// Joins a reason's label to the moderator's note in `moderation_reason`. A label can't
/// contain it, or the label couldn't be split back out of a stored reason.
pub const REASON_NOTE_SEPARATOR: &str = ": ";

/// The reasons a step uses when it has no moderation policy, as `(label, description)`.
/// "Multiple themes" is left out on purpose: splitting is the right action there, and the
/// split flow rejects the original with its own reason.
pub const DEFAULT_REASONS: [(&str, &str); 5] = [
    (
        "Off-topic or unclear",
        "Not about this conversation, or too unclear for people to vote on.",
    ),
    (
        "Harmful or abusive",
        "Hate speech, threats, harassment, or content that breaks the law.",
    ),
    (
        "Advertising or campaigning",
        "Promotes a product, service, political party or campaign.",
    ),
    (
        "Privacy or personal info",
        "Names or identifies a private person, or shares personal details.",
    ),
    (
        "Duplicate",
        "Makes the same point as a statement that is already in the conversation.",
    ),
];

#[derive(Serialize, Deserialize, Debug, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "moderation_policy")]
pub struct ModerationPolicy {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const POLICY_COLUMNS: [ModerationPolicyIden; 5] = [
    ModerationPolicyIden::Id,
    ModerationPolicyIden::ConversationId,
    ModerationPolicyIden::Name,
    ModerationPolicyIden::CreatedAt,
    ModerationPolicyIden::UpdatedAt,
];

#[derive(Serialize, Deserialize, Debug, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "moderation_policy_reason")]
pub struct ModerationPolicyReason {
    pub id: Uuid,
    pub moderation_policy_id: Uuid,
    pub label: String,
    pub description: Option<String>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const REASON_COLUMNS: [ModerationPolicyReasonIden; 7] = [
    ModerationPolicyReasonIden::Id,
    ModerationPolicyReasonIden::ModerationPolicyId,
    ModerationPolicyReasonIden::Label,
    ModerationPolicyReasonIden::Description,
    ModerationPolicyReasonIden::Position,
    ModerationPolicyReasonIden::CreatedAt,
    ModerationPolicyReasonIden::UpdatedAt,
];

/// A policy with its reasons in display order.
#[derive(Debug, Clone)]
pub struct ModerationPolicyWithReasons {
    pub policy: ModerationPolicy,
    pub reasons: Vec<ModerationPolicyReason>,
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct NewModerationPolicyReason {
    pub label: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct CreateModerationPolicy {
    pub name: String,
    /// Leave out to start from the default reasons.
    pub reasons: Option<Vec<NewModerationPolicyReason>>,
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct UpdateModerationPolicyReason {
    /// The id of a reason already on this policy, or none to add a new one.
    pub id: Option<Uuid>,
    pub label: String,
    pub description: Option<String>,
}

/// Replaces a policy's name and its whole reason list. Reasons left out are deleted, and
/// the list order becomes the display order.
#[derive(Deserialize, Debug, JsonSchema)]
pub struct UpdateModerationPolicy {
    pub name: String,
    pub reasons: Vec<UpdateModerationPolicyReason>,
}

#[derive(Debug)]
pub struct CleanReason {
    pub id: Option<Uuid>,
    pub label: String,
    pub description: Option<String>,
}

fn clean_name(name: &str) -> Result<String, ModelError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(
            ValidationError::BadRequest("Moderation policy name can't be blank".into()).into(),
        );
    }
    Ok(name.to_string())
}

/// Trims every reason and rejects the list if a label is blank, contains
/// [`REASON_NOTE_SEPARATOR`], or repeats another label ignoring case. The label is both the
/// key in the reason picker and the stored value, so it has to be unique and splittable.
pub fn clean_reasons<'a>(
    reasons: impl IntoIterator<Item = (Option<Uuid>, &'a str, Option<&'a str>)>,
) -> Result<Vec<CleanReason>, ModelError> {
    let mut seen_labels = HashSet::new();
    let mut seen_ids = HashSet::new();
    let mut cleaned = vec![];

    for (id, label, description) in reasons {
        let label = label.trim();
        if label.is_empty() {
            return Err(ValidationError::BadRequest("Reason labels can't be blank".into()).into());
        }
        if label.contains(REASON_NOTE_SEPARATOR) {
            return Err(ValidationError::BadRequest(format!(
                "Reason label \"{label}\" can't contain \"{REASON_NOTE_SEPARATOR}\""
            ))
            .into());
        }
        if !seen_labels.insert(label.to_lowercase()) {
            return Err(ValidationError::BadRequest(format!(
                "Reason label \"{label}\" is used more than once"
            ))
            .into());
        }
        if id.is_some_and(|id| !seen_ids.insert(id)) {
            return Err(
                ValidationError::BadRequest("A reason id is listed more than once".into()).into(),
            );
        }

        cleaned.push(CleanReason {
            id,
            label: label.to_string(),
            description: description
                .map(str::trim)
                .filter(|description| !description.is_empty())
                .map(str::to_string),
        });
    }

    Ok(cleaned)
}

#[instrument(err(Debug), skip(tx))]
async fn insert_reason(
    tx: &mut PgConnection,
    moderation_policy_id: Uuid,
    position: i32,
    reason: &CleanReason,
) -> Result<(), ModelError> {
    let (sql, values) = Query::insert()
        .into_table(ModerationPolicyReasonIden::Table)
        .columns([
            ModerationPolicyReasonIden::ModerationPolicyId,
            ModerationPolicyReasonIden::Label,
            ModerationPolicyReasonIden::Description,
            ModerationPolicyReasonIden::Position,
        ])
        .values([
            moderation_policy_id.into(),
            reason.label.clone().into(),
            reason.description.clone().into(),
            position.into(),
        ])?
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(&sql, values).execute(&mut *tx).await?;

    Ok(())
}

#[instrument(err(Debug), skip(db))]
async fn list_reasons(
    db: &PgPool,
    policy_ids: &[Uuid],
) -> Result<Vec<ModerationPolicyReason>, ModelError> {
    let (sql, values) = Query::select()
        .columns(REASON_COLUMNS)
        .from(ModerationPolicyReasonIden::Table)
        .and_where(
            Expr::col(ModerationPolicyReasonIden::ModerationPolicyId)
                .is_in(policy_ids.iter().copied()),
        )
        .order_by(ModerationPolicyReasonIden::Position, Order::Asc)
        .build_sqlx(PostgresQueryBuilder);

    let reasons = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(reasons)
}

#[instrument(err(Debug), skip(db))]
pub async fn create(
    db: &PgPool,
    conversation_id: Uuid,
    payload: &CreateModerationPolicy,
) -> Result<ModerationPolicyWithReasons, ModelError> {
    let name = clean_name(&payload.name)?;
    let reasons = match &payload.reasons {
        Some(reasons) => clean_reasons(
            reasons
                .iter()
                .map(|reason| (None, reason.label.as_str(), reason.description.as_deref())),
        )?,
        None => clean_reasons(
            DEFAULT_REASONS
                .iter()
                .map(|(label, description)| (None, *label, Some(*description))),
        )?,
    };

    let mut tx = db.begin().await?;

    let (sql, values) = Query::insert()
        .into_table(ModerationPolicyIden::Table)
        .columns([
            ModerationPolicyIden::ConversationId,
            ModerationPolicyIden::Name,
        ])
        .values([conversation_id.into(), name.into()])?
        .returning(Query::returning().columns(POLICY_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let policy: ModerationPolicy = query_as_with(&sql, values)
        .fetch_one(&mut *tx)
        .await
        .resolve_db_err("Moderation Policy")?;

    for (position, reason) in reasons.iter().enumerate() {
        insert_reason(&mut tx, policy.id, position as i32, reason).await?;
    }

    tx.commit().await?;

    get_by_id(db, conversation_id, policy.id).await
}

#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(
    db: &PgPool,
    conversation_id: Uuid,
    id: Uuid,
) -> Result<ModerationPolicyWithReasons, ModelError> {
    let (sql, values) = Query::select()
        .columns(POLICY_COLUMNS)
        .from(ModerationPolicyIden::Table)
        .and_where(Expr::col(ModerationPolicyIden::Id).eq(id))
        .and_where(Expr::col(ModerationPolicyIden::ConversationId).eq(conversation_id))
        .build_sqlx(PostgresQueryBuilder);

    let policy: ModerationPolicy = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Moderation Policy")?;
    let reasons = list_reasons(db, &[policy.id]).await?;

    Ok(ModerationPolicyWithReasons { policy, reasons })
}

#[instrument(err(Debug), skip(db))]
pub async fn list_for_conversation(
    db: &PgPool,
    conversation_id: Uuid,
) -> Result<Vec<ModerationPolicyWithReasons>, ModelError> {
    let (sql, values) = Query::select()
        .columns(POLICY_COLUMNS)
        .from(ModerationPolicyIden::Table)
        .and_where(Expr::col(ModerationPolicyIden::ConversationId).eq(conversation_id))
        .order_by(ModerationPolicyIden::CreatedAt, Order::Asc)
        .build_sqlx(PostgresQueryBuilder);

    let policies: Vec<ModerationPolicy> = query_as_with(&sql, values).fetch_all(db).await?;
    if policies.is_empty() {
        return Ok(vec![]);
    }

    let policy_ids: Vec<Uuid> = policies.iter().map(|policy| policy.id).collect();
    let mut reasons_by_policy: HashMap<Uuid, Vec<ModerationPolicyReason>> = HashMap::new();
    for reason in list_reasons(db, &policy_ids).await? {
        reasons_by_policy
            .entry(reason.moderation_policy_id)
            .or_default()
            .push(reason);
    }

    Ok(policies
        .into_iter()
        .map(|policy| {
            let reasons = reasons_by_policy.remove(&policy.id).unwrap_or_default();
            ModerationPolicyWithReasons { policy, reasons }
        })
        .collect())
}

/// Saves the policy's name and reason list in one transaction. Reasons with an id are
/// updated in place, so their ids stay stable across saves.
#[instrument(err(Debug), skip(db))]
pub async fn update(
    db: &PgPool,
    conversation_id: Uuid,
    id: Uuid,
    payload: &UpdateModerationPolicy,
) -> Result<ModerationPolicyWithReasons, ModelError> {
    let name = clean_name(&payload.name)?;
    let reasons = clean_reasons(payload.reasons.iter().map(|reason| {
        (
            reason.id,
            reason.label.as_str(),
            reason.description.as_deref(),
        )
    }))?;

    let mut tx = db.begin().await?;

    let (sql, values) = Query::update()
        .table(ModerationPolicyIden::Table)
        .values([
            (ModerationPolicyIden::Name, name.into()),
            (
                ModerationPolicyIden::UpdatedAt,
                Expr::current_timestamp().into(),
            ),
        ])
        .and_where(Expr::col(ModerationPolicyIden::Id).eq(id))
        .and_where(Expr::col(ModerationPolicyIden::ConversationId).eq(conversation_id))
        .build_sqlx(PostgresQueryBuilder);

    let updated = sqlx::query_with(&sql, values).execute(&mut *tx).await?;
    if updated.rows_affected() == 0 {
        return Err(DataError::ResourceNotFound("Moderation Policy".into()).into());
    }

    let kept_ids: Vec<Uuid> = reasons.iter().filter_map(|reason| reason.id).collect();
    let mut delete_dropped = Query::delete();
    delete_dropped
        .from_table(ModerationPolicyReasonIden::Table)
        .and_where(Expr::col(ModerationPolicyReasonIden::ModerationPolicyId).eq(id));
    if !kept_ids.is_empty() {
        delete_dropped.and_where(Expr::col(ModerationPolicyReasonIden::Id).is_not_in(kept_ids));
    }
    let (sql, values) = delete_dropped.build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *tx).await?;

    for (position, reason) in reasons.iter().enumerate() {
        let position = position as i32;
        let Some(reason_id) = reason.id else {
            insert_reason(&mut tx, id, position, reason).await?;
            continue;
        };

        let (sql, values) = Query::update()
            .table(ModerationPolicyReasonIden::Table)
            .values([
                (
                    ModerationPolicyReasonIden::Label,
                    reason.label.clone().into(),
                ),
                (
                    ModerationPolicyReasonIden::Description,
                    reason.description.clone().into(),
                ),
                (ModerationPolicyReasonIden::Position, position.into()),
                (
                    ModerationPolicyReasonIden::UpdatedAt,
                    Expr::current_timestamp().into(),
                ),
            ])
            .and_where(Expr::col(ModerationPolicyReasonIden::Id).eq(reason_id))
            .and_where(Expr::col(ModerationPolicyReasonIden::ModerationPolicyId).eq(id))
            .build_sqlx(PostgresQueryBuilder);

        let updated = sqlx::query_with(&sql, values).execute(&mut *tx).await?;
        if updated.rows_affected() == 0 {
            return Err(DataError::ResourceNotFound(format!(
                "Moderation Policy Reason {reason_id}"
            ))
            .into());
        }
    }

    tx.commit().await?;

    get_by_id(db, conversation_id, id).await
}

/// Counts workflow steps whose preview or live tool config points at the policy. The id
/// lives inside the tool config jsonb, so no foreign key guards it.
#[instrument(err(Debug), skip(tx))]
async fn count_steps_using(tx: &mut PgConnection, id: Uuid) -> Result<i64, ModelError> {
    let count = sqlx::query_scalar(
        "SELECT COUNT(*) FROM workflow_step
            WHERE preview_tool_config ->> 'moderation_policy_id' = $1
               OR tool_config ->> 'moderation_policy_id' = $1",
    )
    .bind(id.to_string())
    .fetch_one(&mut *tx)
    .await?;

    Ok(count)
}

/// Deletes the policy and its reasons. Refuses while a workflow step still uses it.
///
/// The policy row is locked before steps are counted. A step save holds a lock on the same
/// row ([`lock_for_step`]), so either the save waits and then finds the policy gone, or this
/// waits for the save and then counts its step.
#[instrument(err(Debug), skip(db))]
pub async fn delete(db: &PgPool, conversation_id: Uuid, id: Uuid) -> Result<(), ModelError> {
    let mut tx = db.begin().await?;

    // Scoped to the conversation before counting, so another conversation's policy is a 404
    // whether or not a step uses it.
    let (sql, values) = Query::select()
        .column(ModerationPolicyIden::Id)
        .from(ModerationPolicyIden::Table)
        .and_where(Expr::col(ModerationPolicyIden::Id).eq(id))
        .and_where(Expr::col(ModerationPolicyIden::ConversationId).eq(conversation_id))
        .lock(LockType::Update)
        .build_sqlx(PostgresQueryBuilder);

    let locked: Option<Uuid> = sqlx::query_scalar_with(&sql, values)
        .fetch_optional(&mut *tx)
        .await?;
    if locked.is_none() {
        return Err(DataError::ResourceNotFound("Moderation Policy".into()).into());
    }

    let steps_using = count_steps_using(&mut tx, id).await?;
    if steps_using > 0 {
        return Err(DataError::Conflict(format!(
            "Moderation policy is used by {steps_using} workflow step(s)"
        ))
        .into());
    }

    let (sql, values) = Query::delete()
        .from_table(ModerationPolicyIden::Table)
        .and_where(Expr::col(ModerationPolicyIden::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *tx).await?;

    tx.commit().await?;

    Ok(())
}

/// Locks the policy for a workflow step save, and errors unless it belongs to the step's own
/// conversation. There's no foreign key from the tool config jsonb, so call this in the
/// transaction that writes the step. `FOR KEY SHARE` is the lock a foreign key check takes:
/// it blocks [`delete`] until the step is saved, but not edits to the policy.
#[instrument(err(Debug), skip(tx))]
pub async fn lock_for_step(
    tx: &mut PgConnection,
    workflow_step_id: Uuid,
    id: Uuid,
) -> Result<(), ModelError> {
    let locked: Option<Uuid> = sqlx::query_scalar(
        "SELECT moderation_policy.id FROM moderation_policy
            WHERE moderation_policy.id = $1
              AND moderation_policy.conversation_id = (
                  SELECT workflow.conversation_id FROM workflow_step
                      JOIN workflow ON workflow.id = workflow_step.workflow_id
                      WHERE workflow_step.id = $2
              )
            FOR KEY SHARE",
    )
    .bind(id)
    .bind(workflow_step_id)
    .fetch_optional(&mut *tx)
    .await?;

    match locked {
        Some(_) => Ok(()),
        None => Err(ValidationError::BadRequest(format!(
            "Moderation policy {id} doesn't belong to this conversation"
        ))
        .into()),
    }
}
