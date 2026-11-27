import { describe, it, expect } from 'vitest';
import { CalendarDate, Time } from '@internationalized/date';
import {
	dateTimeBoundsError,
	dateTimePartsToIso,
	formatDateTime,
	isoToDateTimeParts
} from './date-time';

const LONDON = 'Europe/London';

describe('isoToDateTimeParts', () => {
	it('reads a UTC timestamp as local date and time', () => {
		const parts = isoToDateTimeParts('2026-07-01T08:30:00Z', LONDON);
		expect(parts.date.toString()).toBe('2026-07-01');
		expect(parts.time.toString()).toBe('09:30:00');
	});

	it('moves to the next local day across midnight', () => {
		const parts = isoToDateTimeParts('2026-07-01T23:15:00Z', LONDON);
		expect(parts.date.toString()).toBe('2026-07-02');
		expect(parts.time.toString()).toBe('00:15:00');
	});
});

describe('dateTimePartsToIso', () => {
	it('converts a local summer time to UTC', () => {
		const iso = dateTimePartsToIso(
			{ date: new CalendarDate(2026, 7, 1), time: new Time(9, 30) },
			LONDON
		);
		expect(iso).toBe('2026-07-01T08:30:00.000Z');
	});
});

describe('formatDateTime', () => {
	it('uses a 24-hour clock', () => {
		expect(formatDateTime('2026-07-01T17:30:00Z', LONDON)).toBe('1 July 2026 at 18:30');
	});
});

describe('dateTimeBoundsError', () => {
	const launch = '2026-07-01T08:00:00Z';
	const close = '2026-08-01T08:00:00Z';

	it('accepts a value inside the bounds', () => {
		expect(
			dateTimeBoundsError('2026-07-15T08:00:00Z', { after: launch, before: close }, LONDON)
		).toBeNull();
	});

	it('accepts any value when the bounds are unset', () => {
		expect(dateTimeBoundsError(launch, { after: null, before: undefined }, LONDON)).toBeNull();
	});

	it('rejects a value equal to or before the lower bound', () => {
		expect(dateTimeBoundsError(launch, { after: launch }, LONDON)).toBe(
			'Must be after 1 July 2026 at 09:00'
		);
	});

	it('rejects a value equal to or after the upper bound', () => {
		expect(dateTimeBoundsError('2026-09-01T08:00:00Z', { before: close }, LONDON)).toBe(
			'Must be before 1 August 2026 at 09:00'
		);
	});
});
