import { ws } from '$lib/api/websockets.svelte';

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

export interface BreakoutSession {
	ends: string; // ISO 8601 datetime string
}

export interface VideoCallState {
	video_call_id: string;
	status: VideoCallStatus;
	participants: Record<string, VideoCallParticipant>;
	breakout_rooms: BreakoutRoomAssignments[];
	breakout_room_assistance_requests: Record<string, BreakoutRoomAssistanceRequest>;
	jitsi_call_id: string;
	current_agenda_step: number;
	breakout_session: BreakoutSession | null;
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

	get breakoutSession(): BreakoutSession | null {
		return this._currentCallState?.breakout_session ?? null;
	}

	private setupListeners() {
		if (this.isListening) return;
		this.isListening = true;

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

	joinCall(eventId: string) {
		ws.sendCustom('video_call:user_joined', {
			event_id: eventId
		});
	}

	leaveCall(eventId: string) {
		ws.sendCustom('video_call:user_left', {
			event_id: eventId
		});
		this._currentCallState = null;
	}

	/** Moderator/facilitator only */
	changeCallState(eventId: string, status: VideoCallStatus) {
		ws.sendCustom('video_call:change_state', {
			event_id: eventId,
			status
		});
	}

	/** Moderator/facilitator only */
	assignBreakoutRooms(eventId: string, maxUsersPerRoom: number) {
		ws.sendCustom('video_call:assign_breakout_rooms', {
			event_id: eventId,
			max_users_per_room: maxUsersPerRoom
		});
	}

	/** Moderator/facilitator only */
	setAgendaItem(eventId: string, agendaItem: number) {
		ws.sendCustom('video_call:set_agenda_item', {
			event_id: eventId,
			agenda_item: agendaItem
		});
	}

	/** Broadcast a message to all participants. Moderator/facilitator only. */
	broadcastMessage(eventId: string, message: string) {
		ws.sendCustom('video_call:send_message', {
			event_id: eventId,
			message
		});
	}

	requestBreakoutRoomAssistance(eventId: string, roomName: string) {
		ws.sendCustom('video_call:breakout_room_assistance_request', {
			event_id: eventId,
			room_name: roomName
		});
	}

	/** Resolve (clear) an assistance request. Moderator/facilitator only. */
	resolveBreakoutRoomAssistanceRequest(eventId: string, roomName: string) {
		ws.sendCustom('video_call:resolve_breakout_room_assistance_request', {
			event_id: eventId,
			room_name: roomName
		});
	}

	/** Moderator/facilitator only */
	startBreakoutSession(eventId: string, ends: string) {
		ws.sendCustom('video_call:start_breakout_session', {
			event_id: eventId,
			ends
		});
	}

	/** Moderator/facilitator only */
	extendBreakoutSession(eventId: string, ends: string) {
		ws.sendCustom('video_call:extend_breakout_session', {
			event_id: eventId,
			ends
		});
	}

	/** Moderator/facilitator only */
	endBreakoutSession(eventId: string) {
		ws.sendCustom('video_call:end_breakout_session', {
			event_id: eventId
		});
	}

	/** Returns the breakout room index for a user, or null if not assigned */
	getUserBreakoutRoom(userId: string): number | null {
		if (!this._currentCallState) return null;

		for (let i = 0; i < this._currentCallState.breakout_rooms.length; i++) {
			if (this._currentCallState.breakout_rooms[i].participants.includes(userId)) {
				return i;
			}
		}
		return null;
	}

	getBreakoutRoomParticipants(roomIndex: number): VideoCallParticipant[] {
		if (!this._currentCallState || roomIndex >= this._currentCallState.breakout_rooms.length) {
			return [];
		}

		const room = this._currentCallState.breakout_rooms[roomIndex];
		return room.participants
			.map((userId) => this._currentCallState!.participants[userId])
			.filter(Boolean);
	}

	isAuthorized(userId: string): boolean {
		if (!this._currentCallState) return false;

		const participant = this._currentCallState.participants[userId];
		if (!participant) return false;

		return participant.role === 'moderator' || participant.role === 'facilitator';
	}

	hasAssistanceRequest(roomName: string): boolean {
		if (!this._currentCallState) return false;
		return roomName in this._currentCallState.breakout_room_assistance_requests;
	}

	getRoomsWithAssistanceRequests(): string[] {
		if (!this._currentCallState) return [];
		return Object.keys(this._currentCallState.breakout_room_assistance_requests);
	}

	getAssistanceRequestUser(roomName: string): string | null {
		if (!this._currentCallState) return null;
		return (
			this._currentCallState.breakout_room_assistance_requests[roomName]?.made_by_user ?? null
		);
	}

	clearLastMessage() {
		this._lastBroadcastMessage = null;
	}

	/** Check if a breakout session is currently active. Uses loose equality (!=) to catch both null and undefined. */
	isBreakoutSessionActive(): boolean {
		return this._currentCallState?.breakout_session != null;
	}

	getBreakoutSessionEndTime(): Date | null {
		if (!this._currentCallState?.breakout_session) return null;
		return new Date(this._currentCallState.breakout_session.ends);
	}

	/** Returns time remaining in milliseconds */
	getBreakoutSessionTimeRemaining(): number | null {
		const endTime = this.getBreakoutSessionEndTime();
		if (!endTime) return null;
		return endTime.getTime() - Date.now();
	}
}

export const videoCallService = new VideoCallService();
