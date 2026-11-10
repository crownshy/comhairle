import { describe, expect, it } from 'vitest';
import { bucketAnswers } from './embedData';

describe('bucketAnswers', () => {
	it('places one answer per count at the middle of its bucket', () => {
		expect(
			bucketAnswers({
				min: 0,
				max: 10,
				minLabel: 'Not at all',
				maxLabel: 'Completely',
				buckets: [
					{ from: 0, count: 2 },
					{ from: 5, count: 0 },
					{ from: 5, count: 1 }
				].slice(0, 2)
			})
		).toEqual([2.5, 2.5]);
	});

	it('is empty for a scale nobody answered', () => {
		expect(
			bucketAnswers({ min: 1, max: 5, minLabel: 'Low', maxLabel: 'High', buckets: [] })
		).toEqual([]);
	});
});
