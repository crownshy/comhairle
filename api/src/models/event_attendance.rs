use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, LockType, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        event::EventIden,
        pagination::{Order, PageOptions, PaginatedResults},
    },
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

#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    new_event_attendance: &CreateEventAttendance,
) -> Result<EventAttendance, ComhairleError> {
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
        .ok_or(ComhairleError::ResourceNotFound("Event".to_string()))?;

    let (sql, values) = Query::select()
        .expr(Expr::col(EventAttendanceIden::Id).count())
        .from(EventAttendanceIden::Table)
        .and_where(
            Expr::col((EventAttendanceIden::Table, EventAttendanceIden::EventId))
                .eq(new_event_attendance.event_id),
        )
        .build_sqlx(PostgresQueryBuilder);

    let current_attendance: i64 = sqlx::query_scalar_with(&sql, values)
        .fetch_one(&mut *tx)
        .await?;

    if let Some(capacity) = capacity {
        if current_attendance >= capacity as i64 {
            return Err(ComhairleError::EventAtCapacity);
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
        .await?;

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

#[instrument(err(Debug))]
pub async fn update(
    db: &PgPool,
    id: &Uuid,
    update_event_attendance: &UpdateEventAttendance,
) -> Result<EventAttendance, ComhairleError> {
    let values = update_event_attendance.to_values();

    if values.is_empty() {
        return Err(ComhairleError::NoValidUpdates);
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

#[derive(Deserialize, Debug, Default, JsonSchema)]
pub struct EventAttendanceOrderOptions {
    created_at: Option<Order>,
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
    event_id: Option<Uuid>,
}

impl EventAttendanceFilterOptions {
    fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(value) = self.event_id {
            query = query
                .and_where(
                    Expr::col((EventAttendanceIden::Table, EventAttendanceIden::EventId)).eq(value),
                )
                .to_owned();
        }

        query.to_owned()
    }
}

#[instrument(err(Debug))]
pub async fn list(
    db: &PgPool,
    page_options: PageOptions,
    filter_options: EventAttendanceFilterOptions,
    order_options: EventAttendanceOrderOptions,
    locale: Option<String>,
) -> Result<PaginatedResults<EventAttendance>, ComhairleError> {
    let query = Query::select()
        .from(EventAttendanceIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (EventAttendanceIden::Table, col)))
        .to_owned();

    let query = filter_options.apply(query);
    let query = order_options.apply(query);

    let events = page_options.fetch_paginated_results(db, query).await?;

    Ok(events)
}

#[instrument(err(Debug))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<EventAttendance, ComhairleError> {
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
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                ComhairleError::ResourceNotFound("EventAttendance".to_string())
            }
            other => ComhairleError::DatabaseError(other),
        })?;

    Ok(event_attendance)
}

#[instrument(err(Debug))]
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<EventAttendance, ComhairleError> {
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

#[cfg(test)]
mod tests {
    use crate::models::{
        event::{self, CreateEvent},
        model_test_helpers::{
            get_random_user_id, get_random_workflow_id, setup_default_app_and_session,
        },
    };

    use super::*;
    use std::error::Error;

    #[sqlx::test]
    async fn should_create_attendance_for_event_without_capacity(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;

        let create_event = CreateEvent {
            name: "test_event".to_string(),
            workflow_id,
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        let new_event = event::create(&pool, &create_event).await?;

        let create_attendance = CreateEventAttendance {
            event_id: new_event.id,
            user_id,
            role: "participant".to_string(),
        };
        let attendance = create(&pool, &create_attendance).await?;

        assert_eq!(attendance.event_id, new_event.id, "incorrect event_id");
        assert_eq!(attendance.user_id, user_id, "incorrect user_id");

        Ok(())
    }

    #[sqlx::test]
    async fn should_create_attendance_for_event_with_capacity(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;

        let create_event = CreateEvent {
            name: "test_event".to_string(),
            workflow_id,
            capacity: Some(10),
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        let new_event = event::create(&pool, &create_event).await?;

        let create_attendance = CreateEventAttendance {
            event_id: new_event.id,
            user_id,
            role: "participant".to_string(),
        };
        let attendance = create(&pool, &create_attendance).await?;

        assert_eq!(attendance.event_id, new_event.id, "incorrect event_id");
        assert_eq!(attendance.user_id, user_id, "incorrect user_id");

        Ok(())
    }

    #[sqlx::test]
    async fn user_cannot_attend_same_event_twice(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;

        let create_event = CreateEvent {
            name: "test_event".to_string(),
            workflow_id,
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        let new_event = event::create(&pool, &create_event).await?;

        let create_attendance = CreateEventAttendance {
            event_id: new_event.id,
            user_id,
            role: "participant".to_string(),
        };
        let _ = create(&pool, &create_attendance).await?;

        let create_attendance = CreateEventAttendance {
            event_id: new_event.id,
            user_id,
            role: "participant".to_string(),
        };
        let err = create(&pool, &create_attendance).await.unwrap_err();

        match err {
            ComhairleError::DatabaseError(ref db_error) => {
                if let Some(pg_error) = db_error.as_database_error() {
                    assert_eq!(
                        pg_error.constraint(),
                        Some("user_event_unique_index"),
                        "incorrect constraint"
                    );
                } else {
                    panic!("Expected Postgres database error");
                }
            }
            _ => panic!("Expected DatabaseError"),
        };

        Ok(())
    }

    #[sqlx::test]
    async fn transaction_will_not_lock_on_failure(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let user_id_1 = get_random_user_id(&app, &mut session).await?;
        let user_id_2 = get_random_user_id(&app, &mut session).await?;

        let create_event_1 = CreateEvent {
            name: "test_event_1".to_string(),
            capacity: Some(1),
            workflow_id,
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        let create_event_2 = CreateEvent {
            name: "test_event_2".to_string(),
            capacity: Some(1),
            workflow_id,
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        let new_event_1 = event::create(&pool, &create_event_1).await?;
        let new_event_2 = event::create(&pool, &create_event_2).await?;

        let create_attendance_1 = CreateEventAttendance {
            event_id: new_event_1.id,
            user_id: user_id_1,
            role: "participant".to_string(),
        };
        let create_attendance_2 = CreateEventAttendance {
            event_id: new_event_1.id,
            user_id: user_id_2,
            role: "participant".to_string(),
        };
        let create_attendance_3 = CreateEventAttendance {
            event_id: new_event_2.id,
            user_id: user_id_1,
            role: "facilitator".to_string(),
        };

        let _ = create(&pool, &create_attendance_1).await?;
        // Fails because event is at capacity
        let _ = create(&pool, &create_attendance_2).await.unwrap_err();
        // Create attendance for different event to check lock is released
        let attendance = create(&pool, &create_attendance_3).await?;

        assert_eq!(attendance.role, "facilitator".to_string(), "incorrect role");

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_attendance(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;

        let create_event = CreateEvent {
            name: "test_event".to_string(),
            workflow_id,
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        let new_event = event::create(&pool, &create_event).await?;

        let create_attendance = CreateEventAttendance {
            event_id: new_event.id,
            user_id,
            role: "participant".to_string(),
        };
        let attendance = create(&pool, &create_attendance).await?;

        let update_attendance = UpdateEventAttendance {
            role: Some("facilitator".to_string()),
        };
        let updated_attendance = update(&pool, &attendance.id, &update_attendance).await?;

        assert_eq!(updated_attendance.id, attendance.id, "ids do not match");
        assert_eq!(
            updated_attendance.role,
            "facilitator".to_string(),
            "role was not updated"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_get_attendance_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;

        let create_event = CreateEvent {
            name: "test_event".to_string(),
            workflow_id,
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        let new_event = event::create(&pool, &create_event).await?;

        let create_attendance = CreateEventAttendance {
            event_id: new_event.id,
            user_id,
            role: "participant".to_string(),
        };
        let attendance = create(&pool, &create_attendance).await?;

        let get_attendance = get_by_id(&pool, &attendance.id).await?;

        assert_eq!(get_attendance.id, attendance.id, "ids do not match");

        Ok(())
    }

    #[sqlx::test]
    async fn should_list_attendance(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let user_id_1 = get_random_user_id(&app, &mut session).await?;
        let user_id_2 = get_random_user_id(&app, &mut session).await?;
        let user_id_3 = get_random_user_id(&app, &mut session).await?;

        let create_event = CreateEvent {
            name: "test_event".to_string(),
            workflow_id,
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        let new_event = event::create(&pool, &create_event).await?;

        let create_attendance_1 = CreateEventAttendance {
            event_id: new_event.id,
            user_id: user_id_1,
            role: "participant".to_string(),
        };
        let create_attendance_2 = CreateEventAttendance {
            event_id: new_event.id,
            user_id: user_id_2,
            role: "participant".to_string(),
        };
        let create_attendance_3 = CreateEventAttendance {
            event_id: new_event.id,
            user_id: user_id_3,
            role: "facilitator".to_string(),
        };
        let _ = create(&pool, &create_attendance_1).await?;
        let _ = create(&pool, &create_attendance_2).await?;
        let _ = create(&pool, &create_attendance_3).await?;

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let filter_options = EventAttendanceFilterOptions { event_id: None };
        let order_options = EventAttendanceOrderOptions { created_at: None };
        let results = list(&pool, page_options, filter_options, order_options, None).await?;

        assert_eq!(results.total, 3, "incorrect total");
        assert_eq!(
            results.records[2].role,
            "facilitator".to_string(),
            "incorrect role type"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_filter_attendance_by_event_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let user_id_1 = get_random_user_id(&app, &mut session).await?;
        let user_id_2 = get_random_user_id(&app, &mut session).await?;
        let user_id_3 = get_random_user_id(&app, &mut session).await?;

        let create_event_1 = CreateEvent {
            name: "test_event_1".to_string(),
            workflow_id,
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        let new_event_1 = event::create(&pool, &create_event_1).await?;
        let create_event_2 = CreateEvent {
            name: "test_event_2".to_string(),
            workflow_id,
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        let new_event_2 = event::create(&pool, &create_event_2).await?;

        let create_attendance_1 = CreateEventAttendance {
            event_id: new_event_1.id,
            user_id: user_id_1,
            role: "participant".to_string(),
        };
        let create_attendance_2 = CreateEventAttendance {
            event_id: new_event_1.id,
            user_id: user_id_2,
            role: "participant".to_string(),
        };
        let create_attendance_3 = CreateEventAttendance {
            event_id: new_event_1.id,
            user_id: user_id_3,
            role: "facilitator".to_string(),
        };
        let create_attendance_4 = CreateEventAttendance {
            event_id: new_event_2.id,
            user_id: user_id_1,
            role: "participant".to_string(),
        };
        let create_attendance_5 = CreateEventAttendance {
            event_id: new_event_2.id,
            user_id: user_id_2,
            role: "facilitator".to_string(),
        };
        let _ = create(&pool, &create_attendance_1).await?;
        let _ = create(&pool, &create_attendance_2).await?;
        let _ = create(&pool, &create_attendance_3).await?;
        let _ = create(&pool, &create_attendance_4).await?;
        let _ = create(&pool, &create_attendance_5).await?;

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let filter_options = EventAttendanceFilterOptions {
            event_id: Some(new_event_2.id),
        };
        let order_options = EventAttendanceOrderOptions { created_at: None };
        let results = list(&pool, page_options, filter_options, order_options, None).await?;

        assert_eq!(results.total, 2, "incorrect total");
        assert_eq!(
            results.records[1].role,
            "facilitator".to_string(),
            "incorrect role type"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_attendance(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;

        let create_event = CreateEvent {
            name: "test_event".to_string(),
            workflow_id,
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        let new_event = event::create(&pool, &create_event).await?;

        let create_attendance = CreateEventAttendance {
            event_id: new_event.id,
            user_id,
            role: "participant".to_string(),
        };
        let attendance = create(&pool, &create_attendance).await?;
        let _ = delete(&pool, &attendance.id).await?;

        let err = get_by_id(&pool, &attendance.id).await.unwrap_err();

        match err {
            ComhairleError::ResourceNotFound(e) => {
                assert_eq!(e, "EventAttendance".to_string(), "incorrect error message");
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
