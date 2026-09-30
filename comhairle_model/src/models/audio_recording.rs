use crate::models::error::ModelError;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use tracing::instrument;
use uuid::Uuid;

use crate::models::error::DataError;
use crate::models::error::EventError;
use crate::models::error::ValidationError;

/// Audio format of an uploaded recording. Stored in the database as the
/// lowercase file extension so it doubles as the on-disk/S3 suffix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AudioFormat {
    Wav,
    Mp3,
    M4a,
    Mp4,
    Ogg,
    Flac,
    Webm,
}

impl AudioFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            AudioFormat::Wav => "wav",
            AudioFormat::Mp3 => "mp3",
            AudioFormat::M4a => "m4a",
            AudioFormat::Mp4 => "mp4",
            AudioFormat::Ogg => "ogg",
            AudioFormat::Flac => "flac",
            AudioFormat::Webm => "webm",
        }
    }

    pub fn try_from_extension(extension: &str) -> Result<Self, ModelError> {
        match extension.trim_start_matches('.').to_lowercase().as_str() {
            "wav" => Ok(AudioFormat::Wav),
            "mp3" => Ok(AudioFormat::Mp3),
            "m4a" => Ok(AudioFormat::M4a),
            "mp4" => Ok(AudioFormat::Mp4),
            "ogg" | "oga" => Ok(AudioFormat::Ogg),
            "flac" => Ok(AudioFormat::Flac),
            "webm" => Ok(AudioFormat::Webm),
            ext => Err(ValidationError::UnsupportedContentType(ext.to_string()).into()),
        }
    }
}

impl std::fmt::Display for AudioFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.extension())
    }
}

/// Status of an audio recording as it moves through the transcription and
/// categorization pipeline. Each name describes what is happening *at* that
/// state, so the natural flow reads:
/// `AwaitingUpload → Transcribing → Categorizing → Complete`
/// with terminal failure states branching off each processing stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AudioRecordingStatus {
    /// Row created; the file is being (or has yet to be) uploaded to bulk storage.
    AwaitingUpload,
    /// File received; transcription job has been enqueued or is running.
    Transcribing,
    /// Transcript produced and submitted to the categorization service; the
    /// report has not arrived yet.
    Categorizing,
    /// Both the transcript and the categorization report are available.
    Complete,
    /// The transcription stage failed.
    TranscriptionFailed,
    /// The categorization stage failed.
    CategorizationFailed,
}

impl ToString for AudioRecordingStatus {
    fn to_string(&self) -> String {
        match self {
            AudioRecordingStatus::AwaitingUpload => "awaiting_upload".to_string(),
            AudioRecordingStatus::Transcribing => "transcribing".to_string(),
            AudioRecordingStatus::Categorizing => "categorizing".to_string(),
            AudioRecordingStatus::Complete => "complete".to_string(),
            AudioRecordingStatus::TranscriptionFailed => "transcription_failed".to_string(),
            AudioRecordingStatus::CategorizationFailed => "categorization_failed".to_string(),
        }
    }
}

/// Parse status from string (handles database TEXT type)
impl AudioRecordingStatus {
    pub fn from_string(s: &str) -> Result<Self, ModelError> {
        match s {
            "awaiting_upload" => Ok(AudioRecordingStatus::AwaitingUpload),
            "transcribing" => Ok(AudioRecordingStatus::Transcribing),
            "categorizing" => Ok(AudioRecordingStatus::Categorizing),
            "complete" => Ok(AudioRecordingStatus::Complete),
            "transcription_failed" => Ok(AudioRecordingStatus::TranscriptionFailed),
            "categorization_failed" => Ok(AudioRecordingStatus::CategorizationFailed),
            _ => Err(DataError::ResourceNotFound(format!("Unknown status: {}", s)).into()),
        }
    }
}

/// An audio recording for a single named room within an event.
///
/// An event may have many recordings, each with a name (or "room name") that is
/// unique within that event. The status tracks transcription and report
/// generation for this recording only.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AudioRecording {
    /// Unique identifier for this recordings's recording
    pub id: Uuid,
    /// Event this recording belongs to
    pub event_id: Uuid,
    /// User-supplied recording/room name, unique within the event
    pub name: String,
    /// S3 key prefix (without extension) used for generating URLs
    pub s3_key_prefix: String,
    /// Audio format of the uploaded recording
    pub file_extension: AudioFormat,
    /// Current status of transcription & report processing for this recording
    pub status: AudioRecordingStatus,
    /// When this recording was created
    pub created_at: DateTime<Utc>,
    /// When this recording's status was last updated
    pub updated_at: DateTime<Utc>,
}

/// Intermediate struct for database queries (with enum_def for sea_query)
#[derive(Debug, FromRow, Clone)]
#[enum_def(table_name = "audio_recording")]
pub struct RawAudioRecording {
    pub id: Uuid,
    pub event_id: Uuid,
    pub name: String,
    pub s3_key_prefix: String,
    pub file_extension: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [RawAudioRecordingIden; 8] = [
    RawAudioRecordingIden::Id,
    RawAudioRecordingIden::EventId,
    RawAudioRecordingIden::Name,
    RawAudioRecordingIden::S3KeyPrefix,
    RawAudioRecordingIden::FileExtension,
    RawAudioRecordingIden::Status,
    RawAudioRecordingIden::CreatedAt,
    RawAudioRecordingIden::UpdatedAt,
];

impl From<RawAudioRecording> for AudioRecording {
    fn from(raw: RawAudioRecording) -> Self {
        Self {
            id: raw.id,
            event_id: raw.event_id,
            name: raw.name,
            s3_key_prefix: raw.s3_key_prefix,
            file_extension: AudioFormat::try_from_extension(&raw.file_extension)
                .unwrap_or(AudioFormat::Wav),
            status: AudioRecordingStatus::from_string(&raw.status)
                .unwrap_or(AudioRecordingStatus::AwaitingUpload),
            created_at: raw.created_at,
            updated_at: raw.updated_at,
        }
    }
}

/// Request to create a new audio recording record
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CreateAudioRecording {
    pub id: Uuid,
    pub event_id: Uuid,
    pub name: String,
    pub s3_key_prefix: String,
    pub file_extension: AudioFormat,
}

/// Create a new audio recording in the database.
#[instrument(err(Debug), skip(db))]
pub async fn create(
    db: &PgPool,
    create_recording: &CreateAudioRecording,
) -> Result<AudioRecording, ModelError> {
    let (sql, values) = Query::insert()
        .into_table(RawAudioRecordingIden::Table)
        .columns([
            RawAudioRecordingIden::Id,
            RawAudioRecordingIden::EventId,
            RawAudioRecordingIden::Name,
            RawAudioRecordingIden::S3KeyPrefix,
            RawAudioRecordingIden::FileExtension,
            RawAudioRecordingIden::Status,
        ])
        .values([
            create_recording.id.into(),
            create_recording.event_id.into(),
            create_recording.name.clone().into(),
            create_recording.s3_key_prefix.clone().into(),
            create_recording.file_extension.extension().into(),
            AudioRecordingStatus::AwaitingUpload.to_string().into(),
        ])
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    match sqlx::query_as_with::<_, RawAudioRecording, _>(&sql, values)
        .fetch_one(db)
        .await
    {
        Ok(recording) => Ok(recording.into()),
        Err(sqlx::Error::Database(db_err)) => {
            let pg_err = db_err.downcast_ref::<sqlx::postgres::PgDatabaseError>();
            if pg_err.code() == "23505" {
                return Err(
                    EventError::DuplicateRecordingName(create_recording.name.clone()).into(),
                );
            }
            Err(DataError::DatabaseError(sqlx::Error::Database(db_err)).into())
        }
        Err(e) => Err(DataError::DatabaseError(e).into()),
    }
}

/// Get an audio recording by ID
#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, recording_id: &Uuid) -> Result<AudioRecording, ModelError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(RawAudioRecordingIden::Table)
        .and_where(Expr::col(RawAudioRecordingIden::Id).eq(*recording_id))
        .build_sqlx(PostgresQueryBuilder);

    let recording = sqlx::query_as_with::<_, RawAudioRecording, _>(&sql, values)
        .fetch_optional(db)
        .await?
        .ok_or(DataError::ResourceNotFound(
            "Audio recording not found".to_string(),
        ))?;

    Ok(recording.into())
}

/// Get an audio recording by ID, scoped to the event it must belong to.
///
/// Returns [`DataError::ResourceNotFound`] if no recording with that id
/// exists for the given event.
#[instrument(err(Debug), skip(db))]
pub async fn get_by_id_and_event(
    db: &PgPool,
    recording_id: &Uuid,
    event_id: &Uuid,
) -> Result<AudioRecording, ModelError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(RawAudioRecordingIden::Table)
        .and_where(Expr::col(RawAudioRecordingIden::Id).eq(*recording_id))
        .and_where(Expr::col(RawAudioRecordingIden::EventId).eq(*event_id))
        .build_sqlx(PostgresQueryBuilder);

    let recording = sqlx::query_as_with::<_, RawAudioRecording, _>(&sql, values)
        .fetch_optional(db)
        .await?
        .ok_or(DataError::ResourceNotFound(
            "Audio recording not found".to_string(),
        ))?;

    Ok(recording.into())
}

/// List all recordings for an event, oldest first.
#[instrument(err(Debug), skip(db))]
pub async fn list_by_event(
    db: &PgPool,
    event_id: &Uuid,
) -> Result<Vec<AudioRecording>, ModelError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(RawAudioRecordingIden::Table)
        .and_where(Expr::col(RawAudioRecordingIden::EventId).eq(*event_id))
        .order_by(RawAudioRecordingIden::CreatedAt, sea_query::Order::Asc)
        .build_sqlx(PostgresQueryBuilder);

    let recordings = sqlx::query_as_with::<_, RawAudioRecording, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(recordings.into_iter().map(Into::into).collect())
}

/// Delete an audio recording scoped to its event.
///
/// Returns [`DataError::ResourceNotFound`] if no recording with that id
/// exists for the given event.
#[instrument(err(Debug), skip(db))]
pub async fn delete(
    db: &PgPool,
    recording_id: &Uuid,
    event_id: &Uuid,
) -> Result<AudioRecording, ModelError> {
    let (sql, values) = Query::delete()
        .from_table(RawAudioRecordingIden::Table)
        .and_where(Expr::col(RawAudioRecordingIden::Id).eq(*recording_id))
        .and_where(Expr::col(RawAudioRecordingIden::EventId).eq(*event_id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let recording = sqlx::query_as_with::<_, RawAudioRecording, _>(&sql, values)
        .fetch_optional(db)
        .await?
        .ok_or(DataError::ResourceNotFound(
            "Audio recording not found".to_string(),
        ))?;

    Ok(recording.into())
}

/// Update the status of an audio recording
#[instrument(err(Debug), skip(db))]
pub async fn update_status(
    db: &PgPool,
    recording_id: &Uuid,
    status: AudioRecordingStatus,
) -> Result<AudioRecording, ModelError> {
    let (sql, values) = Query::update()
        .table(RawAudioRecordingIden::Table)
        .value(RawAudioRecordingIden::Status, status.to_string())
        .value(RawAudioRecordingIden::UpdatedAt, Expr::cust("NOW()"))
        .and_where(Expr::col(RawAudioRecordingIden::Id).eq(*recording_id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let recording = sqlx::query_as_with::<_, RawAudioRecording, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(recording.into())
}
