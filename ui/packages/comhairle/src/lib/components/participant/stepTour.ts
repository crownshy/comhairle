const STORAGE_KEY = 'comhairle-step-tour';

// Keyed by conversation. Finding your way around the chrome is learned once, but a
// participant who has only ever done one conversation has not necessarily learned it, and
// the cost of offering it a second time is one tap.
function storageKey(conversationId: string): string {
	return `${STORAGE_KEY}-${conversationId}`;
}

type ProgressLike = { progressStatus?: string | null };

/**
 * Whether this looks like a first time through: the participant has joined and nothing is
 * finished yet. Read from progress rather than from a flag of our own, so someone who comes
 * back to an unfinished conversation in another browser is still offered it, and someone who
 * has already completed a step never is.
 */
export function isFirstRun(steps: ProgressLike[] | null | undefined): boolean {
	if (!steps?.length) return false;
	return !steps.some((step) => step.progressStatus === 'done');
}

export function hasSeenStepTour(conversationId: string): boolean {
	if (typeof window === 'undefined') return false;
	try {
		return localStorage.getItem(storageKey(conversationId)) === 'seen';
	} catch {
		return false;
	}
}

export function markStepTourSeen(conversationId: string): void {
	if (typeof window === 'undefined') return;
	try {
		localStorage.setItem(storageKey(conversationId), 'seen');
	} catch {
		/* ignore */
	}
}
