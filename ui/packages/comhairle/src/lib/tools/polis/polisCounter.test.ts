import { describe, it, expect } from 'vitest';

import { opinionCounter } from './polisCounter';

describe('opinionCounter', () => {
	it('shows the first opinion when nothing has been voted on', () => {
		expect(opinionCounter(4, 4)).toEqual({ current: 1, total: 4 });
	});

	it('advances as the participant votes', () => {
		expect(opinionCounter(4, 3)).toEqual({ current: 2, total: 4 });
		expect(opinionCounter(4, 2)).toEqual({ current: 3, total: 4 });
		expect(opinionCounter(4, 1)).toEqual({ current: 4, total: 4 });
	});

	it('never exceeds the total once every statement is voted on (no "5 of 4")', () => {
		expect(opinionCounter(4, 0)).toEqual({ current: 4, total: 4 });
	});

	it('grows the denominator when new opinions are added to the pool', () => {
		// A 5th opinion was added; the participant has one left to vote on.
		expect(opinionCounter(5, 1)).toEqual({ current: 5, total: 5 });
		expect(opinionCounter(5, 0)).toEqual({ current: 5, total: 5 });
	});

	it('returns a zeroed counter for an empty pool', () => {
		expect(opinionCounter(0, 0)).toEqual({ current: 0, total: 0 });
	});

	it('clamps remaining that is larger than total', () => {
		expect(opinionCounter(4, 10)).toEqual({ current: 1, total: 4 });
	});

	it('defends against negative inputs', () => {
		expect(opinionCounter(4, -1)).toEqual({ current: 4, total: 4 });
		expect(opinionCounter(-3, 2)).toEqual({ current: 0, total: 0 });
	});

	it('handles a single-statement conversation', () => {
		expect(opinionCounter(1, 1)).toEqual({ current: 1, total: 1 });
		expect(opinionCounter(1, 0)).toEqual({ current: 1, total: 1 });
	});
});
