use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::translations::{TextContentId, localize_translations};

/// Question ids asked by any of the workflows bound to `$1` (a uuid array).
/// Form steps contribute the questions of the version they pin, or of the latest
/// published form version, minus questions the step disables.
const LATEST_PUBLISHED_TEXT_SQL: &str = "
    SELECT lv.text_content_id
    FROM demographics_question_version lv
    WHERE lv.question_id = q.id AND lv.status = 'published'
    ORDER BY lv.version_number DESC
    LIMIT 1";

const ASKED_QUESTIONS_CTE: &str = "
    asked AS (
        SELECT (ws.tool_config->>'question_id')::UUID AS question_id
        FROM workflow_step ws
        WHERE ws.workflow_id = ANY($1::uuid[])
          AND ws.tool_config->>'type' = 'demographicquestion'
        UNION
        SELECT fvq.question_id
        FROM workflow_step ws
        INNER JOIN demographics_form_version fv
            ON fv.form_id = (ws.tool_config->>'form_id')::UUID
            AND fv.status = 'published'
            AND (
                (ws.tool_config->'version'->>'type' = 'pinned'
                    AND fv.id = (ws.tool_config->'version'->>'version_id')::UUID)
                OR (COALESCE(ws.tool_config->'version'->>'type', 'latest') = 'latest'
                    AND fv.version_number = (
                        SELECT MAX(latest.version_number)
                        FROM demographics_form_version latest
                        WHERE latest.form_id = fv.form_id AND latest.status = 'published'
                    ))
            )
        INNER JOIN demographics_form_version_question fvq ON fvq.form_version_id = fv.id
        WHERE ws.workflow_id = ANY($1::uuid[])
          AND ws.tool_config->>'type' = 'demographicform'
          AND NOT EXISTS (
              SELECT 1
              FROM jsonb_array_elements(COALESCE(ws.tool_config->'questions', '[]'::jsonb)) override
              WHERE (override->>'question_id')::UUID = fvq.question_id
                AND (override->>'enabled')::BOOLEAN = false
          )
    )";

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct WorkflowDemographics {
    pub total_participants: i64,
    /// Keyed by question id.
    pub questions: HashMap<Uuid, QuestionCounts>,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct QuestionCounts {
    pub question_id: Uuid,
    pub text: String,
    pub answers: Vec<AnswerCount>,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct AnswerCount {
    pub value: serde_json::Value,
    pub count: i64,
}

#[derive(sqlx::FromRow)]
struct ReportRow {
    question_id: Uuid,
    text_content_id: TextContentId,
    value: serde_json::Value,
    count: i64,
}

/// Counts answers from consented participants to the demographic questions a workflow asks.
/// Only answers to published versions of those questions are counted.
pub async fn for_workflow(
    db: &PgPool,
    workflow_id: &Uuid,
    locale: &str,
) -> Result<WorkflowDemographics, ComhairleError> {
    let total_participants: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT up.user_id)::BIGINT
         FROM user_participation up
         WHERE up.workflow_id = $1",
    )
    .bind(workflow_id)
    .fetch_one(db)
    .await?;

    let rows: Vec<ReportRow> = sqlx::query_as(&format!(
        "WITH {ASKED_QUESTIONS_CTE}
         SELECT
            q.id AS question_id,
            latest.text_content_id AS text_content_id,
            r.value AS value,
            COUNT(DISTINCT up.user_id)::BIGINT AS count
         FROM user_participation up
         INNER JOIN user_profile prof ON prof.user_id = up.user_id AND prof.consented = true
         INNER JOIN demographics_response r ON r.user_id = up.user_id
         INNER JOIN demographics_question_version qv ON qv.id = r.question_version_id
            AND qv.status = 'published'
         INNER JOIN demographics_question q ON q.id = qv.question_id
         INNER JOIN LATERAL ({LATEST_PUBLISHED_TEXT_SQL}) latest ON TRUE
         WHERE up.workflow_id = ANY($1::uuid[]) AND q.id IN (SELECT question_id FROM asked)
         GROUP BY q.id, latest.text_content_id, r.value
         ORDER BY q.id, count DESC"
    ))
    .bind(vec![*workflow_id])
    .fetch_all(db)
    .await?;

    let text_ids: Vec<TextContentId> = rows
        .iter()
        .map(|row| row.text_content_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let texts = localize_translations(db, &text_ids, locale).await?;

    let mut questions: HashMap<Uuid, QuestionCounts> = HashMap::new();
    for row in rows {
        let text = texts.get(&row.text_content_id).cloned().unwrap_or_default();
        let entry = questions
            .entry(row.question_id)
            .or_insert_with(|| QuestionCounts {
                question_id: row.question_id,
                text,
                answers: Vec::new(),
            });
        entry.answers.push(AnswerCount {
            value: row.value,
            count: row.count,
        });
    }

    Ok(WorkflowDemographics {
        total_participants,
        questions,
    })
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ExportQuestion {
    pub question_id: Uuid,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ExportParticipant {
    pub user_id: Uuid,
    pub profile_created_at: DateTime<Utc>,
    /// Latest answer value per question id. Questions without an answer are absent.
    pub answers: HashMap<Uuid, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ConversationDemographicsExport {
    /// Ordered by question text, so the CSV columns are stable.
    pub questions: Vec<ExportQuestion>,
    pub participants: Vec<ExportParticipant>,
}

#[derive(sqlx::FromRow)]
struct ExportAnswerRow {
    user_id: Uuid,
    question_id: Uuid,
    value: serde_json::Value,
}

#[derive(sqlx::FromRow)]
struct ExportQuestionRow {
    question_id: Uuid,
    text_content_id: TextContentId,
}

#[derive(sqlx::FromRow)]
struct ExportParticipantRow {
    user_id: Uuid,
    profile_created_at: DateTime<Utc>,
}

/// Exports the answers of consented participants across all workflows of a conversation.
pub async fn export_for_conversation(
    db: &PgPool,
    conversation_id: &Uuid,
    locale: &str,
) -> Result<ConversationDemographicsExport, ComhairleError> {
    let workflow_ids: Vec<Uuid> =
        sqlx::query_scalar("SELECT id FROM workflow WHERE conversation_id = $1")
            .bind(conversation_id)
            .fetch_all(db)
            .await?;

    let participants: Vec<ExportParticipantRow> = sqlx::query_as(
        "SELECT DISTINCT prof.user_id, prof.created_at AS profile_created_at
         FROM user_profile prof
         INNER JOIN user_participation up ON up.user_id = prof.user_id
         WHERE up.workflow_id = ANY($1::uuid[]) AND prof.consented = true
         ORDER BY prof.created_at DESC",
    )
    .bind(&workflow_ids)
    .fetch_all(db)
    .await?;

    let question_rows: Vec<ExportQuestionRow> = sqlx::query_as(&format!(
        "WITH {ASKED_QUESTIONS_CTE}
         SELECT q.id AS question_id, latest.text_content_id
         FROM demographics_question q
         INNER JOIN LATERAL ({LATEST_PUBLISHED_TEXT_SQL}) latest ON TRUE
         WHERE q.id IN (SELECT question_id FROM asked)"
    ))
    .bind(&workflow_ids)
    .fetch_all(db)
    .await?;

    // The latest answer wins when a user has answered several versions of one question.
    // Special category answers only count while consent is still granted.
    let answer_rows: Vec<ExportAnswerRow> = sqlx::query_as(&format!(
        "WITH {ASKED_QUESTIONS_CTE}
         SELECT DISTINCT ON (prof.user_id, qv.question_id)
            prof.user_id,
            qv.question_id,
            r.value
         FROM user_profile prof
         INNER JOIN user_participation up ON up.user_id = prof.user_id
         INNER JOIN demographics_response r ON r.user_id = prof.user_id
         INNER JOIN demographics_question_version qv ON qv.id = r.question_version_id
            AND qv.status = 'published'
         LEFT JOIN demographics_consent c ON c.question_version_id = r.question_version_id
            AND c.user_id = r.user_id
         WHERE up.workflow_id = ANY($1::uuid[])
           AND prof.consented = true
           AND qv.question_id IN (SELECT question_id FROM asked)
           AND (NOT qv.special_category OR c.granted = true)
         ORDER BY prof.user_id, qv.question_id, r.updated_at DESC"
    ))
    .bind(&workflow_ids)
    .fetch_all(db)
    .await?;

    let text_ids: Vec<TextContentId> = question_rows.iter().map(|q| q.text_content_id).collect();
    let texts = localize_translations(db, &text_ids, locale).await?;

    let mut questions: Vec<ExportQuestion> = question_rows
        .iter()
        .map(|q| ExportQuestion {
            question_id: q.question_id,
            text: texts.get(&q.text_content_id).cloned().unwrap_or_default(),
        })
        .collect();
    questions.sort_by(|a, b| a.text.cmp(&b.text).then(a.question_id.cmp(&b.question_id)));

    let mut answers_by_user: HashMap<Uuid, HashMap<Uuid, serde_json::Value>> = HashMap::new();
    for row in answer_rows {
        answers_by_user
            .entry(row.user_id)
            .or_default()
            .insert(row.question_id, row.value);
    }

    let participants = participants
        .into_iter()
        .map(|p| ExportParticipant {
            answers: answers_by_user.remove(&p.user_id).unwrap_or_default(),
            user_id: p.user_id,
            profile_created_at: p.profile_created_at,
        })
        .collect();

    Ok(ConversationDemographicsExport {
        questions,
        participants,
    })
}
