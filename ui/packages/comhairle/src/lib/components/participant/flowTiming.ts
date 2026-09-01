/**
 * How long the participant has been in this conversation, measured in the browser.
 *
 * There is nothing on the server to read: `user_progress` carries a status and no
 * timestamps, and participation's `created_at` spans every visit, so a participant who
 * finished a week after they started would be told they spent a week on it. The step pages
 * stamp a clock instead, and the thank-you page reports what it says.
 */

const STORAGE_KEY = 'comhairle-flow-time';

/** A gap longer than this is a new sitting rather than a very slow one. */
const SITTING_GAP_MS = 30 * 60_000;

/** Beyond this the stamps are not describing one sitting, so we would rather say nothing. */
const MAX_SITTING_MS = 4 * 60 * 60_000;

export type FlowTiming = { startedAt: number; lastSeenAt: number };

function storageKey(conversationId: string): string {
	return `${STORAGE_KEY}-${conversationId}`;
}

/**
 * The stamps after another moment of activity. A long gap since the last one starts the
 * clock again, so someone who comes back the next day is timed from when they came back.
 */
export function nextTiming(stored: FlowTiming | null, now: number): FlowTiming {
	if (!stored || now < stored.startedAt || now - stored.lastSeenAt > SITTING_GAP_MS) {
		return { startedAt: now, lastSeenAt: now };
	}
	return { startedAt: stored.startedAt, lastSeenAt: now };
}

/**
 * Whole minutes spent, or null when the stamps cannot answer: nothing recorded, a clock
 * that moved under us, or a sitting long enough that the number would be fiction. Anything
 * shorter than a minute reports one, because "0 min" reads as a failure rather than a fast
 * participant.
 */
export function minutesSpent(stored: FlowTiming | null, now: number): number | null {
	if (!stored) return null;
	const elapsed = now - stored.startedAt;
	if (elapsed < 0 || elapsed > MAX_SITTING_MS) return null;
	if (now - stored.lastSeenAt > SITTING_GAP_MS) return null;
	return Math.max(1, Math.round(elapsed / 60_000));
}

function readTiming(conversationId: string): FlowTiming | null {
	if (typeof window === 'undefined') return null;
	try {
		const raw = localStorage.getItem(storageKey(conversationId));
		if (!raw) return null;
		const parsed = JSON.parse(raw) as Partial<FlowTiming>;
		if (typeof parsed?.startedAt !== 'number' || typeof parsed?.lastSeenAt !== 'number') {
			return null;
		}
		return { startedAt: parsed.startedAt, lastSeenAt: parsed.lastSeenAt };
	} catch {
		return null;
	}
}

/** Records that the participant is here now. Called by every step as it opens. */
export function touchFlowTiming(conversationId: string, now = Date.now()): void {
	if (typeof window === 'undefined') return;
	try {
		const next = nextTiming(readTiming(conversationId), now);
		localStorage.setItem(storageKey(conversationId), JSON.stringify(next));
	} catch {
		/* ignore */
	}
}

/** Minutes to report on the thank-you page, or null when we have no honest number. */
export function minutesInFlow(conversationId: string, now = Date.now()): number | null {
	return minutesSpent(readTiming(conversationId), now);
}
