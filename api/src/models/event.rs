use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use comhairle_macros::{DbJsonBEnum, Translatable};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{enum_def, Alias, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::{FromRow, Type},
    Decode, Encode, PgPool, Postgres,
};
use sqlx_postgres::{PgArgumentBuffer, PgValueRef};
use std::str::FromStr;
use tracing::{instrument, warn};
use uuid::Uuid;

#[cfg(test)]
use fake::Dummy;

use crate::{
    config::ComhairleConfig,
    error::ComhairleError,
    mailer::build_calendar_invite,
    models::{
        otp,
        pagination::{Order, PageOptions, PaginatedResults},
        scheduled_email::{self, CreateScheduledEmail, EmailTemplate, ScheduledEmailConfig},
        translations::{new_translation, TextContentId, TextFormat},
        users::User,
    },
    routes::auth::{generate_jwt, OtpClaims},
};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone, PartialEq)]
pub struct BasicEventAgendaItem {
    pub title: String,
    pub description: String,
    pub estimated_time: u32,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone, PartialEq)]
pub struct BreakoutRoomAgendaItem {
    pub prompt: String,
    pub instructions: String,
    pub estimated_time: u32,
    pub time_limit: Option<u32>,
    pub max_per_room: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone, PartialEq)]
pub enum EventAgendaItem {
    Basic(BasicEventAgendaItem),
    BreakoutRoom(BreakoutRoomAgendaItem),
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, DbJsonBEnum, Clone, PartialEq)]
#[serde(transparent)]
pub struct EventAgenda(pub Vec<EventAgendaItem>);

impl Default for EventAgenda {
    fn default() -> Self {
        Self(Vec::new())
    }
}

#[derive(Serialize, Deserialize, Partial, Debug, FromRow, Clone, JsonSchema, Translatable)]
#[enum_def(table_name = "event")]
#[partially(derive(Serialize, Deserialize, Debug, JsonSchema, Default))]
pub struct Event {
    #[partially(omit)]
    pub id: Uuid,
    pub name: TextContentId,
    pub description: TextContentId,
    #[partially(transparent)]
    pub capacity: Option<i32>,
    #[partially(omit)]
    pub conversation_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub signup_mode: String,
    #[partially(omit)]
    pub video_meeting_id: Option<Uuid>,
    #[serde(default)]
    pub agenda: EventAgenda,
    pub default_time_zone: String,
    pub format: EventFormat,
    #[partially(transparent)]
    pub location: Option<EventLocation>,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq, Default)]
pub struct EventLocation {
    pub venue_name: String,
    pub city: String,
    pub state_province: String,
    pub postal_code: String,
    pub country_code: String,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub address_line_3: Option<String>,
}

impl Type<Postgres> for EventLocation {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        <serde_json::Value as Type<Postgres>>::type_info()
    }
}

impl<'q> Encode<'q, Postgres> for EventLocation {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let json = serde_json::to_value(self)?;
        <serde_json::Value as Encode<Postgres>>::encode(json, buf)
    }
}

impl<'r> Decode<'r, Postgres> for EventLocation {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let json: serde_json::Value = Decode::<Postgres>::decode(value)?;
        Ok(serde_json::from_value(json)?)
    }
}

impl From<EventLocation> for sea_query::Value {
    fn from(l: EventLocation) -> Self {
        Self::Json(Some(Box::new(
            // `expect` should be safe here as serialization should fail at the api
            // layer if invalid
            serde_json::to_value(l).expect("EventLocation serialization failed"),
        )))
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, sqlx::Type, Clone, JsonSchema)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(Dummy))]
pub enum EventFormat {
    #[sqlx(rename = "online")]
    Online,
    #[sqlx(rename = "in_person")]
    InPerson,
}

impl From<EventFormat> for sea_query::Value {
    fn from(val: EventFormat) -> Self {
        sea_query::Value::String(Some(Box::new(val.to_string())))
    }
}

impl std::fmt::Display for EventFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EventFormat::Online => "online",
            EventFormat::InPerson => "in_person",
        };
        write!(f, "{}", value)
    }
}

pub trait ResolveTimeZone {
    fn default_time_zone(&self) -> &str;

    fn resolve_time_zone(&self) -> Tz {
        Tz::from_str(self.default_time_zone()).unwrap_or(chrono_tz::UTC)
    }

    fn format_date_with_time_zone(&self, date: DateTime<Utc>, fmt: Option<&str>) -> String {
        date.with_timezone(&self.resolve_time_zone())
            .format(fmt.unwrap_or("%B %d, %Y at %H:%M %Z"))
            .to_string()
    }
}

impl ResolveTimeZone for Event {
    fn default_time_zone(&self) -> &str {
        &self.default_time_zone
    }
}
impl ResolveTimeZone for LocalizedEvent {
    fn default_time_zone(&self) -> &str {
        &self.default_time_zone
    }
}

struct ScheduleEventEmail<'a> {
    db: &'a PgPool,
    config: &'a ComhairleConfig,
    user: &'a User,
    email: &'a str,
    event_path: &'a str,
    calendar_invite: &'a icalendar::Calendar,
}

impl LocalizedEvent {
    pub async fn schedule_event_reminders(
        &self,
        db: &PgPool,
        config: &ComhairleConfig,
        user: &User,
    ) -> Result<(), ComhairleError> {
        if self.start_time <= Utc::now() {
            warn!("Event has past");
            return Ok(());
        }

        let calendar_invite = build_calendar_invite(
            &self.name,
            &self.description,
            self.start_time,
            self.end_time,
        );

        let email = user.email.as_ref().ok_or(ComhairleError::WrongUserType)?;
        let event_path = format!("/conversations/{}/events/{}", self.conversation_id, self.id);

        let params = ScheduleEventEmail {
            db,
            config,
            user,
            email,
            event_path: &event_path,
            calendar_invite: &calendar_invite,
        };

        self.schedule_first_reminder(&params).await?;
        self.schedule_second_reminder(&params).await?;

        Ok(())
    }

    async fn schedule_first_reminder(
        &self,
        params: &ScheduleEventEmail<'_>,
    ) -> Result<(), ComhairleError> {
        let ScheduleEventEmail {
            db,
            user: _user,
            config,
            email,
            event_path,
            calendar_invite,
        } = params;

        let email_config = ScheduledEmailConfig {
            subject: "Upcoming event reminder".to_string(),
            template: EmailTemplate::EventReminder {
                event_name: self.name.clone(),
                event_time: self.format_date_with_time_zone(self.start_time, None),
                event_link: format!("{}{}", &config.domain, event_path),
                organization_name: "Bloom".to_string(), // TODO:
                organization_email: None,
            },
            attachment: Some(calendar_invite.to_string()),
        };
        let scheduled_email_params = CreateScheduledEmail {
            user_email: email.to_string(),
            email_config,
            send_at: self.start_time - chrono::Duration::days(1),
        };
        scheduled_email::create(db, scheduled_email_params).await?;

        Ok(())
    }

    async fn schedule_second_reminder(
        &self,
        params: &ScheduleEventEmail<'_>,
    ) -> Result<(), ComhairleError> {
        let ScheduleEventEmail {
            db,
            user,
            config,
            email,
            event_path,
            calendar_invite,
        } = params;

        let event_live_path = format!("{event_path}/live");

        let otp = otp::create(db, &user.id, Some(event_live_path), Some(self.start_time)).await?;

        let claims = OtpClaims {
            email: email.to_string(),
            otp: otp.code.clone(),
        };
        // Ensure JWT doesn't expire before event begins
        let event_jwt_duration = self.start_time - Utc::now();
        let otp_token = generate_jwt()
            .user(user)
            .secret(&config.jwt_secret)
            .custom_claims(claims)
            .duration(event_jwt_duration)
            .call();

        let encoded_redirect_url = urlencoding::encode(&otp.redirect_url);
        let otp_link = format!(
            "{}/auth/login-otp/{}?backTo={}",
            config.domain, otp_token, encoded_redirect_url
        );

        let email_config = ScheduledEmailConfig {
            subject: "Event beginning soon".to_string(),
            template: EmailTemplate::EventReminder {
                event_name: self.name.clone(),
                event_time: self.format_date_with_time_zone(self.start_time, None),
                event_link: otp_link,
                organization_name: "Bloom".to_string(), // TODO:
                organization_email: None,
            },
            attachment: Some(calendar_invite.to_string()),
        };
        let scheduled_email_params = CreateScheduledEmail {
            user_email: email.to_string(),
            email_config,
            send_at: self.start_time - chrono::Duration::hours(2),
        };

        scheduled_email::create(db, scheduled_email_params).await?;

        Ok(())
    }
}

const DEFAULT_COLUMNS: [EventIden; 15] = [
    EventIden::Id,
    EventIden::Name,
    EventIden::Description,
    EventIden::Capacity,
    EventIden::ConversationId,
    EventIden::StartTime,
    EventIden::EndTime,
    EventIden::SignupMode,
    EventIden::VideoMeetingId,
    EventIden::Agenda,
    EventIden::DefaultTimeZone,
    EventIden::Location,
    EventIden::Format,
    EventIden::CreatedAt,
    EventIden::UpdatedAt,
];

#[derive(Serialize, Deserialize, JsonSchema, Debug, Default)]
pub struct CreateEvent {
    pub name: String,
    pub description: String,
    pub capacity: Option<i32>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub signup_mode: String,
    pub agenda: Option<EventAgenda>,
    pub location: Option<EventLocation>,
    pub default_time_zone: Option<String>,
}

impl CreateEvent {
    pub fn columns(&self) -> Vec<EventIden> {
        let mut columns = vec![
            EventIden::StartTime,
            EventIden::EndTime,
            EventIden::SignupMode,
        ];

        if self.capacity.is_some() {
            columns.push(EventIden::Capacity);
        }

        if self.agenda.is_some() {
            columns.push(EventIden::Agenda)
        }

        if self.default_time_zone.is_some() {
            columns.push(EventIden::DefaultTimeZone)
        }

        columns
    }

    pub fn values(&self) -> Vec<sea_query::SimpleExpr> {
        let mut values = vec![
            self.start_time.into(),
            self.end_time.into(),
            self.signup_mode.to_owned().into(),
        ];

        if let Some(value) = self.capacity {
            values.push(value.into());
        }

        if let Some(ref value) = self.agenda {
            values.push(value.into());
        }

        if let Some(ref value) = self.default_time_zone {
            values.push(value.into());
        }

        values
    }
}

#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    conversation_id: &Uuid,
    new_event: &CreateEvent,
) -> Result<Event, ComhairleError> {
    let mut columns = new_event.columns();
    let mut values = new_event.values();

    columns.push(EventIden::ConversationId);
    values.push((*conversation_id).into());

    let name = new_translation(db, "en", &new_event.name, TextFormat::Plain).await?;
    let description = new_translation(db, "en", &new_event.description, TextFormat::Plain).await?;

    columns.push(EventIden::Name);
    values.push(name.id.into());

    columns.push(EventIden::Description);
    values.push(description.id.into());

    columns.push(EventIden::VideoMeetingId);
    values.push(Uuid::new_v4().into());

    if let Some(location) = &new_event.location {
        let location_json = serde_json::to_value(location)?;
        columns.push(EventIden::Location);
        values.push(location_json.into());
    }

    let (sql, values) = Query::insert()
        .into_table(EventIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let event = sqlx::query_as_with::<_, Event, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(event)
}

impl PartialEvent {
    pub fn to_values(&self) -> Vec<(EventIden, sea_query::SimpleExpr)> {
        let mut values = vec![];
        if let Some(value) = &self.name {
            values.push((EventIden::Name, value.into()));
        }
        if let Some(value) = &self.description {
            values.push((EventIden::Description, value.into()));
        }
        if let Some(value) = &self.capacity {
            values.push((EventIden::Capacity, (*value).into()));
        }
        if let Some(value) = &self.start_time {
            values.push((EventIden::StartTime, (*value).into()));
        }
        if let Some(value) = &self.end_time {
            values.push((EventIden::EndTime, (*value).into()));
        }
        if let Some(value) = &self.signup_mode {
            values.push((EventIden::SignupMode, value.into()));
        }
        if let Some(value) = &self.agenda {
            values.push((EventIden::Agenda, value.into()));
        }
        if let Some(value) = &self.location {
            values.push((EventIden::Location, value.clone().into()));
        }
        if let Some(value) = &self.format {
            values.push((EventIden::Format, value.clone().into()));
        }
        if let Some(value) = &self.default_time_zone {
            values.push((EventIden::DefaultTimeZone, value.into()));
        }

        values
    }
}

#[instrument(err(Debug))]
pub async fn update(
    db: &PgPool,
    id: &Uuid,
    update_event: &PartialEvent,
) -> Result<Event, ComhairleError> {
    let values = update_event.to_values();

    if values.is_empty() {
        return Err(ComhairleError::NoValidUpdates);
    }

    let (sql, values) = Query::update()
        .table(EventIden::Table)
        .values(values)
        .and_where(Expr::col(EventIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let event = sqlx::query_as_with::<_, Event, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(event)
}

#[derive(Deserialize, Debug, Default, JsonSchema)]
pub struct EventOrderOptions {
    name: Option<Order>,
    created_at: Option<Order>,
    start_time: Option<Order>,
}

impl EventOrderOptions {
    fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(order) = &self.created_at {
            query = query
                .order_by((EventIden::Table, EventIden::CreatedAt), order.into())
                .to_owned();
        }
        if let Some(order) = &self.start_time {
            query = query
                .order_by((EventIden::Table, EventIden::StartTime), order.into())
                .to_owned();
        }
        query
    }

    fn apply_to_localized(
        &self,
        mut query: sea_query::SelectStatement,
    ) -> sea_query::SelectStatement {
        use crate::models::translations::TextTranslationIden;
        use sea_query::Alias;

        if let Some(order) = &self.name {
            let tt_name_alias = Alias::new("tt_name");
            query = query
                .order_by((tt_name_alias, TextTranslationIden::Content), order.into())
                .to_owned();
        }
        self.apply(query)
    }
}

#[derive(Deserialize, Debug, Default, JsonSchema)]
pub struct EventFilterOptions {
    pub conversation_id: Option<Uuid>,
    pub time_status: Option<TimeStatus>,
    pub capacity_status: Option<CapacityStatus>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum TimeStatus {
    Past,
    Future,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum CapacityStatus {
    Full,
    Available,
}

impl EventFilterOptions {
    fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(conversation_id) = self.conversation_id {
            query = query
                .and_where(
                    Expr::col((EventIden::Table, EventIden::ConversationId)).eq(conversation_id),
                )
                .to_owned();
        }

        if let Some(time_status) = &self.time_status {
            match time_status {
                TimeStatus::Past => {
                    query = query
                        .and_where(Expr::col(EventIden::StartTime).lt(
                            sea_query::SimpleExpr::Value(sea_query::Value::ChronoDateTime(Some(
                                Box::new(Utc::now().naive_utc()),
                            ))),
                        ))
                        .to_owned()
                }
                TimeStatus::Future => {
                    query = query
                        .and_where(Expr::col(EventIden::StartTime).gt(
                            sea_query::SimpleExpr::Value(sea_query::Value::ChronoDateTime(Some(
                                Box::new(Utc::now().naive_utc()),
                            ))),
                        ))
                        .to_owned()
                }
            }
        }

        if let Some(capacity_status) = &self.capacity_status {
            match capacity_status {
                CapacityStatus::Full => {
                    query = query
                        .and_where(Expr::cust(
                            "(event.capacity IS NOT NULL AND
                            (SELECT COUNT(*)
                            FROM event_attendance
                            WHERE event_attendance.event_id = event.id)
                            >= event.capacity
                        )",
                        ))
                        .to_owned();
                }
                CapacityStatus::Available => {
                    query = query
                        .and_where(Expr::cust(
                            "(event.capacity IS NULL OR
                            (SELECT COUNT(*)
                            FROM event_attendance
                            WHERE event_attendance.event_id = event.id)
                            < event.capacity
                        )",
                        ))
                        .to_owned();
                }
            }
        }

        query
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct LocalizedEventWithAttendance {
    #[sqlx(flatten)]
    #[serde(flatten)]
    pub event: LocalizedEvent,
    pub current_attendance: i64,
}

#[instrument(err(Debug))]
pub async fn list(
    db: &PgPool,
    conversation_id: &Uuid,
    page_options: PageOptions,
    filter_options: EventFilterOptions,
    order_options: EventOrderOptions,
    locale: Option<String>,
) -> Result<PaginatedResults<LocalizedEventWithAttendance>, ComhairleError> {
    let query = Query::select()
        .from(EventIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (EventIden::Table, col)))
        .and_where(Expr::col(EventIden::ConversationId).eq(*conversation_id))
        .to_owned();

    // Add current_attendance computed column using subquery
    let query = add_current_attendance(query);

    let query = LocalizedEvent::query_to_localisation(query, &locale.unwrap_or("en".into()));

    let query = filter_options.apply(query);
    let query = order_options.apply_to_localized(query);

    let events = page_options.fetch_paginated_results(db, query).await?;

    Ok(events)
}

#[instrument(err(Debug))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<Event, ComhairleError> {
    let query = Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (EventIden::Table, col)))
        .from(EventIden::Table)
        .and_where(Expr::col((EventIden::Table, EventIden::Id)).eq(id.to_owned()))
        .to_owned();

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let event = sqlx::query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ComhairleError::ResourceNotFound("Event".into()),
            other => ComhairleError::DatabaseError(other),
        })?;

    Ok(event)
}

#[instrument(err(Debug))]
pub async fn get_localized_by_id(
    db: &PgPool,
    id: &Uuid,
    locale: &str,
) -> Result<LocalizedEvent, ComhairleError> {
    let query = Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (EventIden::Table, col)))
        .from(EventIden::Table)
        .and_where(Expr::col((EventIden::Table, EventIden::Id)).eq(id.to_owned()))
        .to_owned();

    let query = LocalizedEvent::query_to_localisation(query, locale);

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let event = sqlx::query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ComhairleError::ResourceNotFound("Event".into()),
            other => ComhairleError::DatabaseError(other),
        })?;

    Ok(event)
}

#[instrument(err(Debug))]
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<Event, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(EventIden::Table)
        .and_where(Expr::col(EventIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let event = sqlx::query_as_with::<_, Event, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(event)
}

fn add_current_attendance(mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
    query
        .expr_as(
            Expr::cust(
                "(SELECT COUNT(*)
                FROM event_attendance
                WHERE event_attendance.event_id = event.id
                AND event_attendance.role = 'participant')
                ",
            ),
            Alias::new("current_attendance"),
        )
        .to_owned()
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use crate::models::{
        event_attendance::{self, CreateEventAttendance},
        model_test_helpers::{
            get_random_conversation_id, get_random_user_id, setup_default_app_and_session,
        },
    };

    use super::*;
    use std::error::Error;

    #[sqlx::test]
    async fn should_create_and_return_new_event(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let new_event = CreateEvent {
            name: "test_event".to_string(),
            description: "test_desc".to_string(),
            capacity: Some(10),
            start_time: Utc::now(),
            end_time: Utc::now(),
            signup_mode: "invite".to_string(),
            agenda: None,
            ..Default::default()
        };

        let event = create(&pool, &conversation_id, &new_event).await?;

        assert_eq!(event.capacity, Some(10), "incorrect capacity");
        assert_eq!(event.conversation_id, conversation_id, "incorrect capacity");
        assert!(event.start_time < Utc::now(), "start time not past");
        assert_eq!(
            event.video_meeting_id.unwrap().get_version().unwrap(),
            uuid::Version::Random,
            "invalid video_meeting_id on creation"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_create_event_with_location(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let new_event = CreateEvent {
            name: "test_event".to_string(),
            description: "test_desc".to_string(),
            capacity: Some(10),
            start_time: Utc::now(),
            end_time: Utc::now(),
            signup_mode: "invite".to_string(),
            agenda: None,
            location: Some(EventLocation {
                venue_name: "Test venue".to_string(),
                address_line_1: "123 Main Street".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };

        let event = create(&pool, &conversation_id, &new_event).await?;

        assert_eq!(
            event.location.as_ref().unwrap().venue_name,
            "Test venue".to_string(),
            "incorrect venue"
        );
        assert_eq!(
            event.location.as_ref().unwrap().address_line_1,
            "123 Main Street".to_string(),
            "incorrect address"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_event_data(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let new_event = CreateEvent {
            name: "test_event".to_string(),
            description: "test_desc".to_string(),
            capacity: Some(10),
            start_time: Utc::now(),
            end_time: Utc::now(),
            signup_mode: "invite".to_string(),
            agenda: None,
            ..Default::default()
        };
        let event = create(&pool, &conversation_id, &new_event).await?;

        assert_eq!(
            event.capacity,
            Some(10),
            "incorrect capacity after creation"
        );
        assert_eq!(
            event.signup_mode,
            "invite".to_string(),
            "incorrect signup_mode after creation"
        );

        let update_event = PartialEvent {
            capacity: Some(20),
            signup_mode: Some("open".to_string()),
            ..Default::default()
        };
        let event = update(&pool, &event.id, &update_event).await?;

        assert_eq!(event.capacity, Some(20), "incorrect capacity after update");
        assert_eq!(
            event.signup_mode,
            "open".to_string(),
            "incorrect signup_mode after update"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_event_location_data(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let new_event = CreateEvent {
            name: "test_event".to_string(),
            description: "test_desc".to_string(),
            capacity: Some(10),
            start_time: Utc::now(),
            end_time: Utc::now(),
            signup_mode: "invite".to_string(),
            agenda: None,
            location: Some(EventLocation {
                venue_name: "Test venue".to_string(),
                address_line_1: "123 Main Street".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        let event = create(&pool, &conversation_id, &new_event).await?;

        assert_eq!(
            event.location.unwrap().venue_name,
            "Test venue".to_string(),
            "incorrect venue before update"
        );

        let params = PartialEvent {
            location: Some(EventLocation {
                venue_name: "A change of venue".to_string(),
                address_line_1: "123 Main Street".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };

        let event = update(&pool, &event.id, &params).await?;

        assert_eq!(
            event.location.unwrap().venue_name,
            "A change of venue".to_string(),
            "incorrect venue after update"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_get_event_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id_1 = get_random_conversation_id(&app, &mut session).await?;
        let conversation_id_2 = get_random_conversation_id(&app, &mut session).await?;

        let new_event_1 = CreateEvent {
            name: "test_event_1".to_string(),
            description: "test_desc".to_string(),
            capacity: Some(10),
            start_time: Utc::now(),
            end_time: Utc::now(),
            signup_mode: "invite".to_string(),
            agenda: None,
            ..Default::default()
        };
        let new_event_2 = CreateEvent {
            name: "test_event_2".to_string(),
            description: "test_desc".to_string(),
            capacity: Some(10),
            start_time: Utc::now(),
            end_time: Utc::now(),
            signup_mode: "invite".to_string(),
            agenda: None,
            ..Default::default()
        };
        let event_1 = create(&pool, &conversation_id_1, &new_event_1).await?;
        let event_2 = create(&pool, &conversation_id_2, &new_event_2).await?;

        let get_event_1 = get_localized_by_id(&pool, &event_1.id, "en").await?;
        let get_event_2 = get_localized_by_id(&pool, &event_2.id, "en").await?;

        assert_eq!(get_event_1.id, event_1.id, "incorrect id for event 1");
        assert_eq!(get_event_2.id, event_2.id, "incorrect id for event 2");
        assert_eq!(
            get_event_1.name,
            "test_event_1".to_string(),
            "incorrect name for event 1"
        );
        assert_eq!(
            get_event_2.name,
            "test_event_2".to_string(),
            "incorrect name for event 2"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_get_event_with_current_attendance(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let user_id_1 = get_random_user_id(&app, &mut session).await?;
        let user_id_2 = get_random_user_id(&app, &mut session).await?;
        let user_id_3 = get_random_user_id(&app, &mut session).await?;

        let new_event = CreateEvent {
            name: "test_event".to_string(),
            description: "test_desc".to_string(),
            capacity: Some(10),
            start_time: Utc::now(),
            end_time: Utc::now(),
            signup_mode: "invite".to_string(),
            agenda: None,
            ..Default::default()
        };
        let event = create(&pool, &conversation_id, &new_event).await?;

        let create_attendance_1 = CreateEventAttendance {
            event_id: event.id,
            user_id: user_id_1,
            role: "participant".to_string(),
        };
        let create_attendance_2 = CreateEventAttendance {
            event_id: event.id,
            user_id: user_id_2,
            role: "participant".to_string(),
        };
        let create_attendance_3 = CreateEventAttendance {
            event_id: event.id,
            user_id: user_id_3,
            role: "participant".to_string(),
        };
        let _ = event_attendance::create(&pool, &create_attendance_1).await?;
        let _ = event_attendance::create(&pool, &create_attendance_2).await?;
        let _ = event_attendance::create(&pool, &create_attendance_3).await?;

        let get_event = get_localized_by_id(&pool, &event.id, "en").await?;

        assert_eq!(
            get_event.name,
            "test_event".to_string(),
            "incorrect name for event"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_list_events(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let new_event_1 = CreateEvent {
            name: "test_event_1".to_string(),
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        let new_event_2 = CreateEvent {
            name: "test_event_2".to_string(),
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        let new_event_3 = CreateEvent {
            name: "test_event_3".to_string(),
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        let new_event_4 = CreateEvent {
            name: "test_event_4".to_string(),
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        let _ = create(&pool, &conversation_id, &new_event_1).await?;
        let _ = create(&pool, &conversation_id, &new_event_2).await?;
        let _ = create(&pool, &conversation_id, &new_event_3).await?;
        let _ = create(&pool, &conversation_id, &new_event_4).await?;

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let filter_options = EventFilterOptions {
            ..Default::default()
        };
        let order_options = EventOrderOptions {
            ..Default::default()
        };
        let results = list(
            &pool,
            &conversation_id,
            page_options,
            filter_options,
            order_options,
            None,
        )
        .await?;

        assert_eq!(results.total, 4, "incorrect number of events");
        assert_eq!(
            results.records[2].event.name,
            "test_event_3".to_string(),
            "incorrect event name"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_filter_events_by_time_status(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let new_event_1 = CreateEvent {
            name: "test_event_1".to_string(),
            signup_mode: "invite".to_string(),
            start_time: Utc::now() + Duration::days(1),
            ..Default::default()
        };
        let new_event_2 = CreateEvent {
            name: "test_event_2".to_string(),
            signup_mode: "invite".to_string(),
            start_time: Utc::now() + Duration::days(2),
            ..Default::default()
        };
        let new_event_3 = CreateEvent {
            name: "test_event_3".to_string(),
            signup_mode: "invite".to_string(),
            start_time: Utc::now() + Duration::days(3),
            ..Default::default()
        };
        let new_event_4 = CreateEvent {
            name: "test_event_4".to_string(),
            signup_mode: "invite".to_string(),
            start_time: Utc::now() - Duration::days(3),
            ..Default::default()
        };
        let _ = create(&pool, &conversation_id, &new_event_1).await?;
        let _ = create(&pool, &conversation_id, &new_event_2).await?;
        let _ = create(&pool, &conversation_id, &new_event_3).await?;
        let _ = create(&pool, &conversation_id, &new_event_4).await?;

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let future_results = list(
            &pool,
            &conversation_id,
            page_options.clone(),
            EventFilterOptions {
                time_status: Some(TimeStatus::Future),
                ..Default::default()
            },
            EventOrderOptions {
                ..Default::default()
            },
            None,
        )
        .await?;
        let past_results = list(
            &pool,
            &conversation_id,
            page_options.clone(),
            EventFilterOptions {
                time_status: Some(TimeStatus::Past),
                ..Default::default()
            },
            EventOrderOptions {
                ..Default::default()
            },
            None,
        )
        .await?;

        assert_eq!(future_results.total, 3, "incorrect number of past events");
        assert_eq!(
            future_results.records[1].event.name,
            "test_event_2".to_string(),
            "incorrect future event name"
        );
        assert_eq!(past_results.total, 1, "incorrect number of past events");
        assert_eq!(
            past_results.records[0].event.name,
            "test_event_4".to_string(),
            "incorrect past event name"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_filter_events_by_capacity(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let user_id_1 = get_random_user_id(&app, &mut session).await?;
        let user_id_2 = get_random_user_id(&app, &mut session).await?;
        let user_id_3 = get_random_user_id(&app, &mut session).await?;

        // Full: will add one attendee
        let new_event_1 = CreateEvent {
            name: "test_event_1".to_string(),
            capacity: Some(1),
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        // Full: will add three attendees
        let new_event_2 = CreateEvent {
            name: "test_event_2".to_string(),
            capacity: Some(3),
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        // Available: has capacity but will add no attendees
        let new_event_3 = CreateEvent {
            name: "test_event_3".to_string(),
            capacity: Some(1),
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        // Available: capacity null so always has availability
        let new_event_4 = CreateEvent {
            name: "test_event_4".to_string(),
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        // Full: will add 2 attendees
        let new_event_5 = CreateEvent {
            name: "test_event_5".to_string(),
            capacity: Some(2),
            signup_mode: "invite".to_string(),
            ..Default::default()
        };
        let event_1 = create(&pool, &conversation_id, &new_event_1).await?;
        let event_2 = create(&pool, &conversation_id, &new_event_2).await?;
        let _ = create(&pool, &conversation_id, &new_event_3).await?;
        let _ = create(&pool, &conversation_id, &new_event_4).await?;
        let event_5 = create(&pool, &conversation_id, &new_event_5).await?;

        let attendance_1_a = CreateEventAttendance {
            event_id: event_1.id,
            user_id: user_id_1,
            role: "participant".to_string(),
        };
        let attendance_2_a = CreateEventAttendance {
            event_id: event_2.id,
            user_id: user_id_1,
            role: "participant".to_string(),
        };
        let attendance_2_b = CreateEventAttendance {
            event_id: event_2.id,
            user_id: user_id_3,
            role: "participant".to_string(),
        };
        let attendance_2_c = CreateEventAttendance {
            event_id: event_2.id,
            user_id: user_id_2,
            role: "participant".to_string(),
        };
        let attendance_5_a = CreateEventAttendance {
            event_id: event_5.id,
            user_id: user_id_1,
            role: "participant".to_string(),
        };
        let attendance_5_b = CreateEventAttendance {
            event_id: event_5.id,
            user_id: user_id_2,
            role: "participant".to_string(),
        };
        let _ = event_attendance::create(&pool, &attendance_1_a).await?;
        let _ = event_attendance::create(&pool, &attendance_2_a).await?;
        let _ = event_attendance::create(&pool, &attendance_2_b).await?;
        let _ = event_attendance::create(&pool, &attendance_2_c).await?;
        let _ = event_attendance::create(&pool, &attendance_5_a).await?;
        let _ = event_attendance::create(&pool, &attendance_5_b).await?;

        // Event 1 at capacity
        // Event 2 at capacity
        // Event 3 has capacity but no attendees (available)
        // Event 4 has no capacity (available)
        // Event 5 at capacity

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let full_results = list(
            &pool,
            &conversation_id,
            page_options.clone(),
            EventFilterOptions {
                capacity_status: Some(CapacityStatus::Full),
                ..Default::default()
            },
            EventOrderOptions {
                ..Default::default()
            },
            None,
        )
        .await?;
        let available_results = list(
            &pool,
            &conversation_id,
            page_options.clone(),
            EventFilterOptions {
                capacity_status: Some(CapacityStatus::Available),
                ..Default::default()
            },
            EventOrderOptions {
                ..Default::default()
            },
            None,
        )
        .await?;

        assert_eq!(full_results.total, 3, "incorrect number of past events");
        assert_eq!(
            full_results.records[0].event.name,
            "test_event_1".to_string(),
            "incorrect full event name [0]"
        );
        assert_eq!(
            full_results.records[0].current_attendance, 1,
            "incorrect full attendance [0]"
        );
        assert_eq!(
            full_results.records[1].event.name,
            "test_event_2".to_string(),
            "incorrect full event name [1]"
        );
        assert_eq!(
            full_results.records[1].current_attendance, 3,
            "incorrect full attendance [1]"
        );
        assert_eq!(
            full_results.records[2].event.name,
            "test_event_5".to_string(),
            "incorrect full event name [2]"
        );
        assert_eq!(
            full_results.records[2].current_attendance, 2,
            "incorrect full attendance [2]"
        );
        assert_eq!(
            available_results.total, 2,
            "incorrect number of past events"
        );
        assert_eq!(
            available_results.records[0].event.name,
            "test_event_3".to_string(),
            "incorrect available event name [0]"
        );
        assert_eq!(
            available_results.records[0].current_attendance, 0,
            "incorrect available attendance [0]"
        );
        assert_eq!(
            available_results.records[1].event.name,
            "test_event_4".to_string(),
            "incorrect available event name [1]"
        );
        assert_eq!(
            available_results.records[1].current_attendance, 0,
            "incorrect available attendance [1]"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_event(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let new_event = CreateEvent {
            name: "test_event".to_string(),
            description: "test_desc".to_string(),
            capacity: Some(10),
            start_time: Utc::now(),
            end_time: Utc::now(),
            signup_mode: "invite".to_string(),
            agenda: None,
            ..Default::default()
        };

        let event = create(&pool, &conversation_id, &new_event).await?;

        let _ = delete(&pool, &event.id).await?;

        let err = get_localized_by_id(&pool, &event.id, "en")
            .await
            .unwrap_err();

        match err {
            ComhairleError::ResourceNotFound(e) => {
                assert_eq!(e, "Event".to_string(), "incorrect error message");
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
