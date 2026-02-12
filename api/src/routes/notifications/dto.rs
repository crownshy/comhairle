use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::Serialize;
use uuid::Uuid;

use crate::models::{
    notification::{Notification, NotificationContextType, NotificationType},
    notification_delivery::{DeliveryMethod, NotificationDelivery},
};

/// Data transfer object (public API representation) for a Notification.
///
/// This DTO is returned by notification related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `updated_at`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NotificationDto {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub notification_type: NotificationType,
    pub context_type: NotificationContextType,
    pub context_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Data transfer object (public API representation) for a NotificationDelivery.
///
/// This DTO is returned by notification related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `updated_at`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NotificationDeliveryDto {
    pub id: Uuid,
    pub notification_id: Uuid,
    pub user_id: Uuid,
    pub delivered_at: DateTime<Utc>,
    pub read_at: Option<DateTime<Utc>>,
    pub delivery_method: DeliveryMethod,
    pub created_at: DateTime<Utc>,
}

impl From<Notification> for NotificationDto {
    fn from(n: Notification) -> Self {
        Self {
            id: n.id,
            title: n.title,
            content: n.content,
            notification_type: n.notification_type,
            context_type: n.context_type,
            context_id: n.context_id,
            created_at: n.created_at,
        }
    }
}

impl From<NotificationDelivery> for NotificationDeliveryDto {
    fn from(n: NotificationDelivery) -> Self {
        Self {
            id: n.id,
            notification_id: n.notification_id,
            user_id: n.user_id,
            delivered_at: n.delivered_at,
            read_at: n.read_at,
            delivery_method: n.delivery_method,
            created_at: n.created_at,
        }
    }
}
