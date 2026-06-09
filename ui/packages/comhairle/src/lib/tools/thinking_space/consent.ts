// Local stub for participant sharing consent. Swap localStorage for a backend
// call once the API lands — callsites use only get/set, so the seam stays here.
export type ConsentState = 'shared' | 'private';

const KEY_PREFIX = 'thinking_space_consent:';

function key(workflowStepId: string): string {
	return `${KEY_PREFIX}${workflowStepId}`;
}

export function getConsent(workflowStepId: string): ConsentState | null {
	if (typeof window === 'undefined') return null;
	try {
		const v = window.localStorage.getItem(key(workflowStepId));
		return v === 'shared' || v === 'private' ? v : null;
	} catch {
		return null;
	}
}

export function setConsent(workflowStepId: string, value: ConsentState): void {
	if (typeof window === 'undefined') return;
	try {
		window.localStorage.setItem(key(workflowStepId), value);
	} catch {
		// localStorage unavailable (private mode, quota) — demo degrades to
		// in-session-only; toggle still works for the current page lifetime.
	}
}
