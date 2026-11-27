import {
	CalendarDate,
	CalendarDateTime,
	Time,
	parseAbsolute,
	toCalendarDate,
	toZoned
} from '@internationalized/date';

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

export type DateTimeParts = { date: CalendarDate; time: Time };

export type DateTimeBounds = { after?: string | null; before?: string | null };

export function isoToDateTimeParts(iso: string, timeZone: string): DateTimeParts {
	const zoned = parseAbsolute(iso, timeZone);
	return { date: toCalendarDate(zoned), time: new Time(zoned.hour, zoned.minute) };
}

export function dateTimePartsToIso({ date, time }: DateTimeParts, timeZone: string): string {
	const local = new CalendarDateTime(date.year, date.month, date.day, time.hour, time.minute);
	return toZoned(local, timeZone).toAbsoluteString();
}

/** Long date with a 24-hour time, e.g. "1 July 2026 at 09:30". */
export function formatDateTime(iso: string, timeZone: string): string {
	return new Intl.DateTimeFormat('en-GB', {
		dateStyle: 'long',
		timeStyle: 'short',
		hourCycle: 'h23',
		timeZone
	}).format(new Date(iso));
}

export function dateTimeBoundsError(
	iso: string,
	{ after, before }: DateTimeBounds,
	timeZone: string
): string | null {
	const time = Date.parse(iso);
	if (after && time <= Date.parse(after)) {
		return `Must be after ${formatDateTime(after, timeZone)}`;
	}
	if (before && time >= Date.parse(before)) {
		return `Must be before ${formatDateTime(before, timeZone)}`;
	}
	return null;
}
