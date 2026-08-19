import type { JitsiBreakoutRoom } from '$lib/utils/jitsiBreakoutRooms';

/**
 * The payload each Jitsi command takes. Jitsi's external API is a single
 * `executeCommand(name, ...args)` entry point, so this map is what keeps the call sites
 * honest about what they are sending.
 */
export interface JitsiCommands {
	hangup: [];
	toggleAudio: [];
	toggleVideo: [];
	toggleShareScreen: [];
	setTileView: [enabled: boolean];
	addBreakoutRoom: [name?: string];
	/** Called with no jid, this returns the local participant to the main room. */
	joinBreakoutRoom: [roomJid?: string];
	closeBreakoutRoom: [roomId: string];
}

/** The part of the `JitsiMeetExternalAPI` instance we actually call. */
export interface JitsiMeetExternalApi {
	executeCommand<Command extends keyof JitsiCommands>(
		command: Command,
		...args: JitsiCommands[Command]
	): void;
	listBreakoutRooms?(): Promise<Record<string, JitsiBreakoutRoom>>;
	isModeratorEnabled?(): boolean;
	dispose(): void;
}

/**
 * Payload of the `videoConferenceJoined` and `videoConferenceLeft` events. Jitsi is
 * inconsistent about the casing of the display name between events, so both spellings
 * turn up.
 */
export interface JitsiConferenceEvent {
	/** The local participant's Jitsi id. Only sent on join. */
	id?: string;
	roomName: string;
	displayName?: string;
	displayname?: string;
}

/** Payload of the `displayNameChange` event, raised for any participant in the call. */
export interface JitsiDisplayNameChangeEvent {
	id?: string;
	displayName?: string;
	displayname?: string;
}
