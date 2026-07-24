//! Endpoints for managing an event's pre-assigned breakout plan.
//!
//! - `GET  /{event_id}/breakout`      — current plan with resolved display labels
//! - `POST /{event_id}/breakout/seed` — randomly (re)seed from attendees + invites
//! - `PUT  /{event_id}/breakout`      — save an edited plan

use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    ComhairleState,
    error::ComhairleError,
    models::{
        breakout_plan,
        event::{BreakoutPlan, BreakoutPlanRoom},
        event_attendance, invites,
    },
    routes::auth::RequiredAdminUser,
};

/// A seat in the plan, enriched with a human-readable label for the admin UI.
#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BreakoutSeatDto {
    pub user_id: Option<Uuid>,
    pub invite_id: Option<Uuid>,
    /// Display label — attendee email, invited email, or a placeholder.
    pub label: String,
    pub is_moderator: bool,
    /// True when this seat is a reserved slot for an invite that has not signed up yet.
    pub pending: bool,
}

#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BreakoutRoomDto {
    pub seats: Vec<BreakoutSeatDto>,
}

#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BreakoutPlanDto {
    pub rooms: Vec<BreakoutRoomDto>,
}

/// Body for saving an edited plan.
#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SaveBreakoutPlanRequest {
    pub rooms: Vec<BreakoutPlanRoom>,
}

/// Resolve raw seats into labelled DTO seats.
async fn to_dto(
    state: &Arc<ComhairleState>,
    event_id: &Uuid,
    plan: &BreakoutPlan,
) -> Result<BreakoutPlanDto, ComhairleError> {
    let attendances = event_attendance::list_all_for_event(&state.db, event_id).await?;
    let invites = invites::list_for_event(&state.db, event_id).await?;

    let user_labels: HashMap<Uuid, String> = attendances
        .into_iter()
        .map(|a| {
            let label = a.email.unwrap_or_else(|| a.user_id.to_string());
            (a.user_id, label)
        })
        .collect();

    let invite_labels: HashMap<Uuid, String> = invites
        .into_iter()
        .map(|i| {
            let label = match &i.invite_type {
                invites::InviteType::Email(email) => email.clone(),
                invites::InviteType::User(_) => "Invited user".to_string(),
                invites::InviteType::Open => "Open invite".to_string(),
                invites::InviteType::SingleUse => "Single-use invite".to_string(),
            };
            (i.id, label)
        })
        .collect();

    let rooms = plan
        .0
        .iter()
        .map(|room| BreakoutRoomDto {
            seats: room
                .seats
                .iter()
                .map(|seat| {
                    let (label, pending) = if let Some(uid) = seat.user_id {
                        (
                            user_labels
                                .get(&uid)
                                .cloned()
                                .unwrap_or_else(|| uid.to_string()),
                            false,
                        )
                    } else if let Some(iid) = seat.invite_id {
                        (
                            invite_labels
                                .get(&iid)
                                .cloned()
                                .unwrap_or_else(|| "Reserved".to_string()),
                            true,
                        )
                    } else {
                        ("Unknown".to_string(), true)
                    };
                    BreakoutSeatDto {
                        user_id: seat.user_id,
                        invite_id: seat.invite_id,
                        label,
                        is_moderator: seat.is_moderator,
                        pending,
                    }
                })
                .collect(),
        })
        .collect();

    Ok(BreakoutPlanDto { rooms })
}

#[instrument(err(Debug), skip(state))]
pub async fn get_plan(
    State(state): State<Arc<ComhairleState>>,
    Path((_conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<BreakoutPlanDto>), ComhairleError> {
    let plan = breakout_plan::get(&state.db, &event_id).await?;
    let dto = to_dto(&state, &event_id, &plan).await?;
    Ok((StatusCode::OK, Json(dto)))
}

#[instrument(err(Debug), skip(state))]
pub async fn seed_plan(
    State(state): State<Arc<ComhairleState>>,
    Path((_conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<BreakoutPlanDto>), ComhairleError> {
    let plan = breakout_plan::seed_for_event(&state.db, &event_id).await?;
    breakout_plan::save(&state.db, &event_id, &plan).await?;
    let dto = to_dto(&state, &event_id, &plan).await?;
    Ok((StatusCode::OK, Json(dto)))
}

#[instrument(err(Debug), skip(state))]
pub async fn save_plan(
    State(state): State<Arc<ComhairleState>>,
    Path((_conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<SaveBreakoutPlanRequest>,
) -> Result<(StatusCode, Json<BreakoutPlanDto>), ComhairleError> {
    let plan = BreakoutPlan(payload.rooms);
    breakout_plan::save(&state.db, &event_id, &plan).await?;
    let dto = to_dto(&state, &event_id, &plan).await?;
    Ok((StatusCode::OK, Json(dto)))
}
