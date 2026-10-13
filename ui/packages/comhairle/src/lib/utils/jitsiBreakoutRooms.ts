/** The subset of Jitsi's breakout room payload we read. */
export interface JitsiBreakoutRoom {
	// jid: the XMPP address Jitsi identifies a room by, and what you pass to join it.
	jid?: string;
	/** Jitsi's short id for the room, which is what `closeBreakoutRoom` takes. */
	id?: string;
	name?: string;
	isMainRoom?: boolean;
}

/**
 * Translate a 0-based breakout room index into the Jitsi room jid to join.
 *
 * Jitsi hands us the rooms as an object keyed by an internal id, with the main room mixed
 * in, so the ordering our room numbers rely on comes from sorting the non-main rooms by
 * name.
 */
export function getJitsiBreakoutRoomId(
	rooms: Record<string, JitsiBreakoutRoom>,
	roomIndex: number
): string | null {
	const nonMainRooms = Object.values(rooms)
		.filter((r) => !r.isMainRoom)
		.sort((a, b) => (a.name ?? '').localeCompare(b.name ?? ''));
	return nonMainRooms[roomIndex]?.jid ?? null;
}
