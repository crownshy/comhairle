//! Database logic for pre-assigned breakout rooms (`event.breakout_plan`).
//!
//! The plan is a JSONB blob on the `event` row. Mutations that can race with
//! concurrent sign-ups ([`ensure_user_slotted`], [`resolve_invite_seat`]) take a
//! `FOR UPDATE` lock on the event row, serialising read-modify-write the same way
//! the capacity check in `event_attendance::create` does.

use rand::seq::SliceRandom;
use sea_query::{Expr, LockType, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        event::{self, BreakoutPlan, BreakoutPlanRoom, BreakoutSeat, EventIden},
        event_attendance, invites,
    },
};

/// Room size used when the event's agenda has no breakout item with `max_per_room`.
const DEFAULT_MAX_PER_ROOM: usize = 4;

fn is_moderator_role(role: &str) -> bool {
    role == "moderator" || role == "facilitator"
}

/// Reads the current plan for an event (no lock).
#[instrument(err(Debug), skip(db))]
pub async fn get(db: &PgPool, event_id: &Uuid) -> Result<BreakoutPlan, ComhairleError> {
    let event = event::get_by_id(db, event_id).await?;
    Ok(event.breakout_plan)
}

/// Overwrites the plan for an event.
#[instrument(err(Debug), skip(db))]
pub async fn save(db: &PgPool, event_id: &Uuid, plan: &BreakoutPlan) -> Result<(), ComhairleError> {
    let (sql, values) = Query::update()
        .table(EventIden::Table)
        .values([(EventIden::BreakoutPlan, plan.into())])
        .and_where(Expr::col(EventIden::Id).eq(*event_id))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(&sql, values).execute(db).await?;
    Ok(())
}

/// Builds a fresh random plan from the current attendee + invite list.
///
/// - Room size comes from the first breakout agenda item's `max_per_room`
///   (falling back to [`DEFAULT_MAX_PER_ROOM`]).
/// - Moderators/facilitators are spread one-per-room first, then remaining
///   people (extra moderators, participants, and reserved invite placeholders)
///   are shuffled and distributed into the emptiest room under the size limit.
/// - Invites that already correspond to an attendee are skipped (no duplicate seat).
#[instrument(err(Debug), skip(db))]
pub async fn seed_for_event(db: &PgPool, event_id: &Uuid) -> Result<BreakoutPlan, ComhairleError> {
    let event = event::get_by_id(db, event_id).await?;
    let max_per_room = event
        .agenda
        .breakout_max_per_room()
        .map(|m| m as usize)
        .unwrap_or(DEFAULT_MAX_PER_ROOM)
        .max(1);

    let attendances = event_attendance::list_all_for_event(db, event_id).await?;
    let invites = invites::list_for_event(db, event_id).await?;

    // Split attendees into moderators and everyone else.
    let mut moderators: Vec<Uuid> = vec![];
    let mut others: Vec<BreakoutSeat> = vec![];
    let mut attendee_user_ids: Vec<Uuid> = vec![];
    let mut attendee_emails: Vec<String> = vec![];

    for a in &attendances {
        attendee_user_ids.push(a.user_id);
        if let Some(email) = &a.email {
            attendee_emails.push(email.to_lowercase());
        }
        if is_moderator_role(&a.role) {
            moderators.push(a.user_id);
        } else {
            others.push(BreakoutSeat {
                user_id: Some(a.user_id),
                invite_id: None,
                is_moderator: false,
            });
        }
    }

    // Reserved placeholder seats for invites that have not resolved to an
    // attendee yet. Skip any invite whose target is already attending.
    for invite in &invites {
        let already_attending = match &invite.invite_type {
            invites::InviteType::Email(email) => attendee_emails.contains(&email.to_lowercase()),
            invites::InviteType::User(uid) => attendee_user_ids.contains(uid),
            _ => true, // Open / SingleUse links have no single identity to reserve.
        };
        if already_attending {
            continue;
        }
        others.push(BreakoutSeat {
            user_id: None,
            invite_id: Some(invite.id),
            is_moderator: false,
        });
    }

    let total = moderators.len() + others.len();
    if total == 0 {
        return Ok(BreakoutPlan::default());
    }

    let room_count = total.div_ceil(max_per_room).max(1);
    let mut plan = BreakoutPlan(
        (0..room_count)
            .map(|_| BreakoutPlanRoom { seats: vec![] })
            .collect(),
    );

    // One moderator per room first.
    let mut mods = moderators.into_iter();
    for room in plan.0.iter_mut() {
        if let Some(m) = mods.next() {
            room.seats.push(BreakoutSeat {
                user_id: Some(m),
                invite_id: None,
                is_moderator: true,
            });
        }
    }

    // Fill pool: leftover moderators (still flagged) then shuffled everyone else.
    let mut fill: Vec<BreakoutSeat> = mods
        .map(|m| BreakoutSeat {
            user_id: Some(m),
            invite_id: None,
            is_moderator: true,
        })
        .collect();
    others.shuffle(&mut rand::thread_rng());
    fill.extend(others);

    for seat in fill {
        match plan
            .0
            .iter_mut()
            .filter(|r| r.seats.len() < max_per_room)
            .min_by_key(|r| r.seats.len())
        {
            Some(room) => room.seats.push(seat),
            None => plan.0.push(BreakoutPlanRoom { seats: vec![seat] }),
        }
    }

    Ok(plan)
}

/// Ensures a freshly-registered user occupies a seat in the plan.
///
/// No-op when no plan exists. If `invite_id` matches a reserved placeholder the
/// user takes that seat (preserving their intended room); otherwise they are
/// slotted into the emptiest room. Runs under a `FOR UPDATE` lock on the event.
#[instrument(err(Debug), skip(db))]
pub async fn ensure_user_slotted(
    db: &PgPool,
    event_id: &Uuid,
    user_id: &Uuid,
    is_moderator: bool,
    invite_id: Option<Uuid>,
    email: Option<&str>,
) -> Result<(), ComhairleError> {
    // Room size is derived from the agenda and does not change during sign-up.
    let event = event::get_by_id(db, event_id).await?;
    let max_per_room = event
        .agenda
        .breakout_max_per_room()
        .map(|m| m as usize)
        .unwrap_or(DEFAULT_MAX_PER_ROOM)
        .max(1);

    // When we don't know the invite directly (e.g. an already-logged-in user
    // accepting), fall back to matching a reserved email placeholder.
    let invite_id = match (invite_id, email) {
        (Some(id), _) => Some(id),
        (None, Some(email)) => invites::list_for_event(db, event_id)
            .await?
            .into_iter()
            .find(|i| matches!(&i.invite_type, invites::InviteType::Email(e) if e.eq_ignore_ascii_case(email)))
            .map(|i| i.id),
        (None, None) => None,
    };

    let mut tx = db.begin().await?;

    let (sql, values) = Query::select()
        .column(EventIden::BreakoutPlan)
        .from(EventIden::Table)
        .and_where(Expr::col(EventIden::Id).eq(*event_id))
        .lock(LockType::Update)
        .build_sqlx(PostgresQueryBuilder);

    let mut plan: BreakoutPlan = sqlx::query_scalar_with(&sql, values)
        .fetch_one(&mut *tx)
        .await?;

    // Only maintain the plan when one has actually been set up.
    if plan.is_empty() {
        tx.rollback().await?;
        return Ok(());
    }

    let resolved = match invite_id {
        Some(invite_id) => plan.resolve_invite(&invite_id, *user_id, is_moderator),
        None => plan.contains_user(user_id),
    };
    if !resolved {
        plan.slot_user(*user_id, is_moderator, max_per_room);
    }

    let (sql, values) = Query::update()
        .table(EventIden::Table)
        .values([(EventIden::BreakoutPlan, (&plan).into())])
        .and_where(Expr::col(EventIden::Id).eq(*event_id))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *tx).await?;

    tx.commit().await?;
    Ok(())
}
