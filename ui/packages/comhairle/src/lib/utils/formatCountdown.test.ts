import { describe, it, expect } from 'vitest';
import { formatCountdown } from './formatCountdown';

describe('formatCountdown', () => {
	it('formats whole minutes', () => {
		expect(formatCountdown(5 * 60 * 1000)).toBe('5:00');
	});

	it('zero-pads the seconds', () => {
		expect(formatCountdown(61_000)).toBe('1:01');
	});

	it('rounds down to the second', () => {
		expect(formatCountdown(1999)).toBe('0:01');
	});

	it('does not roll minutes over at 60', () => {
		expect(formatCountdown(90 * 60 * 1000)).toBe('90:00');
	});

	it('reads 0:00 once time is up', () => {
		expect(formatCountdown(0)).toBe('0:00');
		expect(formatCountdown(-1000)).toBe('0:00');
	});

	it('reads 0:00 when there is no session', () => {
		expect(formatCountdown(null)).toBe('0:00');
	});
});
