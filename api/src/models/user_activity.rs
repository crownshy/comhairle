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

/// Most inactive seconds a pause can subtract. The browser only reports idle
/// once idle logic has waited out its timeouts so the idle period should be
/// subtracted.
const MAX_INACTIVE_SECONDS: i32 = 90;

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
    /// When the row was last closed. Null while the participant is on the step.
    pub ended_at: Option<DateTime<Utc>>,
    /// Why the row was last closed. Null while it is open.
    pub end_reason: Option<EndReason>,
    /// Active time across every visit
    pub active_seconds: i32,
    /// When `is_active` last changed: the start of the current stretch while
    /// active or when the participant went idle of left.
    pub activity_changed_at: DateTime<Utc>,
    /// Whether the participant is currently active. False once paused or closed.
    pub is_active: bool,
    /// Separate visits to the step. Opening it again after the row was closed counts as one.
    pub sessions: i32,
}

const DEFAULT_COLUMNS: [UserActivityIden; 10] = [
    UserActivityIden::Id,
    UserActivityIden::UserId,
    UserActivityIden::WorkflowStepId,
    UserActivityIden::StartedAt,
    UserActivityIden::EndedAt,
    UserActivityIden::ActiveSeconds,
    UserActivityIden::ActivityChangedAt,
    UserActivityIden::IsActive,
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
///
/// Also used to resume after [`pause`]: a paused row starts a new active stretch,
/// while an already active one keeps its stretch so a reconnect doesn't lose the
/// time so far.
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
                .value(
                    UserActivityIden::ActivityChangedAt,
                    Expr::case(
                        existing(UserActivityIden::IsActive).eq(true),
                        existing(UserActivityIden::ActivityChangedAt),
                    )
                    .finally(Expr::current_timestamp()),
                )
                .value(UserActivityIden::IsActive, true)
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

/// Ends the current active stretch when the participant goes idle or hides the tab,
/// adding it to `active_seconds`.
///
/// `inactive_seconds` is how long they had already been idle when the browser
/// noticed, and is left out of the stretch. It's clamped to `0..=MAX_INACTIVE_SECONDS`.
/// Only an active, open row is paused, so a repeated idle message adds nothing
/// and returns `None`. [`start`] resumes.
#[instrument(err(Debug), skip(db))]
pub async fn pause(
    db: &mut PgConnection,
    user_id: Uuid,
    workflow_step_id: Uuid,
    inactive_seconds: i32,
) -> Result<Option<UserActivity>, ComhairleError> {
    let inactive_seconds = inactive_seconds.clamp(0, MAX_INACTIVE_SECONDS);

    let (sql, values) = Query::update()
        .table(UserActivityIden::Table)
        .values([
            (
                UserActivityIden::ActiveSeconds,
                Expr::cust_with_values(
                    "active_seconds + GREATEST(0, EXTRACT(EPOCH FROM now() - activity_changed_at)::int - $1)",
                    [inactive_seconds]
                )
            ),
            (
                UserActivityIden::ActivityChangedAt,
                Expr::cust_with_values(
                    "GREATEST(activity_changed_at, now() - make_interval(secs => $1))", [inactive_seconds]
                )
            ),
            (UserActivityIden::IsActive, false.into()),
        ])
        .and_where(Expr::col(UserActivityIden::UserId).eq(user_id))
        .and_where(Expr::col(UserActivityIden::WorkflowStepId).eq(workflow_step_id))
        .and_where(Expr::col(UserActivityIden::IsActive).eq(true))
        .and_where(Expr::col(UserActivityIden::EndedAt).is_null())
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let activity = query_as_with(&sql, values)
        .fetch_optional(&mut *db)
        .await
        .resolve_db_err("User Activity")?;

    Ok(activity)
}

/// Closes a user activity record with an `end_reason`.
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
    let now = Utc::now();

    let (sql, values) = Query::update()
        .table(UserActivityIden::Table)
        .values([
            (
                UserActivityIden::ActiveSeconds,
                Expr::cust_with_values(
                    "active_seconds + CASE \
                        WHEN is_active \
                        THEN GREATEST(0, EXTRACT(EPOCH FROM $1 - activity_changed_at)::int) \
                        ELSE 0 \
                        END",
                    [now],
                ),
            ),
            (
                UserActivityIden::ActivityChangedAt,
                Expr::cust_with_values(
                    "CASE WHEN is_active THEN $1 ELSE activity_changed_at END",
                    [now],
                ),
            ),
            (UserActivityIden::EndedAt, now.into()),
            (UserActivityIden::EndReason, end_reason.into()),
            (UserActivityIden::IsActive, false.into()),
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
    pub activity_changed_at: Option<Order>,
    pub active_seconds: Option<Order>,
}

impl UserActivityOrderOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(order) = &self.started_at {
            query = query
                .order_by(UserActivityIden::StartedAt, order.into())
                .to_owned();
        }
        if let Some(order) = &self.activity_changed_at {
            query = query
                .order_by(UserActivityIden::ActivityChangedAt, order.into())
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

        async fn pause(
            &mut self,
            inactive_seconds: i32,
        ) -> Result<Option<UserActivity>, ComhairleError> {
            pause(
                &mut self.conn,
                self.user_id,
                self.workflow_step_id,
                inactive_seconds,
            )
            .await
        }

        async fn close(
            &mut self,
            reason: EndReason,
        ) -> Result<Option<UserActivity>, ComhairleError> {
            close(&mut self.conn, self.user_id, self.workflow_step_id, reason).await
        }

        /// Moves `activity_changed_at` into the past, as if the current stretch
        /// or idle period began `minutes` ago.
        async fn backdate_activity_change(&mut self, minutes: i32) -> sqlx::Result<DateTime<Utc>> {
            sqlx::query_scalar(
                "UPDATE user_activity SET activity_changed_at = now() - make_interval(mins => $1)
                   WHERE user_id = $2 AND workflow_step_id = $3
                   RETURNING activity_changed_at",
            )
            .bind(minutes)
            .bind(self.user_id)
            .bind(self.workflow_step_id)
            .fetch_one(&mut *self.conn)
            .await
        }
    }

    /// Statements run a few milliseconds apart, so elapsed seconds can round either way.
    fn assert_number_within_margin(actual: i32, expected: i32, message: &str) {
        assert!(
            (expected - 2..=expected + 2).contains(&actual),
            "{message}: expected about {expected}, got {actual}"
        );
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
    async fn should_credit_an_active_stretch_on_close(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;
        fixture.start().await?;
        fixture.backdate_activity_change(5).await?;

        let closed = fixture
            .close(EndReason::Finished)
            .await?
            .ok_or("row not closed")?;

        assert_number_within_margin(closed.active_seconds, 300, "the 5 minute stretch");
        assert!(!closed.is_active);
        assert_eq!(Some(closed.activity_changed_at), closed.ended_at);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_keep_current_stretch_when_started_while_active(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;
        fixture.start().await?;
        let stretch_start = fixture.backdate_activity_change(10).await?;

        let activity = fixture.start().await?;

        assert!(activity.is_active);
        assert_eq!(activity.activity_changed_at, stretch_start);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_credit_the_stretch_minus_inactive_time(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;
        fixture.start().await?;
        fixture.backdate_activity_change(10).await?;

        let paused = fixture.pause(60).await?.ok_or("row not paused")?;

        assert_number_within_margin(
            paused.active_seconds,
            540,
            "10 minutes less 60 idle seconds",
        );
        assert!(!paused.is_active);
        assert!(paused.ended_at.is_none(), "pausing doesn't close activity");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_clamp_inactive_seconds(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;
        fixture.start().await?;
        fixture.backdate_activity_change(10).await?;

        let after_huge = fixture.pause(10_000).await?.ok_or("row not paused")?;

        assert_number_within_margin(
            after_huge.active_seconds,
            600 - MAX_INACTIVE_SECONDS,
            "should subtract at most MAX_INACTIVE_SECONDS",
        );

        fixture.start().await?;
        fixture.backdate_activity_change(10).await?;
        let after_negative = fixture.pause(-50).await?.ok_or("row not paused")?;

        assert_number_within_margin(
            after_negative.active_seconds,
            after_huge.active_seconds + 600,
            "negative should add no extract time",
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_count_a_pause_once(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;
        fixture.start().await?;
        fixture.backdate_activity_change(10).await?;

        let first = fixture.pause(0).await?.ok_or("row not paused")?;
        let second = fixture.pause(0).await?;

        assert!(second.is_none(), "an already paused row shouldn't change");
        assert_eq!(
            first.active_seconds,
            fixture.get().await?.active_seconds,
            "active seconds don't match"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_resume_without_new_session_after_pause(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;
        fixture.start().await?;
        fixture.backdate_activity_change(10).await?;

        let paused = fixture.pause(0).await?.ok_or("row not paused")?;
        let resumed = fixture.start().await?;

        assert!(resumed.is_active);
        assert_eq!(resumed.sessions, 1, "coming back from idle isn't a return");
        assert_eq!(
            resumed.active_seconds, paused.active_seconds,
            "active_seconds don't match"
        );
        assert!(resumed.activity_changed_at > paused.activity_changed_at);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_count_new_session_and_reopen_after_close(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;
        let first = fixture.start().await?;
        fixture.backdate_activity_change(5).await?;
        let closed = fixture
            .close(EndReason::Navigation)
            .await?
            .ok_or("row not closed")?;

        let returned = fixture.start().await?;

        assert_eq!(returned.sessions, 2, "should count new session");
        assert!(returned.is_active);
        assert!(returned.ended_at.is_none(), "should have re-opened");
        assert!(returned.end_reason.is_none());
        assert_eq!(
            returned.started_at, first.started_at,
            "should keep same started_at"
        );
        assert_eq!(
            returned.active_seconds, closed.active_seconds,
            "should not update active seconds on re-open"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_maintain_the_active_time_when_closing_a_paused_row(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;
        fixture.start().await?;
        fixture.backdate_activity_change(10).await?;
        let paused = fixture.pause(0).await?.ok_or("row not paused")?;

        let closed = fixture
            .close(EndReason::Disconnected)
            .await?
            .ok_or("row not closed")?;

        assert_eq!(
            closed.active_seconds, paused.active_seconds,
            "idle time isn't active"
        );
        assert_eq!(closed.activity_changed_at, paused.activity_changed_at);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_keep_the_first_reason_when_closed_twice(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;
        fixture.start().await?;

        fixture.close(EndReason::Finished).await?;
        let second = fixture.close(EndReason::Disconnected).await?;

        assert!(second.is_none(), "an already closed row shouldn't change");
        assert_eq!(fixture.get().await?.end_reason, Some(EndReason::Finished));

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_not_pause_a_closed_row(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut fixture = ActivityFixture::new(&pool).await?;
        fixture.start().await?;
        fixture.close(EndReason::Navigation).await?;

        assert!(fixture.pause(0).await?.is_none());

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
