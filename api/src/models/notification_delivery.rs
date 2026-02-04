use super::notification::{Notification, NotificationIden};
use super::pagination::{Order, PageOptions, PaginatedResults};
use crate::error::ComhairleError;
use chrono::{DateTime, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

#[cfg(test)]
use fake::Dummy;

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, sqlx::Type, Clone, JsonSchema)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(Dummy))]
pub enum DeliveryMethod {
    #[sqlx(rename = "in_app")]
    InApp,
    #[sqlx(rename = "email")]
    Email,
}

impl From<DeliveryMethod> for sea_query::Value {
    fn from(val: DeliveryMethod) -> Self {
        sea_query::Value::String(Some(Box::new(val.to_string())))
    }
}

impl std::fmt::Display for DeliveryMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            DeliveryMethod::InApp => "in_app",
            DeliveryMethod::Email => "email",
        };
        write!(f, "{}", value)
    }
}

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "notification_deliveries")]
#[partially(derive(Deserialize, Debug, JsonSchema))]
pub struct NotificationDelivery {
    #[partially(omit)]
    pub id: Uuid,
    #[partially(omit)]
    pub notification_id: Uuid,
    #[partially(omit)]
    pub user_id: Uuid,
    #[partially(omit)]
    pub delivered_at: DateTime<Utc>,
    #[partially(transparent)]
    pub read_at: Option<DateTime<Utc>>,
    pub delivery_method: DeliveryMethod,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [NotificationDeliveryIden; 8] = [
    NotificationDeliveryIden::Id,
    NotificationDeliveryIden::NotificationId,
    NotificationDeliveryIden::UserId,
    NotificationDeliveryIden::DeliveredAt,
    NotificationDeliveryIden::ReadAt,
    NotificationDeliveryIden::DeliveryMethod,
    NotificationDeliveryIden::CreatedAt,
    NotificationDeliveryIden::UpdatedAt,
];

#[derive(Debug, Serialize, Clone, JsonSchema)]
pub struct NotificationWithDelivery {
    #[serde(flatten)]
    pub delivery: NotificationDelivery,
    pub notification: Notification,
}

impl PartialNotificationDelivery {
    pub fn to_values(&self) -> Vec<(NotificationDeliveryIden, sea_query::SimpleExpr)> {
        let mut values = vec![];
        if let Some(value) = &self.read_at {
            values.push((NotificationDeliveryIden::ReadAt, (*value).into()))
        };
        if let Some(value) = &self.delivery_method {
            values.push((
                NotificationDeliveryIden::DeliveryMethod,
                value.clone().into(),
            ))
        };
        values
    }
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct NotificationDeliveryFilterOptions {
    notification_id: Option<Uuid>,
    user_id: Option<Uuid>,
    delivery_method: Option<DeliveryMethod>,
    is_read: Option<bool>,
    delivered_before: Option<DateTime<Utc>>,
    delivered_after: Option<DateTime<Utc>>,
}

impl NotificationDeliveryFilterOptions {
    fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(value) = &self.notification_id {
            query = query
                .and_where(
                    Expr::col(NotificationDeliveryIden::NotificationId).eq(value.to_string()),
                )
                .to_owned();
        };
        if let Some(value) = &self.user_id {
            query = query
                .and_where(Expr::col(NotificationDeliveryIden::UserId).eq(value.to_string()))
                .to_owned();
        };
        if let Some(value) = &self.delivery_method {
            query = query
                .and_where(
                    Expr::col(NotificationDeliveryIden::DeliveryMethod).eq(value.to_string()),
                )
                .to_owned();
        };
        if let Some(value) = self.is_read {
            if value {
                query = query
                    .and_where(Expr::col(NotificationDeliveryIden::ReadAt).is_not_null())
                    .to_owned();
            } else {
                query = query
                    .and_where(Expr::col(NotificationDeliveryIden::ReadAt).is_null())
                    .to_owned();
            }
        };
        if let Some(value) = &self.delivered_before {
            query = query
                .and_where(Expr::col(NotificationDeliveryIden::DeliveredAt).lt(
                    sea_query::SimpleExpr::Value(sea_query::Value::ChronoDateTime(Some(Box::new(
                        value.naive_utc(),
                    )))),
                ))
                .to_owned();
        };
        if let Some(value) = &self.delivered_after {
            query = query
                .and_where(Expr::col(NotificationDeliveryIden::DeliveredAt).gt(
                    sea_query::SimpleExpr::Value(sea_query::Value::ChronoDateTime(Some(Box::new(
                        value.naive_utc(),
                    )))),
                ))
                .to_owned();
        };
        query.to_owned()
    }
}

#[derive(Deserialize, Debug, Default, JsonSchema)]
pub struct NotificationDeliveryOrderOptions {
    delivered_at: Option<Order>,
    read_at: Option<Order>,
}

impl NotificationDeliveryOrderOptions {
    pub fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(order) = &self.delivered_at {
            query = query
                .order_by(NotificationDeliveryIden::DeliveredAt, order.into())
                .to_owned()
        }
        if let Some(order) = &self.read_at {
            query = query
                .order_by(NotificationDeliveryIden::ReadAt, order.into())
                .to_owned()
        }
        query
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Dummy))]
pub struct CreateNotificationDelivery {
    pub notification_id: Uuid,
    pub user_id: Uuid,
    pub delivery_method: Option<DeliveryMethod>,
}

impl CreateNotificationDelivery {
    pub fn columns(&self) -> Vec<NotificationDeliveryIden> {
        vec![
            NotificationDeliveryIden::NotificationId,
            NotificationDeliveryIden::UserId,
            NotificationDeliveryIden::DeliveryMethod,
        ]
    }
    pub fn values(&self) -> Vec<sea_query::SimpleExpr> {
        let delivery_method = self
            .delivery_method
            .clone()
            .unwrap_or(DeliveryMethod::InApp);
        vec![
            self.notification_id.into(),
            self.user_id.into(),
            delivery_method.into(),
        ]
    }
}

pub async fn create(
    db: &PgPool,
    delivery: &CreateNotificationDelivery,
) -> Result<NotificationDelivery, ComhairleError> {
    let columns = delivery.columns();
    let values = delivery.values();

    let (sql, values) = Query::insert()
        .into_table(NotificationDeliveryIden::Table)
        .columns(columns)
        .values(values)
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let delivery_result = sqlx::query_as_with::<_, NotificationDelivery, _>(&sql, values)
        .fetch_one(db)
        .await;

    delivery_result.map_err(ComhairleError::DatabaseError)
}

pub async fn create_bulk(
    db: &PgPool,
    deliveries: &[CreateNotificationDelivery],
) -> Result<Vec<NotificationDelivery>, ComhairleError> {
    if deliveries.is_empty() {
        return Ok(vec![]);
    }

    let mut query = Query::insert()
        .into_table(NotificationDeliveryIden::Table)
        .columns([
            NotificationDeliveryIden::NotificationId,
            NotificationDeliveryIden::UserId,
            NotificationDeliveryIden::DeliveryMethod,
        ])
        .to_owned();

    for delivery in deliveries {
        query = query.values(delivery.values()).unwrap().to_owned();
    }

    let (sql, values) = query
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let deliveries = sqlx::query_as_with::<_, NotificationDelivery, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(deliveries)
}

pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<NotificationDelivery, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(NotificationDeliveryIden::Table)
        .and_where(Expr::col(NotificationDeliveryIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let delivery = sqlx::query_as_with::<_, NotificationDelivery, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("NotificationDelivery".into()))?;

    Ok(delivery)
}

pub async fn mark_as_read(
    db: &PgPool,
    id: &Uuid,
    read_at: DateTime<Utc>,
) -> Result<NotificationDelivery, ComhairleError> {
    let (sql, values) = Query::update()
        .table(NotificationDeliveryIden::Table)
        .values([(NotificationDeliveryIden::ReadAt, read_at.into())])
        .and_where(Expr::col(NotificationDeliveryIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let delivery = sqlx::query_as_with::<_, NotificationDelivery, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(delivery)
}

pub async fn mark_all_as_read_for_user(
    db: &PgPool,
    user_id: &Uuid,
    read_at: DateTime<Utc>,
) -> Result<Vec<NotificationDelivery>, ComhairleError> {
    let (sql, values) = Query::update()
        .table(NotificationDeliveryIden::Table)
        .values([(NotificationDeliveryIden::ReadAt, read_at.into())])
        .and_where(Expr::col(NotificationDeliveryIden::UserId).eq(user_id.to_owned()))
        .and_where(Expr::col(NotificationDeliveryIden::ReadAt).is_null())
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let deliveries = sqlx::query_as_with::<_, NotificationDelivery, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(deliveries)
}

pub async fn update(
    db: &PgPool,
    id: Uuid,
    update: &PartialNotificationDelivery,
) -> Result<NotificationDelivery, ComhairleError> {
    let values = update.to_values();

    if values.is_empty() {
        return Err(ComhairleError::NoValidUpdates);
    }

    let (sql, values) = Query::update()
        .table(NotificationDeliveryIden::Table)
        .values(values)
        .and_where(Expr::col(NotificationDeliveryIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let delivery = sqlx::query_as_with::<_, NotificationDelivery, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(delivery)
}

pub async fn delete(db: &PgPool, id: &Uuid) -> Result<NotificationDelivery, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(NotificationDeliveryIden::Table)
        .and_where(Expr::col(NotificationDeliveryIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let delivery = sqlx::query_as_with::<_, NotificationDelivery, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("NotificationDelivery".into()))?;

    Ok(delivery)
}

pub async fn list_for_user(
    db: &PgPool,
    user_id: &Uuid,
    page_options: PageOptions,
    order_options: NotificationDeliveryOrderOptions,
) -> Result<PaginatedResults<NotificationDelivery>, ComhairleError> {
    let query = Query::select()
        .from(NotificationDeliveryIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(NotificationDeliveryIden::UserId).eq(user_id.to_owned()))
        .to_owned();

    let query = order_options.apply(query);

    let deliveries = page_options.fetch_paginated_results(db, query).await?;

    Ok(deliveries)
}

pub async fn list_unread_for_user(
    db: &PgPool,
    user_id: &Uuid,
    page_options: PageOptions,
    order_options: NotificationDeliveryOrderOptions,
) -> Result<PaginatedResults<NotificationDelivery>, ComhairleError> {
    let query = Query::select()
        .from(NotificationDeliveryIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(NotificationDeliveryIden::UserId).eq(user_id.to_owned()))
        .and_where(Expr::col(NotificationDeliveryIden::ReadAt).is_null())
        .to_owned();

    let query = order_options.apply(query);

    let deliveries = page_options.fetch_paginated_results(db, query).await?;

    Ok(deliveries)
}

pub async fn list(
    db: &PgPool,
    page_options: PageOptions,
    order_options: NotificationDeliveryOrderOptions,
    filter_options: NotificationDeliveryFilterOptions,
) -> Result<PaginatedResults<NotificationDelivery>, ComhairleError> {
    let query = Query::select()
        .from(NotificationDeliveryIden::Table)
        .columns(DEFAULT_COLUMNS)
        .to_owned();

    let query = filter_options.apply(query);
    let query = order_options.apply(query);

    let deliveries = page_options.fetch_paginated_results(db, query).await?;

    Ok(deliveries)
}

pub async fn get_unread_count_for_user(db: &PgPool, user_id: &Uuid) -> Result<i64, ComhairleError> {
    let (sql, values) = Query::select()
        .expr(Expr::count(Expr::asterisk()))
        .from(NotificationDeliveryIden::Table)
        .and_where(Expr::col(NotificationDeliveryIden::UserId).eq(user_id.to_owned()))
        .and_where(Expr::col(NotificationDeliveryIden::ReadAt).is_null())
        .build_sqlx(PostgresQueryBuilder);

    let count: (i64,) = sqlx::query_as_with(&sql, values).fetch_one(db).await?;

    Ok(count.0)
}

pub async fn list_for_user_with_notifications(
    db: &PgPool,
    user_id: &Uuid,
    page_options: PageOptions,
    order_options: NotificationDeliveryOrderOptions,
) -> Result<PaginatedResults<NotificationWithDelivery>, ComhairleError> {
    // First get the deliveries
    let deliveries = list_for_user(db, user_id, page_options.clone(), order_options).await?;

    // Get all notification IDs
    let notification_ids: Vec<Uuid> = deliveries
        .records
        .iter()
        .map(|d| d.notification_id)
        .collect();

    if notification_ids.is_empty() {
        return Ok(PaginatedResults {
            records: vec![],
            total: deliveries.total,
        });
    }

    // Get all notifications in one query
    let (sql, values) = Query::select()
        .columns([
            NotificationIden::Id,
            NotificationIden::Title,
            NotificationIden::Content,
            NotificationIden::NotificationType,
            NotificationIden::ContextType,
            NotificationIden::ContextId,
            NotificationIden::CreatedAt,
            NotificationIden::UpdatedAt,
        ])
        .from(NotificationIden::Table)
        .and_where(Expr::col(NotificationIden::Id).is_in(notification_ids))
        .build_sqlx(PostgresQueryBuilder);

    let notifications: Vec<Notification> = sqlx::query_as_with(&sql, values).fetch_all(db).await?;

    // Create a map for quick lookup
    let notifications_map: std::collections::HashMap<Uuid, Notification> =
        notifications.into_iter().map(|n| (n.id, n)).collect();

    // Combine the data
    let combined_data: Vec<NotificationWithDelivery> = deliveries
        .records
        .into_iter()
        .filter_map(|delivery| {
            notifications_map
                .get(&delivery.notification_id)
                .map(|notification| NotificationWithDelivery {
                    delivery,
                    notification: notification.clone(),
                })
        })
        .collect();

    Ok(PaginatedResults {
        records: combined_data,
        total: deliveries.total,
    })
}

pub async fn list_unread_for_user_with_notifications(
    db: &PgPool,
    user_id: &Uuid,
    page_options: PageOptions,
    order_options: NotificationDeliveryOrderOptions,
) -> Result<PaginatedResults<NotificationWithDelivery>, ComhairleError> {
    // First get the unread deliveries
    let deliveries = list_unread_for_user(db, user_id, page_options.clone(), order_options).await?;

    // Get all notification IDs
    let notification_ids: Vec<Uuid> = deliveries
        .records
        .iter()
        .map(|d| d.notification_id)
        .collect();

    if notification_ids.is_empty() {
        return Ok(PaginatedResults {
            records: vec![],
            total: deliveries.total,
        });
    }

    // Get all notifications in one query
    let (sql, values) = Query::select()
        .columns([
            NotificationIden::Id,
            NotificationIden::Title,
            NotificationIden::Content,
            NotificationIden::NotificationType,
            NotificationIden::ContextType,
            NotificationIden::ContextId,
            NotificationIden::CreatedAt,
            NotificationIden::UpdatedAt,
        ])
        .from(NotificationIden::Table)
        .and_where(Expr::col(NotificationIden::Id).is_in(notification_ids))
        .build_sqlx(PostgresQueryBuilder);

    let notifications: Vec<Notification> = sqlx::query_as_with(&sql, values).fetch_all(db).await?;

    // Create a map for quick lookup
    let notifications_map: std::collections::HashMap<Uuid, Notification> =
        notifications.into_iter().map(|n| (n.id, n)).collect();

    // Combine the data
    let combined_data: Vec<NotificationWithDelivery> = deliveries
        .records
        .into_iter()
        .filter_map(|delivery| {
            notifications_map
                .get(&delivery.notification_id)
                .map(|notification| NotificationWithDelivery {
                    delivery,
                    notification: notification.clone(),
                })
        })
        .collect();

    Ok(PaginatedResults {
        records: combined_data,
        total: deliveries.total,
    })
}
