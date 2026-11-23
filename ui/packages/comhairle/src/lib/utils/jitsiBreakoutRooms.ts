/** The subset of Jitsi's breakout room payload we read. */
export interface JitsiBreakoutRoom {
	// jid: the XMPP address Jitsi identifies a room by, and what you pass to join or remove it.
	jid?: string;
	/** Jitsi's short id for the room. */
	id?: string;
	name?: string;
	isMainRoom?: boolean;
	/** Who Jitsi has in the room, keyed by their jid. */
	participants?: Record<string, unknown>;
}

export function isEmptyRoom(room: JitsiBreakoutRoom): boolean {
	return Object.keys(room.participants ?? {}).length === 0;
}

/**
 * Order rooms the way our room numbers count them. The numeric collation keeps
 * "Breakout room #10" after "#2", and the jid tie-break means every client picks the same
 * room if two ever share a name.
 */
function compareRooms(a: JitsiBreakoutRoom, b: JitsiBreakoutRoom): number {
	const byName = (a.name ?? '').localeCompare(b.name ?? '', 'en', { numeric: true });
	return byName !== 0 ? byName : (a.jid ?? '').localeCompare(b.jid ?? '');
}

/**
 * Translate a 0-based breakout room index into the Jitsi room jid to join.
 *
 * Jitsi hands us the rooms keyed by an internal id, with the main room mixed in, so the
 * ordering our room numbers rely on comes from sorting the non-main rooms by name.
 */
export function getJitsiBreakoutRoomId(
	rooms: JitsiBreakoutRoom[],
	roomIndex: number
): string | null {
	const nonMainRooms = rooms.filter((room) => !room.isMainRoom).sort(compareRooms);
	return nonMainRooms[roomIndex]?.jid ?? null;
}
