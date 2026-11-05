//! Triggers the jitsi-recorder service directly from comhairle-api's own live
//! conversation state (see `websockets::handlers::video_call`), instead of
//! relying on Prosody's event_sync webhook. That webhook has exactly one
//! configurable target, so it can never address a specific tenant's API when
//! one Jitsi cluster serves many comhairle-api deployments.

use std::sync::Arc;

use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Serialize;
use uuid::Uuid;

use crate::ComhairleState;
use crate::config::VideoCallConfig;
use crate::error::ComhairleError;
use crate::models::audio_recording::{self, AudioFormat, CreateAudioRecording};
use crate::routes::auth::derive_recorder_webhook_secret;

const BOT_NAME: &str = "recorder-bot";

#[derive(Serialize)]
struct BotJwtClaims<'a> {
    iss: &'a str,
    aud: &'a str,
    sub: &'a str,
    room: &'a str,
    nbf: i64,
    exp: i64,
    context: BotJwtContext<'a>,
}

#[derive(Serialize)]
struct BotJwtContext<'a> {
    user: BotJwtUser<'a>,
}

#[derive(Serialize)]
struct BotJwtUser<'a> {
    name: &'a str,
    id: &'a str,
    moderator: bool,
}

/// Mints a short-lived Jitsi JWT for the recorder bot, non-moderator, scoped
/// to a single room. Mirrors the claims shape `routes::events::get_jwt`
/// builds for human participants, minus the comhairle `User`-centric wrapper
/// that call reuses (`generate_jwt`) — the bot has no comhairle user row.
fn mint_bot_jwt(config: &VideoCallConfig, room: &str) -> Result<String, ComhairleError> {
    let now = Utc::now().timestamp();
    let claims = BotJwtClaims {
        iss: &config.jwt_app_id,
        aud: &config.jwt_app_id,
        sub: &config.jwt_sub,
        room,
        nbf: now,
        // Comfortably longer than any single recording session.
        exp: now + 60 * 60 * 6,
        context: BotJwtContext {
            user: BotJwtUser {
                name: BOT_NAME,
                id: BOT_NAME,
                moderator: false,
            },
        },
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_app_secret.as_bytes()),
    )
    .map_err(|e| ComhairleError::AuthJWTError(e.to_string()))
}

#[derive(Serialize)]
struct StartRecordingRequest<'a> {
    room_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    muc_domain: Option<&'a str>,
    webhook_url: String,
    webhook_secret: String,
    nickname: &'a str,
    jwt_token: String,
}

/// Starts a jitsi-recorder session for `room_name`.
///
/// For the main room, `room_name` is the event's bare `video_meeting_id` and
/// `muc_domain` is the deployment's fixed main MUC domain
/// (`jitsi_recorder.main_muc_domain`). For a breakout room, both come from
/// splitting the real JID the frontend reported over the live-call
/// WebSocket (breakout rooms live under their own MUC component, which the
/// browser already knows and the API otherwise never sees).
///
/// Creates the `audio_recording` row up front, in the same shape/location
/// `routes::audio_recordings::create_recording` uses for a manually-uploaded
/// recording, so the recorder's completion webhook
/// (`routes::audio_recordings::recorder_webhook`) has a row to fill in and
/// the result flows through the existing transcription/categorization
/// pipeline unchanged.
pub async fn trigger_recording(
    state: &Arc<ComhairleState>,
    conversation_id: Uuid,
    event_id: Uuid,
    room_name: &str,
    muc_domain: Option<&str>,
    recording_name: &str,
) -> Result<(), ComhairleError> {
    let recorder_config = state.required_jitsi_recorder_config()?;
    let video_call_config = state
        .config
        .video_call_service
        .as_ref()
        .ok_or(ComhairleError::NoVideoServiceConfigured)?;

    let recording_id = Uuid::new_v4();
    let s3_key_prefix = format!("events/{event_id}/recordings/{recording_id}");

    audio_recording::create(
        &state.db,
        &CreateAudioRecording {
            id: recording_id,
            event_id,
            name: recording_name.to_string(),
            s3_key_prefix,
            file_extension: AudioFormat::Wav,
        },
    )
    .await?;

    let jwt_token = mint_bot_jwt(video_call_config, room_name)?;
    let webhook_secret =
        derive_recorder_webhook_secret(&recorder_config.webhook_secret, &recording_id)?;
    let webhook_url = format!(
        "{}/api/conversation/{conversation_id}/events/{event_id}/audio_recordings/{recording_id}/recorder_webhook",
        state.config.domain
    );

    let request = StartRecordingRequest {
        room_name,
        muc_domain,
        webhook_url,
        webhook_secret,
        nickname: BOT_NAME,
        jwt_token,
    };

    let response = reqwest::Client::new()
        .post(format!("{}/recordings", recorder_config.base_url))
        .json(&request)
        .send()
        .await
        .map_err(|e| ComhairleError::JitsiRecorderError(e.to_string()))?;

    if !response.status().is_success() {
        return Err(ComhairleError::JitsiRecorderError(format!(
            "jitsi-recorder returned {}",
            response.status()
        )));
    }

    Ok(())
}
