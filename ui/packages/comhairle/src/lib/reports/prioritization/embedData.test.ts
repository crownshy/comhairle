import { describe, expect, it } from 'vitest';
import { bucketAnswers, rankingDomain } from './embedData';

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
					{ from: 5, count: 1 }
				]
			})
		).toEqual([2.5, 2.5, 7.5]);
	});

	it('is empty for a scale nobody answered', () => {
		expect(
			bucketAnswers({ min: 1, max: 5, minLabel: 'Low', maxLabel: 'High', buckets: [] })
		).toEqual([]);
	});
});

describe('rankingDomain', () => {
	const question = { text: 'How aligned is this?', min: -2, max: 2 };

	it('scales one answer range by the most reviews any proposal got', () => {
		expect(
			rankingDomain(question, [
				{ title: 'Buses', alignment: 4, reviews: 3 },
				{ title: 'Trains', alignment: null, reviews: 5 }
			])
		).toEqual([-10, 10]);
	});

	it('keeps one answer range when nobody has reviewed yet', () => {
		expect(rankingDomain(question, [])).toEqual([-2, 2]);
	});
});
