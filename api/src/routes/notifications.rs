use std::sync::Arc;

use aide::axum::{
    routing::{get_with, put_with},
    ApiRouter,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use schemars::JsonSchema;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        notification_delivery::{
            self, NotificationDelivery, NotificationDeliveryOrderOptions, NotificationWithDelivery,
        },
        pagination::{OrderParams, PageOptions, PaginatedResults},
    },
    ComhairleState,
};

use super::auth::RequiredUser;

pub mod dto;

pub async fn get_unread_notifications(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    OrderParams(order_options): OrderParams<NotificationDeliveryOrderOptions>,
    Query(page_options): Query<PageOptions>,
) -> Result<(StatusCode, Json<PaginatedResults<NotificationWithDelivery>>), ComhairleError> {
    let deliveries = notification_delivery::list_unread_for_user_with_notifications(
        &state.db,
        &user.id,
        page_options,
        order_options,
    )
    .await?;

    Ok((StatusCode::OK, Json(deliveries)))
}

#[derive(Serialize, JsonSchema)]
pub struct UnreadCount {
    pub count: i64,
}

pub async fn get_unread_count(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
) -> Result<(StatusCode, Json<UnreadCount>), ComhairleError> {
    let count =
        crate::models::notification_delivery::get_unread_count_for_user(&state.db, &user.id)
            .await?;

    Ok((StatusCode::OK, Json(UnreadCount { count })))
}

pub async fn mark_notification_as_read(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(delivery_id): Path<Uuid>,
) -> Result<(StatusCode, Json<NotificationDelivery>), ComhairleError> {
    use crate::models::notification_delivery;

    // Get the delivery to verify it belongs to the user
    let delivery = notification_delivery::get_by_id(&state.db, &delivery_id).await?;

    if delivery.user_id != user.id {
        return Err(ComhairleError::UserNotAuthorized);
    }

    // Mark as read with current timestamp
    let updated_delivery =
        notification_delivery::mark_as_read(&state.db, &delivery_id, Utc::now()).await?;

    Ok((StatusCode::OK, Json(updated_delivery)))
}

pub async fn mark_all_notifications_as_read(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
) -> Result<(StatusCode, Json<serde_json::Value>), ComhairleError> {
    use crate::models::notification_delivery;

    // Mark all unread notifications as read for this user
    let updated_deliveries =
        notification_delivery::mark_all_as_read_for_user(&state.db, &user.id, Utc::now()).await?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "marked_as_read": updated_deliveries.len(),
            "message": format!("Marked {} notifications as read", updated_deliveries.len())
        })),
    ))
}

pub async fn get_all_notifications(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    OrderParams(order_options): OrderParams<NotificationDeliveryOrderOptions>,
    Query(page_options): Query<PageOptions>,
) -> Result<(StatusCode, Json<PaginatedResults<NotificationWithDelivery>>), ComhairleError> {
    let deliveries = notification_delivery::list_for_user_with_notifications(
        &state.db,
        &user.id,
        page_options,
        order_options,
    )
    .await?;

    Ok((StatusCode::OK, Json(deliveries)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/unread",
            get_with(get_unread_notifications, |op| {
                op.summary("Get unread notifications for current user")
                    .id("GetUnreadNotifications")
                    .description("Returns a paginated list of unread notification deliveries for the authenticated user")
                    .tag("Notifications")
                    .response::<200, Json<PaginatedResults<NotificationWithDelivery>>>()
            }),
        )
        .api_route(
            "/unread/count",
            get_with(get_unread_count, |op| {
                op.summary("Get unread notification count")
                    .id("GetUnreadNotificationsCount")
                    .description("Returns the count of unread notifications for the authenticated user")
                    .tag("Notifications")
                    .response::<200,Json<UnreadCount>>()
            }),
        )
        .api_route(
            "/",
            get_with(get_all_notifications, |op| {
                op.summary("Get all notifications for current user")
                    .id("GetAllNotifications")
                    .description("Returns a paginated list of all notification deliveries for the authenticated user")
                    .tag("Notifications")
                    .response::<200, Json<PaginatedResults<NotificationWithDelivery>>>()
            }),
        )
        .api_route(
            "/delivery/{delivery_id}/read",
            put_with(mark_notification_as_read, |op| {
                op.id("MarkNotificationAsRead")
                    .summary("Mark a notification as read")
                    .description("Marks a specific notification delivery as read for the current user")
                    .tag("Notifications")
                    .response::<200, Json<NotificationDelivery>>()
            }),
        )
        .api_route(
            "/read-all",
            put_with(mark_all_notifications_as_read, |op| {
                op.id("MarkAllNotificationsAsRead")
                    .summary("Mark all notifications as read")
                    .description("Marks all unread notification deliveries as read for the current user")
                    .tag("Notifications")
                    .response::<200, Json<serde_json::Value>>()
            }),
        )
        .with_state(state)
}
