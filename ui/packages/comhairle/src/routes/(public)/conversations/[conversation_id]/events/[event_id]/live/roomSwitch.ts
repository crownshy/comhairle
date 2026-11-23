const JITSI_MEDIA_TIMEOUT_MS = 60_000;
const MARGIN_MS = 10_000;

/** Whether a room switch took long enough that Jitsi must have timed out asking for media (Firefox specific) */
export function mediaTimedOutDuringSwitch(leftAt: number | null, joinedAt: number): boolean {
	if (leftAt === null) return false;
	return joinedAt - leftAt >= JITSI_MEDIA_TIMEOUT_MS - MARGIN_MS;
}
