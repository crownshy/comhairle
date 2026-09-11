/** Format a millisecond countdown as `m:ss`. Anything at or below zero reads `0:00`. */
export function formatCountdown(msRemaining: number | null): string {
	if (msRemaining === null || msRemaining <= 0) return '0:00';
	const totalSeconds = Math.floor(msRemaining / 1000);
	const minutes = Math.floor(totalSeconds / 60);
	const seconds = totalSeconds % 60;
	return `${minutes}:${seconds.toString().padStart(2, '0')}`;
}
