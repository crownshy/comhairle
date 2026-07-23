use schemars::JsonSchema;
use sea_query::{Alias, Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::prelude::FromRow;
use sqlx::types::Json;
use sqlx::{PgPool, Row};
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UploadedPart {
    pub part_number: i32,
    pub etag: String,
    pub size_bytes: i64,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct CreateLiveAudioRecording {
    pub audio_recording_id: Uuid,
    pub multipart_upload_id: String,
    pub owner_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[enum_def(table_name = "live_audio_recording")]
pub struct LiveAudioRecording {
    pub id: Uuid,
    pub audio_recording_id: Uuid,
    pub multipart_upload_id: String,
    pub next_part_number: i32,
    pub uploaded_parts: Vec<UploadedPart>,
    pub owner_id: Option<Uuid>,
    pub locked: bool,
}

impl<'r> FromRow<'r, PgRow> for LiveAudioRecording {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let uploaded_parts: Json<Vec<UploadedPart>> = row.try_get("uploaded_parts")?;

        Ok(Self {
            id: row.try_get("id")?,
            audio_recording_id: row.try_get("audio_recording_id")?,
            multipart_upload_id: row.try_get("multipart_upload_id")?,
            next_part_number: row.try_get("next_part_number")?,
            uploaded_parts: uploaded_parts.0,
            owner_id: row.try_get("owner_id")?,
            locked: row.try_get("locked")?,
        })
    }
}

const DEFAULT_COLUMNS: [LiveAudioRecordingIden; 7] = [
    LiveAudioRecordingIden::Id,
    LiveAudioRecordingIden::AudioRecordingId,
    LiveAudioRecordingIden::MultipartUploadId,
    LiveAudioRecordingIden::NextPartNumber,
    LiveAudioRecordingIden::UploadedParts,
    LiveAudioRecordingIden::OwnerId,
    LiveAudioRecordingIden::Locked,
];

fn not_found() -> ComhairleError {
    ComhairleError::ResourceNotFound("Live audio recording not found".to_string())
}

pub async fn create(
    db: &PgPool,
    payload: &CreateLiveAudioRecording,
) -> Result<LiveAudioRecording, ComhairleError> {
    let (sql, values) = Query::insert()
        .into_table(LiveAudioRecordingIden::Table)
        .columns([
            LiveAudioRecordingIden::AudioRecordingId,
            LiveAudioRecordingIden::MultipartUploadId,
            LiveAudioRecordingIden::NextPartNumber,
            LiveAudioRecordingIden::UploadedParts,
            LiveAudioRecordingIden::OwnerId,
            LiveAudioRecordingIden::Locked,
        ])
        .values([
            payload.audio_recording_id.into(),
            payload.multipart_upload_id.clone().into(),
            1.into(),
            json!([]).into(),
            payload.owner_id.into(),
            false.into(),
        ])
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    Ok(
        sqlx::query_as_with::<_, LiveAudioRecording, _>(&sql, values)
            .fetch_one(db)
            .await?,
    )
}

pub async fn list_by_event(
    db: &PgPool,
    event_id: Uuid,
) -> Result<Vec<LiveAudioRecording>, ComhairleError> {
    let subquery = Query::select()
        .column(Alias::new("id"))
        .from(Alias::new("audio_recording"))
        .and_where(Expr::col(Alias::new("event_id")).eq(event_id))
        .to_owned();

    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(LiveAudioRecordingIden::Table)
        .and_where(Expr::col(LiveAudioRecordingIden::AudioRecordingId).in_subquery(subquery))
        .order_by(LiveAudioRecordingIden::Id, sea_query::Order::Asc)
        .build_sqlx(PostgresQueryBuilder);

    Ok(
        sqlx::query_as_with::<_, LiveAudioRecording, _>(&sql, values)
            .fetch_all(db)
            .await?,
    )
}

pub async fn list_by_event_and_owner(
    db: &PgPool,
    event_id: Uuid,
    owner_user_id: Uuid,
) -> Result<Vec<LiveAudioRecording>, ComhairleError> {
    let subquery = Query::select()
        .column(Alias::new("id"))
        .from(Alias::new("audio_recording"))
        .and_where(Expr::col(Alias::new("event_id")).eq(event_id))
        .to_owned();

    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(LiveAudioRecordingIden::Table)
        .and_where(Expr::col(LiveAudioRecordingIden::AudioRecordingId).in_subquery(subquery))
        .and_where(Expr::col(LiveAudioRecordingIden::OwnerId).eq(owner_user_id))
        .order_by(LiveAudioRecordingIden::Id, sea_query::Order::Asc)
        .build_sqlx(PostgresQueryBuilder);

    Ok(
        sqlx::query_as_with::<_, LiveAudioRecording, _>(&sql, values)
            .fetch_all(db)
            .await?,
    )
}

pub async fn get(
    db: &PgPool,
    live_audio_recording_id: Uuid,
) -> Result<LiveAudioRecording, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(LiveAudioRecordingIden::Table)
        .and_where(Expr::col(LiveAudioRecordingIden::Id).eq(live_audio_recording_id))
        .build_sqlx(PostgresQueryBuilder);

    Ok(
        sqlx::query_as_with::<_, LiveAudioRecording, _>(&sql, values)
            .fetch_optional(db)
            .await?
            .ok_or_else(not_found)?,
    )
}

#[instrument(err(Debug), skip(db))]
pub async fn append_uploaded_part(
    db: &PgPool,
    live_audio_recording_id: Uuid,
    uploaded_part: UploadedPart,
    expected_part_number: i32,
) -> Result<LiveAudioRecording, ComhairleError> {
    let current = get(db, live_audio_recording_id).await?;

    if current.next_part_number != expected_part_number {
        return Err(ComhairleError::CorruptedData(format!(
            "Expected part_number {}, got {}",
            current.next_part_number, expected_part_number
        )));
    }

    let mut next_uploaded_parts = current.uploaded_parts;
    next_uploaded_parts.push(uploaded_part);

    let (sql, values) = Query::update()
        .table(LiveAudioRecordingIden::Table)
        .values([
            (
                LiveAudioRecordingIden::UploadedParts,
                json!(next_uploaded_parts).into(),
            ),
            (
                LiveAudioRecordingIden::NextPartNumber,
                (expected_part_number + 1).into(),
            ),
        ])
        .and_where(Expr::col(LiveAudioRecordingIden::Id).eq(live_audio_recording_id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    Ok(
        sqlx::query_as_with::<_, LiveAudioRecording, _>(&sql, values)
            .fetch_optional(db)
            .await?
            .ok_or_else(not_found)?,
    )
}

#[instrument(err(Debug), skip(db))]
pub async fn delete(
    db: &PgPool,
    live_audio_recording_id: Uuid,
) -> Result<LiveAudioRecording, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(LiveAudioRecordingIden::Table)
        .and_where(Expr::col(LiveAudioRecordingIden::Id).eq(live_audio_recording_id))
        .build_sqlx(PostgresQueryBuilder);

    let deleted = sqlx::query_as_with::<_, LiveAudioRecording, _>(&sql, values)
        .fetch_optional(db)
        .await?
        .ok_or_else(not_found)?;
    Ok(deleted)
}

#[instrument(err(Debug), skip(db))]
pub async fn lock_live_recording(
    db: &PgPool,
    live_audio_recording_id: Uuid,
    user_id: Uuid,
) -> Result<LiveAudioRecording, ComhairleError> {
    let (sql, values) = Query::update()
        .table(LiveAudioRecordingIden::Table)
        .value(LiveAudioRecordingIden::Locked, true)
        .and_where(Expr::col(LiveAudioRecordingIden::Id).eq(live_audio_recording_id))
        .and_where(Expr::col(LiveAudioRecordingIden::OwnerId).eq(user_id))
        .and_where(Expr::col(LiveAudioRecordingIden::Locked).eq(false))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    if let Some(row) = sqlx::query_as_with::<_, LiveAudioRecording, _>(&sql, values)
        .fetch_optional(db)
        .await?
    {
        return Ok(row);
    }

    let current = get(db, live_audio_recording_id).await?;
    if current.locked {
        return Err(ComhairleError::LiveAudioRecordingLocked);
    }

    Err(not_found())
}

#[instrument(err(Debug), skip(db))]
pub async fn unlock_live_recording(
    db: &PgPool,
    live_audio_recording_id: Uuid,
    user_id: Uuid,
) -> Result<(), ComhairleError> {
    let (sql, values) = Query::update()
        .table(LiveAudioRecordingIden::Table)
        .value(LiveAudioRecordingIden::Locked, false)
        .and_where(Expr::col(LiveAudioRecordingIden::Id).eq(live_audio_recording_id))
        .and_where(Expr::col(LiveAudioRecordingIden::OwnerId).eq(user_id))
        .and_where(Expr::col(LiveAudioRecordingIden::Locked).eq(true))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(&sql, values).execute(db).await?;
    Ok(())
}
