use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::audio_recording::{AudioFormat, AudioRecording, AudioRecordingStatus};

/// Data transfer object for a room's audio recording.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioRecordingDto {
    pub id: Uuid,
    pub event_id: Uuid,
    pub name: String,
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
            name: recording.name,
            s3_key_prefix: recording.s3_key_prefix,
            file_extension: recording.file_extension,
            status: recording.status,
            created_at: recording.created_at,
            updated_at: recording.updated_at,
        }
    }
}

/// Request body for creating a room and requesting its upload URL.
#[cfg_attr(test, derive(Serialize))]
#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoomRequest {
    /// Name for the room, unique within the event.
    pub name: String,
    /// Audio format of the file being uploaded.
    pub file_extension: AudioFormat,
}

/// Response after creating a room: the created room plus a presigned upload URL.
#[cfg_attr(test, derive(Deserialize))]
#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoomResponse {
    pub room: AudioRecordingDto,
    pub upload_url: String,
}

/// Signed URLs for downloading a room's recording, transcript, and report.
#[cfg_attr(test, derive(Deserialize))]
#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecordingDownloadUrls {
    pub recording_url: String,
    pub transcript_url: String,
    pub report_url: String,
}

/// A room's details together with its signed download URLs.
#[cfg_attr(test, derive(Deserialize))]
#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoomDetailResponse {
    pub room: AudioRecordingDto,
    pub downloads: RecordingDownloadUrls,
}

/// Response after enqueuing processing for a room.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProcessRoomResponse {
    pub message: String,
    pub job_id: Uuid,
}

/// Response after a categorization report is stored.
#[cfg_attr(test, derive(Deserialize))]
#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmitReportResponse {
    pub url: String,
    pub success: bool,
}
