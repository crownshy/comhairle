import type { VideoCallParticipant } from '$lib/services/videoCallService.svelte';

export type AgendaItemType = 'plenary' | 'breakout';

export interface AgendaItem {
	id: string;
	title: string;
	type: AgendaItemType;
	breakoutQuestion?: string;
	breakoutDescription?: string;
	durationMinutes?: number;
}

export type MeetingPhase = 'lobby' | 'incall' | 'ended';

export type RoomContext = 'plenary' | { type: 'breakout'; roomIndex: number; roomName: string };

export type PanelTab = 'agenda' | 'breakoutRooms';

export interface BreakoutRoomDisplay {
	index: number;
	name: string;
	participants: VideoCallParticipant[];
	hasAssistanceRequest: boolean;
	assistanceRequestUser?: string | null;
}
