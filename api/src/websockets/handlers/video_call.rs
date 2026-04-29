use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::info;
use uuid::Uuid;

use crate::{
    websockets::{messages::WebSocketMessage, WebSocketConnection, WebSocketMessageHandler},
    ComhairleState,
};

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

/// Represents a breakout room assignment with a list of participant UUIDs.
///
/// Used to organize participants into smaller groups during a video call.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BreakoutRoomAssignments {
    pub participants: Vec<Uuid>,
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
    /// List of breakout room assignments
    pub breakout_rooms: Vec<BreakoutRoomAssignments>,
    /// Active requests for assistance from breakout rooms
    pub breakout_room_assistance_requests: HashMap<String, BreakoutRoomAssistanceRequest>,
    /// Current step in the agenda (0-indexed)
    pub current_agenda_step: u32,
    /// Active breakout session with scheduled end time (None when no session is active)
    pub breakout_session: Option<BreakoutSession>,
}

/// WebSocket message handler for video call events.
///
/// Manages the state of all active video calls and handles incoming
/// WebSocket messages related to video call operations.
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
            video_call_id,
            breakout_room_assistance_requests: HashMap::new(),
            current_agenda_step: 0,
            breakout_rooms: vec![],
            breakout_session: None,
        }
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
                if let Some(sender) = call.participants.get(&connection.user.id) {
                    if sender.role == "moderator" || sender.role == "facilitator" {
                        // Return all participant user IDs
                        return Some(call.participants.keys().copied().collect::<Vec<Uuid>>());
                    }
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
    /// with the fewest participants.
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

        // Initialize video call state if it doesn't exist
        {
            let mut video_calls = self
                .video_calls
                .write()
                .map_err(|_| VideoCallWSError::FailedToGetLockOnRoom)?;

            video_calls
                .entry(join_data.event_id)
                .or_insert_with(|| VideoCallState::new(join_data.event_id));
        }

        // Add the participant to the video call and auto-assign to breakout room if needed
        self.with_video_call_state_mut(&join_data.event_id, |call| {
            call.participants.insert(
                user_id,
                VideoCallParticipant {
                    user_id,
                    username,
                    role,
                },
            );

            // If breakout rooms exist, auto-assign the user to a room
            if !call.breakout_rooms.is_empty() {
                // Check if user is already in a breakout room
                let already_assigned = call
                    .breakout_rooms
                    .iter()
                    .any(|room| room.participants.contains(&user_id));

                if !already_assigned {
                    // Find the room with the fewest participants
                    if let Some((room_index, _)) = call
                        .breakout_rooms
                        .iter()
                        .enumerate()
                        .min_by_key(|(_, room)| room.participants.len())
                    {
                        // Add user to the room with the fewest participants
                        call.breakout_rooms[room_index].participants.push(user_id);
                    }
                }
            }
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
        })?;

        self.broadcast_state(&leave_data.event_id, state).await?;

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
                if let Some(participant) = call.participants.get(&user_id) {
                    if participant.role == "moderator" || participant.role == "facilitator" {
                        call.status = new_status;
                        return true;
                    }
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
            .map_err(|e| VideoCallWSError::IncorrectMessageFormat(e))?;

        self.with_video_call_state_mut(event_id, |call| {
            call.breakout_room_assistance_requests
                .entry(request.room_name)
                .or_insert(BreakoutRoomAssistanceRequest {
                    made_by_user: connection.user.id.clone(),
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
                if let Some(participant) = call.participants.get(&user_id) {
                    if participant.role == "moderator" || participant.role == "facilitator" {
                        // Remove the assistance request for this room
                        call.breakout_room_assistance_requests
                            .remove(&request.room_name);
                        return true;
                    }
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
                if let Some(participant) = call.participants.get(&user_id) {
                    if participant.role == "moderator" || participant.role == "facilitator" {
                        let breakout_rooms = if let Some(ref explicit) = assignment_data.room_assignments {
                            // Use explicit assignments, filtering to known participants
                            explicit
                                .iter()
                                .map(|room_ids| BreakoutRoomAssignments {
                                    participants: room_ids
                                        .iter()
                                        .filter(|id| call.participants.contains_key(id))
                                        .copied()
                                        .collect(),
                                })
                                .filter(|room| !room.participants.is_empty())
                                .collect()
                        } else {
                            // Collect all participant IDs
                            let mut participant_ids: Vec<Uuid> =
                                call.participants.keys().copied().collect();

                            // Shuffle participants randomly
                            let mut rng = rand::thread_rng();
                            participant_ids.shuffle(&mut rng);

                            // Divide into rooms with max_users_per_room
                            participant_ids
                                .chunks(max_users)
                                .map(|chunk| BreakoutRoomAssignments {
                                    participants: chunk.to_vec(),
                                })
                                .collect()
                        };

                        call.breakout_rooms = breakout_rooms;
                        return true;
                    }
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
                if let Some(participant) = call.participants.get(&user_id) {
                    if participant.role == "moderator" || participant.role == "facilitator" {
                        call.breakout_session = Some(BreakoutSession { ends });
                        return true;
                    }
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
                if let Some(participant) = call.participants.get(&user_id) {
                    if participant.role == "moderator" || participant.role == "facilitator" {
                        // Update the end time if a session is active
                        if let Some(session) = &mut call.breakout_session {
                            session.ends = new_ends;
                            return true;
                        }
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
                if let Some(participant) = call.participants.get(&user_id) {
                    if participant.role == "moderator" || participant.role == "facilitator" {
                        call.breakout_session = None;
                        return true;
                    }
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
}

#[async_trait]
impl WebSocketMessageHandler for VideoCallMessageHandler {
    fn domain(&self) -> &str {
        "video_call"
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
                            &event_id,
                            data,
                            connection,
                            state,
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
