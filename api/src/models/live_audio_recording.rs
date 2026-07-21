use schemars::JsonSchema;
use sea_query::{Alias, Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::error::ComhairleError;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UploadedPart {
    pub part_number: i32,
    pub etag: String,
    pub size_bytes: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LiveAudioRecording {
    pub id: Uuid,
    pub audio_recording_id: Uuid,
    pub multipart_upload_id: String,
    pub next_part_number: i32,
    pub uploaded_parts: Vec<UploadedPart>,
    pub locked_by_user_id: Option<Uuid>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CreateLiveAudioRecording {
    pub audio_recording_id: Uuid,
    pub multipart_upload_id: String,
    pub locked_by_user_id: Option<Uuid>,
}

#[derive(Debug, FromRow, Clone)]
#[enum_def(table_name = "live_audio_recording")]
struct RawLiveAudioRecording {
    pub id: Uuid,
    pub audio_recording_id: Uuid,
    pub multipart_upload_id: String,
    pub next_part_number: i32,
    pub uploaded_parts: serde_json::Value,
    pub locked_by_user_id: Option<Uuid>,
}

const DEFAULT_COLUMNS: [RawLiveAudioRecordingIden; 6] = [
    RawLiveAudioRecordingIden::Id,
    RawLiveAudioRecordingIden::AudioRecordingId,
    RawLiveAudioRecordingIden::MultipartUploadId,
    RawLiveAudioRecordingIden::NextPartNumber,
    RawLiveAudioRecordingIden::UploadedParts,
    RawLiveAudioRecordingIden::LockedByUserId,
];

impl TryFrom<RawLiveAudioRecording> for LiveAudioRecording {
    type Error = ComhairleError;

    fn try_from(raw: RawLiveAudioRecording) -> Result<Self, Self::Error> {
        Ok(Self {
            id: raw.id,
            audio_recording_id: raw.audio_recording_id,
            multipart_upload_id: raw.multipart_upload_id,
            next_part_number: raw.next_part_number,
            uploaded_parts: serde_json::from_value(raw.uploaded_parts).map_err(|err| {
                ComhairleError::CorruptedData(format!(
                    "Invalid uploaded_parts data for live audio recording {}: {err}",
                    raw.id
                ))
            })?,
            locked_by_user_id: raw.locked_by_user_id,
        })
    }
}

fn not_found() -> ComhairleError {
    ComhairleError::ResourceNotFound("Live audio recording not found".to_string())
}

pub async fn create(
    db: &PgPool,
    payload: &CreateLiveAudioRecording,
) -> Result<LiveAudioRecording, ComhairleError> {
    let (sql, values) = Query::insert()
        .into_table(RawLiveAudioRecordingIden::Table)
        .columns([
            RawLiveAudioRecordingIden::AudioRecordingId,
            RawLiveAudioRecordingIden::MultipartUploadId,
            RawLiveAudioRecordingIden::NextPartNumber,
            RawLiveAudioRecordingIden::UploadedParts,
            RawLiveAudioRecordingIden::LockedByUserId,
        ])
        .values([
            payload.audio_recording_id.into(),
            payload.multipart_upload_id.clone().into(),
            1.into(),
            json!([]).into(),
            payload.locked_by_user_id.into(),
        ])
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let row = sqlx::query_as_with::<_, RawLiveAudioRecording, _>(&sql, values)
        .fetch_one(db)
        .await?;

    row.try_into()
}

pub async fn list_by_event(
    db: &PgPool,
    event_id: &Uuid,
) -> Result<Vec<LiveAudioRecording>, ComhairleError> {
    let subquery = Query::select()
        .column(Alias::new("id"))
        .from(Alias::new("audio_recording"))
        .and_where(Expr::col(Alias::new("event_id")).eq(*event_id))
        .to_owned();

    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(RawLiveAudioRecordingIden::Table)
        .and_where(Expr::col(RawLiveAudioRecordingIden::AudioRecordingId).in_subquery(subquery))
        .order_by(RawLiveAudioRecordingIden::Id, sea_query::Order::Asc)
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, RawLiveAudioRecording, _>(&sql, values)
        .fetch_all(db)
        .await?;

    rows.into_iter().map(TryInto::try_into).collect()
}

pub async fn get_by_id_and_event(
    db: &PgPool,
    live_audio_recording_id: &Uuid,
    event_id: &Uuid,
) -> Result<LiveAudioRecording, ComhairleError> {
    let subquery = Query::select()
        .column(Alias::new("id"))
        .from(Alias::new("audio_recording"))
        .and_where(Expr::col(Alias::new("event_id")).eq(*event_id))
        .to_owned();

    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(RawLiveAudioRecordingIden::Table)
        .and_where(Expr::col(RawLiveAudioRecordingIden::Id).eq(*live_audio_recording_id))
        .and_where(Expr::col(RawLiveAudioRecordingIden::AudioRecordingId).in_subquery(subquery))
        .build_sqlx(PostgresQueryBuilder);

    let row = sqlx::query_as_with::<_, RawLiveAudioRecording, _>(&sql, values)
        .fetch_optional(db)
        .await?
        .ok_or_else(not_found)?;

    row.try_into()
}

pub async fn append_uploaded_part(
    db: &PgPool,
    live_audio_recording_id: &Uuid,
    uploaded_part: UploadedPart,
    expected_part_number: i32,
) -> Result<LiveAudioRecording, ComhairleError> {
    let current = get_by_id(db, live_audio_recording_id).await?;

    if current.next_part_number != expected_part_number {
        return Err(ComhairleError::CorruptedData(format!(
            "Expected part_number {}, got {}",
            current.next_part_number, expected_part_number
        )));
    }

    let mut next_uploaded_parts = current.uploaded_parts;
    next_uploaded_parts.push(uploaded_part);

    let (sql, values) = Query::update()
        .table(RawLiveAudioRecordingIden::Table)
        .values([
            (
                RawLiveAudioRecordingIden::UploadedParts,
                json!(next_uploaded_parts).into(),
            ),
            (
                RawLiveAudioRecordingIden::NextPartNumber,
                (expected_part_number + 1).into(),
            ),
        ])
        .and_where(Expr::col(RawLiveAudioRecordingIden::Id).eq(*live_audio_recording_id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let row = sqlx::query_as_with::<_, RawLiveAudioRecording, _>(&sql, values)
        .fetch_optional(db)
        .await?
        .ok_or_else(not_found)?;

    row.try_into()
}

pub async fn delete(db: &PgPool, live_audio_recording_id: &Uuid) -> Result<(), ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(RawLiveAudioRecordingIden::Table)
        .and_where(Expr::col(RawLiveAudioRecordingIden::Id).eq(*live_audio_recording_id))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(&sql, values).execute(db).await?;
    Ok(())
}

pub async fn get_by_id(
    db: &PgPool,
    live_audio_recording_id: &Uuid,
) -> Result<LiveAudioRecording, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(RawLiveAudioRecordingIden::Table)
        .and_where(Expr::col(RawLiveAudioRecordingIden::Id).eq(*live_audio_recording_id))
        .build_sqlx(PostgresQueryBuilder);

    let row = sqlx::query_as_with::<_, RawLiveAudioRecording, _>(&sql, values)
        .fetch_optional(db)
        .await?
        .ok_or_else(not_found)?;

    row.try_into()
}

pub async fn lock_for_user(
    db: &PgPool,
    live_audio_recording_id: &Uuid,
    user_id: &Uuid,
) -> Result<LiveAudioRecording, ComhairleError> {
    let (sql, values) = Query::update()
        .table(RawLiveAudioRecordingIden::Table)
        .values([(RawLiveAudioRecordingIden::LockedByUserId, (*user_id).into())])
        .and_where(Expr::col(RawLiveAudioRecordingIden::Id).eq(*live_audio_recording_id))
        .and_where(
            Expr::col(RawLiveAudioRecordingIden::LockedByUserId)
                .is_null()
                .or(Expr::col(RawLiveAudioRecordingIden::LockedByUserId).eq(*user_id)),
        )
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    if let Some(row) = sqlx::query_as_with::<_, RawLiveAudioRecording, _>(&sql, values)
        .fetch_optional(db)
        .await?
    {
        return row.try_into();
    }

    let current = get_by_id(db, live_audio_recording_id).await?;
    if let Some(locked_by_user_id) = current.locked_by_user_id
        && locked_by_user_id != *user_id
    {
        return Err(ComhairleError::LiveAudioRecordingLocked {
            live_audio_recording_id: *live_audio_recording_id,
            locked_by_user_id,
        });
    }

    Err(not_found())
}

pub async fn unlock_for_user(
    db: &PgPool,
    live_audio_recording_id: &Uuid,
    user_id: &Uuid,
) -> Result<(), ComhairleError> {
    let (sql, values) = Query::update()
        .table(RawLiveAudioRecordingIden::Table)
        .values([(
            RawLiveAudioRecordingIden::LockedByUserId,
            Option::<Uuid>::None.into(),
        )])
        .and_where(Expr::col(RawLiveAudioRecordingIden::Id).eq(*live_audio_recording_id))
        .and_where(Expr::col(RawLiveAudioRecordingIden::LockedByUserId).eq(*user_id))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(&sql, values).execute(db).await?;
    Ok(())
}
