/** Format a millisecond countdown as `m:ss`. Anything at or below zero reads `0:00`. */
export function formatCountdown(msRemaining: number | null): string {
	if (msRemaining === null || msRemaining <= 0) return '0:00';
	const totalSeconds = Math.floor(msRemaining / 1000);
	const minutes = Math.floor(totalSeconds / 60);
	const seconds = totalSeconds % 60;
	return `${minutes}:${seconds.toString().padStart(2, '0')}`;
}

const ONE_HOUR_MS = 60 * 60 * 1000;

/** Like `formatCountdown`, but reads `h:mm:ss` from an hour up, for whole-meeting times. */
export function formatLongCountdown(msRemaining: number | null): string {
	if (msRemaining === null || msRemaining < ONE_HOUR_MS) return formatCountdown(msRemaining);
	const totalSeconds = Math.floor(msRemaining / 1000);
	const hours = Math.floor(totalSeconds / 3600);
	const minutes = Math.floor((totalSeconds % 3600) / 60);
	const seconds = totalSeconds % 60;
	return `${hours}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
}
