//! Moderation policies: the reasons a moderator can pick when rejecting a statement.
//!
//! A policy belongs to a conversation and holds an ordered list of reasons. A Polis step
//! points at one through `PolisToolConfig::moderation_policy_id`, so steps in the same
//! conversation can share a policy. A step without one uses [`DEFAULT_REASONS`].
//!
//! The picked label is still stored as free text in `polis_statement_aux.moderation_reason`
//! (ADR-0015), so editing a policy never rewrites reasons already recorded.

use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, Order, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, PgPool, prelude::FromRow, query_as_with};
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::SqlxResultExt;

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
struct CleanReason {
    id: Option<Uuid>,
    label: String,
    description: Option<String>,
}

fn clean_name(name: &str) -> Result<String, ComhairleError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(ComhairleError::BadRequest(
            "Moderation policy name can't be blank".into(),
        ));
    }
    Ok(name.to_string())
}

/// Trims every reason and rejects the list if a label is blank, contains
/// [`REASON_NOTE_SEPARATOR`], or repeats another label ignoring case. The label is both the
/// key in the reason picker and the stored value, so it has to be unique and splittable.
fn clean_reasons<'a>(
    reasons: impl IntoIterator<Item = (Option<Uuid>, &'a str, Option<&'a str>)>,
) -> Result<Vec<CleanReason>, ComhairleError> {
    let mut seen_labels = HashSet::new();
    let mut seen_ids = HashSet::new();
    let mut cleaned = vec![];

    for (id, label, description) in reasons {
        let label = label.trim();
        if label.is_empty() {
            return Err(ComhairleError::BadRequest(
                "Reason labels can't be blank".into(),
            ));
        }
        if label.contains(REASON_NOTE_SEPARATOR) {
            return Err(ComhairleError::BadRequest(format!(
                "Reason label \"{label}\" can't contain \"{REASON_NOTE_SEPARATOR}\""
            )));
        }
        if !seen_labels.insert(label.to_lowercase()) {
            return Err(ComhairleError::BadRequest(format!(
                "Reason label \"{label}\" is used more than once"
            )));
        }
        if id.is_some_and(|id| !seen_ids.insert(id)) {
            return Err(ComhairleError::BadRequest(
                "A reason id is listed more than once".into(),
            ));
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
) -> Result<(), ComhairleError> {
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
) -> Result<Vec<ModerationPolicyReason>, ComhairleError> {
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
) -> Result<ModerationPolicyWithReasons, ComhairleError> {
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
) -> Result<ModerationPolicyWithReasons, ComhairleError> {
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
) -> Result<Vec<ModerationPolicyWithReasons>, ComhairleError> {
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
) -> Result<ModerationPolicyWithReasons, ComhairleError> {
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
        return Err(ComhairleError::ResourceNotFound("Moderation Policy".into()));
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
            return Err(ComhairleError::ResourceNotFound(format!(
                "Moderation Policy Reason {reason_id}"
            )));
        }
    }

    tx.commit().await?;

    get_by_id(db, conversation_id, id).await
}

/// Counts workflow steps whose preview or live tool config points at the policy. The id
/// lives inside the tool config jsonb, so no foreign key guards it.
#[instrument(err(Debug), skip(db))]
async fn count_steps_using(db: &PgPool, id: Uuid) -> Result<i64, ComhairleError> {
    let count = sqlx::query_scalar(
        "SELECT COUNT(*) FROM workflow_step
            WHERE preview_tool_config ->> 'moderation_policy_id' = $1
               OR tool_config ->> 'moderation_policy_id' = $1",
    )
    .bind(id.to_string())
    .fetch_one(db)
    .await?;

    Ok(count)
}

/// Deletes the policy and its reasons. Refuses while a workflow step still uses it.
#[instrument(err(Debug), skip(db))]
pub async fn delete(db: &PgPool, conversation_id: Uuid, id: Uuid) -> Result<(), ComhairleError> {
    let steps_using = count_steps_using(db, id).await?;
    if steps_using > 0 {
        return Err(ComhairleError::Conflict(format!(
            "Moderation policy is used by {steps_using} workflow step(s)"
        )));
    }

    let (sql, values) = Query::delete()
        .from_table(ModerationPolicyIden::Table)
        .and_where(Expr::col(ModerationPolicyIden::Id).eq(id))
        .and_where(Expr::col(ModerationPolicyIden::ConversationId).eq(conversation_id))
        .build_sqlx(PostgresQueryBuilder);

    let deleted = sqlx::query_with(&sql, values).execute(db).await?;
    if deleted.rows_affected() == 0 {
        return Err(ComhairleError::ResourceNotFound("Moderation Policy".into()));
    }

    Ok(())
}

/// Errors unless the policy belongs to the conversation. Checked before a step's
/// `moderation_policy_id` is saved, since there's no foreign key to do it.
#[instrument(err(Debug), skip(db))]
pub async fn ensure_in_conversation(
    db: &PgPool,
    conversation_id: Uuid,
    id: Uuid,
) -> Result<(), ComhairleError> {
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM moderation_policy WHERE id = $1 AND conversation_id = $2)",
    )
    .bind(id)
    .bind(conversation_id)
    .fetch_one(db)
    .await?;

    if exists {
        Ok(())
    } else {
        Err(ComhairleError::BadRequest(format!(
            "Moderation policy {id} doesn't belong to this conversation"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;

    use crate::models::model_test_helpers::{
        get_random_conversation_id, setup_default_app_and_session,
    };

    fn new_reason(label: &str) -> (Option<Uuid>, &str, Option<&str>) {
        (None, label, None)
    }

    #[test]
    fn should_trim_labels_and_drop_blank_descriptions() -> Result<(), Box<dyn Error>> {
        let cleaned = clean_reasons([(None, "  Duplicate ", Some("   "))])?;

        assert_eq!(cleaned[0].label, "Duplicate", "label not trimmed");
        assert_eq!(cleaned[0].description, None, "blank description kept");

        Ok(())
    }

    #[test]
    fn should_reject_blank_label() {
        let result = clean_reasons([new_reason("   ")]);

        assert!(matches!(result, Err(ComhairleError::BadRequest(_))));
    }

    #[test]
    fn should_reject_label_containing_the_separator() {
        let result = clean_reasons([new_reason("Off-topic: unclear")]);

        assert!(matches!(result, Err(ComhairleError::BadRequest(_))));
    }

    #[test]
    fn should_reject_labels_repeated_ignoring_case() {
        let result = clean_reasons([new_reason("Duplicate"), new_reason(" duplicate")]);

        assert!(matches!(result, Err(ComhairleError::BadRequest(_))));
    }

    #[test]
    fn should_reject_repeated_reason_ids() {
        let id = Some(Uuid::new_v4());
        let result = clean_reasons([(id, "Duplicate", None), (id, "Harmful", None)]);

        assert!(matches!(result, Err(ComhairleError::BadRequest(_))));
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_policy_with_default_reasons(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let created = create(
            &pool,
            conversation_id,
            &CreateModerationPolicy {
                name: "Default".into(),
                reasons: None,
            },
        )
        .await?;

        let labels: Vec<&str> = created
            .reasons
            .iter()
            .map(|reason| reason.label.as_str())
            .collect();
        let default_labels: Vec<&str> = DEFAULT_REASONS.iter().map(|(label, _)| *label).collect();
        assert_eq!(labels, default_labels, "reasons don't match the defaults");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_reasons_keeping_ids(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let created = create(
            &pool,
            conversation_id,
            &CreateModerationPolicy {
                name: "Policy".into(),
                reasons: Some(vec![
                    NewModerationPolicyReason {
                        label: "Spam".into(),
                        description: None,
                    },
                    NewModerationPolicyReason {
                        label: "Rude".into(),
                        description: None,
                    },
                ]),
            },
        )
        .await?;
        let rude = &created.reasons[1];

        let updated = update(
            &pool,
            conversation_id,
            created.policy.id,
            &UpdateModerationPolicy {
                name: "Renamed".into(),
                reasons: vec![
                    UpdateModerationPolicyReason {
                        id: Some(rude.id),
                        label: "Abusive".into(),
                        description: Some("Insults or threats".into()),
                    },
                    UpdateModerationPolicyReason {
                        id: None,
                        label: "Off-topic".into(),
                        description: None,
                    },
                ],
            },
        )
        .await?;

        assert_eq!(updated.policy.name, "Renamed", "name not updated");
        assert_eq!(updated.reasons.len(), 2, "dropped reason not deleted");
        assert_eq!(updated.reasons[0].id, rude.id, "kept reason got a new id");
        assert_eq!(updated.reasons[0].label, "Abusive", "label not updated");
        assert_eq!(updated.reasons[0].position, 0, "kept reason not reordered");
        assert_eq!(updated.reasons[1].label, "Off-topic", "new reason missing");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_not_update_reason_from_another_policy(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let default_policy = CreateModerationPolicy {
            name: "Policy".into(),
            reasons: None,
        };
        let first = create(&pool, conversation_id, &default_policy).await?;
        let second = create(&pool, conversation_id, &default_policy).await?;

        let result = update(
            &pool,
            conversation_id,
            first.policy.id,
            &UpdateModerationPolicy {
                name: "Policy".into(),
                reasons: vec![UpdateModerationPolicyReason {
                    id: Some(second.reasons[0].id),
                    label: "Stolen".into(),
                    description: None,
                }],
            },
        )
        .await;

        assert!(
            matches!(result, Err(ComhairleError::ResourceNotFound(_))),
            "reason from another policy was accepted"
        );
        let first_after = get_by_id(&pool, conversation_id, first.policy.id).await?;
        assert_eq!(
            first_after.reasons.len(),
            DEFAULT_REASONS.len(),
            "failed update wasn't rolled back"
        );

        Ok(())
    }
}
