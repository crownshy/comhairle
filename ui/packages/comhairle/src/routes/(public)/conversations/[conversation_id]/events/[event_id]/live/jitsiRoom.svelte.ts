import type { JitsiMeetExternalApi } from '$lib/components/JitsiMeet/types';
import { getJitsiBreakoutRoomId, type JitsiBreakoutRoom } from '$lib/utils/jitsiBreakoutRooms';
import { tryCatchAsync } from '$lib/utils/errorHandling';

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

	/** Every breakout room Jitsi has reported, main room excluded. */
	get rooms(): JitsiBreakoutRoom[] {
		return Object.values(this._rooms).filter((room) => !room.isMainRoom);
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
	 * Recheck `ready` against the number of rooms the session expects. Pass null when no
	 * breakout session is running, which drops it back to false.
	 */
	syncReady(expectedRoomCount: number | null) {
		if (expectedRoomCount === null) {
			this._ready = false;
			return;
		}
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
		const roomJid = getJitsiBreakoutRoomId(this._rooms, roomIndex);
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
	 * Close every breakout room. Jitsi returns the people still in them to the main room.
	 * Asks Jitsi for a fresh list first, because ours is only as current as the last
	 * `breakoutRoomsUpdated` we saw.
	 */
	async closeAll() {
		let rooms: JitsiBreakoutRoom[] = [];
		const listed = await tryCatchAsync(async () => this._api?.listBreakoutRooms?.());
		if (listed.err !== null) {
			console.error('[BREAKOUT] Error getting breakout rooms:', listed.err);
			rooms = this.rooms;
		} else if (listed.ok) {
			rooms = Object.values(listed.ok).filter((room) => !room.isMainRoom);
		}

		for (const room of rooms) {
			if (!room.id) continue;
			try {
				this._api?.executeCommand('closeBreakoutRoom', room.id);
			} catch (error) {
				console.error('[BREAKOUT] Error removing breakout room:', error);
			}
		}
	}

	hangup() {
		this._api?.executeCommand('hangup');
	}

	/** Forget the room list. The call itself stays up. */
	reset() {
		this._rooms = {};
		this._ready = false;
	}
}
