/**
 * The value a number that counts up to `target` should show after `elapsed` ms.
 *
 * Eased out, so it arrives the way a real counter settles rather than at a constant rate.
 * Pure, so the component around it is only a frame loop.
 */
export function countUpValue(target: number, elapsed: number, duration: number): number {
	if (duration <= 0 || elapsed >= duration) return target;
	if (elapsed <= 0) return 0;
	const eased = 1 - Math.pow(1 - elapsed / duration, 3);
	return Math.round(target * eased);
}
