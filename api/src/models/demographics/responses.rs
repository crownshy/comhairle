use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use super::VersionStatus;
use super::questions::{QuestionVersionDto, load_version};
use super::validation::{invalid, validate_answer};
use crate::error::ComhairleError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConsentState {
    None,
    Granted,
    Declined,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ConsentInput {
    pub granted: bool,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ResponseInput {
    pub value: Value,
    pub conversation_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, sqlx::FromRow)]
pub struct ResponseDto {
    pub question_version_id: Uuid,
    pub question_id: Uuid,
    pub value: Value,
    pub conversation_id: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
}

const RESPONSE_SELECT: &str = "
    SELECT r.question_version_id, v.question_id, r.value, r.conversation_id, r.updated_at
    FROM demographics_response r
    JOIN demographics_question_version v ON v.id = r.question_version_id";

pub async fn consent_state(
    conn: &mut PgConnection,
    user_id: Uuid,
    version_id: Uuid,
) -> Result<ConsentState, ComhairleError> {
    let granted: Option<bool> = sqlx::query_scalar(
        "SELECT granted FROM demographics_consent
         WHERE question_version_id = $1 AND user_id = $2",
    )
    .bind(version_id)
    .bind(user_id)
    .fetch_optional(&mut *conn)
    .await?;
    Ok(match granted {
        None => ConsentState::None,
        Some(true) => ConsentState::Granted,
        Some(false) => ConsentState::Declined,
    })
}

pub async fn existing_response(
    conn: &mut PgConnection,
    user_id: Uuid,
    version_id: Uuid,
) -> Result<Option<ResponseDto>, ComhairleError> {
    Ok(sqlx::query_as::<_, ResponseDto>(&format!(
        "{RESPONSE_SELECT} WHERE r.question_version_id = $1 AND r.user_id = $2"
    ))
    .bind(version_id)
    .bind(user_id)
    .fetch_optional(&mut *conn)
    .await?)
}

/// An answer to a special category question is only reusable while consent is granted.
/// Everything else is reusable as long as it exists for the exact version.
pub async fn reusable_response(
    conn: &mut PgConnection,
    user_id: Uuid,
    version: &QuestionVersionDto,
) -> Result<Option<ResponseDto>, ComhairleError> {
    if version.special_category
        && consent_state(&mut *conn, user_id, version.id).await? != ConsentState::Granted
    {
        return Ok(None);
    }
    existing_response(conn, user_id, version.id).await
}

pub async fn my_responses(db: &PgPool, user_id: Uuid) -> Result<Vec<ResponseDto>, ComhairleError> {
    Ok(sqlx::query_as::<_, ResponseDto>(&format!(
        "{RESPONSE_SELECT} WHERE r.user_id = $1 ORDER BY r.updated_at DESC"
    ))
    .bind(user_id)
    .fetch_all(db)
    .await?)
}

async fn published_version_for_user(
    db: &PgPool,
    version_id: Uuid,
    locale: &str,
) -> Result<QuestionVersionDto, ComhairleError> {
    let version = load_version(db, version_id, locale).await?;
    if version.status != VersionStatus::Published {
        return Err(invalid("demographic question version is not published"));
    }
    Ok(version)
}

/// Records whether the user agrees to share a special category answer. Declining is kept
/// so that the question is skipped for that user.
pub async fn set_consent(
    db: &PgPool,
    user_id: Uuid,
    version_id: Uuid,
    granted: bool,
    locale: &str,
) -> Result<ConsentState, ComhairleError> {
    let version = published_version_for_user(db, version_id, locale).await?;
    if !version.special_category {
        return Err(invalid(
            "consent is only recorded for special category questions",
        ));
    }
    sqlx::query(
        "INSERT INTO demographics_consent (id, question_version_id, user_id, granted)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (question_version_id, user_id) WHERE user_id IS NOT NULL
         DO UPDATE SET granted = EXCLUDED.granted, updated_at = NOW()",
    )
    .bind(Uuid::new_v4())
    .bind(version_id)
    .bind(user_id)
    .bind(granted)
    .execute(db)
    .await?;
    Ok(if granted {
        ConsentState::Granted
    } else {
        ConsentState::Declined
    })
}

pub async fn upsert_response(
    conn: &mut PgConnection,
    user_id: Uuid,
    version_id: Uuid,
    conversation_id: Option<Uuid>,
    value: &Value,
) -> Result<(), ComhairleError> {
    sqlx::query(
        "INSERT INTO demographics_response (id, question_version_id, user_id, conversation_id, value)
         VALUES ($1, $2, $3, $4, $5)
         ON CONFLICT (question_version_id, user_id) WHERE user_id IS NOT NULL
         DO UPDATE SET value = EXCLUDED.value,
                       conversation_id = EXCLUDED.conversation_id,
                       updated_at = NOW()",
    )
    .bind(Uuid::new_v4())
    .bind(version_id)
    .bind(user_id)
    .bind(conversation_id)
    .bind(value)
    .execute(&mut *conn)
    .await?;
    Ok(())
}

pub async fn put_response(
    db: &PgPool,
    user_id: Uuid,
    version_id: Uuid,
    input: &ResponseInput,
    locale: &str,
) -> Result<ResponseDto, ComhairleError> {
    let version = published_version_for_user(db, version_id, locale).await?;
    let mut conn = db.acquire().await?;
    if version.special_category
        && consent_state(&mut conn, user_id, version_id).await? != ConsentState::Granted
    {
        return Err(ComhairleError::Unprocessable(
            "consent is required before answering this question".into(),
        ));
    }
    validate_answer(&version, &input.value)?;
    upsert_response(
        &mut conn,
        user_id,
        version_id,
        input.conversation_id,
        &input.value,
    )
    .await?;
    existing_response(&mut conn, user_id, version_id)
        .await?
        .ok_or_else(|| ComhairleError::ResourceNotFound("demographic response".into()))
}
