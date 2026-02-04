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
pub enum NotificationType {
    #[sqlx(rename = "info")]
    Info,
    #[sqlx(rename = "warning")]
    Warning,
    #[sqlx(rename = "error")]
    Error,
    #[sqlx(rename = "success")]
    Success,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, sqlx::Type, Clone, JsonSchema)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(Dummy))]
pub enum NotificationContextType {
    #[sqlx(rename = "site")]
    Site,
    #[sqlx(rename = "conversation")]
    Conversation,
}

impl From<NotificationType> for sea_query::Value {
    fn from(val: NotificationType) -> Self {
        sea_query::Value::String(Some(Box::new(val.to_string())))
    }
}

impl std::fmt::Display for NotificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            NotificationType::Info => "info",
            NotificationType::Warning => "warning",
            NotificationType::Error => "error",
            NotificationType::Success => "success",
        };
        write!(f, "{}", value)
    }
}

impl From<NotificationContextType> for sea_query::Value {
    fn from(val: NotificationContextType) -> Self {
        sea_query::Value::String(Some(Box::new(val.to_string())))
    }
}

impl std::fmt::Display for NotificationContextType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            NotificationContextType::Site => "site",
            NotificationContextType::Conversation => "conversation",
        };
        write!(f, "{}", value)
    }
}

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "notifications")]
#[partially(derive(Deserialize, Debug, JsonSchema))]
pub struct Notification {
    #[partially(omit)]
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub notification_type: NotificationType,
    pub context_type: NotificationContextType,
    #[partially(transparent)]
    pub context_id: Option<Uuid>,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [NotificationIden; 8] = [
    NotificationIden::Id,
    NotificationIden::Title,
    NotificationIden::Content,
    NotificationIden::NotificationType,
    NotificationIden::ContextType,
    NotificationIden::ContextId,
    NotificationIden::CreatedAt,
    NotificationIden::UpdatedAt,
];

impl PartialNotification {
    pub fn to_values(&self) -> Vec<(NotificationIden, sea_query::SimpleExpr)> {
        let mut values = vec![];
        if let Some(value) = &self.title {
            values.push((NotificationIden::Title, value.into()))
        };
        if let Some(value) = &self.content {
            values.push((NotificationIden::Content, value.into()))
        };
        if let Some(value) = &self.notification_type {
            values.push((NotificationIden::NotificationType, value.clone().into()))
        };
        if let Some(value) = &self.context_type {
            values.push((NotificationIden::ContextType, value.clone().into()))
        };
        if let Some(value) = &self.context_id {
            values.push((NotificationIden::ContextId, (*value).into()))
        };
        values
    }
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct NotificationFilterOptions {
    notification_type: Option<NotificationType>,
    title: Option<String>,
    created_before: Option<DateTime<Utc>>,
    created_after: Option<DateTime<Utc>>,
}

impl NotificationFilterOptions {
    fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(value) = &self.notification_type {
            query = query
                .and_where(Expr::col(NotificationIden::NotificationType).eq(value.to_string()))
                .to_owned();
        };
        if let Some(value) = &self.title {
            query = query
                .and_where(Expr::col(NotificationIden::Title).like(format!("%{value}%")))
                .to_owned();
        };
        if let Some(value) = &self.created_before {
            query = query
                .and_where(
                    Expr::col(NotificationIden::CreatedAt).lt(sea_query::SimpleExpr::Value(
                        sea_query::Value::ChronoDateTime(Some(Box::new(value.naive_utc()))),
                    )),
                )
                .to_owned();
        };
        if let Some(value) = &self.created_after {
            query = query
                .and_where(
                    Expr::col(NotificationIden::CreatedAt).gt(sea_query::SimpleExpr::Value(
                        sea_query::Value::ChronoDateTime(Some(Box::new(value.naive_utc()))),
                    )),
                )
                .to_owned();
        };
        query.to_owned()
    }
}

#[derive(Deserialize, Debug, Default, JsonSchema)]
pub struct NotificationOrderOptions {
    created_at: Option<Order>,
    title: Option<Order>,
}

impl NotificationOrderOptions {
    pub fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(order) = &self.created_at {
            query = query
                .order_by(NotificationIden::CreatedAt, order.into())
                .to_owned()
        }
        if let Some(order) = &self.title {
            query = query
                .order_by(NotificationIden::Title, order.into())
                .to_owned()
        }
        query
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Dummy))]
pub struct CreateNotification {
    pub title: String,
    pub content: String,
    pub notification_type: Option<NotificationType>,
    pub context_type: Option<NotificationContextType>,
    pub context_id: Option<Uuid>,
}

impl CreateNotification {
    pub fn columns(&self) -> Vec<NotificationIden> {
        vec![
            NotificationIden::Title,
            NotificationIden::Content,
            NotificationIden::NotificationType,
            NotificationIden::ContextType,
            NotificationIden::ContextId,
        ]
    }
    pub fn values(&self) -> Vec<sea_query::SimpleExpr> {
        let notification_type = self
            .notification_type
            .clone()
            .unwrap_or(NotificationType::Info);
        let context_type = self
            .context_type
            .clone()
            .unwrap_or(NotificationContextType::Site);
        vec![
            self.title.clone().into(),
            self.content.clone().into(),
            notification_type.into(),
            context_type.into(),
            self.context_id.into(),
        ]
    }
}

pub async fn create(
    db: &PgPool,
    notification: &CreateNotification,
) -> Result<Notification, ComhairleError> {
    let columns = notification.columns();
    let values = notification.values();

    let (sql, values) = Query::insert()
        .into_table(NotificationIden::Table)
        .columns(columns)
        .values(values)
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let notification = sqlx::query_as_with::<_, Notification, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(notification)
}

pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<Notification, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(NotificationIden::Table)
        .and_where(Expr::col(NotificationIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let notification = sqlx::query_as_with::<_, Notification, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("Notification".into()))?;

    Ok(notification)
}

pub async fn update(
    db: &PgPool,
    id: Uuid,
    update: &PartialNotification,
) -> Result<Notification, ComhairleError> {
    let values = update.to_values();

    if values.is_empty() {
        return Err(ComhairleError::NoValidUpdates);
    }

    let (sql, values) = Query::update()
        .table(NotificationIden::Table)
        .values(values)
        .and_where(Expr::col(NotificationIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let notification = sqlx::query_as_with::<_, Notification, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(notification)
}

pub async fn delete(db: &PgPool, id: &Uuid) -> Result<Notification, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(NotificationIden::Table)
        .and_where(Expr::col(NotificationIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let notification = sqlx::query_as_with::<_, Notification, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("Notification".into()))?;

    Ok(notification)
}

pub async fn list(
    db: &PgPool,
    page_options: PageOptions,
    order_options: NotificationOrderOptions,
    filter_options: NotificationFilterOptions,
) -> Result<PaginatedResults<Notification>, ComhairleError> {
    let query = Query::select()
        .from(NotificationIden::Table)
        .columns(DEFAULT_COLUMNS)
        .to_owned();

    let query = filter_options.apply(query);
    let query = order_options.apply(query);

    let notifications = page_options.fetch_paginated_results(db, query).await?;

    Ok(notifications)
}
