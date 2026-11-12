use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::info;
use uuid::Uuid;

use crate::ComhairleState;
use crate::websockets::messages::WebSocketMessage;
use crate::websockets::{WebSocketConnection, WebSocketMessageHandler};

/// Represents a participant in a video call.
///
/// Contains minimal user information and their role in the call (e.g., moderator, facilitator, participant).
/// Email addresses and other sensitive data are not included for privacy reasons.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoCallParticipant {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub role: String,
}

impl VideoCallParticipant {
    /// Whether this person runs the call (moderator or facilitator) rather than taking part in it.
    pub fn is_host(&self) -> bool {
        self.role == "moderator" || self.role == "facilitator"
    }
}

/// Represents a breakout room assignment with a list of participant UUIDs.
///
/// Used to organize participants into smaller groups during a video call.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BreakoutRoomAssignments {
    pub participants: Vec<Uuid>,
    /// The real Jitsi JID for this room, once a client has actually seen one
    /// (breakout rooms aren't created on Jitsi's side until someone joins).
    /// Reported by the frontend via `video_call:breakout_room_live`; used to
    /// trigger a jitsi-recorder session for this specific room.
    pub room_jid: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BreakoutRoomAssistanceRequest {
    made_by_user: Uuid,
}

/// Represents an active breakout session with a scheduled end time.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BreakoutSession {
    /// The time at which the current breakout session is scheduled to end
    pub ends: DateTime<Utc>,
}

/// Complete state of a video call.
///
/// Contains all information about the call including participants, status,
/// breakout room assignments, and current agenda progress.
#[derive(Serialize, Debug)]
pub struct VideoCallState {
    /// Unique identifier for the video call
    pub video_call_id: Uuid,
    /// Current status of the call (Waiting, InProgress, or Ended)
    pub status: VideoCallStatus,
    /// Map of participant user IDs to their participant data
    pub participants: HashMap<Uuid, VideoCallParticipant>,
    /// User IDs that have actually joined the Jitsi video (as opposed to just opening
    /// the live page / sitting in the lobby). Reported by each client on join/leave, so
    /// it does not depend on the user-editable Jitsi display name.
    pub in_video_participants: HashSet<Uuid>,
    /// The Jitsi display name each user chose, keyed by user id. Reported by the client
    /// on join and whenever it changes, so the UI can show call names instead of usernames.
    pub jitsi_display_names: HashMap<Uuid, String>,
    /// List of breakout room assignments
    pub breakout_rooms: Vec<BreakoutRoomAssignments>,
    /// Active requests for assistance from breakout rooms
    pub breakout_room_assistance_requests: HashMap<String, BreakoutRoomAssistanceRequest>,
    /// Current step in the agenda (0-indexed)
    pub current_agenda_step: u32,
    /// Active breakout session with scheduled end time (None when no session is active)
    pub breakout_session: Option<BreakoutSession>,
    /// Whether a jitsi-recorder session has already been triggered for the
    /// main room. Set on the first real video join, so a later rejoin by the
    /// same or another participant doesn't start a second recorder for a
    /// room that's already being recorded.
    #[serde(skip)]
    pub main_room_recording_started: bool,
}

/// WebSocket message handler for video call events.
///
/// Manages the state of all active video calls and handles incoming
/// WebSocket messages related to video call operations.
#[derive(Default)]
pub struct VideoCallMessageHandler {
    /// Thread-safe map of event IDs to their video call states
    pub video_calls: RwLock<HashMap<Uuid, VideoCallState>>,
}

/// Status of a video call.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VideoCallStatus {
    /// Call is waiting to start
    Waiting,
    /// Call is currently in progress
    InProgress,
    /// Call has ended
    Ended,
}

/// Errors that can occur during video call WebSocket operations.
#[derive(Debug, Error)]
pub enum VideoCallWSError {
    #[error("User is not participating in this call")]
    UserNotOnCall,

    #[error("Wrong message format {0}")]
    IncorrectMessageFormat(#[from] serde_json::Error),

    #[error("Failed to get event id")]
    FailedToDeserializeEventId,

    #[error("No event specified. Please provide an event_id")]
    NoEventSpecified,

    #[error("Failed to get lock on room")]
    FailedToGetLockOnRoom,

    #[error("Video call state not found for event")]
    VideoCallNotFound,

    #[error("User is not authorized to change call state")]
    UnauthorizedStateChange,
}

impl VideoCallState {
    /// Creates a new video call state with default values.
    ///
    /// # Arguments
    ///
    /// * `video_call_id` - Unique identifier for this video call
    /// * `jitsi_call_id` - Jitsi meeting room identifier
    ///
    /// # Returns
    ///
    /// A new `VideoCallState` initialized with:
    /// - Status: `Waiting`
    /// - No participants
    /// - No breakout rooms
    /// - Agenda step: 0
    pub fn new(video_call_id: Uuid) -> Self {
        Self {
            status: VideoCallStatus::Waiting,
            participants: HashMap::new(),
            in_video_participants: HashSet::new(),
            jitsi_display_names: HashMap::new(),
            video_call_id,
            breakout_room_assistance_requests: HashMap::new(),
            current_agenda_step: 0,
            breakout_rooms: vec![],
            breakout_session: None,
            main_room_recording_started: false,
        }
    }

    /// Puts a participant who isn't in a room yet into the room with the fewest people.
    /// Hosts are never placed: they move between rooms themselves, and placing them would
    /// list them as room members.
    fn place_in_breakout_room(&mut self, user_id: Uuid) {
        let is_host = self
            .participants
            .get(&user_id)
            .is_some_and(VideoCallParticipant::is_host);
        let already_placed = self
            .breakout_rooms
            .iter()
            .any(|room| room.participants.contains(&user_id));
        if is_host || already_placed {
            return;
        }

        if let Some(emptiest_room) = self
            .breakout_rooms
            .iter_mut()
            .min_by_key(|room| room.participants.len())
        {
            emptiest_room.participants.push(user_id);
        }
    }

    /// Ids of everyone in the call who should be split into breakout rooms, hosts left out.
    fn breakout_candidates(&self) -> Vec<Uuid> {
        self.participants
            .values()
            .filter(|participant| !participant.is_host())
            .map(|participant| participant.user_id)
            .collect()
    }
}

#[derive(Serialize, Deserialize)]
struct BreakoutRoomAssistanceRequestData {
    pub event_id: Uuid,
    pub room_name: String,
}

/// Data structure for resolving breakout room assistance requests.
#[derive(Serialize, Deserialize)]
struct ResolveBreakoutRoomAssistanceRequestData {
    pub event_id: Uuid,
    pub room_name: String,
}

/// Data structure for user join events.
#[derive(Serialize, Deserialize)]
struct UserJoinData {
    pub event_id: Uuid,
}

/// Data structure for user leave events.
#[derive(Serialize, Deserialize)]
struct UserLeaveData {
    pub event_id: Uuid,
}

/// Data structure for a client reporting that it joined (or renamed itself in) the video.
#[derive(Serialize, Deserialize)]
struct VideoJoinedData {
    pub event_id: Uuid,
    /// The Jitsi display name the user chose, if any.
    #[serde(default)]
    pub display_name: Option<String>,
}

/// Data structure for setting the current agenda item.
#[derive(Serialize, Deserialize)]
struct SetAgendaItemData {
    pub event_id: Uuid,
    pub agenda_item: u32,
}

/// Data structure for broadcasting messages to call participants.
#[derive(Serialize, Deserialize, Debug)]
struct BroadcastMessageData {
    pub event_id: Uuid,
    pub message: String,
}

/// Data structure for changing the call state.
#[derive(Serialize, Deserialize, Debug)]
struct ChangeCallStatusData {
    pub event_id: Uuid,
    pub status: VideoCallStatus,
}

/// Data structure for assigning participants to breakout rooms.
#[derive(Serialize, Deserialize, Debug)]
struct AssignBreakoutRoomsData {
    pub event_id: Uuid,
    pub max_users_per_room: usize,
    /// When provided, use these explicit assignments instead of random chunking.
    /// Each inner Vec is the list of user IDs for one breakout room.
    pub room_assignments: Option<Vec<Vec<Uuid>>>,
}

/// Data structure for starting a breakout session.
#[derive(Serialize, Deserialize, Debug)]
struct StartBreakoutSessionData {
    pub event_id: Uuid,
    pub ends: DateTime<Utc>,
}

/// Data structure for extending an active breakout session.
#[derive(Serialize, Deserialize, Debug)]
struct ExtendBreakoutSessionData {
    pub event_id: Uuid,
    pub ends: DateTime<Utc>,
}

/// Data structure for ending an active breakout session.
#[derive(Serialize, Deserialize, Debug)]
struct EndBreakoutSessionData {
    pub event_id: Uuid,
}

/// Data structure for a client reporting that a breakout room now has a real
/// Jitsi JID (i.e. someone has actually joined it, since Jitsi only creates
/// the underlying MUC room at that point).
#[derive(Serialize, Deserialize, Debug)]
struct BreakoutRoomLiveData {
    pub event_id: Uuid,
    pub room_index: usize,
    pub room_jid: String,
}

impl VideoCallMessageHandler {
    /// Creates a new video call message handler with an empty call registry.
    pub fn new() -> Self {
        Self {
            video_calls: RwLock::new(HashMap::new()),
        }
    }

    /// Executes a function with read-only access to a video call state.
    ///
    /// This method acquires a read lock on the video calls registry and provides
    /// safe read-only access to the requested call state through a closure.
    ///
    /// # Arguments
    ///
    /// * `event_id` - The UUID of the event/call to access
    /// * `f` - Closure that receives a reference to the `VideoCallState`
    ///
    /// # Returns
    ///
    /// * `Ok(Some(R))` - The closure was executed and returned a value
    /// * `Ok(None)` - The event_id was not found in the registry
    /// * `Err(VideoCallWSError)` - Failed to acquire the read lock
    pub fn with_video_call_state<F, R>(
        &self,
        event_id: &Uuid,
        f: F,
    ) -> Result<Option<R>, VideoCallWSError>
    where
        F: FnOnce(&VideoCallState) -> R,
    {
        let video_calls = self
            .video_calls
            .read()
            .map_err(|_| VideoCallWSError::FailedToGetLockOnRoom)?;
        Ok(video_calls.get(event_id).map(f))
    }

    /// Executes a function with mutable access to a video call state.
    ///
    /// This method acquires a write lock on the video calls registry and provides
    /// safe mutable access to the requested call state through a closure.
    ///
    /// # Arguments
    ///
    /// * `event_id` - The UUID of the event/call to access
    /// * `f` - Closure that receives a mutable reference to the `VideoCallState`
    ///
    /// # Returns
    ///
    /// * `Ok(Some(R))` - The closure was executed and returned a value
    /// * `Ok(None)` - The event_id was not found in the registry
    /// * `Err(VideoCallWSError)` - Failed to acquire the write lock
    pub fn with_video_call_state_mut<F, R>(
        &self,
        event_id: &Uuid,
        f: F,
    ) -> Result<Option<R>, VideoCallWSError>
    where
        F: FnOnce(&mut VideoCallState) -> R,
    {
        let mut video_calls = self
            .video_calls
            .write()
            .map_err(|_| VideoCallWSError::FailedToGetLockOnRoom)?;
        Ok(video_calls.get_mut(event_id).map(f))
    }

    /// Sets the current agenda item for a video call.
    ///
    /// Updates the `current_agenda_step` field of the call state and broadcasts
    /// the updated state to all participants.
    ///
    /// # Arguments
    ///
    /// * `event_id` - The UUID of the event/call
    /// * `data` - JSON data containing the new agenda item number
    /// * `_connection` - WebSocket connection of the requesting user (unused)
    /// * `state` - Application state for broadcasting
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - JSON deserialization fails
    /// - Failed to acquire lock on the call state
    /// - Broadcasting the state fails
    pub async fn set_agenda_item(
        &self,
        event_id: &Uuid,
        data: &serde_json::Value,
        _connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), VideoCallWSError> {
        let agenda_update: SetAgendaItemData = serde_json::from_value(data.clone())?;

        self.with_video_call_state_mut(event_id, |call_state| {
            call_state.current_agenda_step = agenda_update.agenda_item
        })?;

        self.broadcast_state(event_id, state).await?;

        Ok(())
    }

    /// Broadcasts a custom message to all participants in a video call.
    ///
    /// Only moderators and facilitators are authorized to broadcast messages.
    /// The message is sent to all participants in the call via WebSocket.
    ///
    /// # Arguments
    ///
    /// * `event_id` - The UUID of the event/call
    /// * `data` - JSON data containing the message to broadcast
    /// * `connection` - WebSocket connection of the user sending the message
    /// * `state` - Application state for sending messages
    ///
    /// # Authorization
    ///
    /// Only users with role "moderator" or "facilitator" can broadcast messages.
    /// Other users' broadcast attempts are silently ignored.
    ///
    /// # Errors
    ///
    /// Returns an error if JSON deserialization fails or lock acquisition fails.
    pub async fn broadcast_message(
        &self,
        event_id: &Uuid,
        data: &serde_json::Value,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), VideoCallWSError> {
        let broadcast_data: BroadcastMessageData = serde_json::from_value(data.clone())?;

        // Extract participant user IDs and check authorization synchronously
        let participant_ids = self
            .with_video_call_state(event_id, |call| {
                // Check if the sender is authorized to broadcast
                if let Some(sender) = call.participants.get(&connection.user.id)
                    && sender.is_host()
                {
                    // Return all participant user IDs
                    return Some(call.participants.keys().copied().collect::<Vec<Uuid>>());
                }
                None
            })?
            .flatten();

        // If authorized, broadcast to all participants
        if let Some(participant_ids) = participant_ids {
            let message = WebSocketMessage::Custom {
                event: "video_call:message".into(),
                data: serde_json::json!({
                    "message": broadcast_data.message
                }),
            };

            // Send to each participant
            for user_id in participant_ids {
                let _ = state.websockets.send_to_user(&user_id, &message).await;
            }
        }

        Ok(())
    }

    /// Handles a user joining a video call.
    ///
    /// This method:
    /// 1. Verifies the user is registered for the event
    /// 2. Adds them to the participants list
    /// 3. Auto-assigns them to a breakout room if rooms exist
    /// 4. Sends the current call state to the joining user
    /// 5. Broadcasts the updated state to all participants
    ///
    /// # Arguments
    ///
    /// * `_event_id` - The UUID of the event (unused, extracted from data)
    /// * `data` - JSON data containing the event_id
    /// * `connection` - WebSocket connection of the joining user
    /// * `state` - Application state for database access and broadcasting
    ///
    /// # Auto-Assignment to Breakout Rooms
    ///
    /// If breakout rooms have been configured and the user is not already
    /// assigned to a room, they will be automatically assigned to the room
    /// with the fewest participants. Moderators and facilitators are never
    /// auto-assigned.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - JSON deserialization fails
    /// - User is not registered for the event
    /// - Failed to acquire lock on the call state
    /// - Broadcasting fails
    pub async fn handle_user_join(
        &self,
        _event_id: &Uuid,
        data: &serde_json::Value,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), VideoCallWSError> {
        let join_data: UserJoinData = serde_json::from_value(data.clone())?;

        // Verify the user is registered for this event
        let attendance = crate::models::event_attendance::get_by_event_and_user(
            &state.db,
            &join_data.event_id,
            &connection.user.id,
        )
        .await
        .map_err(|_| VideoCallWSError::UserNotOnCall)?;

        let role = attendance.role;
        let user_id = connection.user.id;
        let username = connection.user.username.clone();

        // Load any pre-assigned breakout plan before locking, so a fresh call
        // is seeded with the rooms configured ahead of time. Async work cannot
        // happen inside the lock, so fetch it up front.
        let seeded_rooms = crate::models::breakout_plan::get(&state.db, &join_data.event_id)
            .await
            .map(|plan| {
                plan.0
                    .into_iter()
                    .map(|room| BreakoutRoomAssignments {
                        // Only known users can occupy a live room; reserved
                        // invite placeholders are dropped until they join.
                        room_jid: None,
                        participants: room
                            .seats
                            .into_iter()
                            .filter_map(|seat| seat.user_id)
                            .collect(),
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        // Initialize video call state if it doesn't exist
        {
            let mut video_calls = self
                .video_calls
                .write()
                .map_err(|_| VideoCallWSError::FailedToGetLockOnRoom)?;

            let is_new = !video_calls.contains_key(&join_data.event_id);
            let call = video_calls
                .entry(join_data.event_id)
                .or_insert_with(|| VideoCallState::new(join_data.event_id));

            // Seed the breakout rooms from the pre-assigned plan on first join.
            // Pre-assigned users already occupy their room, so the auto-assign
            // step below only affects walk-ins who were not planned for.
            if is_new {
                call.breakout_rooms = seeded_rooms;
            }
        }

        self.with_video_call_state_mut(&join_data.event_id, |call| {
            call.participants.insert(
                user_id,
                VideoCallParticipant {
                    user_id,
                    username,
                    role,
                },
            );
            call.place_in_breakout_room(user_id);
        })?;

        // Send the current state directly to the joining user
        let call_state = self
            .with_video_call_state(&join_data.event_id, |call| serde_json::to_value(call).ok())?
            .flatten();

        if let Some(call_state) = call_state {
            let message = WebSocketMessage::Custom {
                event: "video_call:state_update".into(),
                data: call_state,
            };
            let _ = state.websockets.send_to_user(&user_id, &message).await;
        }

        // Broadcast the updated state to all participants
        self.broadcast_state(&join_data.event_id, state).await?;

        Ok(())
    }

    /// Handles a user leaving a video call.
    ///
    /// Removes the user from the participants list and broadcasts the
    /// updated state to all remaining participants.
    ///
    /// # Arguments
    ///
    /// * `_event_id` - The UUID of the event (unused, extracted from data)
    /// * `data` - JSON data containing the event_id
    /// * `connection` - WebSocket connection of the leaving user
    /// * `state` - Application state for broadcasting
    ///
    /// # Note
    ///
    /// This method does not remove the user from breakout room assignments.
    /// The user will remain in the breakout room assignments list even after leaving.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - JSON deserialization fails
    /// - Failed to acquire lock on the call state
    /// - Broadcasting fails
    pub async fn handle_user_leave(
        &self,
        _event_id: &Uuid,
        data: &serde_json::Value,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), VideoCallWSError> {
        let leave_data: UserLeaveData = serde_json::from_value(data.clone())?;

        let user_id = connection.user.id;

        // Remove the participant from the video call
        self.with_video_call_state_mut(&leave_data.event_id, |call| {
            call.participants.remove(&user_id);
            call.in_video_participants.remove(&user_id);
            call.jitsi_display_names.remove(&user_id);
        })?;

        self.broadcast_state(&leave_data.event_id, state).await?;

        Ok(())
    }

    /// Marks a user as having actually joined the Jitsi video for this event and records
    /// the Jitsi display name they chose, then broadcasts the updated state. Called when a
    /// client's `videoConferenceJoined` fires (and again on rename) — a reliable,
    /// rename-proof signal of real call presence keyed by the authenticated user id.
    ///
    /// The first real join after the main room was empty also triggers a
    /// jitsi-recorder session for the main room (see `jitsi_recorder::trigger_recording`).
    /// This is comhairle-api's own replacement for the old Prosody
    /// room-created webhook, which could never address a specific tenant.
    pub async fn handle_video_joined(
        &self,
        event_id: &Uuid,
        data: &serde_json::Value,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), VideoCallWSError> {
        let joined_data: VideoJoinedData = serde_json::from_value(data.clone())?;
        let user_id = connection.user.id;

        let should_trigger_recording = self
            .with_video_call_state_mut(event_id, |call| {
                let was_empty = call.in_video_participants.is_empty();
                call.in_video_participants.insert(user_id);
                match joined_data.display_name {
                    Some(name) if !name.trim().is_empty() => {
                        call.jitsi_display_names.insert(user_id, name);
                    }
                    _ => {
                        call.jitsi_display_names.remove(&user_id);
                    }
                }

                let should_trigger = was_empty && !call.main_room_recording_started;
                if should_trigger {
                    call.main_room_recording_started = true;
                }
                should_trigger
            })?
            .unwrap_or(false);

        self.broadcast_state(event_id, state).await?;

        if should_trigger_recording {
            let event_id = *event_id;
            let state = state.clone();
            tokio::spawn(async move {
                if let Err(err) = trigger_main_room_recording(&event_id, &state).await {
                    tracing::error!(?err, %event_id, "failed to trigger jitsi-recorder for main room");
                }
            });
        }

        Ok(())
    }

    /// Marks a user as no longer in the Jitsi video (left the call but may still have
    /// the live page open), then broadcasts the updated state.
    pub async fn handle_video_left(
        &self,
        event_id: &Uuid,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), VideoCallWSError> {
        let user_id = connection.user.id;

        self.with_video_call_state_mut(event_id, |call| {
            call.in_video_participants.remove(&user_id);
            call.jitsi_display_names.remove(&user_id);
        })?;

        self.broadcast_state(event_id, state).await?;

        Ok(())
    }

    /// Changes the status of a video call.
    ///
    /// Only moderators and facilitators are authorized to change the call state.
    /// The state can be changed between Waiting, InProgress, and Ended.
    ///
    /// # Arguments
    ///
    /// * `_event_id` - The UUID of the event (unused, extracted from data)
    /// * `data` - JSON data containing the event_id and new status
    /// * `connection` - WebSocket connection of the user requesting the change
    /// * `state` - Application state for broadcasting
    ///
    /// # Authorization
    ///
    /// Only users with role "moderator" or "facilitator" can change call state.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - JSON deserialization fails
    /// - User is not authorized (`UnauthorizedStateChange`)
    /// - Failed to acquire lock on the call state
    /// - Broadcasting fails
    pub async fn change_call_status(
        &self,
        _event_id: &Uuid,
        data: &serde_json::Value,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), VideoCallWSError> {
        let state_data: ChangeCallStatusData = serde_json::from_value(data.clone())?;

        let user_id = connection.user.id;
        let new_status = state_data.status;

        // Check authorization and update the call state
        let authorized = self
            .with_video_call_state_mut(&state_data.event_id, |call| {
                // Check if the user is authorized (moderator or facilitator)
                if let Some(participant) = call.participants.get(&user_id)
                    && participant.is_host()
                {
                    call.status = new_status;
                    return true;
                }
                false
            })?
            .unwrap_or(false);

        if !authorized {
            return Err(VideoCallWSError::UnauthorizedStateChange);
        }

        self.broadcast_state(&state_data.event_id, state).await?;

        Ok(())
    }

    /// Requests assistance from moderators or facilitators in a breakout room
    ///
    /// Can be triggered by a participant in a breakout room if they need assistance
    /// from a facilitator or moderator. Only one request per room
    ///
    /// # Arguments
    ///
    /// * `event_id` - The UUID of the event (unused, extracted from data)
    /// * `data` - JSON data containing event_id and the room_name
    /// * `connection` - WebSocket connection of the user requesting the assignment
    /// * `state` - Application state for broadcasting
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - JSON deserialization fails
    async fn breakout_room_assistance_request(
        &self,
        event_id: &Uuid,
        data: &serde_json::Value,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), VideoCallWSError> {
        let request: BreakoutRoomAssistanceRequestData = serde_json::from_value(data.clone())
            .map_err(VideoCallWSError::IncorrectMessageFormat)?;

        self.with_video_call_state_mut(event_id, |call| {
            call.breakout_room_assistance_requests
                .entry(request.room_name)
                .or_insert(BreakoutRoomAssistanceRequest {
                    made_by_user: connection.user.id,
                });
        })?;

        self.broadcast_state(&request.event_id, state).await?;

        Ok(())
    }

    /// Resolves (clears) an assistance request from a breakout room.
    ///
    /// Only moderators and facilitators are authorized to resolve assistance requests.
    /// This removes the request from the active assistance requests map and broadcasts
    /// the updated state to all participants.
    ///
    /// # Arguments
    ///
    /// * `_event_id` - The UUID of the event (unused, extracted from data)
    /// * `data` - JSON data containing event_id and room_name
    /// * `connection` - WebSocket connection of the user resolving the request
    /// * `state` - Application state for broadcasting
    ///
    /// # Authorization
    ///
    /// Only users with role "moderator" or "facilitator" can resolve assistance requests.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - JSON deserialization fails
    /// - User is not authorized (`UnauthorizedStateChange`)
    /// - Failed to acquire lock on the call state
    /// - Broadcasting fails
    pub async fn resolve_breakout_room_assistance_request(
        &self,
        _event_id: &Uuid,
        data: &serde_json::Value,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), VideoCallWSError> {
        let request: ResolveBreakoutRoomAssistanceRequestData =
            serde_json::from_value(data.clone())?;

        let user_id = connection.user.id;

        // Check authorization and remove the assistance request
        let authorized = self
            .with_video_call_state_mut(&request.event_id, |call| {
                // Check if the user is authorized (moderator or facilitator)
                if let Some(participant) = call.participants.get(&user_id)
                    && participant.is_host()
                {
                    // Remove the assistance request for this room
                    call.breakout_room_assistance_requests
                        .remove(&request.room_name);
                    return true;
                }
                false
            })?
            .unwrap_or(false);

        if !authorized {
            return Err(VideoCallWSError::UnauthorizedStateChange);
        }

        self.broadcast_state(&request.event_id, state).await?;

        Ok(())
    }

    /// Randomly assigns all participants to breakout rooms.
    ///
    /// Only moderators and facilitators are authorized to assign breakout rooms.
    /// Participants are randomly shuffled and then divided into rooms based on
    /// the specified maximum users per room.
    ///
    /// # Arguments
    ///
    /// * `_event_id` - The UUID of the event (unused, extracted from data)
    /// * `data` - JSON data containing event_id and max_users_per_room
    /// * `connection` - WebSocket connection of the user requesting the assignment
    /// * `state` - Application state for broadcasting
    ///
    /// # Algorithm
    ///
    /// 1. Collects all current participant IDs
    /// 2. Randomly shuffles the participants
    /// 3. Divides them into chunks of `max_users_per_room` size
    /// 4. Creates a `BreakoutRoomAssignments` for each chunk
    ///
    /// If there are 13 participants and `max_users_per_room` is 5, this will create
    /// 3 rooms: two with 5 participants and one with 3 participants.
    ///
    /// # Authorization
    ///
    /// Only users with role "moderator" or "facilitator" can assign breakout rooms.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - JSON deserialization fails
    /// - User is not authorized (`UnauthorizedStateChange`)
    /// - Failed to acquire lock on the call state
    /// - Broadcasting fails
    pub async fn assign_breakout_rooms(
        &self,
        _event_id: &Uuid,
        data: &serde_json::Value,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), VideoCallWSError> {
        let assignment_data: AssignBreakoutRoomsData = serde_json::from_value(data.clone())?;

        let user_id = connection.user.id;
        let max_users = assignment_data.max_users_per_room;

        // Check authorization and assign breakout rooms
        let authorized = self
            .with_video_call_state_mut(&assignment_data.event_id, |call| {
                // Check if the user is authorized (moderator or facilitator)
                if let Some(participant) = call.participants.get(&user_id)
                    && participant.is_host()
                {
                    let breakout_rooms =
                        if let Some(ref explicit) = assignment_data.room_assignments {
                            // Empty rooms are kept: room numbers are positions in this list,
                            // so dropping one would renumber the rooms after it and send
                            // their participants to the wrong Jitsi room.
                            explicit
                                .iter()
                                .map(|room_ids| BreakoutRoomAssignments {
                                    participants: room_ids
                                        .iter()
                                        .filter(|id| call.participants.contains_key(id))
                                        .copied()
                                        .collect(),
                                    room_jid: None,
                                })
                                .collect()
                        } else {
                            let mut participant_ids = call.breakout_candidates();
                            let mut rng = rand::thread_rng();
                            participant_ids.shuffle(&mut rng);

                            // Divide into rooms with max_users_per_room
                            participant_ids
                                .chunks(max_users)
                                .map(|chunk| BreakoutRoomAssignments {
                                    participants: chunk.to_vec(),
                                    room_jid: None,
                                })
                                .collect()
                        };

                    call.breakout_rooms = breakout_rooms;
                    return true;
                }
                false
            })?
            .unwrap_or(false);

        if !authorized {
            return Err(VideoCallWSError::UnauthorizedStateChange);
        }

        self.broadcast_state(&assignment_data.event_id, state)
            .await?;

        Ok(())
    }

    /// Starts a breakout session with a specified end time.
    ///
    /// Only moderators and facilitators are authorized to start breakout sessions.
    /// The breakout session will be scheduled to end at the specified time.
    ///
    /// # Arguments
    ///
    /// * `_event_id` - The UUID of the event (unused, extracted from data)
    /// * `data` - JSON data containing event_id and the end time
    /// * `connection` - WebSocket connection of the user requesting to start the session
    /// * `state` - Application state for broadcasting
    ///
    /// # Authorization
    ///
    /// Only users with role "moderator" or "facilitator" can start breakout sessions.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - JSON deserialization fails
    /// - User is not authorized (`UnauthorizedStateChange`)
    /// - Failed to acquire lock on the call state
    /// - Broadcasting fails
    pub async fn start_breakout_session(
        &self,
        _event_id: &Uuid,
        data: &serde_json::Value,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), VideoCallWSError> {
        let session_data: StartBreakoutSessionData = serde_json::from_value(data.clone())?;

        let user_id = connection.user.id;
        let ends = session_data.ends;

        // Check authorization and start the breakout session
        let authorized = self
            .with_video_call_state_mut(&session_data.event_id, |call| {
                // Check if the user is authorized (moderator or facilitator)
                if let Some(participant) = call.participants.get(&user_id)
                    && participant.is_host()
                {
                    call.breakout_session = Some(BreakoutSession { ends });
                    return true;
                }
                false
            })?
            .unwrap_or(false);

        if !authorized {
            return Err(VideoCallWSError::UnauthorizedStateChange);
        }

        self.broadcast_state(&session_data.event_id, state).await?;

        Ok(())
    }

    /// Extends the end time of an active breakout session.
    ///
    /// Only moderators and facilitators are authorized to extend breakout sessions.
    /// If no breakout session is currently active, this operation will fail.
    ///
    /// # Arguments
    ///
    /// * `_event_id` - The UUID of the event (unused, extracted from data)
    /// * `data` - JSON data containing event_id and the new end time
    /// * `connection` - WebSocket connection of the user requesting the extension
    /// * `state` - Application state for broadcasting
    ///
    /// # Authorization
    ///
    /// Only users with role "moderator" or "facilitator" can extend breakout sessions.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - JSON deserialization fails
    /// - User is not authorized (`UnauthorizedStateChange`)
    /// - No breakout session is currently active
    /// - Failed to acquire lock on the call state
    /// - Broadcasting fails
    pub async fn extend_breakout_session(
        &self,
        _event_id: &Uuid,
        data: &serde_json::Value,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), VideoCallWSError> {
        let extension_data: ExtendBreakoutSessionData = serde_json::from_value(data.clone())?;

        let user_id = connection.user.id;
        let new_ends = extension_data.ends;

        // Check authorization and extend the breakout session
        let authorized = self
            .with_video_call_state_mut(&extension_data.event_id, |call| {
                // Check if the user is authorized (moderator or facilitator)
                if let Some(participant) = call.participants.get(&user_id)
                    && participant.is_host()
                {
                    // Update the end time if a session is active
                    if let Some(session) = &mut call.breakout_session {
                        session.ends = new_ends;
                        return true;
                    }
                }
                false
            })?
            .unwrap_or(false);

        if !authorized {
            return Err(VideoCallWSError::UnauthorizedStateChange);
        }

        self.broadcast_state(&extension_data.event_id, state)
            .await?;

        Ok(())
    }

    /// Ends an active breakout session.
    ///
    /// Only moderators and facilitators are authorized to end breakout sessions.
    /// This sets the breakout_session field to None, indicating no session is active.
    ///
    /// # Arguments
    ///
    /// * `_event_id` - The UUID of the event (unused, extracted from data)
    /// * `data` - JSON data containing event_id
    /// * `connection` - WebSocket connection of the user requesting to end the session
    /// * `state` - Application state for broadcasting
    ///
    /// # Authorization
    ///
    /// Only users with role "moderator" or "facilitator" can end breakout sessions.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - JSON deserialization fails
    /// - User is not authorized (`UnauthorizedStateChange`)
    /// - Failed to acquire lock on the call state
    /// - Broadcasting fails
    pub async fn end_breakout_session(
        &self,
        _event_id: &Uuid,
        data: &serde_json::Value,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), VideoCallWSError> {
        let end_data: EndBreakoutSessionData = serde_json::from_value(data.clone())?;

        let user_id = connection.user.id;

        // Check authorization and end the breakout session
        let authorized = self
            .with_video_call_state_mut(&end_data.event_id, |call| {
                // Check if the user is authorized (moderator or facilitator)
                if let Some(participant) = call.participants.get(&user_id)
                    && participant.is_host()
                {
                    call.breakout_session = None;
                    return true;
                }
                false
            })?
            .unwrap_or(false);

        if !authorized {
            return Err(VideoCallWSError::UnauthorizedStateChange);
        }

        self.broadcast_state(&end_data.event_id, state).await?;

        Ok(())
    }

    /// Records a breakout room's real Jitsi JID, reported by whichever
    /// client first observes it, and triggers a jitsi-recorder session for
    /// that room the first time it's seen. Idempotent: a room's JID is only
    /// set (and only triggers) once, so duplicate reports from other
    /// participants' browsers for the same room are harmless no-ops.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - JSON deserialization fails
    /// - Failed to acquire lock on the call state
    pub async fn handle_breakout_room_live(
        &self,
        event_id: &Uuid,
        data: &serde_json::Value,
        _connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), VideoCallWSError> {
        let live_data: BreakoutRoomLiveData = serde_json::from_value(data.clone())?;

        let should_trigger = self
            .with_video_call_state_mut(event_id, |call| {
                let Some(room) = call.breakout_rooms.get_mut(live_data.room_index) else {
                    return false;
                };
                if room.room_jid.is_some() {
                    return false;
                }
                room.room_jid = Some(live_data.room_jid.clone());
                true
            })?
            .unwrap_or(false);

        if should_trigger {
            let event_id = *event_id;
            let room_jid = live_data.room_jid;
            let room_index = live_data.room_index;
            let state = state.clone();
            tokio::spawn(async move {
                if let Err(err) =
                    trigger_breakout_room_recording(&event_id, &room_jid, room_index, &state).await
                {
                    tracing::error!(?err, %event_id, room_jid, "failed to trigger jitsi-recorder for breakout room");
                }
            });
        }

        Ok(())
    }

    /// Broadcasts the current video call state to all participants.
    ///
    /// This is a convenience method used internally to synchronize state across
    /// all connected clients. The complete state (including participants, status,
    /// breakout rooms, and agenda step) is serialized and sent to each participant.
    ///
    /// # Arguments
    ///
    /// * `event_id` - The UUID of the event/call to broadcast
    /// * `state` - Application state for sending WebSocket messages
    ///
    /// # Called Automatically
    ///
    /// This method is automatically called after:
    /// - A user joins or leaves the call
    /// - The call state changes (Waiting → InProgress → Ended)
    /// - Breakout rooms are assigned
    /// - The agenda item is updated
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Failed to acquire lock on the call state
    /// - Call state not found for the given event_id
    /// - Serialization fails
    pub async fn broadcast_state(
        &self,
        event_id: &Uuid,
        state: &Arc<ComhairleState>,
    ) -> Result<(), VideoCallWSError> {
        // Extract the current state and participant IDs
        let (participant_ids, call_state) = self
            .with_video_call_state(event_id, |call| {
                let ids: Vec<Uuid> = call.participants.keys().copied().collect();
                // Serialize the state within the closure while we have the lock
                let serialized = serde_json::to_value(call).ok()?;
                Some((ids, serialized))
            })?
            .flatten()
            .ok_or(VideoCallWSError::VideoCallNotFound)?;

        // Broadcast the state to all participants
        let message = WebSocketMessage::Custom {
            event: "video_call:state_update".into(),
            data: call_state,
        };

        for user_id in participant_ids {
            let _ = state.websockets.send_to_user(&user_id, &message).await;
        }

        Ok(())
    }

    /// Pushes an updated agenda to every participant currently on the call for `event_id`,
    /// across all API instances in the cluster.
    ///
    /// `agenda` must already be serialized in the same shape the frontend receives from
    /// the events API (i.e. the `EventDto.agenda` value). No-op if nobody is on the call,
    /// so callers can fire this unconditionally.
    pub async fn broadcast_agenda_update(
        &self,
        event_id: &Uuid,
        agenda: serde_json::Value,
        state: &Arc<ComhairleState>,
    ) -> Result<(), VideoCallWSError> {
        let message = WebSocketMessage::Custom {
            event: "video_call:agenda_updated".into(),
            data: agenda,
        };

        // Fan out across the cluster: each instance resolves its own local participants
        // for this event (see `local_room_members`), so users on any instance receive it.
        let _ = state
            .websockets
            .broadcast_to_room(self.domain(), event_id, &message)
            .await;

        Ok(())
    }
}

/// Looks up the event and starts a jitsi-recorder session for its main room.
/// Called once, from a spawned task, the first time a real participant joins
/// an empty main room (see `VideoCallMessageHandler::handle_video_joined`).
async fn trigger_main_room_recording(
    event_id: &Uuid,
    state: &Arc<ComhairleState>,
) -> Result<(), crate::error::ComhairleError> {
    let event = crate::models::event::get_by_id(&state.db, event_id).await?;
    let video_meeting_id = event
        .video_meeting_id
        .ok_or(crate::error::ComhairleError::NoVideoMeetingId)?;
    let recorder_config = state.required_jitsi_recorder_config()?;

    crate::jitsi_recorder::trigger_recording(
        state,
        event.conversation_id,
        *event_id,
        &video_meeting_id.to_string(),
        Some(&recorder_config.main_muc_domain),
        "Main room",
    )
    .await
}

/// Looks up the event and starts a jitsi-recorder session for one breakout
/// room, given its real JID (`localpart@muc_domain`). Called once, from a
/// spawned task, the first time any client reports a given room's JID (see
/// `VideoCallMessageHandler::handle_breakout_room_live`).
async fn trigger_breakout_room_recording(
    event_id: &Uuid,
    room_jid: &str,
    room_index: usize,
    state: &Arc<ComhairleState>,
) -> Result<(), crate::error::ComhairleError> {
    let event = crate::models::event::get_by_id(&state.db, event_id).await?;
    let (room_name, muc_domain) = room_jid.split_once('@').ok_or_else(|| {
        crate::error::ComhairleError::JitsiRecorderError(format!(
            "breakout room jid {room_jid} is not a full JID"
        ))
    })?;

    crate::jitsi_recorder::trigger_recording(
        state,
        event.conversation_id,
        *event_id,
        room_name,
        Some(muc_domain),
        &format!("Breakout room #{}", room_index + 1),
    )
    .await
}

#[async_trait]
impl WebSocketMessageHandler for VideoCallMessageHandler {
    fn domain(&self) -> &str {
        "video_call"
    }

    /// Participants of the call for `room_id` (the event id) known to THIS instance.
    fn local_room_members(&self, room_id: &Uuid) -> Vec<Uuid> {
        self.with_video_call_state(room_id, |call| {
            call.participants.keys().copied().collect::<Vec<Uuid>>()
        })
        .ok()
        .flatten()
        .unwrap_or_default()
    }

    async fn handle_message(
        &self,
        message: &WebSocketMessage,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), crate::websockets::error::WebsocketError> {
        match message {
            WebSocketMessage::Custom { event, data } if event.starts_with("video_call:") => {
                let event_id: Uuid = serde_json::from_value(
                    data.get("event_id")
                        .ok_or(VideoCallWSError::NoEventSpecified)?
                        .clone(),
                )
                .map_err(|_| VideoCallWSError::FailedToDeserializeEventId)?;

                match event.as_str() {
                    "video_call:user_joined" => {
                        self.handle_user_join(&event_id, data, connection, state)
                            .await?
                    }
                    "video_call:user_left" => {
                        self.handle_user_leave(&event_id, data, connection, state)
                            .await?
                    }
                    "video_call:video_joined" => {
                        self.handle_video_joined(&event_id, data, connection, state)
                            .await?
                    }
                    "video_call:video_left" => {
                        self.handle_video_left(&event_id, connection, state).await?
                    }
                    "video_call:change_state" => {
                        self.change_call_status(&event_id, data, connection, state)
                            .await?
                    }
                    "video_call:assign_breakout_rooms" => {
                        self.assign_breakout_rooms(&event_id, data, connection, state)
                            .await?
                    }
                    "video_call:breakout_room_assistance_request" => {
                        self.breakout_room_assistance_request(&event_id, data, connection, state)
                            .await?
                    }
                    "video_call:resolve_breakout_room_assistance_request" => {
                        self.resolve_breakout_room_assistance_request(
                            &event_id, data, connection, state,
                        )
                        .await?
                    }
                    "video_call:set_agenda_item" => {
                        self.set_agenda_item(&event_id, data, connection, state)
                            .await?
                    }
                    "video_call:send_message" => {
                        self.broadcast_message(&event_id, data, connection, state)
                            .await?
                    }
                    "video_call:start_breakout_session" => {
                        self.start_breakout_session(&event_id, data, connection, state)
                            .await?
                    }
                    "video_call:extend_breakout_session" => {
                        self.extend_breakout_session(&event_id, data, connection, state)
                            .await?
                    }
                    "video_call:end_breakout_session" => {
                        self.end_breakout_session(&event_id, data, connection, state)
                            .await?
                    }
                    "video_call:breakout_room_live" => {
                        self.handle_breakout_room_live(&event_id, data, connection, state)
                            .await?
                    }
                    _ => {
                        info!("Unhandled video call event: {}", event);
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call_with(participants: &[(Uuid, &str)], rooms: Vec<Vec<Uuid>>) -> VideoCallState {
        let mut call = VideoCallState::new(Uuid::new_v4());
        for (user_id, role) in participants {
            call.participants.insert(
                *user_id,
                VideoCallParticipant {
                    user_id: *user_id,
                    username: None,
                    role: role.to_string(),
                },
            );
        }
        call.breakout_rooms = rooms
            .into_iter()
            .map(|participants| BreakoutRoomAssignments {
                participants,
                room_jid: None,
            })
            .collect();
        call
    }

    fn room_of(call: &VideoCallState, user_id: Uuid) -> Option<usize> {
        call.breakout_rooms
            .iter()
            .position(|room| room.participants.contains(&user_id))
    }

    #[test]
    fn walk_in_participant_goes_to_the_emptiest_room() {
        let (seated, walk_in) = (Uuid::new_v4(), Uuid::new_v4());
        let mut call = call_with(
            &[(seated, "participant"), (walk_in, "participant")],
            vec![vec![seated], vec![]],
        );

        call.place_in_breakout_room(walk_in);

        assert_eq!(room_of(&call, walk_in), Some(1));
    }

    #[test]
    fn hosts_are_never_placed_in_a_room() {
        let (moderator, facilitator) = (Uuid::new_v4(), Uuid::new_v4());
        let mut call = call_with(
            &[(moderator, "moderator"), (facilitator, "facilitator")],
            vec![vec![], vec![]],
        );

        call.place_in_breakout_room(moderator);
        call.place_in_breakout_room(facilitator);

        assert_eq!(room_of(&call, moderator), None);
        assert_eq!(room_of(&call, facilitator), None);
    }

    #[test]
    fn rejoining_keeps_a_participant_in_their_room() {
        let participant = Uuid::new_v4();
        let mut call = call_with(
            &[(participant, "participant")],
            vec![vec![participant], vec![]],
        );

        call.place_in_breakout_room(participant);

        assert_eq!(room_of(&call, participant), Some(0));
        assert_eq!(call.breakout_rooms[1].participants.len(), 0);
    }

    #[test]
    fn nobody_is_placed_before_rooms_exist() {
        let participant = Uuid::new_v4();
        let mut call = call_with(&[(participant, "participant")], vec![]);

        call.place_in_breakout_room(participant);

        assert!(call.breakout_rooms.is_empty());
    }

    #[test]
    fn random_split_leaves_hosts_out() {
        let (facilitator, participant) = (Uuid::new_v4(), Uuid::new_v4());
        let call = call_with(
            &[(facilitator, "facilitator"), (participant, "participant")],
            vec![],
        );

        assert_eq!(call.breakout_candidates(), vec![participant]);
    }
}
