import { describe, expect, it } from 'vitest';
import { mediaTimedOutDuringSwitch } from './roomSwitch';

describe('mediaTimedOutDuringSwitch', () => {
	it('ignores the first join, which has no switch before it', () => {
		expect(mediaTimedOutDuringSwitch(null, 1_000)).toBe(false);
	});

	it('is false for a normal room switch', () => {
		expect(mediaTimedOutDuringSwitch(0, 3_000)).toBe(false);
	});

	it('is true when the join waited out the media timeout', () => {
		expect(mediaTimedOutDuringSwitch(0, 60_500)).toBe(true);
	});
});
