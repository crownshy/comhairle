use async_trait::async_trait;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{notification, notification_delivery},
    websockets::{messages::WebSocketMessage, WebSocketConnection, WebSocketMessageHandler},
    ComhairleState,
};

/// Handler for notification-related WebSocket messages.
///
/// This handler manages real-time notifications over WebSockets, providing:
/// - Instant notification delivery to connected users
/// - Bidirectional communication (client can mark notifications as read)
/// - Reactive unread count updates
/// - Graceful fallback to database-only storage for offline users
///
/// # Message Events
///
/// ## Server → Client
/// - `notification:new` - New notification created
/// - `notification:marked_read` - Confirmation that notification was marked as read
/// - `notification:all_marked_read` - Confirmation that all notifications were marked as read
/// - `notification:unread_count` - Current unread notification count
///
/// ## Client → Server
/// - `notification:mark_read` - Request to mark a notification as read
/// - `notification:mark_all_read` - Request to mark all notifications as read
/// - `notification:get_unread_count` - Request current unread count
///
/// # Example Usage
///
/// ```rust
/// use crate::websockets::handlers::notifications::NotificationMessageHandler;
/// use crate::models::notification::{NotificationType, NotificationContextType};
///
/// // Send a notification to a user (creates DB record + sends via WebSocket)
/// NotificationMessageHandler::create_and_send_notification(
///     &state,
///     &user_id,
///     "New Message",
///     "You have a new message from Alice",
///     NotificationType::Info,
///     NotificationContextType::Conversation,
///     Some(&conversation_id),
/// ).await?;
/// ```
///
/// # Registration
///
/// This handler is automatically registered in `websockets::setup::register_handlers()`.
pub struct NotificationMessageHandler;

impl NotificationMessageHandler {
    /// Create a new notification message handler.
    pub fn new() -> Self {
        Self
    }

    /// Send a notification to a specific user via WebSocket (low-level).
    ///
    /// This method sends a notification message to a user if they are currently connected.
    /// It does NOT create the notification in the database or create a delivery record.
    ///
    /// # When to Use
    ///
    /// Use this method when:
    /// - You've already created the notification and delivery records
    /// - You want to manually control the database transaction
    /// - You're re-sending an existing notification
    ///
    /// For most use cases, prefer [`create_and_send_notification`](#method.create_and_send_notification)
    /// which handles both database storage and WebSocket delivery.
    ///
    /// # Parameters
    ///
    /// - `state`: Application state
    /// - `user_id`: ID of the user to send the notification to
    /// - `notification_id`: ID of the notification (for tracking)
    /// - `title`: Notification title
    /// - `message`: Notification message content
    /// - `level`: Notification level ("info", "warning", "error", "success")
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the message was sent successfully or user is not connected
    /// - `Err(ComhairleError)` if there was an error sending the message
    ///
    /// # Example
    ///
    /// ```rust
    /// NotificationMessageHandler::send_notification_to_user(
    ///     &state,
    ///     &user_id,
    ///     &notification.id,
    ///     "System Alert",
    ///     "Your session will expire in 5 minutes",
    ///     "warning",
    /// ).await?;
    /// ```
    pub async fn send_notification_to_user(
        state: &Arc<ComhairleState>,
        user_id: &Uuid,
        notification_id: &Uuid,
        title: &str,
        message: &str,
        level: &str,
    ) -> Result<(), ComhairleError> {
        let ws_message = WebSocketMessage::Custom {
            event: "notification:new".to_string(),
            data: serde_json::json!({
                "id": notification_id,
                "title": title,
                "message": message,
                "level": level,
            }),
        };

        let sent_count = state.websockets.send_to_user(user_id, &ws_message).await?;

        if sent_count > 0 {
            info!(
                "Sent notification {} to user {} via WebSocket",
                notification_id, user_id
            );
        } else {
            info!(
                "User {} not connected, notification {} stored in database only",
                user_id, notification_id
            );
        }

        Ok(())
    }

    /// Create a notification, store it in the database, and send via WebSocket if connected.
    ///
    /// This is the recommended way to send notifications as it handles both:
    /// 1. Persistent storage (database record + delivery record)
    /// 2. Real-time delivery (WebSocket if user is connected)
    ///
    /// If the user is not currently connected, the notification is stored and will be
    /// available when they next check their notifications via the REST API.
    ///
    /// # Parameters
    ///
    /// - `state`: Application state
    /// - `user_id`: ID of the user to send the notification to
    /// - `title`: Notification title
    /// - `message`: Notification message content
    /// - `notification_type`: Type of notification (Info, Warning, Error, Success)
    /// - `context_type`: Context of the notification (Site, Conversation, etc.)
    /// - `context_id`: Optional ID linking the notification to a specific entity
    ///
    /// # Returns
    ///
    /// - `Ok(Notification)` with the created notification
    /// - `Err(ComhairleError)` if database or WebSocket operations fail
    ///
    /// # Example
    ///
    /// ```rust
    /// use crate::models::notification::{NotificationType, NotificationContextType};
    ///
    /// // Notify user of a new conversation message
    /// let notification = NotificationMessageHandler::create_and_send_notification(
    ///     &state,
    ///     &recipient_id,
    ///     "New Message",
    ///     "Alice sent you a message in 'Project Discussion'",
    ///     NotificationType::Info,
    ///     NotificationContextType::Conversation,
    ///     Some(&conversation_id),
    /// ).await?;
    /// ```
    pub async fn create_and_send_notification(
        state: &Arc<ComhairleState>,
        user_id: &Uuid,
        title: &str,
        message: &str,
        notification_type: notification::NotificationType,
        context_type: notification::NotificationContextType,
        context_id: Option<&Uuid>,
    ) -> Result<notification::Notification, ComhairleError> {
        // Create the notification in the database
        let new_notification = notification::CreateNotification {
            title: title.to_string(),
            content: message.to_string(),
            notification_type: Some(notification_type),
            context_type: Some(context_type),
            context_id: context_id.copied(),
        };

        let created_notification = notification::create(&state.db, &new_notification).await?;

        // Create delivery record
        let delivery = notification_delivery::CreateNotificationDelivery {
            notification_id: created_notification.id,
            user_id: *user_id,
            delivery_method: Some(notification_delivery::DeliveryMethod::InApp),
        };

        let _created_delivery = notification_delivery::create(&state.db, &delivery).await?;

        // Send via WebSocket if user is connected
        Self::send_notification_to_user(
            state,
            user_id,
            &created_notification.id,
            title,
            message,
            &created_notification.notification_type.to_string(),
        )
        .await?;

        Ok(created_notification)
    }
}

#[async_trait]
impl WebSocketMessageHandler for NotificationMessageHandler {
    fn domain(&self) -> &str {
        "notification"
    }

    async fn handle_message(
        &self,
        message: &WebSocketMessage,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), ComhairleError> {
        match message {
            WebSocketMessage::Custom { event, data } if event.starts_with("notification:") => {
                match event.as_str() {
                    "notification:mark_read" => {
                        self.handle_mark_read(data, connection, state).await
                    }
                    "notification:mark_all_read" => {
                        self.handle_mark_all_read(connection, state).await
                    }
                    "notification:get_unread_count" => {
                        self.handle_get_unread_count(connection, state).await
                    }
                    _ => {
                        info!("Unhandled notification event: {}", event);
                        Ok(())
                    }
                }
            }
            _ => Ok(()),
        }
    }
}

impl NotificationMessageHandler {
    async fn handle_mark_read(
        &self,
        data: &serde_json::Value,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), ComhairleError> {
        let delivery_id = data["delivery_id"]
            .as_str()
            .and_then(|s| Uuid::parse_str(s).ok())
            .ok_or_else(|| ComhairleError::BadRequest("Missing or invalid delivery_id".into()))?;

        // Verify the delivery belongs to this user
        let delivery = notification_delivery::get_by_id(&state.db, &delivery_id).await?;
        if delivery.user_id != connection.user.id {
            return Err(ComhairleError::UserNotAuthorized);
        }

        // Mark as read
        notification_delivery::mark_as_read(&state.db, &delivery_id, chrono::Utc::now()).await?;

        // Send confirmation back
        let response = WebSocketMessage::Custom {
            event: "notification:marked_read".to_string(),
            data: serde_json::json!({
                "delivery_id": delivery_id,
                "success": true,
            }),
        };
        connection.send_message(&response).await?;

        // Also send updated unread count
        self.send_unread_count(connection, state).await?;

        info!(
            "User {} marked notification {} as read",
            connection.user.id, delivery_id
        );

        Ok(())
    }

    async fn handle_mark_all_read(
        &self,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), ComhairleError> {
        // Mark all as read for this user
        let updated_deliveries = notification_delivery::mark_all_as_read_for_user(
            &state.db,
            &connection.user.id,
            chrono::Utc::now(),
        )
        .await?;

        // Send confirmation back
        let response = WebSocketMessage::Custom {
            event: "notification:all_marked_read".to_string(),
            data: serde_json::json!({
                "count": updated_deliveries.len(),
                "success": true,
            }),
        };
        connection.send_message(&response).await?;

        // Also send updated unread count (should be 0)
        self.send_unread_count(connection, state).await?;

        info!(
            "User {} marked {} notifications as read",
            connection.user.id,
            updated_deliveries.len()
        );

        Ok(())
    }

    async fn handle_get_unread_count(
        &self,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), ComhairleError> {
        self.send_unread_count(connection, state).await
    }

    async fn send_unread_count(
        &self,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), ComhairleError> {
        let count =
            notification_delivery::get_unread_count_for_user(&state.db, &connection.user.id)
                .await?;

        let response = WebSocketMessage::Custom {
            event: "notification:unread_count".to_string(),
            data: serde_json::json!({
                "count": count,
            }),
        };
        connection.send_message(&response).await?;

        Ok(())
    }
}
