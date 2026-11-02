const PREFIX = 'comhairle-polis-onboarding';

export function hasSeenOnboarding(scope: string): boolean {
	if (typeof window === 'undefined') return false;
	try {
		return localStorage.getItem(`${PREFIX}-${scope}`) === 'seen';
	} catch {
		return false;
	}
}

export function markOnboardingSeen(scope: string): void {
	if (typeof window === 'undefined') return;
	try {
		localStorage.setItem(`${PREFIX}-${scope}`, 'seen');
	} catch {
		// Private browsing or blocked site data. Showing onboarding twice beats never showing it.
	}
}
