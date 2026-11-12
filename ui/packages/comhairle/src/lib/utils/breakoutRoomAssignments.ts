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

/**
 * Turn a facilitator's edited room layout into the assignments to send mid-session.
 *
 * The edit dialog works from a snapshot taken when it opened. Anyone placed in a room after
 * that (a late joiner, for example) is missing from the edit, so they keep the room they are
 * in now rather than ending up with no room at all. The room count never drops below the
 * current one, because each index maps to a Jitsi room that already exists.
 */
export function applyRoomEdits(
	editedRooms: string[][],
	currentRooms: BreakoutRoomAssignments[]
): string[][] {
	const roomCount = Math.max(editedRooms.length, currentRooms.length);
	const result = Array.from({ length: roomCount }, (_, index) => [...(editedRooms[index] ?? [])]);
	const placed = new Set(editedRooms.flat());

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
