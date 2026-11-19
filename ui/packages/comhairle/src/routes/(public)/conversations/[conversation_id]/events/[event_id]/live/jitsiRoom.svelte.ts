import { SvelteSet } from 'svelte/reactivity';
import type { JitsiMeetExternalApi } from '$lib/components/JitsiMeet/types';
import {
	getJitsiBreakoutRoomId,
	isEmptyRoom,
	type JitsiBreakoutRoom
} from '$lib/utils/jitsiBreakoutRooms';
import { tryCatchAsync } from '$lib/utils/errorHandling';

const ROOMS_EMPTY_POLL_MS = 250;
const ROOMS_EMPTY_TIMEOUT_MS = 5000;

/**
 * The live event page's handle on the Jitsi call.
 *
 * Everything that talks to Jitsi goes through here, so the page never holds the api
 * object, a command name or a room jid. Rooms are not what we asked for, they are what
 * Jitsi has told us about over `breakoutRoomsUpdated`, which arrives a round trip after
 * the commands that create them. That lag is what `ready` tracks.
 */
export class JitsiRoom {
	private _api: JitsiMeetExternalApi | null = $state(null);
	private _rooms = $state<Record<string, JitsiBreakoutRoom>>({});
	private _ready = $state(false);
	private _sessionRunning = false;
	/** Rooms from sessions that have ended, ignored if Jitsi reports them again. */
	private _retiredRooms = new SvelteSet<string>();

	/** Every breakout room Jitsi has reported for this session, main room excluded. */
	get rooms(): JitsiBreakoutRoom[] {
		return Object.values(this._rooms).filter(
			(room) => !room.isMainRoom && !this._retiredRooms.has(roomKey(room))
		);
	}

	/** Whether Jitsi has caught up with the rooms we asked it to create. */
	get ready() {
		return this._ready;
	}

	get connected() {
		return this._api !== null;
	}

	attach(api: JitsiMeetExternalApi) {
		this._api = api;
	}

	/** Takes the room list straight from Jitsi's `breakoutRoomsUpdated` event. */
	setRooms(rooms: Record<string, JitsiBreakoutRoom>) {
		this._rooms = rooms;
	}

	/**
	 * The real Jitsi jid for the room at `roomIndex`, or null if Jitsi hasn't reported
	 * one yet. Same index/ordering `joinBreakout` uses, so a caller that gets a jid back
	 * here can hand it to the backend as that room's identity.
	 */
	jidForIndex(roomIndex: number): string | null {
		return getJitsiBreakoutRoomId(this._rooms, roomIndex);
	}

	/**
	 * Recheck `ready` against the number of rooms the session expects. Pass null when no
	 * breakout session is running. When a session ends this forgets its rooms on every
	 * client, so the next session waits for its own rooms instead of joining old ones.
	 */
	syncReady(expectedRoomCount: number | null) {
		if (expectedRoomCount === null) {
			if (this._sessionRunning) this.reset();
			this._sessionRunning = false;
			this._ready = false;
			return;
		}
		this._sessionRunning = true;
		const reported = this.rooms.length;
		if (reported > 0 && reported >= expectedRoomCount) {
			this._ready = true;
		}
	}

	create(roomCount: number) {
		for (let i = 0; i < roomCount; i++) {
			this._api?.executeCommand('addBreakoutRoom', `Breakout room #${i + 1}`);
		}
	}

	/**
	 * Move the local participant into the nth breakout room. Returns false when Jitsi has
	 * no room at that index, which is the desync behind #752.
	 */
	joinBreakout(roomIndex: number): boolean {
		const roomJid = getJitsiBreakoutRoomId(this.rooms, roomIndex);
		if (!roomJid) {
			console.warn(
				'[BREAKOUT] No Jitsi room ID found for index:',
				roomIndex,
				'— available rooms:',
				this.rooms.map((room) => ({ id: room.id, name: room.name }))
			);
			return false;
		}
		this._api?.executeCommand('joinBreakoutRoom', roomJid);
		return true;
	}

	returnToMain() {
		try {
			this._api?.executeCommand('joinBreakoutRoom');
		} catch (error) {
			console.error('[BREAKOUT] Error returning to main room:', error);
		}
	}

	/**
	 * Remove every breakout room. Removing a room with people still inside can drop them
	 * out of the call, which shows them "You left the meeting". So close the rooms first,
	 * which moves everyone to the main room, wait until Jitsi reports them empty, then
	 * remove them. Closing alone is not enough: a closed room stays, and the next session's
	 * rooms would reuse its name.
	 */
	async removeAll() {
		const rooms = await this.listRooms();
		for (const room of rooms) {
			if (room.id) this.send('closeBreakoutRoom', room.id);
		}
		await this.waitUntilRoomsEmpty();
		for (const room of rooms) {
			if (room.jid) this.send('removeBreakoutRoom', room.jid);
		}
	}

	/**
	 * Ask Jitsi for a fresh list, because ours is only as current as the last
	 * `breakoutRoomsUpdated` we saw. Falls back to ours if Jitsi won't answer.
	 */
	private async listRooms(): Promise<JitsiBreakoutRoom[]> {
		const listed = await tryCatchAsync(async () => this._api?.listBreakoutRooms?.());
		if (listed.err !== null) {
			console.error('[BREAKOUT] Error getting breakout rooms:', listed.err);
			return this.rooms;
		}
		return Object.values(listed.ok ?? {}).filter((room) => !room.isMainRoom);
	}

	private async waitUntilRoomsEmpty() {
		const giveUpAt = Date.now() + ROOMS_EMPTY_TIMEOUT_MS;
		while (Date.now() < giveUpAt) {
			const rooms = await this.listRooms();
			if (rooms.every(isEmptyRoom)) return;
			await new Promise((resolve) => setTimeout(resolve, ROOMS_EMPTY_POLL_MS));
		}
		console.warn('[BREAKOUT] Rooms still had people in them after closing; removing anyway');
	}

	private send<Command extends 'closeBreakoutRoom' | 'removeBreakoutRoom'>(
		command: Command,
		roomAddress: string
	) {
		try {
			this._api?.executeCommand(command, roomAddress);
		} catch (error) {
			console.error(`[BREAKOUT] Error sending ${command}:`, error);
		}
	}

	hangup() {
		this._api?.executeCommand('hangup');
	}

	/** Forget the room list, including rooms Jitsi has yet to remove. The call stays up. */
	reset() {
		for (const room of Object.values(this._rooms)) {
			if (!room.isMainRoom) this._retiredRooms.add(roomKey(room));
		}
		this._rooms = {};
		this._ready = false;
	}
}

function roomKey(room: JitsiBreakoutRoom): string {
	return room.jid ?? room.id ?? '';
}
