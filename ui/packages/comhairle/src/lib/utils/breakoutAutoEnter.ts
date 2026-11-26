type AutoEnterInput = {
	isModerator: boolean;
	assignedRoom: number | null;
	moderatorAlreadyAutoEntered: boolean;
	keptInPlenary: boolean;
};

/**
 * The breakout room to move this client into when a session starts, or null to stay in
 * plenary. Moderators go in once per session, so leaving early to help another room
 * sticks, and only when they have a room. Participants always go back in, falling back
 * to room 0 for mock sessions with no assignment, unless a host kept them in plenary.
 */
export function breakoutAutoEnterRoom({
	isModerator,
	assignedRoom,
	moderatorAlreadyAutoEntered,
	keptInPlenary
}: AutoEnterInput): number | null {
	if (keptInPlenary) return null;
	if (!isModerator) return assignedRoom ?? 0;
	if (moderatorAlreadyAutoEntered) return null;
	return assignedRoom;
}
