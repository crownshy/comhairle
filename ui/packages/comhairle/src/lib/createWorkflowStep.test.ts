import { describe, it, expect } from 'vitest';
import { nextStepOrder } from './createWorkflowStep';

describe('nextStepOrder', () => {
	it('returns 1 for an empty workflow', () => {
		expect(nextStepOrder([])).toBe(1);
	});

	it('returns one past the current maximum', () => {
		expect(nextStepOrder([{ stepOrder: 1 }, { stepOrder: 2 }, { stepOrder: 3 }])).toBe(4);
	});

	it('uses the maximum, not the count, when orders are non-contiguous', () => {
		expect(nextStepOrder([{ stepOrder: 1 }, { stepOrder: 5 }])).toBe(6);
	});

	it('ignores ordering of the input array', () => {
		expect(nextStepOrder([{ stepOrder: 3 }, { stepOrder: 1 }, { stepOrder: 2 }])).toBe(4);
	});
});
