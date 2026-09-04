import { describe, it, expect } from 'vitest';
import { BREAKOUT_WRAP_UP_MS, isBreakoutWrappingUp, wrapUpEndTime } from './breakoutWrapUp';

describe('isBreakoutWrappingUp', () => {
	it('is false while more than a minute is left', () => {
		expect(isBreakoutWrappingUp(BREAKOUT_WRAP_UP_MS + 1)).toBe(false);
	});

	it('is true from the last minute onwards, including once the timer has run out', () => {
		expect(isBreakoutWrappingUp(BREAKOUT_WRAP_UP_MS)).toBe(true);
		expect(isBreakoutWrappingUp(1000)).toBe(true);
		expect(isBreakoutWrappingUp(0)).toBe(true);
	});

	it('is false when there is no session running', () => {
		expect(isBreakoutWrappingUp(null)).toBe(false);
	});
});

describe('wrapUpEndTime', () => {
	const now = new Date('2026-08-27T10:00:00.000Z').getTime();

	it('cuts a long session down to the wrap-up minute', () => {
		const end = wrapUpEndTime(new Date(now + 10 * 60 * 1000), now);
		expect(end.toISOString()).toBe('2026-08-27T10:01:00.000Z');
	});

	it('leaves an end time that is already sooner alone', () => {
		const end = wrapUpEndTime(new Date(now + 20_000), now);
		expect(end.toISOString()).toBe('2026-08-27T10:00:20.000Z');
	});
});
