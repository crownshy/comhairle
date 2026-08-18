/** Format a millisecond countdown as `m:ss`. Anything at or below zero reads `0:00`. */
export function formatCountdown(msRemaining: number | null): string {
	if (msRemaining === null || msRemaining <= 0) return '0:00';
	const totalSecs = Math.floor(msRemaining / 1000);
	const min = Math.floor(totalSecs / 60);
	const sec = totalSecs % 60;
	return `${min}:${sec.toString().padStart(2, '0')}`;
}
