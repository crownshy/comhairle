use std::collections::HashMap;

use schemars::JsonSchema;
use sea_query::{
    Alias, Expr, JoinType, LockType, OnConflict, Order, PostgresQueryBuilder, Query, enum_def,
};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
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
    pub owner_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[enum_def(table_name = "live_audio_recording")]
pub struct LiveAudioRecording {
    pub id: Uuid,
    pub audio_recording_id: Uuid,
    pub owner_id: Option<Uuid>,
    pub locked: bool,
}

impl<'r> FromRow<'r, PgRow> for LiveAudioRecording {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            audio_recording_id: row.try_get("audio_recording_id")?,
            owner_id: row.try_get("owner_id")?,
            locked: row.try_get("locked")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct LiveAudioRecordingState {
    pub id: Uuid,
    pub audio_recording_id: Uuid,
    pub multipart_upload_id: String,
    pub next_part_number: i32,
    pub uploaded_parts: Vec<UploadedPart>,
    pub owner_id: Option<Uuid>,
    pub locked: bool,
}

#[derive(Debug, Clone, FromRow)]
#[enum_def(table_name = "live_audio_recording_multipart_upload")]
pub struct LiveAudioRecordingMultipartUpload {
    pub live_audio_recording_id: Uuid,
    pub multipart_upload_id: String,
    pub next_part_number: i32,
    pub uploaded_parts: Json<Vec<UploadedPart>>,
}

const LIVE_DEFAULT_COLUMNS: [LiveAudioRecordingIden; 4] = [
    LiveAudioRecordingIden::Id,
    LiveAudioRecordingIden::AudioRecordingId,
    LiveAudioRecordingIden::OwnerId,
    LiveAudioRecordingIden::Locked,
];

const MULTIPART_DEFAULT_COLUMNS: [LiveAudioRecordingMultipartUploadIden; 4] = [
    LiveAudioRecordingMultipartUploadIden::LiveAudioRecordingId,
    LiveAudioRecordingMultipartUploadIden::MultipartUploadId,
    LiveAudioRecordingMultipartUploadIden::NextPartNumber,
    LiveAudioRecordingMultipartUploadIden::UploadedParts,
];

fn not_found() -> ComhairleError {
    ComhairleError::ResourceNotFound("Live audio recording".to_string())
}

fn hydrate_state(
    row: LiveAudioRecording,
    multipart: Option<&LiveAudioRecordingMultipartUpload>,
) -> LiveAudioRecordingState {
    LiveAudioRecordingState {
        id: row.id,
        audio_recording_id: row.audio_recording_id,
        multipart_upload_id: multipart
            .map(|m| m.multipart_upload_id.clone())
            .unwrap_or_default(),
        next_part_number: multipart.map(|m| m.next_part_number).unwrap_or(1),
        uploaded_parts: multipart
            .map(|m| m.uploaded_parts.0.clone())
            .unwrap_or_default(),
        owner_id: row.owner_id,
        locked: row.locked,
    }
}

async fn list_multipart_upload_state_by_recording_ids(
    db: &PgPool,
    live_audio_recording_ids: &[Uuid],
) -> Result<HashMap<Uuid, LiveAudioRecordingMultipartUpload>, ComhairleError> {
    if live_audio_recording_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let (sql, values) = Query::select()
        .columns(MULTIPART_DEFAULT_COLUMNS)
        .from(LiveAudioRecordingMultipartUploadIden::Table)
        .and_where(
            Expr::col(LiveAudioRecordingMultipartUploadIden::LiveAudioRecordingId)
                .is_in(live_audio_recording_ids.to_vec()),
        )
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, LiveAudioRecordingMultipartUpload, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| (row.live_audio_recording_id, row))
        .collect())
}

pub async fn create(
    db: &PgPool,
    payload: &CreateLiveAudioRecording,
) -> Result<LiveAudioRecordingState, ComhairleError> {
    let (sql, values) = Query::insert()
        .into_table(LiveAudioRecordingIden::Table)
        .columns([
            LiveAudioRecordingIden::AudioRecordingId,
            LiveAudioRecordingIden::OwnerId,
            LiveAudioRecordingIden::Locked,
        ])
        .values_panic([
            payload.audio_recording_id.into(),
            payload.owner_id.into(),
            false.into(),
        ])
        .returning(Query::returning().columns(LIVE_DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let row = sqlx::query_as_with::<_, LiveAudioRecording, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(hydrate_state(row, None))
}

pub async fn list_by_event(
    db: &PgPool,
    event_id: Uuid,
) -> Result<Vec<LiveAudioRecordingState>, ComhairleError> {
    let audio_recording_table = Alias::new("audio_recording");
    let (sql, values) = Query::select()
        .column((LiveAudioRecordingIden::Table, LiveAudioRecordingIden::Id))
        .column((
            LiveAudioRecordingIden::Table,
            LiveAudioRecordingIden::AudioRecordingId,
        ))
        .column((
            LiveAudioRecordingIden::Table,
            LiveAudioRecordingIden::OwnerId,
        ))
        .column((
            LiveAudioRecordingIden::Table,
            LiveAudioRecordingIden::Locked,
        ))
        .from(LiveAudioRecordingIden::Table)
        .join(
            JoinType::InnerJoin,
            audio_recording_table.clone(),
            Expr::col((audio_recording_table.clone(), Alias::new("id"))).equals((
                LiveAudioRecordingIden::Table,
                LiveAudioRecordingIden::AudioRecordingId,
            )),
        )
        .and_where(Expr::col((audio_recording_table, Alias::new("event_id"))).eq(event_id))
        .order_by(LiveAudioRecordingIden::Id, Order::Asc)
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, LiveAudioRecording, _>(&sql, values)
        .fetch_all(db)
        .await?;

    let ids = rows.iter().map(|row| row.id).collect::<Vec<_>>();
    let multipart_by_id = list_multipart_upload_state_by_recording_ids(db, &ids).await?;

    Ok(rows
        .into_iter()
        .map(|row| {
            let multipart = multipart_by_id.get(&row.id);
            hydrate_state(row, multipart)
        })
        .collect())
}

pub async fn list_by_event_and_owner(
    db: &PgPool,
    event_id: Uuid,
    owner_user_id: Uuid,
) -> Result<Vec<LiveAudioRecordingState>, ComhairleError> {
    let audio_recording_table = Alias::new("audio_recording");
    let (sql, values) = Query::select()
        .column((LiveAudioRecordingIden::Table, LiveAudioRecordingIden::Id))
        .column((
            LiveAudioRecordingIden::Table,
            LiveAudioRecordingIden::AudioRecordingId,
        ))
        .column((
            LiveAudioRecordingIden::Table,
            LiveAudioRecordingIden::OwnerId,
        ))
        .column((
            LiveAudioRecordingIden::Table,
            LiveAudioRecordingIden::Locked,
        ))
        .from(LiveAudioRecordingIden::Table)
        .join(
            JoinType::InnerJoin,
            audio_recording_table.clone(),
            Expr::col((audio_recording_table.clone(), Alias::new("id"))).equals((
                LiveAudioRecordingIden::Table,
                LiveAudioRecordingIden::AudioRecordingId,
            )),
        )
        .and_where(Expr::col((audio_recording_table, Alias::new("event_id"))).eq(event_id))
        .and_where(Expr::col(LiveAudioRecordingIden::OwnerId).eq(owner_user_id))
        .order_by(LiveAudioRecordingIden::Id, Order::Asc)
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, LiveAudioRecording, _>(&sql, values)
        .fetch_all(db)
        .await?;

    let ids = rows.iter().map(|row| row.id).collect::<Vec<_>>();
    let multipart_by_id = list_multipart_upload_state_by_recording_ids(db, &ids).await?;

    Ok(rows
        .into_iter()
        .map(|row| {
            let multipart = multipart_by_id.get(&row.id);
            hydrate_state(row, multipart)
        })
        .collect())
}

pub async fn get(
    db: &PgPool,
    live_audio_recording_id: Uuid,
) -> Result<LiveAudioRecordingState, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(LIVE_DEFAULT_COLUMNS)
        .from(LiveAudioRecordingIden::Table)
        .and_where(Expr::col(LiveAudioRecordingIden::Id).eq(live_audio_recording_id))
        .build_sqlx(PostgresQueryBuilder);

    let row = sqlx::query_as_with::<_, LiveAudioRecording, _>(&sql, values)
        .fetch_optional(db)
        .await?
        .ok_or_else(not_found)?;

    let multipart = get_multipart_upload_state(db, live_audio_recording_id).await?;
    Ok(hydrate_state(row, multipart.as_ref()))
}

pub async fn create_multipart_upload_state(
    db: &PgPool,
    live_audio_recording_id: Uuid,
    multipart_upload_id: &str,
) -> Result<LiveAudioRecordingMultipartUpload, ComhairleError> {
    let (sql, values) = Query::insert()
        .into_table(LiveAudioRecordingMultipartUploadIden::Table)
        .columns([
            LiveAudioRecordingMultipartUploadIden::LiveAudioRecordingId,
            LiveAudioRecordingMultipartUploadIden::MultipartUploadId,
            LiveAudioRecordingMultipartUploadIden::NextPartNumber,
            LiveAudioRecordingMultipartUploadIden::UploadedParts,
        ])
        .values_panic([
            live_audio_recording_id.into(),
            multipart_upload_id.to_string().into(),
            1.into(),
            json!([]).into(),
        ])
        .on_conflict(OnConflict::new().do_nothing().to_owned())
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(&sql, values).execute(db).await?;

    get_multipart_upload_state(db, live_audio_recording_id)
        .await?
        .ok_or_else(not_found)
}

pub async fn get_multipart_upload_state(
    db: &PgPool,
    live_audio_recording_id: Uuid,
) -> Result<Option<LiveAudioRecordingMultipartUpload>, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(MULTIPART_DEFAULT_COLUMNS)
        .from(LiveAudioRecordingMultipartUploadIden::Table)
        .and_where(
            Expr::col(LiveAudioRecordingMultipartUploadIden::LiveAudioRecordingId)
                .eq(live_audio_recording_id),
        )
        .build_sqlx(PostgresQueryBuilder);

    Ok(
        sqlx::query_as_with::<_, LiveAudioRecordingMultipartUpload, _>(&sql, values)
            .fetch_optional(db)
            .await?,
    )
}

#[instrument(err(Debug), skip(db))]
pub async fn append_uploaded_part(
    db: &PgPool,
    live_audio_recording_id: Uuid,
    uploaded_part: UploadedPart,
    expected_part_number: i32,
) -> Result<LiveAudioRecordingState, ComhairleError> {
    let mut tx = db.begin().await?;

    let (sql, values) = Query::select()
        .columns(MULTIPART_DEFAULT_COLUMNS)
        .from(LiveAudioRecordingMultipartUploadIden::Table)
        .and_where(
            Expr::col(LiveAudioRecordingMultipartUploadIden::LiveAudioRecordingId)
                .eq(live_audio_recording_id),
        )
        .lock(LockType::Update)
        .build_sqlx(PostgresQueryBuilder);

    let current = sqlx::query_as_with::<_, LiveAudioRecordingMultipartUpload, _>(&sql, values)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(not_found)?;

    if current.next_part_number != expected_part_number {
        return Err(ComhairleError::CorruptedData(format!(
            "Expected part_number {}, got {}",
            current.next_part_number, expected_part_number
        )));
    }

    let mut next_uploaded_parts = current.uploaded_parts.0;
    next_uploaded_parts.push(uploaded_part);
    let uploaded_parts_value: Value = json!(next_uploaded_parts);

    let (sql, values) = Query::update()
        .table(LiveAudioRecordingMultipartUploadIden::Table)
        .values([
            (
                LiveAudioRecordingMultipartUploadIden::UploadedParts,
                uploaded_parts_value.into(),
            ),
            (
                LiveAudioRecordingMultipartUploadIden::NextPartNumber,
                (expected_part_number + 1).into(),
            ),
        ])
        .and_where(
            Expr::col(LiveAudioRecordingMultipartUploadIden::LiveAudioRecordingId)
                .eq(live_audio_recording_id),
        )
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(&sql, values).execute(&mut *tx).await?;

    tx.commit().await?;

    get(db, live_audio_recording_id).await
}

#[instrument(err(Debug), skip(db))]
pub async fn delete(
    db: &PgPool,
    live_audio_recording_id: Uuid,
) -> Result<LiveAudioRecordingState, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(LiveAudioRecordingIden::Table)
        .and_where(Expr::col(LiveAudioRecordingIden::Id).eq(live_audio_recording_id))
        .returning(Query::returning().columns(LIVE_DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let deleted = sqlx::query_as_with::<_, LiveAudioRecording, _>(&sql, values)
        .fetch_optional(db)
        .await?
        .ok_or_else(not_found)?;

    Ok(hydrate_state(deleted, None))
}
