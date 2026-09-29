use crate::models::error::ModelError;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use comhairle_macros::{DbJsonBEnum, Translatable};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{Alias, Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{
    Decode, Encode, PgPool, Postgres,
    prelude::{FromRow, Type},
};
use sqlx_postgres::{PgArgumentBuffer, PgValueRef};
use std::str::FromStr;
use tracing::{instrument, warn};
use uuid::Uuid;

#[cfg(any(test, feature = "test-util"))]
use fake::Dummy;

use crate::models::error::UserError;
use crate::models::error::ValidationError;
use crate::models::{
    SqlxResultExt,
    pagination::{Order, PageOptions, PaginatedResults},
    scheduled_email::{self, CreateScheduledEmail, EmailTemplate, ScheduledEmailConfig},
    translations::{TextContentId, TextFormat, new_translation},
    users::User,
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
#[derive(Default)]
pub struct EventAgenda(pub Vec<EventAgendaItem>);

impl EventAgenda {
    /// Returns the `max_per_room` of the first breakout-room agenda item, if any.
    /// Used to size the pre-assigned breakout plan.
    pub fn breakout_max_per_room(&self) -> Option<u32> {
        self.0.iter().find_map(|item| match item {
            EventAgendaItem::BreakoutRoom(b) => b.max_per_room,
            _ => None,
        })
    }
}

/// A single seat in a pre-assigned breakout room.
///
/// A seat references either a known attendee (`user_id`) or a reserved
/// placeholder for an invite that has not yet signed up (`invite_id`).
#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone, PartialEq)]
pub struct BreakoutSeat {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub user_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub invite_id: Option<Uuid>,
    #[serde(default)]
    pub is_moderator: bool,
}

/// A pre-assigned breakout room: an ordered list of seats.
#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone, PartialEq)]
pub struct BreakoutPlanRoom {
    #[serde(default)]
    pub seats: Vec<BreakoutSeat>,
}

/// The pre-assigned breakout plan for an event. Stored as JSONB on `event`.
/// Room number is the index into the vector.
#[derive(Serialize, Deserialize, Debug, JsonSchema, DbJsonBEnum, Clone, PartialEq, Default)]
#[serde(transparent)]
pub struct BreakoutPlan(pub Vec<BreakoutPlanRoom>);

impl BreakoutPlan {
    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|room| room.seats.is_empty())
    }

    /// True if the given user already occupies a seat.
    pub fn contains_user(&self, user_id: &Uuid) -> bool {
        self.0
            .iter()
            .flat_map(|room| &room.seats)
            .any(|seat| seat.user_id.as_ref() == Some(user_id))
    }

    /// Resolve a reserved placeholder seat (matched by `invite_id`) to a real
    /// user. Returns true if a placeholder was found and updated.
    pub fn resolve_invite(&mut self, invite_id: &Uuid, user_id: Uuid, is_moderator: bool) -> bool {
        if self.contains_user(&user_id) {
            return true;
        }
        for room in &mut self.0 {
            for seat in &mut room.seats {
                if seat.invite_id.as_ref() == Some(invite_id) {
                    seat.user_id = Some(user_id);
                    seat.invite_id = None;
                    seat.is_moderator = is_moderator;
                    return true;
                }
            }
        }
        false
    }

    /// Place a user into the emptiest room that is still under `max_per_room`,
    /// creating a new room if every existing room is full. No-op if already seated.
    pub fn slot_user(&mut self, user_id: Uuid, is_moderator: bool, max_per_room: usize) {
        if self.contains_user(&user_id) {
            return;
        }
        let seat = BreakoutSeat {
            user_id: Some(user_id),
            invite_id: None,
            is_moderator,
        };

        let target = self
            .0
            .iter_mut()
            .filter(|room| room.seats.len() < max_per_room)
            .min_by_key(|room| room.seats.len());

        match target {
            Some(room) => room.seats.push(seat),
            None => self.0.push(BreakoutPlanRoom { seats: vec![seat] }),
        }
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
    pub signup_mode: SignupMode,
    #[partially(omit)]
    pub video_meeting_id: Option<Uuid>,
    #[serde(default)]
    pub agenda: EventAgenda,
    #[serde(default)]
    #[partially(omit)]
    pub breakout_plan: BreakoutPlan,
    pub default_time_zone: String,
    pub format: EventFormat,
    #[partially(transparent)]
    pub custom_event_link: Option<String>,
    #[partially(transparent)]
    pub location: Option<EventLocation>,
    pub metadata: Option<serde_json::Value>,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, PartialOrd, sqlx::Type, Clone, JsonSchema, Default,
)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(Dummy))]
pub enum SignupMode {
    #[default]
    Invite,
    Open,
}

impl From<SignupMode> for sea_query::Value {
    fn from(val: SignupMode) -> Self {
        sea_query::Value::String(Some(Box::new(val.to_string())))
    }
}

impl std::fmt::Display for SignupMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            SignupMode::Invite => "invite",
            SignupMode::Open => "open",
        };
        write!(f, "{}", value)
    }
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
#[cfg_attr(any(test, feature = "test-util"), derive(Dummy))]
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

impl LocalizedEvent {
    pub async fn schedule_event_reminders(
        &self,
        db: &PgPool,
        recipient: &User,
        owner_id: Uuid,
        locale: &str,
    ) -> Result<(), ModelError> {
        if self.start_time <= Utc::now() {
            warn!("Event has past");
            return Ok(());
        }

        let email = recipient.email.as_ref().ok_or(UserError::WrongUserType)?;

        let email_config = ScheduledEmailConfig {
            template: EmailTemplate::EventReminder {
                event_id: self.id,
                recipient_id: recipient.id,
                owner_id,
                locale: locale.to_string(),
            },
        };
        let params_24_hours = CreateScheduledEmail {
            user_email: email.to_string(),
            email_config: email_config.clone(),
            send_at: self.start_time - chrono::Duration::days(1),
        };
        scheduled_email::create(db, params_24_hours).await?;

        let params_2_hours = CreateScheduledEmail {
            user_email: email.to_string(),
            email_config,
            send_at: self.start_time - chrono::Duration::hours(2),
        };
        scheduled_email::create(db, params_2_hours).await?;

        Ok(())
    }
}

const DEFAULT_COLUMNS: [EventIden; 18] = [
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
    EventIden::BreakoutPlan,
    EventIden::DefaultTimeZone,
    EventIden::Location,
    EventIden::Metadata,
    EventIden::Format,
    EventIden::CustomEventLink,
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
    pub signup_mode: SignupMode,
    pub agenda: Option<EventAgenda>,
    pub location: Option<EventLocation>,
    pub default_time_zone: Option<String>,
    pub custom_event_link: Option<String>,
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

        if self.custom_event_link.is_some() {
            columns.push(EventIden::CustomEventLink)
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

        if let Some(ref value) = self.custom_event_link {
            values.push(value.into());
        }

        values
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn create(
    db: &PgPool,
    conversation_id: &Uuid,
    new_event: &CreateEvent,
) -> Result<Event, ModelError> {
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
            values.push((EventIden::SignupMode, value.clone().into()));
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
        if let Some(value) = &self.custom_event_link {
            if value.trim().is_empty() {
                values.push((
                    EventIden::CustomEventLink,
                    sea_query::Value::String(None).into(),
                ));
            } else {
                values.push((EventIden::CustomEventLink, value.clone().into()));
            }
        }
        if let Some(value) = &self.default_time_zone {
            values.push((EventIden::DefaultTimeZone, value.into()));
        }
        if let Some(value) = &self.metadata {
            values.push((EventIden::Metadata, value.clone().into()));
        }

        values
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn update(
    db: &PgPool,
    id: &Uuid,
    update_event: &PartialEvent,
) -> Result<Event, ModelError> {
    let values = update_event.to_values();

    if values.is_empty() {
        return Err(ValidationError::NoValidUpdates.into());
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

/// Get the event's `metadata` jsonb column.
#[instrument(err(Debug), skip(db))]
pub async fn get_metadata(db: &PgPool, id: &Uuid) -> Result<Option<serde_json::Value>, ModelError> {
    let query = Query::select()
        .columns([EventIden::Metadata].map(|col| (EventIden::Table, col)))
        .from(EventIden::Table)
        .and_where(Expr::col((EventIden::Table, EventIden::Id)).eq(id.to_owned()))
        .to_owned();

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let (metadata,) = sqlx::query_as_with::<_, (Option<serde_json::Value>,), _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(metadata)
}

/// Merge the supplied object into the event's `metadata` jsonb column at the
/// top level. Existing keys are overwritten by the patch.
pub async fn patch_metadata(
    db: &PgPool,
    id: &Uuid,
    patch: serde_json::Value,
) -> Result<Event, ModelError> {
    if !patch.is_object() {
        return Err(
            ValidationError::BadRequest("metadata patch must be a JSON object".into()).into(),
        );
    }

    let event = sqlx::query_as::<_, Event>(
        "UPDATE event
            SET metadata = COALESCE(metadata, '{}'::jsonb) || $1::jsonb,
                updated_at = NOW()
            WHERE id = $2
            RETURNING *",
    )
    .bind(patch)
    .bind(id)
    .fetch_one(db)
    .await
    .resolve_db_err("Event")?;

    Ok(event)
}

#[derive(Deserialize, Debug, Default, JsonSchema)]
pub struct EventOrderOptions {
    pub name: Option<Order>,
    pub created_at: Option<Order>,
    pub start_time: Option<Order>,
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

#[instrument(err(Debug), skip(db))]
pub async fn list(
    db: &PgPool,
    conversation_id: &Uuid,
    page_options: PageOptions,
    filter_options: EventFilterOptions,
    order_options: EventOrderOptions,
    locale: Option<String>,
) -> Result<PaginatedResults<LocalizedEventWithAttendance>, ModelError> {
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

#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<Event, ModelError> {
    let query = Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (EventIden::Table, col)))
        .from(EventIden::Table)
        .and_where(Expr::col((EventIden::Table, EventIden::Id)).eq(id.to_owned()))
        .to_owned();

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let event = sqlx::query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Event")?;

    Ok(event)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_localized_by_id(
    db: &PgPool,
    id: &Uuid,
    locale: &str,
) -> Result<LocalizedEvent, ModelError> {
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
        .resolve_db_err("Event")?;

    Ok(event)
}

#[instrument(err(Debug), skip(db))]
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<Event, ModelError> {
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
