export const PARTICIPANT_ACTIVITY_EVENT = 'comhairle:participant-activity';

export function reportParticipantActivity() {
	window.dispatchEvent(new Event(PARTICIPANT_ACTIVITY_EVENT));
}
