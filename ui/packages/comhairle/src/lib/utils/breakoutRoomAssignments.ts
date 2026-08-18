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
