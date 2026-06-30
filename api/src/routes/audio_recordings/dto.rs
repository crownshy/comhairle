use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::audio_recording::{AudioFormat, AudioRecording, AudioRecordingStatus};

/// Data transfer object for an audio recording.
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

/// Request body for creating an audio recording and requesting its upload URL.
#[cfg_attr(test, derive(Serialize))]
#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecordingRequest {
    /// Name for the recording, unique within the event.
    pub name: String,
    /// Audio format of the file being uploaded.
    pub file_extension: AudioFormat,
}

/// Response after creating an audio recording: the created recording plus a presigned upload URL.
#[cfg_attr(test, derive(Deserialize))]
#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecordingResponse {
    pub recording: AudioRecordingDto,
    pub upload_url: String,
}

/// Signed URLs for downloading a recording's audio, transcript, and report.
#[cfg_attr(test, derive(Deserialize))]
#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecordingDownloadUrls {
    pub recording_url: String,
    pub transcript_url: String,
    pub report_url: String,
}

/// A recording's details together with its signed download URLs.
#[cfg_attr(test, derive(Deserialize))]
#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecordingDetailResponse {
    pub recording: AudioRecordingDto,
    pub downloads: RecordingDownloadUrls,
}

/// Response after enqueuing processing for a recording.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProcessRecordingResponse {
    pub message: String,
    pub job_id: Uuid,
}

/// Response after deleting a recording.
#[cfg_attr(test, derive(Deserialize))]
#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRecordingResponse {
    pub recording: AudioRecordingDto,
}

/// Response after a categorization report is stored.
#[cfg_attr(test, derive(Deserialize))]
#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmitReportResponse {
    pub url: String,
    pub success: bool,
}
