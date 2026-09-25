import { describe, it, expect } from 'vitest';
import { max } from './maths';

describe('maths', () => {
	it('gets the maximum number with numbers', () => {
		expect(max([2, 3, 1, 6, 4, 9, 10, 5], (v) => v)).toBe(10);
	});

	it('gets the maximum number with objects', () => {
		expect(
			max(
				[
					{ id: 1, total: 2 },
					{ id: 2, total: 3 },
					{ id: 3, total: 1 },
					{ id: 4, total: 6 },
					{ id: 5, total: 4 },
					{ id: 6, total: 9 },
					{ id: 7, total: 10 },
					{ id: 8, total: 5 }
				],
				(v) => v.total
			)
		).toBe(10);
	});
});
