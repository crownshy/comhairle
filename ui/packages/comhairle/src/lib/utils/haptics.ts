/**
 * A short buzz for a tap that mattered. Web haptics are the Vibration API only, which
 * Android browsers expose and iOS Safari does not, so this is a bonus where it exists
 * and a no-op everywhere else. Browsers also refuse to vibrate before the first user
 * gesture on the page, which every caller here has already had.
 */
export type HapticKind = 'light' | 'medium' | 'success';

const PATTERNS: Record<HapticKind, number | number[]> = {
	// One tick for a page turn or a vote.
	light: 12,
	// A firmer tick for a commitment: Start, Proceed.
	medium: 24,
	// Two ticks for a step finished.
	success: [18, 70, 36]
};

/** Returns whether the device vibrated. Never throws. */
export function haptic(kind: HapticKind): boolean {
	if (typeof navigator === 'undefined' || typeof navigator.vibrate !== 'function') {
		return false;
	}
	try {
		return navigator.vibrate(PATTERNS[kind]);
	} catch {
		return false;
	}
}
