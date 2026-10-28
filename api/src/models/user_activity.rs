use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{
    Expr, Keyword, OnConflict, PostgresQueryBuilder, Query, SelectStatement, enum_def,
};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::{PgConnection, PgPool, query_as_with};
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::SqlxResultExt;
use crate::models::pagination::{Order, PageOptions, PaginatedResults};

/// Most active seconds one heartbeat can add. The browser counts its own active time, so a
/// client bug (a laptop waking from sleep, a counter not reset after a reconnect) could report
/// far more time than has passed. 45 is the 30 second heartbeat interval plus some slack.
const MAX_SECONDS_PER_HEARTBEAT: i32 = 45;

/// One participant's activity on one workflow step. There is a single row per participant
/// and step, however many times they visit it. Whether the step is done lives in
/// `user_progress`, not here.
#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "user_activity")]
pub struct UserActivity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub workflow_step_id: Uuid,
    /// When the participant first opened the step.
    pub started_at: DateTime<Utc>,
    /// The last start or heartbeat. Drives "active now" and the idle sweeper.
    pub last_active_at: DateTime<Utc>,
    /// When the row was last closed. Null while the participant is on the step.
    pub ended_at: Option<DateTime<Utc>>,
    /// Active time across every visit, added up from heartbeats.
    pub active_seconds: i32,
    /// Separate visits to the step. Opening it again after the row was closed counts as one.
    pub sessions: i32,
    /// Why the row was last closed. Null while it is open.
    pub end_reason: Option<EndReason>,
}

const DEFAULT_COLUMNS: [UserActivityIden; 9] = [
    UserActivityIden::Id,
    UserActivityIden::UserId,
    UserActivityIden::WorkflowStepId,
    UserActivityIden::StartedAt,
    UserActivityIden::LastActiveAt,
    UserActivityIden::EndedAt,
    UserActivityIden::ActiveSeconds,
    UserActivityIden::Sessions,
    UserActivityIden::EndReason,
];

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EndReason {
    IdleTimeout,
    Navigation,
    Finished,
    Disconnected,
}

impl From<EndReason> for sea_query::Value {
    fn from(value: EndReason) -> Self {
        sea_query::Value::String(Some(Box::new(value.to_string())))
    }
}

impl std::fmt::Display for EndReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EndReason::IdleTimeout => "idle_timeout",
            EndReason::Navigation => "navigation",
            EndReason::Finished => "finished",
            EndReason::Disconnected => "disconnected",
        };
        write!(f, "{}", value)
    }
}

/// Records that a participant opened a step, creating their row on the first visit.
///
/// On a later visit the existing row is reopened. `sessions` only goes up if the row had been
/// closed, so a reconnect or a second tab during an open visit doesn't count as a return.
#[instrument(err(Debug), skip(db))]
pub async fn start(
    db: &mut PgConnection,
    user_id: Uuid,
    workflow_step_id: Uuid,
) -> Result<UserActivity, ComhairleError> {
    let columns = vec![UserActivityIden::UserId, UserActivityIden::WorkflowStepId];
    let values = vec![user_id.into(), workflow_step_id.into()];

    // Qualified with the table: inside ON CONFLICT a bare column name is ambiguous between
    // the existing row and the proposed one, and Postgres rejects the query.
    let existing = |column| Expr::col((UserActivityIden::Table, column));
    let returned_after_ending =
        Expr::case(existing(UserActivityIden::EndedAt).is_not_null(), 1).finally(0);

    let (sql, values) = Query::insert()
        .into_table(UserActivityIden::Table)
        .columns(columns)
        .values(values)?
        .on_conflict(
            OnConflict::columns([UserActivityIden::UserId, UserActivityIden::WorkflowStepId])
                .value(
                    UserActivityIden::Sessions,
                    existing(UserActivityIden::Sessions).add(returned_after_ending),
                )
                .value(UserActivityIden::EndedAt, Keyword::Null)
                .value(UserActivityIden::EndReason, Keyword::Null)
                .value(UserActivityIden::LastActiveAt, Expr::current_timestamp())
                .to_owned(),
        )
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let activity = query_as_with(&sql, values)
        .fetch_one(&mut *db)
        .await
        .resolve_db_err("User Activity")?;

    Ok(activity)
}

/// Closes a user activity record with an `end_reason`.
///
/// `ended_at` is the last activity rather than now, so time between the last heartbeat and the
/// close (a timeout, a dropped connection) isn't counted.
///
/// Only an open row is closed: when finishing, the idle sweeper and a disconnect
/// race, the first reason wins and the rest return `None`, preventing cases
/// where a user finishes a step then closes their browser tab resulting in
/// `EndReason::Finished` being overwritten by `EndReason::Disconnected`.
#[instrument(err(Debug), skip(db))]
pub async fn close(
    db: &mut PgConnection,
    user_id: Uuid,
    workflow_step_id: Uuid,
    end_reason: EndReason,
) -> Result<Option<UserActivity>, ComhairleError> {
    let (sql, values) = Query::update()
        .table(UserActivityIden::Table)
        .values([
            (
                UserActivityIden::EndedAt,
                Expr::col(UserActivityIden::LastActiveAt).into(),
            ),
            (UserActivityIden::EndReason, end_reason.into()),
        ])
        .and_where(Expr::col(UserActivityIden::UserId).eq(user_id))
        .and_where(Expr::col(UserActivityIden::WorkflowStepId).eq(workflow_step_id))
        .and_where(Expr::col(UserActivityIden::EndedAt).is_null())
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let activity = query_as_with(&sql, values)
        .fetch_optional(&mut *db)
        .await
        .resolve_db_err("User Activity")?;

    Ok(activity)
}

#[instrument(err(Debug), skip(db))]
pub async fn record_heartbeat(
    db: &mut PgConnection,
    user_id: Uuid,
    workflow_step_id: Uuid,
    reported_seconds: i32,
) -> Result<UserActivity, ComhairleError> {
    let credited_seconds = reported_seconds.clamp(0, MAX_SECONDS_PER_HEARTBEAT);

    // set ended_at and end_reason to null in case one browser tab is idle, times
    // out and closes the record but another tab is reporting heartbeat
    let (sql, values) = Query::update()
        .table(UserActivityIden::Table)
        .values([
            (
                UserActivityIden::ActiveSeconds,
                Expr::col(UserActivityIden::ActiveSeconds).add(credited_seconds),
            ),
            (
                UserActivityIden::LastActiveAt,
                Expr::current_timestamp().into(),
            ),
            (UserActivityIden::EndedAt, Expr::cust("NULL")),
            (UserActivityIden::EndReason, Expr::cust("NULL")),
        ])
        .and_where(Expr::col(UserActivityIden::UserId).eq(user_id))
        .and_where(Expr::col(UserActivityIden::WorkflowStepId).eq(workflow_step_id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let activity = query_as_with(&sql, values)
        .fetch_one(&mut *db)
        .await
        .resolve_db_err("User Activity")?;

    Ok(activity)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &mut PgConnection, id: Uuid) -> Result<UserActivity, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(UserActivityIden::Table)
        .and_where(Expr::col(UserActivityIden::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    let activity = query_as_with(&sql, values)
        .fetch_one(&mut *db)
        .await
        .resolve_db_err("User Activity")?;

    Ok(activity)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_for_user_step(
    db: &mut PgConnection,
    user_id: Uuid,
    workflow_step_id: Uuid,
) -> Result<UserActivity, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(UserActivityIden::Table)
        .and_where(Expr::col(UserActivityIden::UserId).eq(user_id))
        .and_where(Expr::col(UserActivityIden::WorkflowStepId).eq(workflow_step_id))
        .build_sqlx(PostgresQueryBuilder);

    let activity = query_as_with(&sql, values)
        .fetch_one(&mut *db)
        .await
        .resolve_db_err("User Activity")?;

    Ok(activity)
}

#[derive(Deserialize, Debug, Default, JsonSchema)]
pub struct UserActivityFilterOptions {
    pub workflow_step_id: Option<Uuid>,
    pub end_reason: Option<EndReason>,
}

impl UserActivityFilterOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(value) = self.workflow_step_id {
            query = query
                .and_where(Expr::col(UserActivityIden::WorkflowStepId).eq(value))
                .to_owned();
        }
        if let Some(value) = self.end_reason {
            query = query
                .and_where(Expr::col(UserActivityIden::EndReason).eq(value))
                .to_owned();
        }

        query
    }
}

#[derive(Deserialize, Debug, Default, JsonSchema)]
pub struct UserActivityOrderOptions {
    pub started_at: Option<Order>,
    pub last_active_at: Option<Order>,
    pub active_seconds: Option<Order>,
}

impl UserActivityOrderOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(order) = &self.started_at {
            query = query
                .order_by(UserActivityIden::StartedAt, order.into())
                .to_owned();
        }
        if let Some(order) = &self.last_active_at {
            query = query
                .order_by(UserActivityIden::LastActiveAt, order.into())
                .to_owned();
        }
        if let Some(order) = &self.active_seconds {
            query = query
                .order_by(UserActivityIden::ActiveSeconds, order.into())
                .to_owned();
        }

        query
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn list_for_user(
    db: &PgPool,
    user_id: Uuid,
    page_options: PageOptions,
    filter_options: UserActivityFilterOptions,
    order_options: UserActivityOrderOptions,
) -> Result<PaginatedResults<UserActivity>, ComhairleError> {
    let query = Query::select()
        .from(UserActivityIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(UserActivityIden::UserId).eq(user_id))
        .to_owned();

    let query = filter_options.apply(query);
    let query = order_options.apply(query);

    let activity = page_options.fetch_paginated_results(db, query).await?;

    Ok(activity)
}

#[instrument(err(Debug), skip(db))]
pub async fn delete(db: &mut PgConnection, id: Uuid) -> Result<UserActivity, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(UserActivityIden::Table)
        .and_where(Expr::col(UserActivityIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let activity = query_as_with(&sql, values)
        .fetch_one(&mut *db)
        .await
        .resolve_db_err("User Activity")?;

    Ok(activity)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use sqlx::Postgres;
    use sqlx::pool::PoolConnection;

    use super::*;
    use crate::models::model_test_helpers::{
        get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
    };
    use crate::models::users;
    use crate::routes::workflow_steps::dto::WorkflowStepDto;

    struct ActivityFixture {
        conn: PoolConnection<Postgres>,
        user_id: Uuid,
        workflow_step_id: Uuid,
    }

    impl ActivityFixture {
        async fn new(pool: &PgPool) -> Result<Self, Box<dyn Error>> {
            let (app, mut session) = setup_default_app_and_session(pool).await?;
            let conversation_id = get_random_conversation_id(&app, &mut session).await?;
            let workflow_id = get_random_workflow_id(&app, &mut session, &conversation_id).await?;
            let step = session
                .create_random_workflow_steps(
                    &app,
                    &conversation_id.to_string(),
                    &workflow_id.to_string(),
                    1,
                )
                .await?
                .into_iter()
                .next()
                .ok_or("no workflow steps created")?;
            let step: WorkflowStepDto = serde_json::from_value(step)?;
            let user = users::create_guest_user(pool).await?;

            Ok(Self {
                conn: pool.acquire().await?,
                user_id: user.id,
                workflow_step_id: step.id,
            })
        }

        async fn start(&mut self) -> Result<UserActivity, ComhairleError> {
            start(&mut self.conn, self.user_id, self.workflow_step_id).await
        }

        async fn get(&mut self) -> Result<UserActivity, ComhairleError> {
            get_for_user_step(&mut self.conn, self.user_id, self.workflow_step_id).await
        }

        async fn heartbeat(&mut self, seconds: i32) -> Result<UserActivity, ComhairleError> {
            record_heartbeat(&mut self.conn, self.user_id, self.workflow_step_id, seconds).await
        }

        async fn close(
            &mut self,
            reason: EndReason,
        ) -> Result<Option<UserActivity>, ComhairleError> {
            close(&mut self.conn, self.user_id, self.workflow_step_id, reason).await
        }

        /// Moves the last activity into the past, as if the participant stopped sending
        /// heartbeats `minutes` ago.
        async fn backdate_last_active(&mut self, minutes: i32) -> sqlx::Result<DateTime<Utc>> {
            sqlx::query_scalar(
                "UPDATE user_activity SET last_active_at = now() - make_interval(mins => $1)
                   WHERE user_id = $2 AND workflow_step_id = $3
                   RETURNING last_active_at",
            )
            .bind(minutes)
            .bind(self.user_id)
            .bind(self.workflow_step_id)
            .fetch_one(&mut *self.conn)
            .await
        }
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_start_an_open_first_session(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;

        let activity = fixture.start().await?;

        assert_eq!(activity.sessions, 1, "first visit should be one session");
        assert_eq!(
            activity.active_seconds, 0,
            "should start with 0 active seconds"
        );
        assert!(activity.ended_at.is_none(), "should be open");
        assert!(activity.end_reason.is_none(), "should have no end_reason");

        Ok(())
    }

    /// A reconnect or a second tab sends `start` again while the row is still open.
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_not_count_a_new_session_when_started_while_open(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;

        let first = fixture.start().await?;
        let second = fixture.start().await?;

        assert_eq!(second.id, first.id, "should reuse the same row");
        assert_eq!(second.sessions, 1, "a reconnect isn't a return");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_count_a_return_and_reopen_when_started_after_close(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;

        let first = fixture.start().await?;
        fixture.heartbeat(30).await?;
        fixture.close(EndReason::Navigation).await?;
        let returned = fixture.start().await?;

        assert_eq!(returned.sessions, 2, "should count the return");
        assert!(returned.ended_at.is_none(), "should be reopened");
        assert!(returned.end_reason.is_none(), "end_reason should be None");
        assert_eq!(
            returned.started_at, first.started_at,
            "should keep the first start"
        );
        assert_eq!(returned.active_seconds, 30, "should keep earlier time");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_add_reported_seconds_on_each_heartbeat(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;
        fixture.start().await?;

        fixture.heartbeat(30).await?;
        let activity = fixture.heartbeat(20).await?;

        assert_eq!(activity.active_seconds, 50);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_clamp_reported_seconds(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;
        fixture.start().await?;

        let after_negative = fixture.heartbeat(-10).await?;
        assert_eq!(
            after_negative.active_seconds, 0,
            "negative should add nothing"
        );

        let after_huge = fixture.heartbeat(10_000).await?;
        assert_eq!(after_huge.active_seconds, MAX_SECONDS_PER_HEARTBEAT);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_mark_active_now_on_heartbeat(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;
        fixture.start().await?;
        let earlier = fixture.backdate_last_active(10).await?;

        let activity = fixture.heartbeat(5).await?;

        assert!(
            activity.last_active_at > earlier,
            "should move last_active_at forward"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_reopen_without_a_new_session_on_heartbeat_after_close(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;
        fixture.start().await?;
        fixture.close(EndReason::Disconnected).await?;

        let activity = fixture.heartbeat(10).await?;

        assert!(activity.ended_at.is_none(), "should be reopened");
        assert!(activity.end_reason.is_none());
        assert_eq!(activity.sessions, 1);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_not_find_a_row_for_a_heartbeat_before_start(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;

        let result = fixture.heartbeat(10).await;

        assert!(matches!(result, Err(ComhairleError::ResourceNotFound(_))));

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_record_the_reason_on_close(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut fixutre = ActivityFixture::new(&pool).await?;
        fixutre.start().await?;

        let closed = fixutre
            .close(EndReason::Disconnected)
            .await?
            .ok_or("no activity")?;

        assert_eq!(closed.end_reason, Some(EndReason::Disconnected));
        assert!(closed.ended_at.is_some());

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_end_at_the_last_activity_not_the_close(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;
        fixture.start().await?;
        let last_active = fixture.backdate_last_active(10).await?;

        let closed = fixture
            .close(EndReason::IdleTimeout)
            .await?
            .ok_or("no activity")?;

        assert_eq!(closed.ended_at, Some(last_active), "times don't match");
        assert_eq!(closed.last_active_at, last_active, "closing isn't activity");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_not_find_a_row_for_a_step_never_opened(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;

        let result = fixture.get().await;

        assert!(matches!(result, Err(ComhairleError::ResourceNotFound(_))));

        Ok(())
    }
}
