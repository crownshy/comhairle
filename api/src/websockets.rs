pub mod config;
pub mod error;
pub mod handlers;
pub mod messages;
pub mod routes;
pub mod setup;

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use axum::{
    extract::{
        ConnectInfo, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
};
use dashmap::DashMap;
use futures_util::{SinkExt, StreamExt};
use messages::{NotificationLevel, WebSocketMessage};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[cfg(test)]
use mockall::{automock, predicate::*};

use async_trait::async_trait;

use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::models::users::User;
use crate::routes::auth::RequiredUser;
use crate::websockets::config::WebsocketConfig;

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
/// use comhairle::websockets::error::WebsocketError;
/// use comhairle::ComhairleState;
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
///     ) -> Result<(), WebsocketError> {
///         match message {
///             WebSocketMessage::Custom { event, data } if event.starts_with("chat:") => {
///                 // Handle chat messages
///                 let response = WebSocketMessage::Custom {
///                     event: "chat:response".to_string(),
///                     data: serde_json::json!({"status": "received"}),
///                 };
///                 // Ignore send errors in this example
///                 let _ = connection.send_message(&response).await;
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
    /// - `Err(WebsocketError)` if an error occurred
    ///
    /// # Implementation Note
    ///
    /// Each handler can define its own error type and convert it to WebsocketError
    /// using the `?` operator or `.map_err(Into::into)`, since each handler's error
    /// type implements `Into<WebsocketError>` via the `#[from]` attribute.
    async fn handle_message(
        &self,
        message: &WebSocketMessage,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), crate::websockets::error::WebsocketError>;

    /// Local user IDs this handler considers members of `room_id`, used for
    /// cluster-wide fan-out via [`WebSocketService::broadcast_to_room`].
    ///
    /// "Local" means only users whose connection this instance is aware of; each
    /// instance answers for its own members. Default: the handler manages no rooms.
    fn local_room_members(&self, _room_id: &Uuid) -> Vec<Uuid> {
        Vec::new()
    }
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

#[derive(Clone, Debug)]
struct ComhairleWebSocketServiceRedis {
    redis_publisher: redis::aio::MultiplexedConnection,
    redis_url: String,
    instance_id: Uuid,
}

#[derive(Clone)]
pub struct ComhairleWebSocketService {
    pub connections: ConnectionMap,
    pub user_connections: UserConnectionMap,
    pub handlers: HandlerRegistry,
    redis_service: Option<ComhairleWebSocketServiceRedis>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WebsocketPubSubMessage {
    Broadcast {
        sender_id: Uuid,
        message: serde_json::Value,
        authenticated_only: bool,
    },
    SendToUser {
        sender_id: Uuid,
        user_id: String,
        message: serde_json::Value,
    },
    SendToConnections {
        sender_id: Uuid,
        connection_ids: Vec<usize>,
        message: serde_json::Value,
    },
    /// Deliver `message` to the members of `room_id`, as reported by the handler
    /// registered for `domain`. Each instance resolves its OWN local members, so
    /// participants spread across instances all receive the message.
    SendToRoom {
        sender_id: Uuid,
        domain: String,
        room_id: String,
        message: serde_json::Value,
    },
}

impl WebsocketPubSubMessage {
    pub fn to_websocket_message(&self) -> Result<WebSocketMessage, ComhairleError> {
        let message: serde_json::Value = match self {
            WebsocketPubSubMessage::Broadcast { message, .. } => message.clone(),
            WebsocketPubSubMessage::SendToUser { message, .. } => message.clone(),
            WebsocketPubSubMessage::SendToConnections { message, .. } => message.clone(),
            WebsocketPubSubMessage::SendToRoom { message, .. } => message.clone(),
        };
        serde_json::from_value(message)
            .map_err(|e| ComhairleError::DeserializationError(e.to_string()))
    }
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

    /// Send `message` to every member of `room_id` across ALL instances.
    ///
    /// Membership is resolved per-instance by the handler registered for `domain`
    /// (via [`WebSocketMessageHandler::local_room_members`]), so participants
    /// connected to different instances all receive the message.
    async fn broadcast_to_room(
        &self,
        domain: &str,
        room_id: &Uuid,
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
        websockets
            .expect_broadcast_to_room()
            .returning(|_, _, _| Box::pin(async move { Ok(0) }));
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
    pub async fn new(config: Option<&WebsocketConfig>) -> Result<Self, ComhairleError> {
        let redis_service = if let Some(config) = &config {
            let client = redis::Client::open(config.redis_pubsub_url.as_str())
                .map_err(|e| ComhairleError::RedisError(e.to_string()))?;

            // Get a multiplexed connection for PUBLISHING
            let redis_publisher = client
                .get_multiplexed_async_connection()
                .await
                .map_err(|e| ComhairleError::RedisError(e.to_string()))?;

            Some(ComhairleWebSocketServiceRedis {
                redis_publisher,
                redis_url: config.redis_pubsub_url.clone(),
                instance_id: Uuid::new_v4(),
            })
        } else {
            None
        };

        Ok(Self {
            connections: Arc::new(DashMap::new()),
            user_connections: Arc::new(DashMap::new()),
            handlers: Arc::new(DashMap::new()),
            redis_service,
        })
    }

    pub async fn start_pubsub_subscriber(
        &self,
    ) -> Result<tokio::task::JoinHandle<()>, ComhairleError> {
        if self.redis_service.is_none() {
            return Err(ComhairleError::RedisError(
                "Redis pub/sub is not configured".to_string(),
            ));
        }
        let redis_service = self.redis_service.as_ref().unwrap();
        // Create a DEDICATED connection just for listening
        let client = redis::Client::open(redis_service.redis_url.as_str())
            .map_err(|e| ComhairleError::RedisError(e.to_string()))?;
        let mut pubsub = client
            .get_async_pubsub()
            .await
            .map_err(|e| ComhairleError::RedisError(e.to_string()))?;

        pubsub
            .subscribe("comhairle_api_websocket_messages")
            .await
            .map_err(|e| ComhairleError::RedisError(e.to_string()))?;

        let self_clone = self.clone();

        let handle = tokio::spawn(async move {
            while let Some(msg) = pubsub.on_message().next().await {
                let payload: Vec<u8> = msg.get_payload_bytes().to_vec();
                let pubsub_msg: WebsocketPubSubMessage = match serde_json::from_slice(&payload) {
                    Ok(m) => m,
                    Err(_) => continue,
                };

                match pubsub_msg {
                    WebsocketPubSubMessage::Broadcast {
                        sender_id,
                        message,
                        authenticated_only,
                    } => {
                        // PREVENT ECHO: Skip if we sent this message!
                        if sender_id == self_clone.redis_service.as_ref().unwrap().instance_id {
                            continue;
                        }

                        if let Ok(ws_message) = serde_json::from_value(message) {
                            if authenticated_only {
                                let _ = self_clone
                                    .broadcast_to_authenticated_users_local(&ws_message)
                                    .await;
                            } else {
                                // Important: Call the LOCAL method, not the trait method,
                                // to avoid publishing it back to Redis!
                                let _ = self_clone.broadcast_to_all_local(&ws_message).await;
                            }
                        }
                    }
                    WebsocketPubSubMessage::SendToUser {
                        sender_id,
                        user_id,
                        message,
                    } => {
                        if sender_id == self_clone.redis_service.as_ref().unwrap().instance_id {
                            continue;
                        }

                        if let Ok(ws_message) = serde_json::from_value(message) {
                            if let Ok(user_uuid) = Uuid::parse_str(&user_id) {
                                let _ =
                                    self_clone.send_to_user_local(&user_uuid, &ws_message).await;
                            }
                        }
                    }
                    WebsocketPubSubMessage::SendToConnections {
                        sender_id,
                        connection_ids,
                        message,
                    } => {
                        if sender_id == self_clone.redis_service.as_ref().unwrap().instance_id {
                            continue;
                        }

                        if let Ok(ws_message) = serde_json::from_value(message) {
                            let connection_ids = connection_ids
                                .into_iter()
                                .map(ConnectionId)
                                .collect::<Vec<_>>();
                            let _ = self_clone
                                .send_to_connections_local(&connection_ids, &ws_message)
                                .await;
                        }
                    }
                    WebsocketPubSubMessage::SendToRoom {
                        domain,
                        room_id,
                        message,
                        ..
                    } => {
                        // Deliberately NO echo-skip: the publishing instance did not deliver
                        // locally (see `broadcast_to_room`), so it must also forward here.
                        if let Ok(ws_message) = serde_json::from_value(message) {
                            if let Ok(room_uuid) = Uuid::parse_str(&room_id) {
                                let _ = self_clone
                                    .send_to_room_members_local(&domain, &room_uuid, &ws_message)
                                    .await;
                            }
                        }
                    }
                }
            }
        });

        Ok(handle)
    }

    async fn publish_to_redis(
        &self,
        message: &WebsocketPubSubMessage,
    ) -> Result<(), ComhairleError> {
        if self.redis_service.is_none() {
            return Err(ComhairleError::RedisError(
                "Redis pub/sub is not configured".to_string(),
            ));
        }
        let redis_service = self.redis_service.as_ref().unwrap();
        let mut conn = redis_service.redis_publisher.clone();
        if let Ok(payload) = serde_json::to_string(message) {
            use redis::AsyncCommands;
            conn.publish::<_, _, ()>("comhairle_api_websocket_messages", payload)
                .await
                .map_err(|e| ComhairleError::RedisError(e.to_string()))?;
        }
        Ok(())
    }

    async fn broadcast_to_all_local(
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

    async fn broadcast_to_authenticated_users_local(
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

    async fn send_to_user_local(
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

    /// Deliver `message` to this instance's locally-connected members of `room_id`,
    /// as reported by the handler for `domain`. Does not touch Redis.
    async fn send_to_room_members_local(
        &self,
        domain: &str,
        room_id: &Uuid,
        message: &WebSocketMessage,
    ) -> Result<usize, ComhairleError> {
        let members = match self.handlers.get(domain) {
            Some(handler) => handler.local_room_members(room_id),
            None => return Ok(0),
        };

        let mut sent_count = 0;
        for user_id in members {
            sent_count += self.send_to_user_local(&user_id, message).await?;
        }

        Ok(sent_count)
    }

    async fn send_to_connections_local(
        &self,
        conn_uuids: &[ConnectionId],
        message: &WebSocketMessage,
    ) -> Result<usize, ComhairleError> {
        let mut sent_count = 0;
        let mut failed_connections = Vec::new();

        for connection_id in conn_uuids {
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
        let sent_count = self.broadcast_to_all_local(message).await?;

        if let Some(redis_service) = &self.redis_service {
            let pubsub_message = WebsocketPubSubMessage::Broadcast {
                sender_id: redis_service.instance_id,
                message: serde_json::to_value(message)
                    .map_err(|e| ComhairleError::SerializationError(e.to_string()))?,
                authenticated_only: false,
            };

            self.publish_to_redis(&pubsub_message).await?;
        }

        Ok(sent_count)
    }

    async fn broadcast_to_authenticated_users(
        &self,
        message: &WebSocketMessage,
    ) -> Result<usize, ComhairleError> {
        let sent_count = self.broadcast_to_authenticated_users_local(message).await?;

        if let Some(redis_service) = &self.redis_service {
            let pubsub_message = WebsocketPubSubMessage::Broadcast {
                sender_id: redis_service.instance_id,
                message: serde_json::to_value(message)
                    .map_err(|e| ComhairleError::SerializationError(e.to_string()))?,
                authenticated_only: true,
            };

            self.publish_to_redis(&pubsub_message).await?;
        }

        Ok(sent_count)
    }

    async fn send_to_user(
        &self,
        user_id: &Uuid,
        message: &WebSocketMessage,
    ) -> Result<usize, ComhairleError> {
        let sent_count = self.send_to_user_local(user_id, message).await?;

        if let Some(redis_service) = &self.redis_service {
            let pubsub_message = WebsocketPubSubMessage::SendToUser {
                sender_id: redis_service.instance_id,
                user_id: user_id.to_string(),
                message: serde_json::to_value(message)
                    .map_err(|e| ComhairleError::SerializationError(e.to_string()))?,
            };

            self.publish_to_redis(&pubsub_message).await?;
        }

        Ok(sent_count)
    }

    async fn send_to_connections(
        &self,
        connection_ids: &[ConnectionId],
        message: &WebSocketMessage,
    ) -> Result<usize, ComhairleError> {
        let sent_count = self
            .send_to_connections_local(connection_ids, message)
            .await?;

        if let Some(redis_service) = &self.redis_service {
            let pubsub_message = WebsocketPubSubMessage::SendToConnections {
                sender_id: redis_service.instance_id,
                connection_ids: connection_ids.iter().map(|id| id.0).collect(),
                message: serde_json::to_value(message)
                    .map_err(|e| ComhairleError::SerializationError(e.to_string()))?,
            };

            self.publish_to_redis(&pubsub_message).await?;
        }

        Ok(sent_count)
    }

    async fn broadcast_to_room(
        &self,
        domain: &str,
        room_id: &Uuid,
        message: &WebSocketMessage,
    ) -> Result<usize, ComhairleError> {
        // With Redis, publish once and let EVERY instance — including this one, via its
        // own subscription — forward to the members it holds. That keeps a single code
        // path and avoids local/remote duplication.
        //
        // Without Redis (single instance / dev) there is no subscriber loop, so publishing
        // would reach nobody; deliver locally as a fallback instead.
        match &self.redis_service {
            Some(redis_service) => {
                let pubsub_message = WebsocketPubSubMessage::SendToRoom {
                    sender_id: redis_service.instance_id,
                    domain: domain.to_string(),
                    room_id: room_id.to_string(),
                    message: serde_json::to_value(message)
                        .map_err(|e| ComhairleError::SerializationError(e.to_string()))?,
                };

                self.publish_to_redis(&pubsub_message).await?;
                Ok(0)
            }
            None => {
                self.send_to_room_members_local(domain, room_id, message)
                    .await
            }
        }
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
                if let WebSocketMessage::Ping { timestamp } = &ws_message {
                    let pong = WebSocketMessage::Pong {
                        timestamp: *timestamp,
                    };
                    connection.send_message(&pong).await?;
                    return Ok(());
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

    if let Some(domain) = domain
        && let Some(handler) = state.websockets.get_handler(domain)
    {
        handler
            .handle_message(message, connection, state)
            .await
            .map_err(|e| ComhairleError::WebSocketHandlerError(Box::new(e)))?;
        return Ok(true);
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use redis_test::server::RedisServer;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_websocket_broadcast_to_all_with_redis_pubsub() {
        // Keep Redis server alive throughout the test (for temporary servers)
        let _redis_server: Option<redis_test::server::RedisServer>;
        let redis_url: String;

        // Try to connect to localhost:6379 first (CI environment)
        let client = redis::Client::open("redis://localhost:6379/");
        let can_connect = if let Ok(client) = client {
            client.get_connection().is_ok()
        } else {
            false
        };

        if can_connect {
            _redis_server = None;
            redis_url = "redis://localhost:6379/".to_string();
        } else {
            // Try to start a temporary server for local development
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| RedisServer::new())) {
                Ok(server) => {
                    let addr = server.connection_info().addr().to_string();
                    redis_url = format!("redis://{}/", addr);

                    // Wait for the Redis server to start up
                    for attempt in 0..5 {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        if let Ok(client) = redis::Client::open(redis_url.as_str()) {
                            if client.get_connection().is_ok() {
                                break;
                            }
                        }
                        if attempt == 4 {
                            return;
                        }
                    }
                    _redis_server = Some(server);
                }
                Err(_) => {
                    return;
                }
            }
        }

        // Create two independent WebSocket services with Redis pub/sub enabled
        let websocket_config = Some(WebsocketConfig {
            redis_pubsub_url: redis_url.clone(),
        });

        let service_1 = ComhairleWebSocketService::new(websocket_config.as_ref())
            .await
            .expect("Failed to create service 1");
        let service_2 = ComhairleWebSocketService::new(websocket_config.as_ref())
            .await
            .expect("Failed to create service 2");

        // Start subscribers for both services
        let _subscriber_1 = service_1
            .start_pubsub_subscriber()
            .await
            .expect("Failed to start subscriber 1");
        let _subscriber_2 = service_2
            .start_pubsub_subscriber()
            .await
            .expect("Failed to start subscriber 2");

        // Create mock users
        let user_id = Uuid::new_v4();
        let user = crate::models::users::User {
            id: user_id,
            email: Some("test@example.com".to_string()),
            username: Some("test_user".to_string()),
            password: None,
            avatar_url: None,
            email_verified: true,
            auth_type: crate::models::users::UserAuthType::EmailPassword,
            organization_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            signup_ip: None,
        };

        let addr = "127.0.0.1:9999".parse().unwrap();
        let (connection_1, mut receiver_1) = WebSocketConnection::new(user.clone(), addr);
        let (connection_2, mut receiver_2) = WebSocketConnection::new(user.clone(), addr);

        service_1.add_connection(connection_1.clone());
        service_2.add_connection(connection_2.clone());

        // Give time for subscriptions to be ready
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        // Create a test message
        let test_message = WebSocketMessage::Notification {
            title: "Test Broadcast".to_string(),
            message: "Hello from service 1".to_string(),
            level: NotificationLevel::Info,
        };

        // Broadcast from service 1 (this should be received by service 2's connections via Redis)
        let sent_count = service_1
            .broadcast_to_all(&test_message)
            .await
            .expect("Failed to broadcast from service 1");

        assert_eq!(
            sent_count, 1,
            "Service 1 should have sent to its own connection"
        );

        // Give time for the message to be published and received via Redis
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        // Check if connection 1 received the message (from its own broadcast)
        if let Ok(msg) =
            tokio::time::timeout(std::time::Duration::from_millis(100), receiver_1.recv()).await
        {
            if let Some(msg) = msg {
                if let Ok(ws_msg) =
                    serde_json::from_str::<WebSocketMessage>(msg.to_text().unwrap_or(""))
                {
                    // Message should be received via the channel
                    assert!(matches!(ws_msg, WebSocketMessage::Notification { .. }));
                }
            }
        }

        // Check if service_2's connections received the broadcast via Redis pub/sub
        // The message should have been published to Redis and received by the subscriber
        if let Ok(msg) =
            tokio::time::timeout(std::time::Duration::from_millis(500), receiver_2.recv()).await
        {
            if let Some(msg) = msg {
                if let Ok(ws_msg) =
                    serde_json::from_str::<WebSocketMessage>(msg.to_text().unwrap_or(""))
                {
                    assert!(matches!(ws_msg, WebSocketMessage::Notification { .. }));
                }
            }
        }

        // Verify the message was received by both services
        assert_eq!(service_1.get_connection_count(), 1);
        assert_eq!(service_2.get_connection_count(), 1);
    }

    /// Returns a usable Redis URL (plus the temp-server guard, if one was spawned),
    /// or None if Redis is unavailable and the test should skip.
    fn redis_url_or_skip() -> Option<(Option<RedisServer>, String)> {
        let can_connect = redis::Client::open("redis://localhost:6379/")
            .map(|c| c.get_connection().is_ok())
            .unwrap_or(false);
        if can_connect {
            return Some((None, "redis://localhost:6379/".to_string()));
        }

        let server =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(RedisServer::new)).ok()?;
        let redis_url = format!("redis://{}/", server.connection_info().addr());
        for _ in 0..5 {
            std::thread::sleep(std::time::Duration::from_millis(100));
            if let Ok(client) = redis::Client::open(redis_url.as_str()) {
                if client.get_connection().is_ok() {
                    return Some((Some(server), redis_url));
                }
            }
        }
        None
    }

    fn test_user(id: Uuid) -> crate::models::users::User {
        crate::models::users::User {
            id,
            email: Some("test@example.com".to_string()),
            username: Some("test_user".to_string()),
            password: None,
            avatar_url: None,
            email_verified: true,
            auth_type: crate::models::users::UserAuthType::EmailPassword,
            organization_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            signup_ip: None,
        }
    }

    /// Registers `user_id` as a participant of `event_id`'s call in the handler's local map.
    fn insert_call_participant(
        handler: &crate::websockets::handlers::video_call::VideoCallMessageHandler,
        event_id: Uuid,
        user_id: Uuid,
    ) {
        use crate::websockets::handlers::video_call::{VideoCallParticipant, VideoCallState};
        let mut calls = handler.video_calls.write().unwrap();
        let mut state = VideoCallState::new(event_id);
        state.participants.insert(
            user_id,
            VideoCallParticipant {
                user_id,
                username: Some("test_user".to_string()),
                role: "participant".to_string(),
            },
        );
        calls.insert(event_id, state);
    }

    async fn assert_received_agenda_update(
        receiver: &mut mpsc::UnboundedReceiver<Message>,
        who: &str,
    ) {
        let msg = tokio::time::timeout(std::time::Duration::from_millis(1000), receiver.recv())
            .await
            .unwrap_or_else(|_| panic!("{who} timed out waiting for a message"))
            .unwrap_or_else(|| panic!("{who} channel closed"));
        let text = msg.to_text().expect("text frame");
        let ws_msg: WebSocketMessage = serde_json::from_str(text).expect("parse ws message");
        match ws_msg {
            WebSocketMessage::Custom { event, .. } => {
                assert_eq!(
                    event, "video_call:agenda_updated",
                    "{who} received wrong event"
                );
            }
            other => panic!("{who} expected Custom message, got {other:?}"),
        }
    }

    /// A participant connected to a DIFFERENT instance than the one that publishes an
    /// agenda update must still receive it, via Redis fan-out through `broadcast_to_room`.
    #[tokio::test]
    #[serial]
    async fn test_broadcast_to_room_fans_out_across_instances() {
        use crate::websockets::handlers::video_call::VideoCallMessageHandler;

        let Some((_redis_server, redis_url)) = redis_url_or_skip() else {
            return;
        };

        let websocket_config = Some(WebsocketConfig {
            redis_pubsub_url: redis_url.clone(),
        });

        let service_1 = ComhairleWebSocketService::new(websocket_config.as_ref())
            .await
            .expect("Failed to create service 1");
        let service_2 = ComhairleWebSocketService::new(websocket_config.as_ref())
            .await
            .expect("Failed to create service 2");

        let _subscriber_1 = service_1
            .start_pubsub_subscriber()
            .await
            .expect("Failed to start subscriber 1");
        let _subscriber_2 = service_2
            .start_pubsub_subscriber()
            .await
            .expect("Failed to start subscriber 2");

        let event_id = Uuid::new_v4();
        let user_a = Uuid::new_v4(); // connected to service_1 (the publisher)
        let user_b = Uuid::new_v4(); // connected to service_2 (a different instance)

        // Each instance's handler knows only the participant connected to it.
        let handler_1 = Arc::new(VideoCallMessageHandler::new());
        insert_call_participant(&handler_1, event_id, user_a);
        service_1.register_handler(handler_1);

        let handler_2 = Arc::new(VideoCallMessageHandler::new());
        insert_call_participant(&handler_2, event_id, user_b);
        service_2.register_handler(handler_2);

        let addr = "127.0.0.1:9999".parse().unwrap();
        let (conn_a, mut recv_a) = WebSocketConnection::new(test_user(user_a), addr);
        let (conn_b, mut recv_b) = WebSocketConnection::new(test_user(user_b), addr);
        service_1.add_connection(conn_a);
        service_2.add_connection(conn_b);

        // Let both subscriptions settle.
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let message = WebSocketMessage::Custom {
            event: "video_call:agenda_updated".to_string(),
            data: serde_json::json!([{ "Basic": { "title": "Updated agenda item" } }]),
        };

        // Publish from service_1 ONLY.
        service_1
            .broadcast_to_room("video_call", &event_id, &message)
            .await
            .expect("broadcast_to_room failed");

        // The publisher's own participant receives it (no local pre-delivery — it comes
        // back through service_1's own subscription)...
        assert_received_agenda_update(&mut recv_a, "user_a (publishing instance)").await;
        // ...and so does the participant on the OTHER instance. This is the fan-out.
        assert_received_agenda_update(&mut recv_b, "user_b (remote instance)").await;
    }
}
