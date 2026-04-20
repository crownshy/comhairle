pub mod handlers;
pub mod messages;
pub mod routes;
pub mod setup;

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
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[cfg(test)]
use mockall::{automock, predicate::*};

use async_trait::async_trait;

use crate::{
    error::ComhairleError, models::users::User, routes::auth::RequiredUser, ComhairleState,
};

/// Trait for handling domain-specific WebSocket messages.
///
/// Implement this trait to create handlers for specific message domains.
/// Handlers are registered with the WebSocket service and automatically receive
/// messages that match their domain.
///
/// # Message Routing
///
/// Messages are routed to handlers based on their type or event prefix:
/// - `UserStartedWorkflowStep`, `UserFinishedWorkflowStep`, `UserIdle` → domain "workflow"
/// - `Custom { event: "notification:xyz", ... }` → domain "notification"
/// - `Custom { event: "my_domain:xyz", ... }` → domain "my_domain"
///
/// # Example
///
/// ```rust,no_run
/// use async_trait::async_trait;
/// use std::sync::Arc;
/// use comhairle::websockets::{WebSocketMessageHandler, WebSocketConnection};
/// use comhairle::websockets::messages::WebSocketMessage;
/// use comhairle::{ComhairleState, error::ComhairleError};
///
/// pub struct ChatHandler;
///
/// #[async_trait]
/// impl WebSocketMessageHandler for ChatHandler {
///     fn domain(&self) -> &str {
///         "chat"
///     }
///
///     async fn handle_message(
///         &self,
///         message: &WebSocketMessage,
///         connection: &WebSocketConnection,
///         state: &Arc<ComhairleState>,
///     ) -> Result<(), ComhairleError> {
///         match message {
///             WebSocketMessage::Custom { event, data } if event.starts_with("chat:") => {
///                 // Handle chat messages
///                 let response = WebSocketMessage::Custom {
///                     event: "chat:response".to_string(),
///                     data: serde_json::json!({"status": "received"}),
///                 };
///                 connection.send_message(&response).await?;
///             }
///             _ => {}
///         }
///         Ok(())
///     }
/// }
///
/// // Register the handler
/// # let state: Arc<ComhairleState> = unimplemented!();
/// state.websockets.register_handler(Arc::new(ChatHandler));
/// ```
#[async_trait]
pub trait WebSocketMessageHandler: Send + Sync {
    /// Returns the domain/service identifier this handler manages.
    ///
    /// The domain is used to route messages to the appropriate handler.
    /// Common domains include "notification", "workflow", "chat", etc.
    fn domain(&self) -> &str;

    /// Handle an incoming WebSocket message.
    ///
    /// This method is called when a message matching this handler's domain is received.
    /// The handler can:
    /// - Query the database via `state.db`
    /// - Send responses via `connection.send_message()`
    /// - Broadcast to other users via `state.websockets`
    /// - Access user information via `connection.user`
    ///
    /// # Parameters
    ///
    /// - `message`: The parsed WebSocket message
    /// - `connection`: Information about the sender's connection
    /// - `state`: Application state (database, services, etc.)
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the message was handled successfully
    /// - `Err(ComhairleError)` if an error occurred
    async fn handle_message(
        &self,
        message: &WebSocketMessage,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), ComhairleError>;
}

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
pub type HandlerRegistry = Arc<DashMap<String, Arc<dyn WebSocketMessageHandler>>>;

#[derive(Clone)]
pub struct ComhairleWebSocketService {
    pub connections: ConnectionMap,
    pub user_connections: UserConnectionMap,
    pub handlers: HandlerRegistry,
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

    // Handler registry methods
    fn register_handler(&self, handler: Arc<dyn WebSocketMessageHandler>);

    fn unregister_handler(&self, domain: &str) -> Option<Arc<dyn WebSocketMessageHandler>>;

    fn get_handler(&self, domain: &str) -> Option<Arc<dyn WebSocketMessageHandler>>;
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
        websockets.expect_register_handler().returning(|_| ());
        websockets.expect_unregister_handler().returning(|_| None);
        websockets.expect_get_handler().returning(|_| None);
        websockets
    }
}

impl ComhairleWebSocketService {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(DashMap::new()),
            user_connections: Arc::new(DashMap::new()),
            handlers: Arc::new(DashMap::new()),
        }
    }
}

impl Default for ComhairleWebSocketService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WebSocketService for ComhairleWebSocketService {
    fn add_connection(&self, connection: WebSocketConnection) {
        let connection_id = connection.id.clone();

        let user_id = connection.user.id;
        self.user_connections
            .entry(user_id)
            .or_default()
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
            if (connection.send_message(message).await).is_err() {
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
            if (connection.send_message(message).await).is_err() {
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
                if (connection.send_message(message).await).is_err() {
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
                if (connection.send_message(message).await).is_err() {
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

    /// Register a message handler for a specific domain.
    ///
    /// Handlers are routed messages based on their domain. Multiple handlers
    /// cannot be registered for the same domain - the last one wins.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Assuming you have a handler implementation:
    /// let handler = Arc::new(MyHandler::new());
    /// state.websockets.register_handler(handler);
    /// ```
    fn register_handler(&self, handler: Arc<dyn WebSocketMessageHandler>) {
        let domain = handler.domain().to_string();
        info!("Registering WebSocket handler for domain: {}", domain);
        self.handlers.insert(domain, handler);
    }

    /// Unregister a message handler for a specific domain.
    ///
    /// Returns the removed handler if one was registered for this domain.
    fn unregister_handler(&self, domain: &str) -> Option<Arc<dyn WebSocketMessageHandler>> {
        info!("Unregistering WebSocket handler for domain: {}", domain);
        self.handlers.remove(domain).map(|(_, handler)| handler)
    }

    /// Get a handler for a specific domain.
    ///
    /// Returns `None` if no handler is registered for this domain.
    fn get_handler(&self, domain: &str) -> Option<Arc<dyn WebSocketMessageHandler>> {
        self.handlers.get(domain).map(|entry| entry.value().clone())
    }
}

#[instrument(skip(state, ws))]
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
) -> Response {
    info!(
        "WebSocket connection from {}, user: {} (id: {})",
        addr,
        user.username.as_deref().unwrap_or("anonymous"),
        user.id
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
    state: &Arc<ComhairleState>,
) -> Result<(), ComhairleError> {
    match msg {
        Message::Text(text) => {
            if let Ok(ws_message) = serde_json::from_str::<WebSocketMessage>(&text) {
                // Handle core protocol messages
                match &ws_message {
                    WebSocketMessage::Ping { timestamp } => {
                        let pong = WebSocketMessage::Pong {
                            timestamp: *timestamp,
                        };
                        connection.send_message(&pong).await?;
                        return Ok(());
                    }
                    _ => {}
                }

                // Route message to registered handlers based on message type
                let handled = route_to_handler(&ws_message, connection, state).await?;

                if !handled {
                    info!(
                        "Unhandled message from connection {:?}: {:?}",
                        connection.id, ws_message
                    );
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

/// Route a message to the appropriate registered handler based on message type or event prefix.
///
/// # Routing Rules
///
/// - `UserStartedWorkflowStep`, `UserFinishedWorkflowStep`, `UserIdle` → domain "workflow"
/// - `Custom { event: "domain:action", ... }` → extracts "domain" from event prefix
/// - Other message types → not routed (handled by core protocol)
///
/// # Returns
///
/// - `Ok(true)` if a handler was found and executed
/// - `Ok(false)` if no handler was found for this message
/// - `Err(ComhairleError)` if the handler execution failed
async fn route_to_handler(
    message: &WebSocketMessage,
    connection: &WebSocketConnection,
    state: &Arc<ComhairleState>,
) -> Result<bool, ComhairleError> {
    // Determine domain from message type
    let domain = match message {
        WebSocketMessage::UserStartedWorkflowStep { .. }
        | WebSocketMessage::UserFinishedWorkflowStep { .. }
        | WebSocketMessage::UserIdle { .. } => Some("workflow"),
        WebSocketMessage::Custom { event, .. } => {
            // For custom messages, extract domain from event prefix if present
            // Format: "domain:event_name" or just use the event as-is
            event.split(':').next()
        }
        _ => None,
    };

    if let Some(domain) = domain {
        if let Some(handler) = state.websockets.get_handler(domain) {
            handler.handle_message(message, connection, state).await?;
            return Ok(true);
        }
    }

    Ok(false)
}
