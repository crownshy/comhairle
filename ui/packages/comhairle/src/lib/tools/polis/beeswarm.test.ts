import { describe, it, expect } from 'vitest';
import { scoredComments, mostDivisiveTid } from './beeswarm';
import type { ReportComment } from './reportTypes';

function comment(tid: number, divisiveness: number | null | undefined): ReportComment {
	return {
		tid,
		text: `stmt ${tid}`,
		is_seed: false,
		overall_votes: { agrees: 1, disagrees: 1, passes: 0 },
		group_votes: [],
		divisiveness
	};
}

describe('scoredComments', () => {
	it('keeps only comments with a numeric divisiveness score', () => {
		const kept = scoredComments([
			comment(0, 2),
			comment(1, null),
			comment(2, undefined),
			comment(3, 0)
		]);
		expect(kept.map((c) => c.tid)).toEqual([0, 3]);
	});

	it('returns an empty array when nothing is scored', () => {
		expect(scoredComments([comment(0, null), comment(1, undefined)])).toEqual([]);
		expect(scoredComments([])).toEqual([]);
	});
});

describe('mostDivisiveTid', () => {
	it('picks the highest-divisiveness comment', () => {
		expect(mostDivisiveTid([comment(0, 1), comment(1, 5), comment(2, 3)])).toBe(1);
	});

	it('ignores unscored comments when choosing', () => {
		expect(mostDivisiveTid([comment(0, null), comment(1, 2), comment(2, undefined)])).toBe(1);
	});

	it('returns null when no comment is scored', () => {
		expect(mostDivisiveTid([comment(0, null)])).toBeNull();
		expect(mostDivisiveTid([])).toBeNull();
	});
});
