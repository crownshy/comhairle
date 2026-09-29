use crate::models::error::ModelError;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, SelectStatement, SimpleExpr, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{
    Decode, Encode, PgPool, Postgres,
    encode::IsNull,
    prelude::{FromRow, Type},
    query_as_with,
};
use sqlx_postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef};
use tracing::instrument;
use uuid::Uuid;

#[cfg(any(test, feature = "test-util"))]
use fake::Dummy;

use crate::models::error::ValidationError;
use crate::models::{
    SqlxResultExt,
    pagination::{Order, PageOptions, PaginatedResults},
};

#[derive(Serialize, Deserialize, Debug, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "scheduled_email")]
pub struct ScheduledEmail {
    pub id: Uuid,
    pub user_email: String,
    pub email_config: ScheduledEmailConfig,
    pub status: EmailStatus,
    pub send_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct ScheduledEmailConfig {
    pub template: EmailTemplate,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EmailTemplate {
    // Extend with other templates relevant to scheduling
    EventReminder {
        event_id: Uuid,
        recipient_id: Uuid,
        owner_id: Uuid,
        locale: String,
    },
}

impl std::fmt::Display for EmailTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EmailTemplate::EventReminder { .. } => "event_reminder.html",
        };
        write!(f, "{}", value)
    }
}

impl Type<Postgres> for ScheduledEmailConfig {
    fn type_info() -> PgTypeInfo {
        <serde_json::Value as Type<Postgres>>::type_info()
    }
}

impl<'q> Encode<'q, Postgres> for EmailTemplate {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, sqlx::error::BoxDynError> {
        let json = serde_json::to_value(self)?;
        <serde_json::Value as Encode<Postgres>>::encode(json, buf)
    }
}

impl<'r> Decode<'r, Postgres> for ScheduledEmailConfig {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let json: serde_json::Value = Decode::<Postgres>::decode(value)?;
        Ok(serde_json::from_value(json)?)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, sqlx::Type, Clone, JsonSchema)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(any(test, feature = "test-util"), derive(Dummy))]
pub enum EmailStatus {
    #[sqlx(rename = "pending")]
    Pending,
    #[sqlx(rename = "sent")]
    Sent,
    #[sqlx(rename = "failed")]
    Failed,
}

impl From<EmailStatus> for sea_query::Value {
    fn from(val: EmailStatus) -> Self {
        sea_query::Value::String(Some(Box::new(val.to_string())))
    }
}

impl std::fmt::Display for EmailStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EmailStatus::Pending => "pending",
            EmailStatus::Sent => "sent",
            EmailStatus::Failed => "failed",
        };
        write!(f, "{}", value)
    }
}

const DEFAULT_COLUMNS: [ScheduledEmailIden; 7] = [
    ScheduledEmailIden::Id,
    ScheduledEmailIden::UserEmail,
    ScheduledEmailIden::EmailConfig,
    ScheduledEmailIden::Status,
    ScheduledEmailIden::SendAt,
    ScheduledEmailIden::CreatedAt,
    ScheduledEmailIden::UpdatedAt,
];

impl ScheduledEmail {
    /// Convenience method for updating the status of a scheduled_email as sent
    /// once email is sent.
    pub async fn sent(&self, db: &PgPool) -> Result<Self, ModelError> {
        let (sql, values) = Query::update()
            .table(ScheduledEmailIden::Table)
            .value(ScheduledEmailIden::Status, EmailStatus::Sent.to_string())
            .and_where(Expr::col(ScheduledEmailIden::Id).eq(self.id))
            .returning(Query::returning().columns(DEFAULT_COLUMNS))
            .build_sqlx(PostgresQueryBuilder);

        let scheduled_email = query_as_with(&sql, values).fetch_one(db).await?;

        Ok(scheduled_email)
    }

    /// Convenience method for updating the status of a scheduled_email as failed
    /// if an error occurs when sending.
    pub async fn failed(&self, db: &PgPool) -> Result<Self, ModelError> {
        let (sql, values) = Query::update()
            .table(ScheduledEmailIden::Table)
            .value(ScheduledEmailIden::Status, EmailStatus::Failed.to_string())
            .and_where(Expr::col(ScheduledEmailIden::Id).eq(self.id))
            .returning(Query::returning().columns(DEFAULT_COLUMNS))
            .build_sqlx(PostgresQueryBuilder);

        let scheduled_email = query_as_with(&sql, values).fetch_one(db).await?;

        Ok(scheduled_email)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct CreateScheduledEmail {
    pub user_email: String,
    pub send_at: DateTime<Utc>,
    pub email_config: ScheduledEmailConfig,
}

#[instrument(err(Debug), skip(db))]
pub async fn create(
    db: &PgPool,
    email: CreateScheduledEmail,
) -> Result<ScheduledEmail, ModelError> {
    let mut columns = vec![ScheduledEmailIden::UserEmail, ScheduledEmailIden::SendAt];
    let mut values: Vec<SimpleExpr> = vec![email.user_email.into(), email.send_at.into()];

    let email_config = serde_json::to_value(email.email_config)?;
    columns.push(ScheduledEmailIden::EmailConfig);
    values.push(email_config.into());

    let (sql, values) = Query::insert()
        .into_table(ScheduledEmailIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let scheduled_email = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(scheduled_email)
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct UpdateScheduledEmail {
    pub status: Option<EmailStatus>,
}

impl UpdateScheduledEmail {
    fn to_values(&self) -> Vec<(ScheduledEmailIden, SimpleExpr)> {
        let mut values = vec![];

        if let Some(value) = &self.status {
            values.push((ScheduledEmailIden::Status, value.clone().into()))
        }

        values
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn update(
    db: &PgPool,
    id: Uuid,
    update_email: UpdateScheduledEmail,
) -> Result<ScheduledEmail, ModelError> {
    let values = update_email.to_values();

    if values.is_empty() {
        return Err(ValidationError::NoValidUpdates.into());
    }

    let (sql, values) = Query::update()
        .table(ScheduledEmailIden::Table)
        .values(values)
        .and_where(Expr::col(ScheduledEmailIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let email = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(email)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, id: Uuid) -> Result<ScheduledEmail, ModelError> {
    let (sql, values) = Query::select()
        .from(ScheduledEmailIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(ScheduledEmailIden::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    let email = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Scheduled Email")?;

    Ok(email)
}

#[derive(Deserialize, Debug, Default, JsonSchema)]
pub struct ScheduledEmailFilterOptions {
    pub user_email: Option<String>,
}

impl ScheduledEmailFilterOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(user_email) = &self.user_email {
            query = query
                .and_where(
                    Expr::col((ScheduledEmailIden::Table, ScheduledEmailIden::UserEmail))
                        .eq(user_email.to_owned()),
                )
                .to_owned();
        }

        query
    }
}

#[derive(Deserialize, Debug, Default, JsonSchema)]
pub struct ScheduledEmailOrderOptions {
    pub created_at: Option<Order>,
    pub send_at: Option<Order>,
}

impl ScheduledEmailOrderOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(order) = &self.created_at {
            query = query
                .order_by(
                    (ScheduledEmailIden::Table, ScheduledEmailIden::CreatedAt),
                    order.into(),
                )
                .to_owned();
        }
        if let Some(order) = &self.send_at {
            query = query
                .order_by(
                    (ScheduledEmailIden::Table, ScheduledEmailIden::SendAt),
                    order.into(),
                )
                .to_owned();
        }

        query
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn list(
    db: &PgPool,
    page_options: PageOptions,
    filter_options: ScheduledEmailFilterOptions,
    order_options: ScheduledEmailOrderOptions,
) -> Result<PaginatedResults<ScheduledEmail>, ModelError> {
    let query = Query::select()
        .from(ScheduledEmailIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (ScheduledEmailIden::Table, col)))
        .to_owned();

    let query = filter_options.apply(query);
    let query = order_options.apply(query);

    let emails = page_options.fetch_paginated_results(db, query).await?;

    Ok(emails)
}

#[instrument(err(Debug), skip(db))]
pub async fn list_upcoming_scheduled_emails(
    db: &PgPool,
    upcoming_duration: chrono::Duration,
) -> Result<Vec<ScheduledEmail>, ModelError> {
    let (sql, values) = Query::select()
        .from(ScheduledEmailIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(ScheduledEmailIden::Status).eq(EmailStatus::Pending.to_string()))
        .and_where(
            // Cron task runs every 5 minutes so account for emails scheduled at exact
            // time of cron schedule
            Expr::col(ScheduledEmailIden::SendAt).gt(Utc::now() - chrono::Duration::minutes(1)),
        )
        .and_where(Expr::col(ScheduledEmailIden::SendAt).lte(Utc::now() + upcoming_duration))
        .build_sqlx(PostgresQueryBuilder);

    let scheduled_emails = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(scheduled_emails)
}

#[instrument(err(Debug), skip(db))]
pub async fn delete(db: &PgPool, id: Uuid) -> Result<ScheduledEmail, ModelError> {
    let (sql, values) = Query::delete()
        .from_table(ScheduledEmailIden::Table)
        .and_where(Expr::col(ScheduledEmailIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let email = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(email)
}
