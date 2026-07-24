import { describe, it, expect } from 'vitest';
import { computeMemberVoteBars } from './report';
import type { ReportComment, ReportGroup } from './reportTypes';

function group(group_id: number, total_members: number): ReportGroup {
	return { group_id, total_members, members: [], representative_comments: [] };
}

function comment(
	overall: [number, number, number],
	groupVotes: Array<{ group_id: number; a: number; d: number; p: number }>
): ReportComment {
	return {
		tid: 1,
		text: 'stmt',
		is_seed: false,
		overall_votes: { agrees: overall[0], disagrees: overall[1], passes: overall[2] },
		group_votes: groupVotes.map((g) => ({
			group_id: g.group_id,
			agrees: g.a,
			disagrees: g.d,
			passes: g.p
		}))
	};
}

describe('computeMemberVoteBars', () => {
	it('takes shares over membership, leaving a not-voted remainder', () => {
		const groups = [group(0, 10), group(1, 10)];
		const c = comment(
			[10, 4, 2], // 16 of 20 voted overall -> 20% not voted
			[
				{ group_id: 0, a: 8, d: 1, p: 1 }, // 10 of 10 -> 0% not voted
				{ group_id: 1, a: 2, d: 3, p: 1 } // 6 of 10 -> 40% not voted
			]
		);

		const { overall, groups: bars } = computeMemberVoteBars(c, groups);

		expect(overall).toMatchObject({
			label: 'OVERALL',
			agreed: 50,
			disagreed: 20,
			passed: 10,
			notVoted: 20
		});
		expect(bars.map((b) => b.label)).toEqual(['GROUP A', 'GROUP B']);
		expect(bars[0]).toMatchObject({ agreed: 80, disagreed: 10, passed: 10, notVoted: 0 });
		expect(bars[1]).toMatchObject({ agreed: 20, disagreed: 30, passed: 10, notVoted: 40 });
	});

	it('is N-group aware, not a fixed pair', () => {
		const oneGroup = computeMemberVoteBars(
			comment([5, 0, 0], [{ group_id: 0, a: 5, d: 0, p: 0 }]),
			[group(0, 10)]
		);
		expect(oneGroup.groups.map((b) => b.label)).toEqual(['GROUP A']);

		const threeGroups = computeMemberVoteBars(comment([3, 0, 0], []), [
			group(0, 5),
			group(1, 5),
			group(2, 5)
		]);
		expect(threeGroups.groups.map((b) => b.label)).toEqual(['GROUP A', 'GROUP B', 'GROUP C']);
	});

	it('renders a group with no votes on the statement as all not-voted', () => {
		const { groups: bars } = computeMemberVoteBars(comment([0, 0, 0], []), [group(0, 8)]);
		expect(bars[0]).toMatchObject({ agreed: 0, disagreed: 0, passed: 0, notVoted: 100 });
	});

	it('guards against zero members (no NaN)', () => {
		const { overall } = computeMemberVoteBars(comment([0, 0, 0], []), [group(0, 0)]);
		expect(overall).toMatchObject({ agreed: 0, disagreed: 0, passed: 0, notVoted: 0 });
	});
});
