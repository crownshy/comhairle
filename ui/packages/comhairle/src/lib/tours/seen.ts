const PREFIX = 'comhairle-tour';

/**
 * A tour is dismissed per scope, not globally. The step tour scopes to the conversation:
 * finding your way around the chrome is learned once, but a participant who has only ever
 * done one conversation has not necessarily learned it, and the cost of offering it a second
 * time is one tap.
 */
function storageKey(tourId: string, scope: string): string {
	return `${PREFIX}-${tourId}-${scope}`;
}

export function hasSeenTour(tourId: string, scope: string): boolean {
	if (typeof window === 'undefined') return false;
	try {
		return localStorage.getItem(storageKey(tourId, scope)) === 'seen';
	} catch {
		return false;
	}
}

export function markTourSeen(tourId: string, scope: string): void {
	if (typeof window === 'undefined') return;
	try {
		localStorage.setItem(storageKey(tourId, scope), 'seen');
	} catch {
		/* Private browsing and blocked site data. Shown twice beats never shown. */
	}
}
