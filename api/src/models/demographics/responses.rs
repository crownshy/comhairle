use schemars::JsonSchema;
use sea_query::{Expr, OnConflict, Order, PostgresQueryBuilder, Query, SelectStatement};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use crate::error::ComhairleError;

use super::VersionStatus;
use super::questions::load_version;
use super::types::*;
use super::validation::{invalid, validate_answer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConsentState {
    None,
    Granted,
    Declined,
}

fn response_select() -> SelectStatement {
    Query::select()
        .column((DemographicResponseIden::Table, DemographicResponseIden::Id))
        .column((
            DemographicResponseIden::Table,
            DemographicResponseIden::QuestionVersionId,
        ))
        .column((
            DemographicQuestionVersionIden::Table,
            DemographicQuestionVersionIden::QuestionId,
        ))
        .columns([
            (
                DemographicResponseIden::Table,
                DemographicResponseIden::Value,
            ),
            (
                DemographicResponseIden::Table,
                DemographicResponseIden::ConversationId,
            ),
            (
                DemographicResponseIden::Table,
                DemographicResponseIden::CreatedAt,
            ),
        ])
        .from(DemographicResponseIden::Table)
        .inner_join(
            DemographicQuestionVersionIden::Table,
            Expr::col((
                DemographicQuestionVersionIden::Table,
                DemographicQuestionVersionIden::Id,
            ))
            .equals((
                DemographicResponseIden::Table,
                DemographicResponseIden::QuestionVersionId,
            )),
        )
        .to_owned()
}

pub async fn consent_state(
    conn: &mut PgConnection,
    user_id: Uuid,
    version_id: Uuid,
) -> Result<ConsentState, ComhairleError> {
    let (sql, values) = Query::select()
        .column(DemographicConsentIden::Granted)
        .from(DemographicConsentIden::Table)
        .and_where(Expr::col(DemographicConsentIden::QuestionVersionId).eq(version_id))
        .and_where(Expr::col(DemographicConsentIden::UserId).eq(user_id))
        .build_sqlx(PostgresQueryBuilder);
    let granted: Option<bool> = sqlx::query_scalar_with(&sql, values)
        .fetch_optional(&mut *conn)
        .await?;
    Ok(match granted {
        None => ConsentState::None,
        Some(true) => ConsentState::Granted,
        Some(false) => ConsentState::Declined,
    })
}

/// The user's most recent answer to a question version, from any conversation.
pub async fn latest_response(
    conn: &mut PgConnection,
    user_id: Uuid,
    version_id: Uuid,
) -> Result<Option<ResponseDto>, ComhairleError> {
    let (sql, values) = response_select()
        .and_where(
            Expr::col((
                DemographicResponseIden::Table,
                DemographicResponseIden::QuestionVersionId,
            ))
            .eq(version_id),
        )
        .and_where(
            Expr::col((
                DemographicResponseIden::Table,
                DemographicResponseIden::UserId,
            ))
            .eq(user_id),
        )
        .order_by(
            (
                DemographicResponseIden::Table,
                DemographicResponseIden::CreatedAt,
            ),
            Order::Desc,
        )
        .limit(1)
        .build_sqlx(PostgresQueryBuilder);
    Ok(sqlx::query_as_with::<_, ResponseDto, _>(&sql, values)
        .fetch_optional(&mut *conn)
        .await?)
}

async fn response_in_conversation(
    conn: &mut PgConnection,
    user_id: Uuid,
    version_id: Uuid,
    conversation_id: Option<Uuid>,
) -> Result<Option<ResponseDto>, ComhairleError> {
    let conversation = (
        DemographicResponseIden::Table,
        DemographicResponseIden::ConversationId,
    );
    let (sql, values) = response_select()
        .and_where(
            Expr::col((
                DemographicResponseIden::Table,
                DemographicResponseIden::QuestionVersionId,
            ))
            .eq(version_id),
        )
        .and_where(
            Expr::col((
                DemographicResponseIden::Table,
                DemographicResponseIden::UserId,
            ))
            .eq(user_id),
        )
        .and_where(match conversation_id {
            Some(id) => Expr::col(conversation).eq(id),
            None => Expr::col(conversation).is_null(),
        })
        .build_sqlx(PostgresQueryBuilder);
    Ok(sqlx::query_as_with::<_, ResponseDto, _>(&sql, values)
        .fetch_optional(&mut *conn)
        .await?)
}

/// An answer to a special category question is only reusable while consent is granted.
/// Everything else is reusable as long as it exists for the exact version. Reusing an answer
/// stores a copy for the conversation it is confirmed in; the original never changes.
pub async fn reusable_response(
    conn: &mut PgConnection,
    user_id: Uuid,
    version: &QuestionVersionDto,
) -> Result<Option<ResponseDto>, ComhairleError> {
    if version.version.special_category
        && consent_state(&mut *conn, user_id, version.version.id).await? != ConsentState::Granted
    {
        return Ok(None);
    }
    latest_response(conn, user_id, version.version.id).await
}

async fn my_responses(db: &PgPool, user_id: Uuid) -> Result<Vec<ResponseDto>, ComhairleError> {
    let (sql, values) = response_select()
        .and_where(
            Expr::col((
                DemographicResponseIden::Table,
                DemographicResponseIden::UserId,
            ))
            .eq(user_id),
        )
        .order_by(
            (
                DemographicResponseIden::Table,
                DemographicResponseIden::CreatedAt,
            ),
            Order::Desc,
        )
        .build_sqlx(PostgresQueryBuilder);
    Ok(sqlx::query_as_with::<_, ResponseDto, _>(&sql, values)
        .fetch_all(db)
        .await?)
}

/// Every answer the user has given, newest first, with the version it answered and the
/// user's consent for that version.
pub async fn my_answers(
    db: &PgPool,
    user_id: Uuid,
    locale: &str,
) -> Result<Vec<MyDemographicAnswer>, ComhairleError> {
    let mut answers = Vec::new();
    for answer in my_responses(db, user_id).await? {
        let version = load_version(db, answer.question_version_id, locale).await?;
        let mut conn = db.acquire().await?;
        let consent = consent_state(&mut conn, user_id, answer.question_version_id).await?;
        answers.push(MyDemographicAnswer {
            version,
            answer,
            consent,
        });
    }
    Ok(answers)
}

async fn published_version_for_user(
    db: &PgPool,
    version_id: Uuid,
    locale: &str,
) -> Result<QuestionVersionDto, ComhairleError> {
    let version = load_version(db, version_id, locale).await?;
    if version.version.status != VersionStatus::Published {
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
    if !version.version.special_category {
        return Err(invalid(
            "consent is only recorded for special category questions",
        ));
    }
    let (sql, values) = Query::insert()
        .into_table(DemographicConsentIden::Table)
        .columns([
            DemographicConsentIden::Id,
            DemographicConsentIden::QuestionVersionId,
            DemographicConsentIden::UserId,
            DemographicConsentIden::Granted,
        ])
        .values([
            Uuid::new_v4().into(),
            version_id.into(),
            user_id.into(),
            granted.into(),
        ])?
        .on_conflict(
            OnConflict::columns([
                DemographicConsentIden::QuestionVersionId,
                DemographicConsentIden::UserId,
            ])
            .target_and_where(Expr::col(DemographicConsentIden::UserId).is_not_null())
            .update_column(DemographicConsentIden::Granted)
            .value(DemographicConsentIden::UpdatedAt, Expr::current_timestamp())
            .to_owned(),
        )
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(db).await?;
    Ok(if granted {
        ConsentState::Granted
    } else {
        ConsentState::Declined
    })
}

/// Stores an answer. Returns false when the user already has one for this question version and
/// conversation, which is left as it was.
pub async fn record_response(
    conn: &mut PgConnection,
    user_id: Uuid,
    version_id: Uuid,
    conversation_id: Option<Uuid>,
    value: &Value,
) -> Result<bool, ComhairleError> {
    let (sql, values) = Query::insert()
        .into_table(DemographicResponseIden::Table)
        .columns([
            DemographicResponseIden::Id,
            DemographicResponseIden::QuestionVersionId,
            DemographicResponseIden::UserId,
            DemographicResponseIden::ConversationId,
            DemographicResponseIden::Value,
        ])
        .values([
            Uuid::new_v4().into(),
            version_id.into(),
            user_id.into(),
            conversation_id.into(),
            value.clone().into(),
        ])?
        .on_conflict(OnConflict::new().do_nothing().to_owned())
        .build_sqlx(PostgresQueryBuilder);
    let result = sqlx::query_with(&sql, values).execute(&mut *conn).await?;
    Ok(result.rows_affected() == 1)
}

/// Submits an answer. Submitting the same answer again is accepted, a different one is
/// refused because submitted answers never change.
pub async fn put_response(
    db: &PgPool,
    user_id: Uuid,
    version_id: Uuid,
    input: &ResponseInput,
    locale: &str,
) -> Result<ResponseDto, ComhairleError> {
    let version = published_version_for_user(db, version_id, locale).await?;
    let mut conn = db.acquire().await?;
    if version.version.special_category
        && consent_state(&mut conn, user_id, version_id).await? != ConsentState::Granted
    {
        return Err(ComhairleError::Unprocessable(
            "consent is required before answering this question".into(),
        ));
    }
    validate_answer(&version, &input.value)?;
    record_response(
        &mut conn,
        user_id,
        version_id,
        input.conversation_id,
        &input.value,
    )
    .await?;
    let saved = response_in_conversation(&mut conn, user_id, version_id, input.conversation_id)
        .await?
        .ok_or_else(|| ComhairleError::ResourceNotFound("demographic response".into()))?;
    if saved.value != input.value {
        return Err(ComhairleError::Conflict(
            "this question was already answered and answers cannot be changed".into(),
        ));
    }
    Ok(saved)
}

/// Detaches an answer from the user. The answer itself stays so past results do not change.
pub async fn anonymise_response(
    db: &PgPool,
    user_id: Uuid,
    response_id: Uuid,
) -> Result<(), ComhairleError> {
    let (sql, values) = Query::update()
        .table(DemographicResponseIden::Table)
        .value(DemographicResponseIden::UserId, Option::<Uuid>::None)
        .and_where(Expr::col(DemographicResponseIden::Id).eq(response_id))
        .and_where(Expr::col(DemographicResponseIden::UserId).eq(user_id))
        .build_sqlx(PostgresQueryBuilder);
    let result = sqlx::query_with(&sql, values).execute(db).await?;
    if result.rows_affected() == 0 {
        return Err(ComhairleError::ResourceNotFound(
            "demographic response".into(),
        ));
    }
    Ok(())
}
