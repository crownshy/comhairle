import { describe, it, expect } from 'vitest';
import { isFirstRun } from './stepTour';

describe('isFirstRun', () => {
	it('is true while nothing has been finished', () => {
		expect(isFirstRun([{ progressStatus: 'in_progress' }, { progressStatus: null }])).toBe(
			true
		);
	});

	it('is false once any step is done', () => {
		expect(isFirstRun([{ progressStatus: 'done' }, { progressStatus: null }])).toBe(false);
	});

	it('is false when there are no steps to have finished', () => {
		expect(isFirstRun([])).toBe(false);
		expect(isFirstRun(undefined)).toBe(false);
	});
});
