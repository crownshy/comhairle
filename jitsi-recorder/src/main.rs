use anyhow::{Context, Result};
use askama::Template;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;
use tracing::{error, info, warn};
use uuid::Uuid;

// ============================================================================
// Data Structures
// ============================================================================


/// Recording session status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum RecordingStatus {
    Recording,
    Uploading,
    Completed,
    Failed,
}

/// Full recording session state (stored in Redis)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RecordingSession {
    session_id: Uuid,
    room_name: String,
    status: RecordingStatus,
    started_at: DateTime<Utc>,
    ended_at: Option<DateTime<Utc>>,
    s3_bucket: String,
    s3_path: String,
    webhook_url: String,
    nickname: String,
    local_file_path: Option<String>,
    s3_url: Option<String>,
    duration_seconds: Option<u64>,
    error_message: Option<String>,
}

/// Request body for starting a new recording
#[derive(Debug, Deserialize)]
struct StartRecordingRequest {
    room_name: String,
    s3_bucket: String,
    s3_path: String,
    webhook_url: String,
    #[serde(default = "default_nickname")]
    nickname: String,
    /// Optional JWT token for Jitsi authentication. If not provided, uses anonymous auth.
    #[serde(default)]
    jwt_token: Option<String>,
}

fn default_nickname() -> String {
    "recorder-bot".to_string()
}

#[derive(Debug, Serialize)]
struct JitsiJwtUser {
    name: String,
    moderator: bool,
}

#[derive(Debug, Serialize)]
struct JitsiJwtContext {
    user: JitsiJwtUser,
}

#[derive(Debug, Serialize)]
struct JitsiJwtClaims {
    aud: String,
    iss: String,
    sub: String,
    room: String,
    nbf: i64,
    exp: i64,
    context: JitsiJwtContext,
}

/// Mints a short-lived JWT for the Prosody-triggered recording path, which has
/// no caller to supply one (unlike the manual `/recordings` API). Returns None
/// if JWT_APP_ID/JWT_APP_SECRET aren't configured, in which case gst-meet falls
/// back to anonymous auth.
fn generate_jwt_token(config: &Config, room_name: &str, nickname: &str) -> Option<String> {
    let app_id = config.jwt_app_id.as_ref()?;
    let app_secret = config.jwt_app_secret.as_ref()?;

    let sub = config
        .jitsi_websocket_url
        .split("://")
        .nth(1)
        .and_then(|rest| rest.split('/').next())
        .unwrap_or(app_id)
        .to_string();

    let now = Utc::now().timestamp();
    let claims = JitsiJwtClaims {
        aud: app_id.clone(),
        iss: app_id.clone(),
        sub,
        room: room_name.to_string(),
        nbf: now,
        exp: now + 3600,
        context: JitsiJwtContext {
            user: JitsiJwtUser {
                name: nickname.to_string(),
                moderator: false,
            },
        },
    };

    match jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(app_secret.as_bytes()),
    ) {
        Ok(token) => Some(token),
        Err(e) => {
            error!("Failed to sign JWT for room {}: {}", room_name, e);
            None
        }
    }
}

/// Response for recording operations
#[derive(Debug, Serialize)]
struct RecordingResponse {
    session_id: Uuid,
    room_name: String,
    status: RecordingStatus,
    started_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ended_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
}

impl From<RecordingSession> for RecordingResponse {
    fn from(session: RecordingSession) -> Self {
        RecordingResponse {
            session_id: session.session_id,
            room_name: session.room_name,
            status: session.status,
            started_at: session.started_at,
            ended_at: session.ended_at,
            s3_url: session.s3_url,
            duration_seconds: session.duration_seconds,
            error_message: session.error_message,
        }
    }
}

/// List of recording sessions
#[derive(Debug, Serialize)]
struct RecordingsListResponse {
    recordings: Vec<RecordingResponse>,
}

/// Webhook notification payload
#[derive(Debug, Serialize)]
struct WebhookPayload {
    session_id: Uuid,
    room_name: String,
    status: RecordingStatus,
    started_at: DateTime<Utc>,
    ended_at: Option<DateTime<Utc>>,
    s3_url: Option<String>,
    duration_seconds: Option<u64>,
}

/// Application state shared across handlers
#[derive(Clone)]
struct AppState {
    redis_client: redis::Client,
    s3_client: aws_sdk_s3::Client,
    http_client: reqwest::Client,
    room_sessions: Arc<RwLock<HashMap<String, (Uuid, u32)>>>,
    config: Arc<Config>,
}

/// Server configuration loaded from environment
#[derive(Debug, Clone)]
struct Config {
    jitsi_websocket_url: String,
    xmpp_domain: String,
    /// gst-meet defaults this to "conference.<xmpp-domain>" when unset, which is
    /// wrong for deployments like this one (actual MUC domain is "muc.meet.jitsi",
    /// not "conference.meet.jitsi") - Jicofo rejects with "not-allowed:
    /// Communication with remote domains is not enabled".
    muc_domain: String,
    gst_meet_path: String,
    recordings_dir: String,
    port: u16,
    default_s3_bucket: String,
    default_webhook_url: String,
    event_sync_token: String,
    /// Both unset => webhook-triggered recordings join anonymously (fails if the
    /// Jitsi deployment requires JWT auth). Both set => a token is minted per
    /// room for the Prosody-triggered path, matching this deployment's
    /// JWT_APP_ID/JWT_APP_SECRET (see comhairle_jitsi values.yml).
    jwt_app_id: Option<String>,
    jwt_app_secret: Option<String>,
}


// ============================================================================
// Redis Operations
// ============================================================================

async fn save_session(redis_client: &redis::Client, session: &RecordingSession) -> Result<()> {
    let mut con = redis_client
        .get_multiplexed_async_connection()
        .await
        .context("Failed to get Redis connection")?;

    let key = format!("recording:{}", session.session_id);
    let value = serde_json::to_string(session).context("Failed to serialize session")?;

    con.set::<_, _, ()>(&key, value)
        .await
        .context("Failed to save session to Redis")?;

    con.sadd::<_, _, ()>("recording:all", session.session_id.to_string())
        .await
        .context("Failed to add session to index")?;

    Ok(())
}

async fn load_session(redis_client: &redis::Client, session_id: Uuid) -> Result<RecordingSession> {
    let mut con = redis_client
        .get_multiplexed_async_connection()
        .await
        .context("Failed to get Redis connection")?;

    let key = format!("recording:{}", session_id);
    let value: String = con
        .get(&key)
        .await
        .context("Failed to get session from Redis")?;

    serde_json::from_str(&value).context("Failed to deserialize session")
}

async fn list_all_sessions(redis_client: &redis::Client) -> Result<Vec<RecordingSession>> {
    let mut con = redis_client
        .get_multiplexed_async_connection()
        .await
        .context("Failed to get Redis connection")?;

    let session_ids: Vec<String> = con
        .smembers("recording:all")
        .await
        .context("Failed to get session list")?;

    let mut sessions = Vec::new();
    for id_str in session_ids {
        if let Ok(session_id) = Uuid::parse_str(&id_str) {
            if let Ok(session) = load_session(redis_client, session_id).await {
                sessions.push(session);
            }
        }
    }

    sessions.sort_by(|a, b| b.started_at.cmp(&a.started_at));
    Ok(sessions)
}

// ============================================================================
// S3 Upload
// ============================================================================

async fn upload_to_s3(
    s3_client: &aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
    file_path: &str,
) -> Result<String> {
    info!("Uploading {} to s3://{}/{}", file_path, bucket, key);

    let body = aws_sdk_s3::primitives::ByteStream::from_path(std::path::Path::new(file_path))
        .await
        .context("Failed to read file for upload")?;

    s3_client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .send()
        .await
        .context("Failed to upload to S3")?;

    let s3_url = format!("s3://{}/{}", bucket, key);
    info!("Successfully uploaded to {}", s3_url);

    Ok(s3_url)
}

// ============================================================================
// Webhook Notification
// ============================================================================

async fn send_webhook(
    http_client: &reqwest::Client,
    webhook_url: &str,
    payload: &WebhookPayload,
) -> Result<()> {
    info!("Sending webhook to {}", webhook_url);

    let response = http_client
        .post(webhook_url)
        .json(payload)
        .send()
        .await
        .context("Failed to send webhook")?;

    if response.status().is_success() {
        info!("Webhook sent successfully");
        Ok(())
    } else {
        warn!("Webhook returned non-success status: {}", response.status());
        Err(anyhow::anyhow!(
            "Webhook failed with status {}",
            response.status()
        ))
    }
}

// ============================================================================
// Recording Management
// ============================================================================

async fn start_gst_meet_process(
    config: &Config,
    room_name: &str,
    nickname: &str,
    output_file: &str,
    jwt_token: Option<&str>,
) -> Result<tokio::process::Child> {
    let recv_pipeline = format!(
        "audiomixer name=audio ! audioconvert ! audioresample ! wavenc ! filesink location={}",
        output_file
    );

    info!("Starting gst-meet for room: {}", room_name);
    info!("Output file: {}", output_file);

    let mut cmd = tokio::process::Command::new(&config.gst_meet_path);
    cmd.arg("--web-socket-url")
        .arg(&config.jitsi_websocket_url)
        .arg("--xmpp-domain")
        .arg(&config.xmpp_domain)
        .arg("--muc-domain")
        .arg(&config.muc_domain)
        .arg("--room-name")
        .arg(room_name)
        .arg("--nick")
        .arg(nickname)
        .arg("--recv-pipeline")
        .arg(&recv_pipeline);

    if let Some(token) = jwt_token {
        cmd.arg("--xmpp-jwt").arg(token);
    }

    // stdout/stderr are never read anywhere in this process. With Stdio::piped()
    // (and --verbose, now dropped) gst-meet fills the kernel pipe buffer and
    // blocks on its next write - freezing the whole process, XMPP pings
    // included, until the server kicks it for inactivity ~60-70s later. This
    // was silently killing every recording a short way in.
    let child = cmd
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .context("Failed to spawn gst-meet process")?;

    info!("gst-meet process started with PID: {:?}", child.id());

    Ok(child)
}

async fn monitor_recording_process(
    state: AppState,
    session_id: Uuid,
    mut child: tokio::process::Child,
) {
    info!("Monitoring recording process for session {}", session_id);

    let start_time = Utc::now();

    let status = child.wait().await;

    let end_time = Utc::now();
    let duration_seconds = (end_time - start_time).num_seconds() as u64;

    info!(
        "Recording process for session {} exited with status: {:?}",
        session_id, status
    );

    let mut session = match load_session(&state.redis_client, session_id).await {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to load session {}: {}", session_id, e);
            return;
        }
    };

    {
        let mut room_sessions = state.room_sessions.write().await;
        if room_sessions.get(&session.room_name).map(|(id, _)| *id) == Some(session_id) {
            room_sessions.remove(&session.room_name);
        }
    }

    session.ended_at = Some(end_time);
    session.duration_seconds = Some(duration_seconds);

    if !status.map(|s| s.success()).unwrap_or(false) {
        error!("Recording process failed for session {}", session_id);
        session.status = RecordingStatus::Failed;
        session.error_message = Some("Recording process exited with non-zero status".to_string());

        if let Err(e) = save_session(&state.redis_client, &session).await {
            error!("Failed to save failed session: {}", e);
        }
        return;
    }

    session.status = RecordingStatus::Uploading;
    if let Err(e) = save_session(&state.redis_client, &session).await {
        error!("Failed to update session status to uploading: {}", e);
    }

    let local_file = session.local_file_path.clone().unwrap();

    let mut retry_count = 0;
    let max_retries = 3;
    let mut upload_success = false;

    while retry_count < max_retries {
        match upload_to_s3(
            &state.s3_client,
            &session.s3_bucket,
            &session.s3_path,
            &local_file,
        )
        .await
        {
            Ok(s3_url) => {
                session.s3_url = Some(s3_url.clone());
                upload_success = true;
                info!("Upload successful on attempt {}", retry_count + 1);
                break;
            }
            Err(e) => {
                retry_count += 1;
                error!("Upload attempt {} failed: {}", retry_count, e);
                if retry_count < max_retries {
                    tokio::time::sleep(tokio::time::Duration::from_secs(2_u64.pow(retry_count)))
                        .await;
                }
            }
        }
    }

    if !upload_success {
        session.status = RecordingStatus::Failed;
        session.error_message = Some(format!(
            "Failed to upload to S3 after {} retries",
            max_retries
        ));
        if let Err(e) = save_session(&state.redis_client, &session).await {
            error!("Failed to save failed session: {}", e);
        }
        return;
    }

    session.status = RecordingStatus::Completed;
    if let Err(e) = save_session(&state.redis_client, &session).await {
        error!("Failed to save completed session: {}", e);
    }

    let webhook_payload = WebhookPayload {
        session_id: session.session_id,
        room_name: session.room_name.clone(),
        status: session.status.clone(),
        started_at: session.started_at,
        ended_at: session.ended_at,
        s3_url: session.s3_url.clone(),
        duration_seconds: session.duration_seconds,
    };

    if let Err(e) = send_webhook(&state.http_client, &session.webhook_url, &webhook_payload).await {
        warn!("Failed to send webhook (non-critical): {}", e);
    }

    if let Err(e) = tokio::fs::remove_file(&local_file).await {
        warn!("Failed to clean up local file {}: {}", local_file, e);
    } else {
        info!("Cleaned up local file: {}", local_file);
    }
}

// ============================================================================
// API Handlers
// ============================================================================

/// Shared recording-start logic used by both the manual `/recordings` API and the
/// Prosody `room/created` webhook.
async fn start_recording_for_room(
    state: &AppState,
    room_name: String,
    nickname: String,
    jwt_token: Option<String>,
    s3_bucket: String,
    s3_path: String,
    webhook_url: String,
) -> Result<RecordingSession> {
    let session_id = Uuid::new_v4();

    let session_dir = format!("{}/{}", state.config.recordings_dir, session_id);
    tokio::fs::create_dir_all(&session_dir)
        .await
        .context("Failed to create recording directory")?;

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("recording_{}_{}.wav", room_name, timestamp);
    let local_file_path = format!("{}/{}", session_dir, filename);

    let session = RecordingSession {
        session_id,
        room_name: room_name.clone(),
        status: RecordingStatus::Recording,
        started_at: Utc::now(),
        ended_at: None,
        s3_bucket,
        s3_path,
        webhook_url,
        nickname: nickname.clone(),
        local_file_path: Some(local_file_path.clone()),
        s3_url: None,
        duration_seconds: None,
        error_message: None,
    };

    save_session(&state.redis_client, &session)
        .await
        .context("Failed to save session")?;

    let child = start_gst_meet_process(
        &state.config,
        &room_name,
        &nickname,
        &local_file_path,
        jwt_token.as_deref(),
    )
    .await
    .context("Failed to start recording")?;

    let pid = child.id().context("gst-meet process has no pid")?;
    state
        .room_sessions
        .write()
        .await
        .insert(room_name, (session_id, pid));

    let state_clone = state.clone();
    tokio::spawn(async move {
        monitor_recording_process(state_clone, session_id, child).await;
    });

    Ok(session)
}

async fn start_recording(
    State(state): State<AppState>,
    Json(req): Json<StartRecordingRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let session = start_recording_for_room(
        &state,
        req.room_name,
        req.nickname,
        req.jwt_token,
        req.s3_bucket,
        req.s3_path,
        req.webhook_url,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response: RecordingResponse = session.into();
    Ok((StatusCode::CREATED, Json(response)))
}

// ============================================================================
// Prosody event_sync Webhooks
// ============================================================================

/// Mirrors the payload sent by Prosody's `mod_event_sync_component.lua`
/// (jitsi-contrib/prosody-plugins). For breakout rooms, `room_name` is always the
/// *main* room's name (the module deliberately reports the parent room there) -
/// `breakout_room_id` holds the breakout room's own local JID part, which is what
/// must actually be joined to record that breakout room.
#[derive(Debug, Deserialize)]
struct RoomEventPayload {
    room_name: String,
    #[serde(default)]
    is_breakout: bool,
    #[serde(default)]
    breakout_room_id: Option<String>,
}

impl RoomEventPayload {
    /// The room name gst-meet should actually join: the breakout room's own name
    /// when this event is for a breakout room, otherwise the main room's name.
    fn target_room_name(&self) -> String {
        if self.is_breakout {
            self.breakout_room_id.clone().unwrap_or_else(|| self.room_name.clone())
        } else {
            self.room_name.clone()
        }
    }
}

fn check_event_sync_token(state: &AppState, headers: &HeaderMap) -> Result<(), (StatusCode, String)> {
    let token = headers
        .get("x-event-sync-token")
        .and_then(|v| v.to_str().ok());

    if token == Some(state.config.event_sync_token.as_str()) {
        Ok(())
    } else {
        Err((StatusCode::UNAUTHORIZED, "Invalid event sync token".to_string()))
    }
}

async fn room_created_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(event): Json<RoomEventPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    check_event_sync_token(&state, &headers)?;

    let room_name = event.target_room_name();
    let nickname = if event.is_breakout {
        info!("Breakout room created: {} (of main room {})", room_name, event.room_name);
        "recorder-bot-breakout".to_string()
    } else {
        info!("Room created: {}", room_name);
        default_nickname()
    };

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let s3_path = format!("recordings/{}/{}.wav", room_name, timestamp);

    let jwt_token = generate_jwt_token(&state.config, &room_name, &nickname);

    start_recording_for_room(
        &state,
        room_name,
        nickname,
        jwt_token,
        state.config.default_s3_bucket.clone(),
        s3_path,
        state.config.default_webhook_url.clone(),
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}

async fn room_destroyed_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(event): Json<RoomEventPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    check_event_sync_token(&state, &headers)?;

    let room_name = event.target_room_name();
    let entry = state.room_sessions.write().await.remove(&room_name);

    let Some((session_id, pid)) = entry else {
        warn!(
            "room/destroyed received for {} but no active recording session found",
            room_name
        );
        return Ok(StatusCode::OK);
    };

    info!(
        "Room destroyed: {} - stopping recording session {} (pid {})",
        room_name, session_id, pid
    );

    if let Err(e) = nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid as i32), nix::sys::signal::Signal::SIGTERM)
    {
        warn!("Failed to send SIGTERM to gst-meet pid {}: {}", pid, e);
    }

    Ok(StatusCode::OK)
}

async fn list_recordings(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let sessions = list_all_sessions(&state.redis_client).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to list sessions: {}", e),
        )
    })?;

    let recordings: Vec<RecordingResponse> = sessions.into_iter().map(|s| s.into()).collect();

    Ok(Json(RecordingsListResponse { recordings }))
}

async fn get_recording(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let session = load_session(&state.redis_client, session_id)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "Recording not found".to_string()))?;

    let response: RecordingResponse = session.into();
    Ok(Json(response))
}

// ============================================================================
// Web UI Templates
// ============================================================================

struct RecordingRow {
    session_id: String,
    room_name: String,
    status: String,
    started_at: String,
    ended_at: String,
    duration: String,
    s3_url: Option<String>,
}

#[derive(Template)]
#[template(path = "recordings.html")]
struct RecordingsTemplate {
    rows: Vec<RecordingRow>,
}

#[derive(Template)]
#[template(path = "new_recording.html")]
struct NewRecordingTemplate;

// ============================================================================
// Web UI Handlers
// ============================================================================

async fn ui_list_recordings(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let sessions = list_all_sessions(&state.redis_client).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to list sessions: {}", e),
        )
    })?;

    let rows = sessions
        .into_iter()
        .map(|s| RecordingRow {
            session_id: s.session_id.to_string(),
            room_name: s.room_name,
            status: match s.status {
                RecordingStatus::Recording => "recording",
                RecordingStatus::Uploading => "uploading",
                RecordingStatus::Completed => "completed",
                RecordingStatus::Failed => "failed",
            }
            .to_string(),
            started_at: s.started_at.format("%Y-%m-%d %H:%M UTC").to_string(),
            ended_at: s
                .ended_at
                .map(|t| t.format("%Y-%m-%d %H:%M UTC").to_string())
                .unwrap_or_else(|| "-".to_string()),
            duration: s
                .duration_seconds
                .map(|d| format!("{}s", d))
                .unwrap_or_else(|| "-".to_string()),
            s3_url: s.s3_url,
        })
        .collect();

    let html = RecordingsTemplate { rows }
        .render()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Html(html))
}

async fn ui_new_recording() -> Result<impl IntoResponse, (StatusCode, String)> {
    let html = NewRecordingTemplate
        .render()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Html(html))
}

// ============================================================================
// Server Setup
// ============================================================================

fn load_config() -> Result<Config> {
    Ok(Config {
        jitsi_websocket_url: std::env::var("JITSI_WEBSOCKET_URL")
            .unwrap_or_else(|_| "wss://jitsi.comhairle.scot/xmpp-websocket".to_string()),
        // The XMPP virtualhost (e.g. "meet.jitsi") is not the public websocket
        // hostname - gst-meet's own default (host part of --web-socket-url) is
        // wrong for deployments like this one and gets a host-unknown stream
        // error from Prosody.
        xmpp_domain: std::env::var("XMPP_DOMAIN").unwrap_or_else(|_| "meet.jitsi".to_string()),
        muc_domain: std::env::var("MUC_DOMAIN").unwrap_or_else(|_| "muc.meet.jitsi".to_string()),
        gst_meet_path: std::env::var("GST_MEET_PATH").unwrap_or_else(|_| "gst-meet".to_string()),
        recordings_dir: std::env::var("RECORDINGS_DIR")
            .unwrap_or_else(|_| "/tmp/jitsi-recordings".to_string()),
        port: std::env::var("PORT")
            .unwrap_or_else(|_| "3001".to_string())
            .parse::<u16>()
            .unwrap_or(3001),
        default_s3_bucket: std::env::var("DEFAULT_S3_BUCKET").unwrap_or_default(),
        default_webhook_url: std::env::var("DEFAULT_WEBHOOK_URL").unwrap_or_default(),
        event_sync_token: std::env::var("EVENT_SYNC_TOKEN")
            .context("EVENT_SYNC_TOKEN must be set to authenticate incoming Prosody room events")?,
        jwt_app_id: std::env::var("JWT_APP_ID").ok(),
        jwt_app_secret: std::env::var("JWT_APP_SECRET").ok(),
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt::init();

    let config = load_config()?;
    info!("Configuration loaded");
    info!("  Jitsi WebSocket URL: {}", config.jitsi_websocket_url);
    info!("  gst-meet path: {}", config.gst_meet_path);
    info!("  Recordings directory: {}", config.recordings_dir);

    tokio::fs::create_dir_all(&config.recordings_dir)
        .await
        .context("Failed to create recordings directory")?;

    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let redis_client =
        redis::Client::open(redis_url.as_str()).context("Failed to create Redis client")?;

    redis_client
        .get_multiplexed_async_connection()
        .await
        .context("Failed to connect to Redis")?;
    info!("Connected to Redis at {}", redis_url);

    let aws_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let s3_client = aws_sdk_s3::Client::new(&aws_config);
    info!("Initialized S3 client");

    let http_client = reqwest::Client::new();

    let port = config.port;

    let state = AppState {
        redis_client,
        s3_client,
        http_client,
        room_sessions: Arc::new(RwLock::new(HashMap::new())),
        config: Arc::new(config),
    };

    let app = Router::new()
        .route("/", get(ui_list_recordings))
        .route("/ui/new", get(ui_new_recording))
        .route("/recordings", post(start_recording))
        .route("/recordings", get(list_recordings))
        .route("/recordings/{session_id}", get(get_recording))
        .route("/events/room/created", post(room_created_webhook))
        .route("/events/room/destroyed", post(room_destroyed_webhook))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
