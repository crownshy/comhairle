use std::sync::Arc;

use crate::{
    error::ComhairleError,
    routes::auth::{RequiredAdminUser, RequiredUser},
    ComhairleState,
};
use aide::{
    axum::{
        routing::{get_with, post_with},
        ApiRouter,
    },
    OperationIo,
};
use axum::{extract::State, routing::get, Json};
use serde::{Deserialize, Serialize};

use schemars::JsonSchema;
use uuid::Uuid;

use super::{
    messages::{NotificationLevel, WebSocketMessage},
    websocket_handler,
};

#[derive(Serialize, OperationIo, JsonSchema)]
struct WebSocketStats {
    total_connections: usize,
    connected_users: Vec<Uuid>,
}

#[derive(Deserialize, OperationIo, JsonSchema)]
struct BroadcastMessage {
    message: String,
    authenticated_only: Option<bool>,
}

#[derive(Deserialize, OperationIo, JsonSchema)]
struct SendToUserMessage {
    user_id: Uuid,
    message: String,
}

#[derive(Serialize, OperationIo, JsonSchema)]
struct BroadcastResponse {
    sent_to: usize,
    message: String,
}

async fn get_websocket_stats(
    State(state): State<Arc<ComhairleState>>,
    _user: RequiredAdminUser,
) -> Json<WebSocketStats> {
    let ws_service = &state.websockets;
    Json(WebSocketStats {
        total_connections: ws_service.get_connection_count(),
        connected_users: ws_service.get_connected_user_ids(),
    })
}

async fn broadcast_message_to_workflow_participants(
    State(state): State<Arc<ComhairleState>>,
    _user: RequiredAdminUser,
    Json(payload): Json<BroadcastMessage>,
) -> Result<Json<BroadcastResponse>, ComhairleError> {
    let ws_service = &state.websockets;
    let message = WebSocketMessage::Broadcast {
        message: payload.message.clone(),
        from_user: None,
    };

    let sent_count = if payload.authenticated_only.unwrap_or(false) {
        ws_service
            .broadcast_to_authenticated_users(&message)
            .await?
    } else {
        ws_service.broadcast_to_all(&message).await?
    };

    Ok(Json(BroadcastResponse {
        sent_to: sent_count,
        message: payload.message,
    }))
}

async fn broadcast_message(
    State(state): State<Arc<ComhairleState>>,
    _user: RequiredAdminUser,
    Json(payload): Json<BroadcastMessage>,
) -> Result<Json<BroadcastResponse>, ComhairleError> {
    let ws_service = &state.websockets;
    let message = WebSocketMessage::Broadcast {
        message: payload.message.clone(),
        from_user: None,
    };

    let sent_count = if payload.authenticated_only.unwrap_or(false) {
        ws_service
            .broadcast_to_authenticated_users(&message)
            .await?
    } else {
        ws_service.broadcast_to_all(&message).await?
    };

    Ok(Json(BroadcastResponse {
        sent_to: sent_count,
        message: payload.message,
    }))
}

async fn send_to_user(
    State(state): State<Arc<ComhairleState>>,
    _user: RequiredAdminUser,
    Json(payload): Json<SendToUserMessage>,
) -> Result<Json<BroadcastResponse>, ComhairleError> {
    let ws_service = &state.websockets;
    let message = WebSocketMessage::Notification {
        title: "Message".to_string(),
        message: payload.message.clone(),
        level: NotificationLevel::Info,
    };

    let sent_count = ws_service.send_to_user(&payload.user_id, &message).await?;

    Ok(Json(BroadcastResponse {
        sent_to: sent_count,
        message: payload.message,
    }))
}

pub fn websocket_routes() -> ApiRouter<Arc<ComhairleState>> {
    ApiRouter::new()
        .route("/", get(websocket_handler))
        .api_route(
            "/stats",
            get_with(get_websocket_stats, |op| {
                op.id("GetWebSocketStats")
                    .summary("Get WebSocket connection statistics")
                    .response::<200, Json<WebSocketStats>>()
            }),
        )
        .api_route(
            "/broadcast",
            post_with(broadcast_message, |op| {
                op.id("BroadcastMessage")
                    .summary("Broadcast a message to all connected clients")
                    .response::<200, Json<BroadcastResponse>>()
            }),
        )
        .api_route(
            "/broadcast/{workflow_id}",
            post_with(broadcast_message_to_workflow_participants, |op| {
                op.id("BroadcastMessageToWorkflowParticipants")
                    .summary(
                        "Broadcast a message to all connected clients participating in a workflow",
                    )
                    .response::<200, Json<BroadcastResponse>>()
            }),
        )
        .api_route(
            "/send",
            post_with(send_to_user, |op| {
                op.id("SendToUser")
                    .summary("Send a message to a specific user")
                    .response::<200, Json<BroadcastResponse>>()
            }),
        )
}
