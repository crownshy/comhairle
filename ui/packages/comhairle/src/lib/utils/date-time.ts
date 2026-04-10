/**
 * Timestamps are stored in UTC in the database. Times from UTC timestamps should
 * be localised before using in HTML form inputs and other functionality to account
 * for timezones and daylight savings.
 *
 * Eg, in the UK from April `Europe/London` timezone operates under British Summer Time (BST),
 * which is UTC+1hour, and can create bugs if not accounted for.
 */
export function utcTimeToLocal(datetime: string, timeZone: string): string {
	const date = new Date(datetime);
	return date.toLocaleTimeString('en-GB', {
		timeZone,
		hour: '2-digit',
		minute: '2-digit',
		hour12: false
	});
}
