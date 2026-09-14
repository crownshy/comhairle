/** How long participants get to wrap up after a facilitator ends the session. */
export const BREAKOUT_WRAP_UP_MS = 60_000;

/**
 * True once the session is inside its final wrap-up minute, whether the facilitator
 * started it or the timer simply ran down. The phase is read off the remaining time
 * rather than stored, so a reload or a second facilitator sees the same thing.
 */
export function isBreakoutWrappingUp(msRemaining: number | null): boolean {
	return msRemaining !== null && msRemaining <= BREAKOUT_WRAP_UP_MS;
}

/**
 * The end time to broadcast when a facilitator ends the session. Only ever shortens the
 * session, so ending it never hands participants extra time.
 */
export function wrapUpEndTime(currentEnd: Date, now: number): Date {
	return new Date(Math.min(currentEnd.getTime(), now + BREAKOUT_WRAP_UP_MS));
}
