use chrono::{DateTime, Utc};
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::types::Json;
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::workflow::WorkflowIden;
use crate::models::workflow_step::{WorkflowStep, WorkflowStepIden};
use crate::tools::ToolConfig;
use crate::tools::demographics::{
    DemographicFormToolConfig, DemographicQuestionToolConfig, QuestionOverride, VersionPolicy,
};

use super::VersionStatus;
use super::forms::{latest_published_form_version_id, load_entries, published_form_version_of};
use super::questions::{
    is_question_visible, latest_published_version_id, load_version, usable_published_version,
};
use super::responses::{ConsentState, consent_state, reusable_response};
use super::types::*;
use super::validation::invalid;

/// Lists the questions a participant must answer for a demographic step, in order.
pub async fn pending_questions(
    db: &PgPool,
    user_id: Uuid,
    step: &WorkflowStep,
    locale: &str,
) -> Result<Vec<PendingQuestion>, ComhairleError> {
    let questions = resolve_step_questions(db, step, locale).await?;

    let mut pending = Vec::with_capacity(questions.len());
    for (version_id, required) in questions {
        let version = load_version(db, version_id, locale).await?;
        if version.version.status != VersionStatus::Published {
            return Err(invalid("demographic question version is not published"));
        }
        let mut conn = db.acquire().await?;
        pending.push(pending_question(&mut conn, user_id, version, required).await?);
    }
    Ok(pending)
}

async fn pending_question(
    conn: &mut PgConnection,
    user_id: Uuid,
    version: QuestionVersionDto,
    required: bool,
) -> Result<PendingQuestion, ComhairleError> {
    let consent = consent_state(&mut *conn, user_id, version.version.id).await?;
    let skipped = consent == ConsentState::Declined;
    let answer = if skipped {
        None
    } else {
        reusable_response(&mut *conn, user_id, &version).await?
    };
    Ok(PendingQuestion {
        version,
        required,
        consent,
        answer,
        skipped,
    })
}

/// The published question versions a step asks, with their required flags. Pinned
/// versions are checked for publication when they are loaded.
async fn resolve_step_questions(
    db: &PgPool,
    step: &WorkflowStep,
    locale: &str,
) -> Result<Vec<(Uuid, bool)>, ComhairleError> {
    match &step.tool_config {
        Some(ToolConfig::DemographicQuestion(config)) => {
            let version_id = match config.version {
                VersionPolicy::Latest => latest_published_version_id(db, config.question_id)
                    .await?
                    .ok_or_else(|| invalid("demographic question has no published version"))?,
                VersionPolicy::Pinned { version_id } => version_id,
            };
            Ok(vec![(version_id, step.required)])
        }
        Some(ToolConfig::DemographicForm(config)) => form_questions(db, config, locale).await,
        _ => Err(invalid("step does not collect demographics")),
    }
}

async fn form_questions(
    db: &PgPool,
    config: &DemographicFormToolConfig,
    locale: &str,
) -> Result<Vec<(Uuid, bool)>, ComhairleError> {
    let mut conn = db.acquire().await?;
    let form_version_id = match config.version {
        VersionPolicy::Latest => latest_published_form_version_id(&mut conn, config.form_id)
            .await?
            .ok_or_else(|| invalid("demographic form has no published version"))?,
        VersionPolicy::Pinned { version_id } => {
            published_form_version_of(&mut conn, config.form_id, version_id).await?
        }
    };
    drop(conn);
    let mut entries = load_entries(db, &[form_version_id], locale).await?;
    let mut entries = entries.remove(&form_version_id).unwrap_or_default();
    entries.sort_by_key(|dto| dto.entry.position);
    Ok(entries
        .into_iter()
        .filter_map(|dto| {
            let entry = dto.entry;
            let version_id = entry.question_version_id?;
            apply_override(entry.question_id, entry.required, &config.questions)
                .map(|required| (version_id, required))
        })
        .collect())
}

/// Returns whether a form question is asked and whether it is required, given the step's overrides.
fn apply_override(
    question_id: Uuid,
    form_required: bool,
    overrides: &[QuestionOverride],
) -> Option<bool> {
    match overrides.iter().find(|o| o.question_id == question_id) {
        Some(o) if !o.enabled => None,
        Some(o) => Some(o.required),
        None => Some(form_required),
    }
}

/// Checks a demographic step's references when the step is created or updated. Other tool
/// configs are not checked here.
pub async fn validate_tool_config(
    conn: &mut PgConnection,
    owner: Option<Uuid>,
    config: &ToolConfig,
) -> Result<(), ComhairleError> {
    match config {
        ToolConfig::DemographicQuestion(config) => {
            validate_question_step(conn, owner, config).await
        }
        ToolConfig::DemographicForm(config) => validate_form_step(conn, owner, config).await,
        _ => Ok(()),
    }
}

/// Replaces "latest" with the version that is latest right now, so a live step keeps asking
/// the same version. Pinned steps and other tools are returned unchanged.
pub async fn freeze_tool_config(
    db: &PgPool,
    config: ToolConfig,
) -> Result<ToolConfig, ComhairleError> {
    Ok(match config {
        ToolConfig::DemographicQuestion(config) => {
            ToolConfig::DemographicQuestion(freeze_question_config(db, config).await?)
        }
        ToolConfig::DemographicForm(config) => {
            ToolConfig::DemographicForm(freeze_form_config(db, config).await?)
        }
        other => other,
    })
}

pub async fn freeze_question_config(
    db: &PgPool,
    mut config: DemographicQuestionToolConfig,
) -> Result<DemographicQuestionToolConfig, ComhairleError> {
    if config.version == VersionPolicy::Latest {
        let version_id = latest_published_version_id(db, config.question_id)
            .await?
            .ok_or_else(|| invalid("demographic question has no published version"))?;
        config.version = VersionPolicy::Pinned { version_id };
    }
    Ok(config)
}

/// A form version already pins the version of each of its questions.
pub async fn freeze_form_config(
    db: &PgPool,
    mut config: DemographicFormToolConfig,
) -> Result<DemographicFormToolConfig, ComhairleError> {
    if config.version == VersionPolicy::Latest {
        let mut conn = db.acquire().await?;
        let version_id = latest_published_form_version_id(&mut conn, config.form_id)
            .await?
            .ok_or_else(|| invalid("demographic form has no published version"))?;
        config.version = VersionPolicy::Pinned { version_id };
    }
    Ok(config)
}

/// Rejects moving a conversation to another hosting organization while its demographic steps
/// use questions or forms that organization cannot see. Archived items are fine: steps keep
/// their pinned versions.
pub async fn ensure_steps_usable_by(
    db: &PgPool,
    conversation_id: Uuid,
    organization_id: Option<Uuid>,
) -> Result<(), ComhairleError> {
    let (sql, values) = Query::select()
        .columns([
            (WorkflowStepIden::Table, WorkflowStepIden::ToolConfig),
            (WorkflowStepIden::Table, WorkflowStepIden::PreviewToolConfig),
        ])
        .from(WorkflowStepIden::Table)
        .inner_join(
            WorkflowIden::Table,
            Expr::col((WorkflowIden::Table, WorkflowIden::Id))
                .equals((WorkflowStepIden::Table, WorkflowStepIden::WorkflowId)),
        )
        .and_where(
            Expr::col((WorkflowIden::Table, WorkflowIden::ConversationId)).eq(conversation_id),
        )
        .build_sqlx(PostgresQueryBuilder);
    let rows =
        sqlx::query_as_with::<_, (Option<Json<ToolConfig>>, Json<ToolConfig>), _>(&sql, values)
            .fetch_all(db)
            .await?;

    let mut conn = db.acquire().await?;
    for (tool_config, preview_tool_config) in rows {
        for config in tool_config.into_iter().chain([preview_tool_config]) {
            match config.0 {
                ToolConfig::DemographicQuestion(config)
                    if !is_question_visible(&mut *conn, organization_id, config.question_id)
                        .await? =>
                {
                    return Err(invalid(
                        "a demographic question in this conversation is not in the new organization's question bank",
                    ));
                }
                ToolConfig::DemographicForm(config) => {
                    let owned_form = Query::select()
                        .expr(Expr::val(1))
                        .from(DemographicFormIden::Table)
                        .and_where(Expr::col(DemographicFormIden::Id).eq(config.form_id))
                        .and_where(
                            Expr::col(DemographicFormIden::OrganizationId).eq(organization_id),
                        )
                        .to_owned();
                    let (sql, values) = Query::select()
                        .expr(Expr::exists(owned_form))
                        .build_sqlx(PostgresQueryBuilder);
                    let owned: bool = sqlx::query_scalar_with(&sql, values)
                        .fetch_one(&mut *conn)
                        .await?;
                    if !owned {
                        return Err(invalid(
                            "a demographic form in this conversation is not in the new organization's form bank",
                        ));
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

async fn validate_question_step(
    conn: &mut PgConnection,
    owner: Option<Uuid>,
    config: &DemographicQuestionToolConfig,
) -> Result<(), ComhairleError> {
    usable_published_version(&mut *conn, owner, config.question_id).await?;
    if let VersionPolicy::Pinned { version_id } = config.version {
        let published_version = Query::select()
            .expr(Expr::val(1))
            .from(DemographicQuestionVersionIden::Table)
            .and_where(Expr::col(DemographicQuestionVersionIden::Id).eq(version_id))
            .and_where(Expr::col(DemographicQuestionVersionIden::QuestionId).eq(config.question_id))
            .and_where(
                Expr::col(DemographicQuestionVersionIden::Status).eq(VersionStatus::Published),
            )
            .to_owned();
        let (sql, values) = Query::select()
            .expr(Expr::exists(published_version))
            .build_sqlx(PostgresQueryBuilder);
        let is_published: bool = sqlx::query_scalar_with(&sql, values)
            .fetch_one(&mut *conn)
            .await?;
        if !is_published {
            return Err(invalid(
                "pinned question version is not a published version of this question",
            ));
        }
    }
    Ok(())
}

async fn validate_form_step(
    conn: &mut PgConnection,
    owner: Option<Uuid>,
    config: &DemographicFormToolConfig,
) -> Result<(), ComhairleError> {
    let (sql, values) = Query::select()
        .column(DemographicFormIden::ArchivedAt)
        .from(DemographicFormIden::Table)
        .and_where(Expr::col(DemographicFormIden::Id).eq(config.form_id))
        .and_where(Expr::col(DemographicFormIden::OrganizationId).eq(owner))
        .build_sqlx(PostgresQueryBuilder);
    let archived_at = sqlx::query_scalar_with::<_, Option<DateTime<Utc>>, _>(&sql, values)
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| invalid("demographic form is not available to this organization"))?;
    if archived_at.is_some() {
        return Err(invalid("demographic form is archived"));
    }

    let form_version_id = match config.version {
        VersionPolicy::Latest => latest_published_form_version_id(&mut *conn, config.form_id)
            .await?
            .ok_or_else(|| invalid("demographic form has no published version"))?,
        VersionPolicy::Pinned { version_id } => {
            published_form_version_of(&mut *conn, config.form_id, version_id).await?
        }
    };

    let (sql, values) = Query::select()
        .column(DemographicFormVersionQuestionIden::QuestionId)
        .from(DemographicFormVersionQuestionIden::Table)
        .and_where(Expr::col(DemographicFormVersionQuestionIden::FormVersionId).eq(form_version_id))
        .build_sqlx(PostgresQueryBuilder);
    let form_question_ids: Vec<Uuid> = sqlx::query_scalar_with(&sql, values)
        .fetch_all(&mut *conn)
        .await?;
    if config
        .questions
        .iter()
        .any(|o| !form_question_ids.contains(&o.question_id))
    {
        return Err(invalid("question override is not part of this form"));
    }
    Ok(())
}
