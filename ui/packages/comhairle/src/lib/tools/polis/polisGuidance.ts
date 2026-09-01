const STORAGE_KEY = 'comhairle-polis-opinion-guidance';

// Keyed by participant only, not by step or poll. The rules for writing a good opinion are
// the same everywhere, so someone who has read them once should not be shown them again on
// the next Polis step.
function storageKey(userId: string): string {
	return `${STORAGE_KEY}-${userId}`;
}

export function hasSeenOpinionGuidance(userId: string): boolean {
	if (typeof window === 'undefined') return false;
	try {
		return localStorage.getItem(storageKey(userId)) === 'seen';
	} catch {
		return false;
	}
}

export function markOpinionGuidanceSeen(userId: string): void {
	if (typeof window === 'undefined') return;
	try {
		localStorage.setItem(storageKey(userId), 'seen');
	} catch {
		/* ignore */
	}
}
