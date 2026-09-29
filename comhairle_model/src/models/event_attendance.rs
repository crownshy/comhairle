use crate::models::error::ModelError;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, JoinType, LockType, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use tracing::instrument;
use uuid::Uuid;

use crate::models::error::DataError;
use crate::models::error::EventError;
use crate::models::error::ValidationError;
use crate::models::{
    SqlxResultExt,
    event::EventIden,
    pagination::{Order, PageOptions, PaginatedResults},
    users::UserIden,
};

#[derive(Serialize, Deserialize, Debug, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "event_attendance")]
pub struct EventAttendance {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_id: Uuid,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [EventAttendanceIden; 6] = [
    EventAttendanceIden::Id,
    EventAttendanceIden::UserId,
    EventAttendanceIden::EventId,
    EventAttendanceIden::Role,
    EventAttendanceIden::CreatedAt,
    EventAttendanceIden::UpdatedAt,
];

#[derive(JsonSchema, Debug)]
pub struct CreateEventAttendance {
    pub user_id: Uuid,
    pub event_id: Uuid,
    pub role: String,
}

impl CreateEventAttendance {
    fn columns(&self) -> Vec<EventAttendanceIden> {
        vec![
            EventAttendanceIden::UserId,
            EventAttendanceIden::EventId,
            EventAttendanceIden::Role,
        ]
    }

    fn values(&self) -> Vec<sea_query::SimpleExpr> {
        vec![
            self.user_id.into(),
            self.event_id.into(),
            self.role.clone().into(),
        ]
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn create(
    db: &PgPool,
    new_event_attendance: &CreateEventAttendance,
) -> Result<EventAttendance, ModelError> {
    let mut tx = db.begin().await?;

    let (sql, values) = Query::select()
        .column(EventIden::Capacity)
        .from(EventIden::Table)
        .and_where(Expr::col(EventIden::Id).eq(new_event_attendance.event_id))
        .lock(LockType::Update)
        .build_sqlx(PostgresQueryBuilder);

    let capacity: Option<i32> = sqlx::query_scalar_with::<_, Option<i32>, _>(&sql, values)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(DataError::ResourceNotFound("Event".to_string()))?;

    let (sql, values) = Query::select()
        .expr(Expr::col(EventAttendanceIden::Id).count())
        .from(EventAttendanceIden::Table)
        .and_where(
            Expr::col((EventAttendanceIden::Table, EventAttendanceIden::EventId))
                .eq(new_event_attendance.event_id),
        )
        // Only check capacity against participant attendees
        .and_where(
            Expr::col((EventAttendanceIden::Table, EventAttendanceIden::Role))
                .eq("participant".to_string()),
        )
        .build_sqlx(PostgresQueryBuilder);

    let current_attendance: i64 = sqlx::query_scalar_with(&sql, values)
        .fetch_one(&mut *tx)
        .await?;

    if let Some(capacity) = capacity {
        // Only disallow creation of new participant attendees if at participant capacity
        if current_attendance >= capacity as i64 && new_event_attendance.role == "participant" {
            return Err(EventError::EventAtCapacity.into());
        }
    }

    let columns = new_event_attendance.columns();
    let values = new_event_attendance.values();

    let (sql, values) = Query::insert()
        .into_table(EventAttendanceIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let event_attendance = sqlx::query_as_with::<_, EventAttendance, _>(&sql, values)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) => {
                let pg_err = db_err.downcast_ref::<sqlx::postgres::PgDatabaseError>();
                if pg_err.code() == "23505" {
                    return ModelError::Event(EventError::UserAlreadyRegisteredForEvent(
                        new_event_attendance.event_id.to_string(),
                    ));
                }
                ModelError::Data(DataError::DatabaseError(sqlx::Error::Database(db_err)))
            }
            other => ModelError::Data(DataError::DatabaseError(other)),
        })?;

    tx.commit().await?;
    Ok(event_attendance)
}

#[derive(JsonSchema, Debug)]
pub struct UpdateEventAttendance {
    pub role: Option<String>,
}

impl UpdateEventAttendance {
    pub fn to_values(&self) -> Vec<(EventAttendanceIden, sea_query::SimpleExpr)> {
        let mut values = vec![];
        if let Some(value) = &self.role {
            values.push((EventAttendanceIden::Role, value.into()));
        }

        values
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn update(
    db: &PgPool,
    id: &Uuid,
    update_event_attendance: &UpdateEventAttendance,
) -> Result<EventAttendance, ModelError> {
    let values = update_event_attendance.to_values();

    if values.is_empty() {
        return Err(ValidationError::NoValidUpdates.into());
    }

    let (sql, values) = Query::update()
        .table(EventAttendanceIden::Table)
        .values(values)
        .and_where(Expr::col(EventAttendanceIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let event_attendance = sqlx::query_as_with::<_, EventAttendance, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(event_attendance)
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EventAttendanceEtx {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_id: Uuid,
    pub role: String,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Default, JsonSchema)]
pub struct EventAttendanceOrderOptions {
    pub created_at: Option<Order>,
}

impl EventAttendanceOrderOptions {
    fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(order) = &self.created_at {
            query = query
                .order_by(
                    (EventAttendanceIden::Table, EventAttendanceIden::CreatedAt),
                    order.into(),
                )
                .to_owned();
        }
        query
    }
}

#[derive(Deserialize, Debug, Default, JsonSchema)]
pub struct EventAttendanceFilterOptions {
    pub role: Option<String>,
}

impl EventAttendanceFilterOptions {
    fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(value) = &self.role {
            query = query
                .and_where(
                    Expr::col((EventAttendanceIden::Table, EventAttendanceIden::Role)).eq(value),
                )
                .to_owned();
        }

        query.to_owned()
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn list(
    db: &PgPool,
    event_id: Uuid,
    page_options: PageOptions,
    filter_options: EventAttendanceFilterOptions,
    order_options: EventAttendanceOrderOptions,
) -> Result<PaginatedResults<EventAttendanceEtx>, ModelError> {
    let query = Query::select()
        .from(EventAttendanceIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (EventAttendanceIden::Table, col)))
        .column((UserIden::Table, UserIden::Email))
        .join(
            JoinType::InnerJoin,
            UserIden::Table,
            Expr::col((UserIden::Table, UserIden::Id))
                .equals((EventAttendanceIden::Table, EventAttendanceIden::UserId)),
        )
        .and_where(
            Expr::col((EventAttendanceIden::Table, EventAttendanceIden::EventId)).eq(event_id),
        )
        .to_owned();

    let query = filter_options.apply(query);
    let query = order_options.apply(query);

    let events = page_options.fetch_paginated_results(db, query).await?;

    Ok(events)
}

/// Every attendee of an event (with email), unpaginated. Used to seed the
/// pre-assigned breakout plan.
#[instrument(err(Debug), skip(db))]
pub async fn list_all_for_event(
    db: &PgPool,
    event_id: &Uuid,
) -> Result<Vec<EventAttendanceEtx>, ModelError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (EventAttendanceIden::Table, col)))
        .column((UserIden::Table, UserIden::Email))
        .from(EventAttendanceIden::Table)
        .join(
            JoinType::InnerJoin,
            UserIden::Table,
            Expr::col((UserIden::Table, UserIden::Id))
                .equals((EventAttendanceIden::Table, EventAttendanceIden::UserId)),
        )
        .and_where(
            Expr::col((EventAttendanceIden::Table, EventAttendanceIden::EventId)).eq(*event_id),
        )
        .build_sqlx(PostgresQueryBuilder);

    let attendances = sqlx::query_as_with::<_, EventAttendanceEtx, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(attendances)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<EventAttendance, ModelError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (EventAttendanceIden::Table, col)))
        .from(EventAttendanceIden::Table)
        .and_where(
            Expr::col((EventAttendanceIden::Table, EventAttendanceIden::Id)).eq(id.to_owned()),
        )
        .build_sqlx(PostgresQueryBuilder);

    let event_attendance = sqlx::query_as_with::<_, EventAttendance, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Event Attendance")?;

    Ok(event_attendance)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_by_event_and_user(
    db: &PgPool,
    event_id: &Uuid,
    user_id: &Uuid,
) -> Result<EventAttendance, ModelError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (EventAttendanceIden::Table, col)))
        .from(EventAttendanceIden::Table)
        .and_where(
            Expr::col((EventAttendanceIden::Table, EventAttendanceIden::EventId))
                .eq(event_id.to_owned()),
        )
        .and_where(
            Expr::col((EventAttendanceIden::Table, EventAttendanceIden::UserId))
                .eq(user_id.to_owned()),
        )
        .build_sqlx(PostgresQueryBuilder);

    let event_attendance = sqlx::query_as_with::<_, EventAttendance, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Event Attendance")?;

    Ok(event_attendance)
}

#[instrument(err(Debug), skip(db))]
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<EventAttendance, ModelError> {
    let (sql, values) = Query::delete()
        .from_table(EventAttendanceIden::Table)
        .and_where(Expr::col(EventAttendanceIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let event_attendance = sqlx::query_as_with::<_, EventAttendance, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(event_attendance)
}
