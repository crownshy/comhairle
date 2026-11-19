type AutoEnterInput = {
	isModerator: boolean;
	assignedRoom: number | null;
	moderatorAlreadyAutoEntered: boolean;
};

/**
 * The breakout room to move this client into when a session starts, or null to stay in
 * plenary. Moderators go in once per session, so leaving early to help another room
 * sticks, and only when they have a room. Participants always go back in, falling back
 * to room 0 for mock sessions with no assignment.
 */
export function breakoutAutoEnterRoom({
	isModerator,
	assignedRoom,
	moderatorAlreadyAutoEntered
}: AutoEnterInput): number | null {
	if (!isModerator) return assignedRoom ?? 0;
	if (moderatorAlreadyAutoEntered) return null;
	return assignedRoom;
}
