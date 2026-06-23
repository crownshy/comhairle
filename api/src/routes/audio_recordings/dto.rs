use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::audio_recording::{AudioFormat, AudioRecording, AudioRecordingStatus};

/// Data transfer object for an AudioRecording
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioRecordingDto {
    pub id: Uuid,
    pub event_id: Uuid,
    pub breakout_room_ids: Vec<String>,
    pub s3_key_prefix: String,
    pub file_extension: AudioFormat,
    pub status: AudioRecordingStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<AudioRecording> for AudioRecordingDto {
    fn from(recording: AudioRecording) -> Self {
        Self {
            id: recording.id,
            event_id: recording.event_id,
            breakout_room_ids: recording.breakout_room_ids,
            s3_key_prefix: recording.s3_key_prefix,
            file_extension: recording.file_extension,
            status: recording.status,
            created_at: recording.created_at,
            updated_at: recording.updated_at,
        }
    }
}

/// Request body for requesting signed upload URLs
#[cfg_attr(test, derive(Serialize))]
#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequestUploadUrlsRequest {
    /// List of breakout room IDs (empty list means only main room)
    pub breakout_rooms: Vec<String>,
    /// Audio format of the files being uploaded (all files share the same format)
    pub file_extension: AudioFormat,
}

/// Response with signed upload URLs for main and breakout rooms
#[cfg_attr(test, derive(Deserialize))]
#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequestUploadUrlsResponse {
    pub main: String,
    pub breakout_rooms: Vec<(String, String)>,
}

/// Response with signed download URLs for main and breakout rooms
#[cfg_attr(test, derive(Deserialize))]
#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecordingDownloadUrls {
    pub recording_url: String,
    pub transcript_url: String,
    pub report_url: String,
}

/// Signed URL information for downloading recordings, transcripts, and reports for main and breakout rooms.
#[cfg_attr(test, derive(Deserialize))]
#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SignedDownloadUrls {
    pub main: RecordingDownloadUrls,
    pub breakout_rooms: Vec<(String, RecordingDownloadUrls)>,
}
