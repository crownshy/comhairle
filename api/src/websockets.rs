pub mod messages;
pub mod routes;

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        ConnectInfo, State,
    },
    response::Response,
};
use dashmap::DashMap;
use futures_util::{SinkExt, StreamExt};
use messages::{NotificationLevel, WebSocketMessage};
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tracing::{error, info, warn};
use uuid::Uuid;

#[cfg(test)]
use mockall::{automock, predicate::*};

use async_trait::async_trait;

use crate::{
    error::ComhairleError, models::users::User, routes::auth::RequiredUser, ComhairleState,
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
    pub user: User,
    pub addr: SocketAddr,
    pub sender: mpsc::UnboundedSender<Message>,
}

impl WebSocketConnection {
    pub fn new(user: User, addr: SocketAddr) -> (Self, mpsc::UnboundedReceiver<Message>) {
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

pub type ConnectionMap = Arc<DashMap<ConnectionId, WebSocketConnection>>;
pub type UserConnectionMap = Arc<DashMap<Uuid, Vec<ConnectionId>>>;

#[derive(Clone)]

pub struct ComhairleWebSocketService {
    pub connections: ConnectionMap,
    pub user_connections: UserConnectionMap,
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait WebSocketService: Send + Sync {
    fn add_connection(&self, connection: WebSocketConnection);

    fn remove_connection(&self, connection_id: &ConnectionId) -> Option<WebSocketConnection>;

    async fn broadcast_to_all(&self, message: &WebSocketMessage) -> Result<usize, ComhairleError>;

    async fn broadcast_to_authenticated_users(
        &self,
        message: &WebSocketMessage,
    ) -> Result<usize, ComhairleError>;

    async fn send_to_user(
        &self,
        user_id: &Uuid,
        message: &WebSocketMessage,
    ) -> Result<usize, ComhairleError>;

    async fn send_to_connections(
        &self,
        connection_ids: &[ConnectionId],
        message: &WebSocketMessage,
    ) -> Result<usize, ComhairleError>;

    fn get_connection_count(&self) -> usize;

    fn get_user_connection_count(&self, user_id: &Uuid) -> usize;

    fn get_connected_user_ids(&self) -> Vec<Uuid>;
}

#[cfg(test)]
impl MockWebSocketService {
    pub fn base() -> MockWebSocketService {
        let mut websockets = MockWebSocketService::new();
        websockets.expect_add_connection().returning(|_| ());
        websockets.expect_remove_connection().returning(|_| None);
        websockets
            .expect_broadcast_to_all()
            .returning(|_| Box::pin(async move { Ok(0) }));
        websockets
            .expect_broadcast_to_authenticated_users()
            .returning(|_| Box::pin(async move { Ok(0) }));
        websockets
            .expect_send_to_user()
            .returning(|_, _| Box::pin(async move { Ok(0) }));
        websockets
            .expect_send_to_connections()
            .returning(|_, _| Box::pin(async move { Ok(0) }));
        websockets.expect_get_connection_count().returning(|| 0);
        websockets
            .expect_get_user_connection_count()
            .returning(|_| 0);
        websockets
            .expect_get_connected_user_ids()
            .returning(Vec::new);
        websockets
    }
}

impl ComhairleWebSocketService {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(DashMap::new()),
            user_connections: Arc::new(DashMap::new()),
        }
    }
}

#[async_trait]
impl WebSocketService for ComhairleWebSocketService {
    fn add_connection(&self, connection: WebSocketConnection) {
        let connection_id = connection.id.clone();

        let user_id = connection.user.id;
        self.user_connections
            .entry(user_id)
            .or_insert_with(Vec::new)
            .push(connection_id.clone());

        self.connections.insert(connection_id, connection);
    }

    fn remove_connection(&self, connection_id: &ConnectionId) -> Option<WebSocketConnection> {
        if let Some((_, connection)) = self.connections.remove(connection_id) {
            let user_id = connection.user.id;
            if let Some(mut user_connections) = self.user_connections.get_mut(&user_id) {
                user_connections.retain(|id| id.0 != connection_id.0);
                if user_connections.is_empty() {
                    drop(user_connections);
                    self.user_connections.remove(&user_id);
                }
            }
            Some(connection)
        } else {
            None
        }
    }

    async fn broadcast_to_all(&self, message: &WebSocketMessage) -> Result<usize, ComhairleError> {
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

    async fn broadcast_to_authenticated_users(
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

    async fn send_to_user(
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

    async fn send_to_connections(
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

    fn get_connection_count(&self) -> usize {
        self.connections.len()
    }

    fn get_user_connection_count(&self, user_id: &Uuid) -> usize {
        self.user_connections
            .get(user_id)
            .map(|connections| connections.len())
            .unwrap_or(0)
    }

    fn get_connected_user_ids(&self) -> Vec<Uuid> {
        self.user_connections
            .iter()
            .map(|entry| *entry.key())
            .collect()
    }
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
) -> Response {
    info!(
        "WebSocket connection from {}, user: {:?}",
        addr, user.username
    );

    ws.on_upgrade(move |socket| handle_websocket(socket, user, addr, state))
}

async fn handle_websocket(
    socket: WebSocket,
    user: User,
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
    let welcome_msg = WebSocketMessage::Notification {
        title: "Connected".to_string(),
        message: format!("Welcome, {}!", user.username.as_deref().unwrap_or("User")),
        level: NotificationLevel::Success,
    };
    let _ = connection.send_message(&welcome_msg).await;

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
                    WebSocketMessage::UserStartedWorkflowStep { workflow_step_id } => {
                        info!("User started workflow step {}", workflow_step_id);
                    }
                    WebSocketMessage::UserFinishedWorkflowStep { workflow_step_id } => {
                        info!("User finished workflow step {}", workflow_step_id);
                    }
                    WebSocketMessage::UserIdle { workflow_step_id } => {
                        info!("User idle on {workflow_step_id}");
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
