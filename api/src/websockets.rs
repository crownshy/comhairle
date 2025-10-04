use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        ConnectInfo, Query, State,
    },
    response::Response,
};
use dashmap::DashMap;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::users::User,
    routes::auth::{validate_jwt, OptionalUser, AUTH_KEY},
    ComhairleState,
};

static NEXT_CONNECTION_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConnectionId(pub usize);

impl ConnectionId {
    fn new() -> Self {
        Self(NEXT_CONNECTION_ID.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Debug, Clone)]
pub struct WebSocketConnection {
    pub id: ConnectionId,
    pub user: Option<User>,
    pub addr: SocketAddr,
    pub sender: mpsc::UnboundedSender<Message>,
}

impl WebSocketConnection {
    pub fn new(user: Option<User>, addr: SocketAddr) -> (Self, mpsc::UnboundedReceiver<Message>) {
        let id = ConnectionId::new();
        let (sender, receiver) = mpsc::unbounded_channel();

        let connection = Self {
            id,
            user,
            addr,
            sender,
        };

        (connection, receiver)
    }

    pub async fn send_message(&self, message: &WebSocketMessage) -> Result<(), ComhairleError> {
        let text = serde_json::to_string(message)
            .map_err(|e| ComhairleError::SerializationError(e.to_string()))?;

        self.sender
            .send(Message::Text(text.into()))
            .map_err(|_| ComhairleError::WebSocketSendError("Connection closed".to_string()))
    }

    pub async fn send_text(&self, text: String) -> Result<(), ComhairleError> {
        self.sender
            .send(Message::Text(text.into()))
            .map_err(|_| ComhairleError::WebSocketSendError("Connection closed".to_string()))
    }

    pub async fn send_binary(&self, data: Vec<u8>) -> Result<(), ComhairleError> {
        self.sender
            .send(Message::Binary(data.into()))
            .map_err(|_| ComhairleError::WebSocketSendError("Connection closed".to_string()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WebSocketMessage {
    #[serde(rename = "ping")]
    Ping { timestamp: u64 },

    #[serde(rename = "pong")]
    Pong { timestamp: u64 },

    #[serde(rename = "notification")]
    Notification {
        title: String,
        message: String,
        level: NotificationLevel,
    },

    #[serde(rename = "user_joined")]
    UserJoined {
        user_id: Uuid,
        username: Option<String>,
    },

    #[serde(rename = "user_left")]
    UserLeft {
        user_id: Uuid,
        username: Option<String>,
    },

    #[serde(rename = "broadcast")]
    Broadcast {
        message: String,
        from_user: Option<Uuid>,
    },

    #[serde(rename = "error")]
    Error { code: String, message: String },

    #[serde(rename = "custom")]
    Custom {
        event: String,
        data: serde_json::Value,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationLevel {
    Info,
    Warning,
    Error,
    Success,
}

pub type ConnectionMap = Arc<DashMap<ConnectionId, WebSocketConnection>>;
pub type UserConnectionMap = Arc<DashMap<Uuid, Vec<ConnectionId>>>;

#[derive(Clone)]
pub struct WebSocketService {
    pub connections: ConnectionMap,
    pub user_connections: UserConnectionMap,
}

impl WebSocketService {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(DashMap::new()),
            user_connections: Arc::new(DashMap::new()),
        }
    }

    pub fn add_connection(&self, connection: WebSocketConnection) {
        let connection_id = connection.id.clone();

        if let Some(user) = &connection.user {
            let user_id = user.id;
            self.user_connections
                .entry(user_id)
                .or_insert_with(Vec::new)
                .push(connection_id.clone());
        }

        self.connections.insert(connection_id, connection);
    }

    pub fn remove_connection(&self, connection_id: &ConnectionId) -> Option<WebSocketConnection> {
        if let Some((_, connection)) = self.connections.remove(connection_id) {
            if let Some(user) = &connection.user {
                let user_id = user.id;
                if let Some(mut user_connections) = self.user_connections.get_mut(&user_id) {
                    user_connections.retain(|id| id.0 != connection_id.0);
                    if user_connections.is_empty() {
                        drop(user_connections);
                        self.user_connections.remove(&user_id);
                    }
                }
            }
            Some(connection)
        } else {
            None
        }
    }

    pub async fn broadcast_to_all(
        &self,
        message: &WebSocketMessage,
    ) -> Result<usize, ComhairleError> {
        let mut sent_count = 0;
        let mut failed_connections = Vec::new();

        for connection_ref in self.connections.iter() {
            let connection = connection_ref.value();
            if let Err(_) = connection.send_message(message).await {
                failed_connections.push(connection.id.clone());
            } else {
                sent_count += 1;
            }
        }

        for failed_id in failed_connections {
            self.remove_connection(&failed_id);
        }

        Ok(sent_count)
    }

    pub async fn broadcast_to_authenticated_users(
        &self,
        message: &WebSocketMessage,
    ) -> Result<usize, ComhairleError> {
        let mut sent_count = 0;
        let mut failed_connections = Vec::new();

        for connection_ref in self.connections.iter() {
            let connection = connection_ref.value();
            if connection.user.is_some() {
                if let Err(_) = connection.send_message(message).await {
                    failed_connections.push(connection.id.clone());
                } else {
                    sent_count += 1;
                }
            }
        }

        for failed_id in failed_connections {
            self.remove_connection(&failed_id);
        }

        Ok(sent_count)
    }

    pub async fn send_to_user(
        &self,
        user_id: &Uuid,
        message: &WebSocketMessage,
    ) -> Result<usize, ComhairleError> {
        let connection_ids = match self.user_connections.get(user_id) {
            Some(ids) => ids.clone(),
            None => return Ok(0),
        };

        let mut sent_count = 0;
        let mut failed_connections = Vec::new();

        for connection_id in &connection_ids {
            if let Some(connection) = self.connections.get(connection_id) {
                if let Err(_) = connection.send_message(message).await {
                    failed_connections.push(connection_id.clone());
                } else {
                    sent_count += 1;
                }
            }
        }

        for failed_id in failed_connections {
            self.remove_connection(&failed_id);
        }

        Ok(sent_count)
    }

    pub async fn send_to_connections(
        &self,
        connection_ids: &[ConnectionId],
        message: &WebSocketMessage,
    ) -> Result<usize, ComhairleError> {
        let mut sent_count = 0;
        let mut failed_connections = Vec::new();

        for connection_id in connection_ids {
            if let Some(connection) = self.connections.get(connection_id) {
                if let Err(_) = connection.send_message(message).await {
                    failed_connections.push(connection_id.clone());
                } else {
                    sent_count += 1;
                }
            }
        }

        for failed_id in failed_connections {
            self.remove_connection(&failed_id);
        }

        Ok(sent_count)
    }

    pub fn get_connection_count(&self) -> usize {
        self.connections.len()
    }

    pub fn get_authenticated_connection_count(&self) -> usize {
        self.connections
            .iter()
            .filter(|conn| conn.user.is_some())
            .count()
    }

    pub fn get_user_connection_count(&self, user_id: &Uuid) -> usize {
        self.user_connections
            .get(user_id)
            .map(|connections| connections.len())
            .unwrap_or(0)
    }

    pub fn get_connected_user_ids(&self) -> Vec<Uuid> {
        self.user_connections
            .iter()
            .map(|entry| *entry.key())
            .collect()
    }
}

#[derive(Deserialize)]
pub struct WebSocketQuery {
    token: Option<String>,
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(query): Query<WebSocketQuery>,
    State(state): State<Arc<ComhairleState>>,
    optional_user: OptionalUser,
) -> Response {
    let user = if let Some(token) = query.token {
        validate_jwt(&state, &token).await.ok()
    } else {
        optional_user.0
    };

    info!(
        "WebSocket connection from {}, user: {:?}",
        addr,
        user.as_ref().map(|u| &u.username)
    );

    ws.on_upgrade(move |socket| handle_websocket(socket, user, addr, state))
}

async fn handle_websocket(
    socket: WebSocket,
    user: Option<User>,
    addr: SocketAddr,
    state: Arc<ComhairleState>,
) {
    let (connection, mut receiver) = WebSocketConnection::new(user.clone(), addr);
    let connection_id = connection.id.clone();

    // Add connection to global state
    state.websockets.add_connection(connection.clone());

    let (mut ws_sender, mut ws_receiver) = socket.split();

    // Spawn task to handle outgoing messages
    let outgoing_task = {
        let connection_id = connection_id.clone();
        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                if let Err(e) = ws_sender.send(message).await {
                    error!(
                        "Failed to send WebSocket message for connection {:?}: {}",
                        connection_id, e
                    );
                    break;
                }
            }
        })
    };

    // Handle incoming messages
    let incoming_task = {
        let connection = connection.clone();
        let state = state.clone();
        let connection_id_for_task = connection_id.clone();
        tokio::spawn(async move {
            while let Some(result) = ws_receiver.next().await {
                match result {
                    Ok(msg) => {
                        if let Err(e) = handle_websocket_message(msg, &connection, &state).await {
                            error!("Error handling WebSocket message: {}", e);
                            let error_msg = WebSocketMessage::Error {
                                code: "MESSAGE_HANDLER_ERROR".to_string(),
                                message: e.to_string(),
                            };
                            let _ = connection.send_message(&error_msg).await;
                        }
                    }
                    Err(e) => {
                        warn!(
                            "WebSocket error for connection {:?}: {}",
                            connection_id_for_task, e
                        );
                        break;
                    }
                }
            }
        })
    };

    // Send welcome message
    if let Some(ref user) = user {
        let welcome_msg = WebSocketMessage::Notification {
            title: "Connected".to_string(),
            message: format!("Welcome, {}!", user.username.as_deref().unwrap_or("User")),
            level: NotificationLevel::Success,
        };
        let _ = connection.send_message(&welcome_msg).await;
    }

    // Wait for either task to complete
    tokio::select! {
        _ = outgoing_task => {
            info!("Outgoing task completed for connection {:?}", connection_id);
        }
        _ = incoming_task => {
            info!("Incoming task completed for connection {:?}", connection_id);
        }
    }

    // Remove connection from global state
    state.websockets.remove_connection(&connection_id);
    info!("WebSocket connection closed: {:?}", connection_id);
}

async fn handle_websocket_message(
    msg: Message,
    connection: &WebSocketConnection,
    _state: &Arc<ComhairleState>,
) -> Result<(), ComhairleError> {
    match msg {
        Message::Text(text) => {
            if let Ok(ws_message) = serde_json::from_str::<WebSocketMessage>(&text) {
                match ws_message {
                    WebSocketMessage::Ping { timestamp } => {
                        let pong = WebSocketMessage::Pong { timestamp };
                        connection.send_message(&pong).await?;
                    }
                    WebSocketMessage::Custom { event, data } => {
                        info!(
                            "Received custom event '{}' from connection {:?}: {}",
                            event, connection.id, data
                        );
                        // Handle custom events here
                    }
                    _ => {
                        info!(
                            "Received message from connection {:?}: {:?}",
                            connection.id, ws_message
                        );
                    }
                }
            } else {
                info!(
                    "Received raw text from connection {:?}: {}",
                    connection.id, text
                );
            }
        }
        Message::Binary(data) => {
            info!(
                "Received binary data from connection {:?}: {} bytes",
                connection.id,
                data.len()
            );
        }
        Message::Ping(data) => {
            connection.sender.send(Message::Pong(data)).map_err(|_| {
                ComhairleError::WebSocketSendError("Failed to send pong".to_string())
            })?;
        }
        Message::Pong(_) => {
            // Handle pong if needed
        }
        Message::Close(_) => {
            info!("Connection {:?} closed", connection.id);
        }
    }

    Ok(())
}

pub mod routes {
    use super::*;
    use crate::routes::auth::RequiredUser;
    use aide::{
        axum::{
            routing::{get_with, post_with},
            ApiRouter,
        },
        OperationIo,
    };
    use axum::{routing::get, Json};
    use serde::{Deserialize, Serialize};

    use schemars::JsonSchema;

    #[derive(Serialize, OperationIo, JsonSchema)]
    struct WebSocketStats {
        total_connections: usize,
        authenticated_connections: usize,
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
        _user: RequiredUser,
    ) -> Json<WebSocketStats> {
        let ws_service = &state.websockets;
        Json(WebSocketStats {
            total_connections: ws_service.get_connection_count(),
            authenticated_connections: ws_service.get_authenticated_connection_count(),
            connected_users: ws_service.get_connected_user_ids(),
        })
    }

    async fn broadcast_message(
        State(state): State<Arc<ComhairleState>>,
        _user: RequiredUser,
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
        _user: RequiredUser,
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
            .route("/ws", get(websocket_handler))
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
                "/send",
                post_with(send_to_user, |op| {
                    op.id("SendToUser")
                        .summary("Send a message to a specific user")
                        .response::<200, Json<BroadcastResponse>>()
                }),
            )
    }
}

