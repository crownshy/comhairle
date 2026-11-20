import type {
	BreakoutRoomAssignments,
	VideoCallParticipant
} from '$lib/services/videoCallService.svelte';

/**
 * Group participants by the breakout room they are assigned to.
 *
 * Returns one bucket per room, in room order, so the caller can index by room number.
 * Participants with no assignment are dropped; a room with no participants stays as an
 * empty bucket, which keeps the room indices lined up with Jitsi's.
 */
export function groupParticipantsByRoom(
	participants: VideoCallParticipant[],
	rooms: BreakoutRoomAssignments[]
): VideoCallParticipant[][] {
	const roomIndexByUserId = new Map<string, number>();
	rooms.forEach((room, index) => {
		room.participants.forEach((userId) => roomIndexByUserId.set(userId, index));
	});

	const roomAssignments: VideoCallParticipant[][] = rooms.map(() => []);

	participants.forEach((p) => {
		const roomIndex = roomIndexByUserId.get(p.user_id);

		if (roomIndex !== undefined) {
			roomAssignments[roomIndex].push(p);
		}
	});

	return roomAssignments;
}

/** People who are not in any of the given rooms, so they stay in the main call. */
export function peopleOutsideRooms(
	participants: VideoCallParticipant[],
	rooms: VideoCallParticipant[][]
): VideoCallParticipant[] {
	const inARoom = new Set(rooms.flat().map((p) => p.user_id));
	return participants.filter((p) => !inARoom.has(p.user_id));
}

/** Where someone is during a breakout session: a room index, or the main call. */
export type BreakoutPlacement = number | 'plenary';

/**
 * Where a user has been put, or null when they have no placement at all (a host who was
 * never assigned, for example).
 */
export function placementOf(
	userId: string,
	rooms: BreakoutRoomAssignments[],
	plenaryIds: Set<string>
): BreakoutPlacement | null {
	const roomIndex = rooms.findIndex((room) => room.participants.includes(userId));
	if (roomIndex !== -1) return roomIndex;
	return plenaryIds.has(userId) ? 'plenary' : null;
}

/**
 * Turn a facilitator's edited room layout into the assignments to send mid-session.
 *
 * The edit dialog works from a snapshot taken when it opened. Anyone placed in a room after
 * that (a late joiner, for example) is missing from the edit, so they keep the room they are
 * in now rather than ending up with no room at all. People moved to plenary are left out of
 * every room. The room count never drops below the current one, because each index maps to
 * a Jitsi room that already exists.
 */
export function applyRoomEdits(
	editedRooms: string[][],
	currentRooms: BreakoutRoomAssignments[],
	plenary: string[] = []
): string[][] {
	const roomCount = Math.max(editedRooms.length, currentRooms.length);
	const result = Array.from({ length: roomCount }, (_, index) => [...(editedRooms[index] ?? [])]);
	const placed = new Set([...editedRooms.flat(), ...plenary]);

	currentRooms.forEach((room, index) => {
		room.participants
			.filter((userId) => !placed.has(userId))
			.forEach((userId) => {
				result[index].push(userId);
				placed.add(userId);
			});
	});

	return result;
}

/** Whether an event role can run a breakout room (moderators and facilitators). */
export function isFacilitatorRole(role: string): boolean {
	return role === 'moderator' || role === 'facilitator';
}
