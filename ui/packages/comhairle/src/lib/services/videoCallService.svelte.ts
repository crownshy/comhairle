import { ws } from '$lib/api/websockets.svelte';

// Types matching backend structures
export type VideoCallStatus = 'Waiting' | 'InProgress' | 'Ended';

export interface VideoCallParticipant {
	user_id: string;
	username: string | null;
	role: string;
}

export interface BreakoutRoomAssignments {
	participants: string[];
}

export interface BreakoutRoomAssistanceRequest {
	made_by_user: string;
}

export interface VideoCallState {
	video_call_id: string;
	status: VideoCallStatus;
	participants: Record<string, VideoCallParticipant>;
	breakout_rooms: BreakoutRoomAssignments[];
	breakout_room_assistance_requests: Record<string, BreakoutRoomAssistanceRequest>;
	jitsi_call_id: string;
	current_agenda_step: number;
}

export interface BroadcastMessage {
	message: string;
}

export class VideoCallService {
	private _currentCallState = $state<VideoCallState | null>(null);
	private _lastBroadcastMessage = $state<string | null>(null);
	private isListening = false;

	constructor() {
		this.setupListeners();
	}

	get currentCallState(): VideoCallState | null {
		return this._currentCallState;
	}

	get lastBroadcastMessage(): string | null {
		return this._lastBroadcastMessage;
	}

	get isInCall(): boolean {
		return this._currentCallState !== null;
	}

	get participants(): VideoCallParticipant[] {
		if (!this._currentCallState) return [];
		return Object.values(this._currentCallState.participants);
	}

	get breakoutRooms(): BreakoutRoomAssignments[] {
		return this._currentCallState?.breakout_rooms ?? [];
	}

	get currentAgendaStep(): number {
		return this._currentCallState?.current_agenda_step ?? 0;
	}

	get callStatus(): VideoCallStatus | null {
		return this._currentCallState?.status ?? null;
	}

	get assistanceRequests(): Record<string, BreakoutRoomAssistanceRequest> {
		return this._currentCallState?.breakout_room_assistance_requests ?? {};
	}

	private setupListeners() {
		if (this.isListening) return;
		this.isListening = true;

		// Listen for video call events from server
		ws.on('custom', (payload) => {
			if (payload.event === 'video_call:state_update') {
				this.handleStateUpdate(payload.data as VideoCallState);
			} else if (payload.event === 'video_call:message') {
				this.handleBroadcastMessage(payload.data as BroadcastMessage);
			}
		});
	}

	private handleStateUpdate(state: VideoCallState) {
		console.log('Video call state updated:', state);
		this._currentCallState = state;
	}

	private handleBroadcastMessage(data: BroadcastMessage) {
		console.log('Received broadcast message:', data.message);
		this._lastBroadcastMessage = data.message;
	}

	// User joins a video call
	joinCall(eventId: string) {
		ws.sendCustom('video_call:user_joined', {
			event_id: eventId
		});
	}

	// User leaves a video call
	leaveCall(eventId: string) {
		ws.sendCustom('video_call:user_left', {
			event_id: eventId
		});
		// Clear local state when leaving
		this._currentCallState = null;
	}

	// Change the call state (moderator/facilitator only)
	changeCallState(eventId: string, status: VideoCallStatus) {
		ws.sendCustom('video_call:change_state', {
			event_id: eventId,
			status
		});
	}

	// Assign participants to breakout rooms (moderator/facilitator only)
	assignBreakoutRooms(eventId: string, maxUsersPerRoom: number) {
		ws.sendCustom('video_call:assign_breakout_rooms', {
			event_id: eventId,
			max_users_per_room: maxUsersPerRoom
		});
	}

	// Set the current agenda item (moderator/facilitator only)
	setAgendaItem(eventId: string, agendaItem: number) {
		ws.sendCustom('video_call:set_agenda_item', {
			event_id: eventId,
			agenda_item: agendaItem
		});
	}

	// Broadcast a message to all participants (moderator/facilitator only)
	broadcastMessage(eventId: string, message: string) {
		ws.sendCustom('video_call:send_message', {
			event_id: eventId,
			message
		});
	}

	// Request assistance in a breakout room
	requestBreakoutRoomAssistance(eventId: string, roomName: string) {
		ws.sendCustom('video_call:breakout_room_assistance_request', {
			event_id: eventId,
			room_name: roomName
		});
	}

	// Resolve (clear) an assistance request from a breakout room (moderator/facilitator only)
	resolveBreakoutRoomAssistanceRequest(eventId: string, roomName: string) {
		ws.sendCustom('video_call:resolve_breakout_room_assistance_request', {
			event_id: eventId,
			room_name: roomName
		});
	}

	// Helper to check if current user is in a specific breakout room
	getUserBreakoutRoom(userId: string): number | null {
		if (!this._currentCallState) return null;

		for (let i = 0; i < this._currentCallState.breakout_rooms.length; i++) {
			if (this._currentCallState.breakout_rooms[i].participants.includes(userId)) {
				return i;
			}
		}
		return null;
	}

	// Helper to get participants in a specific breakout room
	getBreakoutRoomParticipants(roomIndex: number): VideoCallParticipant[] {
		if (!this._currentCallState || roomIndex >= this._currentCallState.breakout_rooms.length) {
			return [];
		}

		const room = this._currentCallState.breakout_rooms[roomIndex];
		return room.participants
			.map((userId) => this._currentCallState!.participants[userId])
			.filter(Boolean);
	}

	// Helper to check if user has moderator/facilitator role
	isAuthorized(userId: string): boolean {
		if (!this._currentCallState) return false;

		const participant = this._currentCallState.participants[userId];
		if (!participant) return false;

		return participant.role === 'moderator' || participant.role === 'facilitator';
	}

	// Check if a specific room has an active assistance request
	hasAssistanceRequest(roomName: string): boolean {
		if (!this._currentCallState) return false;
		return roomName in this._currentCallState.breakout_room_assistance_requests;
	}

	// Get all room names with active assistance requests
	getRoomsWithAssistanceRequests(): string[] {
		if (!this._currentCallState) return [];
		return Object.keys(this._currentCallState.breakout_room_assistance_requests);
	}

	// Get the user who made the assistance request for a specific room
	getAssistanceRequestUser(roomName: string): string | null {
		if (!this._currentCallState) return null;
		return (
			this._currentCallState.breakout_room_assistance_requests[roomName]?.made_by_user ?? null
		);
	}

	// Clear last broadcast message
	clearLastMessage() {
		this._lastBroadcastMessage = null;
	}
}

// Singleton instance
export const videoCallService = new VideoCallService();
