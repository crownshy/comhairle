import { describe, it, expect } from 'vitest';
import { nextTiming, minutesSpent, type FlowTiming } from './flowTiming';

const MINUTE = 60_000;
const start = 1_700_000_000_000;

describe('nextTiming', () => {
	it('starts the clock when there is nothing stored', () => {
		expect(nextTiming(null, start)).toEqual({ startedAt: start, lastSeenAt: start });
	});

	it('keeps the start and moves the last seen stamp', () => {
		const stored: FlowTiming = { startedAt: start, lastSeenAt: start + MINUTE };
		expect(nextTiming(stored, start + 5 * MINUTE)).toEqual({
			startedAt: start,
			lastSeenAt: start + 5 * MINUTE
		});
	});

	it('starts again after a long gap', () => {
		const stored: FlowTiming = { startedAt: start, lastSeenAt: start + MINUTE };
		const later = start + 90 * MINUTE;
		expect(nextTiming(stored, later)).toEqual({ startedAt: later, lastSeenAt: later });
	});
});

describe('minutesSpent', () => {
	it('rounds to whole minutes', () => {
		const stored: FlowTiming = { startedAt: start, lastSeenAt: start + 7 * MINUTE };
		expect(minutesSpent(stored, start + 7 * MINUTE + 20_000)).toBe(7);
	});

	it('reports a minute for anything shorter than one', () => {
		const stored: FlowTiming = { startedAt: start, lastSeenAt: start + 5_000 };
		expect(minutesSpent(stored, start + 5_000)).toBe(1);
	});

	it('says nothing without stamps, after a stale sitting, or when the clock went back', () => {
		expect(minutesSpent(null, start)).toBeNull();
		expect(
			minutesSpent({ startedAt: start, lastSeenAt: start + MINUTE }, start + 90 * MINUTE)
		).toBeNull();
		expect(minutesSpent({ startedAt: start, lastSeenAt: start }, start - MINUTE)).toBeNull();
	});

	it('says nothing when the sitting is too long to be one sitting', () => {
		const stored: FlowTiming = { startedAt: start, lastSeenAt: start + 300 * MINUTE };
		expect(minutesSpent(stored, start + 300 * MINUTE)).toBeNull();
	});
});
