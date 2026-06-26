use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

use crate::error::ComhairleError;

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

    pub fn try_from_extension(extension: &str) -> Result<Self, ComhairleError> {
        match extension.trim_start_matches('.').to_lowercase().as_str() {
            "wav" => Ok(AudioFormat::Wav),
            "mp3" => Ok(AudioFormat::Mp3),
            "m4a" => Ok(AudioFormat::M4a),
            "mp4" => Ok(AudioFormat::Mp4),
            "ogg" | "oga" => Ok(AudioFormat::Ogg),
            "flac" => Ok(AudioFormat::Flac),
            "webm" => Ok(AudioFormat::Webm),
            ext => Err(ComhairleError::UnsupportedContentType(ext.to_string())),
        }
    }
}

impl std::fmt::Display for AudioFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.extension())
    }
}

/// Status of an audio recording's transcription and processing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AudioRecordingStatus {
    /// Recording uploaded, waiting to be processed
    Pending,
    /// Recording has been transcribed and report generated
    Completed,
    /// Recording failed during transcription/processing
    Failed,
}

impl ToString for AudioRecordingStatus {
    fn to_string(&self) -> String {
        match self {
            AudioRecordingStatus::Pending => "pending".to_string(),
            AudioRecordingStatus::Completed => "completed".to_string(),
            AudioRecordingStatus::Failed => "failed".to_string(),
        }
    }
}

/// Parse status from string (handles database TEXT type)
impl AudioRecordingStatus {
    pub fn from_string(s: &str) -> Result<Self, ComhairleError> {
        match s {
            "pending" => Ok(AudioRecordingStatus::Pending),
            "completed" => Ok(AudioRecordingStatus::Completed),
            "failed" => Ok(AudioRecordingStatus::Failed),
            _ => Err(ComhairleError::ResourceNotFound(format!(
                "Unknown status: {}",
                s
            ))),
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
                .unwrap_or(AudioRecordingStatus::Pending),
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
pub async fn create(
    db: &PgPool,
    create_recording: &CreateAudioRecording,
) -> Result<AudioRecording, ComhairleError> {
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
            "pending".into(),
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
                return Err(ComhairleError::DuplicateRecordingName(
                    create_recording.name.clone(),
                ));
            }
            Err(ComhairleError::DatabaseError(sqlx::Error::Database(db_err)))
        }
        Err(e) => Err(ComhairleError::DatabaseError(e)),
    }
}

/// Get an audio recording by ID
pub async fn get_by_id(db: &PgPool, recording_id: &Uuid) -> Result<AudioRecording, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(RawAudioRecordingIden::Table)
        .and_where(Expr::col(RawAudioRecordingIden::Id).eq(*recording_id))
        .build_sqlx(PostgresQueryBuilder);

    let recording = sqlx::query_as_with::<_, RawAudioRecording, _>(&sql, values)
        .fetch_optional(db)
        .await?
        .ok_or(ComhairleError::ResourceNotFound(
            "Audio recording not found".to_string(),
        ))?;

    Ok(recording.into())
}

/// List all recordings for an event, oldest first.
pub async fn list_by_event(
    db: &PgPool,
    event_id: &Uuid,
) -> Result<Vec<AudioRecording>, ComhairleError> {
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

/// Update the status of an audio recording
pub async fn update_status(
    db: &PgPool,
    recording_id: &Uuid,
    status: AudioRecordingStatus,
) -> Result<AudioRecording, ComhairleError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;

    use crate::routes::conversations::dto::ConversationDto;
    use crate::routes::events::dto::EventDto;
    use crate::setup_server;
    use crate::test_helpers::{test_config, test_state, UserSession};

    async fn create_random_event(
        session: &mut UserSession,
        app: &axum::Router,
    ) -> Result<EventDto, Box<dyn std::error::Error>> {
        let conversation_response = session.create_random_conversation(app).await?;
        let conversation: ConversationDto = serde_json::from_value(conversation_response.1)?;
        let conversation_id: String = conversation.id.to_string();

        let event_response = session.create_random_event(app, &conversation_id).await?;
        let event: EventDto = serde_json::from_value(event_response.1)?;

        Ok(event)
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_create_audio_recording(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(test_state().db(pool.clone()).config(config).call()?);
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let event = create_random_event(&mut session, &app).await?;

        let create_req = CreateAudioRecording {
            id: Uuid::new_v4(),
            event_id: event.id,
            name: "Main Room".to_string(),
            s3_key_prefix: "test/prefix".to_string(),
            file_extension: AudioFormat::Wav,
        };

        let recording = create(&pool, &create_req).await?;
        assert_eq!(recording.id, create_req.id);
        assert_eq!(recording.event_id, create_req.event_id);
        assert_eq!(recording.name, create_req.name);
        assert_eq!(recording.s3_key_prefix, create_req.s3_key_prefix);
        assert_eq!(recording.status, AudioRecordingStatus::Pending);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_create_duplicate_name_for_event_conflicts(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(test_state().db(pool.clone()).config(config).call()?);
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let event = create_random_event(&mut session, &app).await?;

        let create_req = CreateAudioRecording {
            id: Uuid::new_v4(),
            event_id: event.id,
            name: "Room A".to_string(),
            s3_key_prefix: "test/prefix".to_string(),
            file_extension: AudioFormat::Wav,
        };

        // First create with this name succeeds.
        create(&pool, &create_req).await?;

        // A second room (distinct id) with the same name in the same event conflicts.
        let err = create(
            &pool,
            &CreateAudioRecording {
                id: Uuid::new_v4(),
                ..create_req.clone()
            },
        )
        .await
        .unwrap_err();
        assert!(matches!(err, ComhairleError::DuplicateRecordingName(_)));

        // A second room with a different name in the same event is allowed.
        create(
            &pool,
            &CreateAudioRecording {
                id: Uuid::new_v4(),
                name: "Room B".to_string(),
                ..create_req.clone()
            },
        )
        .await?;

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_get_by_id(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(test_state().db(pool.clone()).config(config).call()?);
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let event = create_random_event(&mut session, &app).await?;

        let create_req = CreateAudioRecording {
            id: Uuid::new_v4(),
            event_id: event.id,
            name: "Room 1".to_string(),
            s3_key_prefix: "test/prefix".to_string(),
            file_extension: AudioFormat::Wav,
        };

        let created = create(&pool, &create_req).await?;
        let fetched = get_by_id(&pool, &created.id).await?;

        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.event_id, created.event_id);
        assert_eq!(fetched.s3_key_prefix, created.s3_key_prefix);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_list_by_event(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(test_state().db(pool.clone()).config(config).call()?);
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let event = create_random_event(&mut session, &app).await?;

        let room_a = create(
            &pool,
            &CreateAudioRecording {
                id: Uuid::new_v4(),
                event_id: event.id,
                name: "Room A".to_string(),
                s3_key_prefix: "test/prefix/a".to_string(),
                file_extension: AudioFormat::Wav,
            },
        )
        .await?;
        let room_b = create(
            &pool,
            &CreateAudioRecording {
                id: Uuid::new_v4(),
                event_id: event.id,
                name: "Room B".to_string(),
                s3_key_prefix: "test/prefix/b".to_string(),
                file_extension: AudioFormat::Wav,
            },
        )
        .await?;

        let rooms = list_by_event(&pool, &event.id).await?;
        assert_eq!(rooms.len(), 2);
        // Ordered oldest first.
        assert_eq!(rooms[0].id, room_a.id);
        assert_eq!(rooms[1].id, room_b.id);

        // An event with no rooms returns an empty list (not an error).
        let other_event = create_random_event(&mut session, &app).await?;
        assert!(list_by_event(&pool, &other_event.id).await?.is_empty());

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_update_status(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(test_state().db(pool.clone()).config(config).call()?);
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let event = create_random_event(&mut session, &app).await?;

        let create_req = CreateAudioRecording {
            id: Uuid::new_v4(),
            event_id: event.id,
            name: "Room 1".to_string(),
            s3_key_prefix: "test/prefix".to_string(),
            file_extension: AudioFormat::Wav,
        };

        let created = create(&pool, &create_req).await?;
        assert_eq!(created.status, AudioRecordingStatus::Pending);

        let updated = update_status(&pool, &created.id, AudioRecordingStatus::Completed).await?;
        assert_eq!(updated.status, AudioRecordingStatus::Completed);
        assert!(updated.updated_at > created.updated_at);
        assert!(updated.created_at == created.created_at);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_get_by_id_not_found(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(test_state().db(pool.clone()).config(config).call()?);
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let _event = create_random_event(&mut session, &app).await?;

        let nonexistent_id = uuid::Uuid::new_v4();
        let result = get_by_id(&pool, &nonexistent_id).await;
        assert!(result.is_err());
        Ok(())
    }
}
