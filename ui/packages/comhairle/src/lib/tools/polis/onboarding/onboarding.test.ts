import { describe, it, expect } from 'vitest';
import {
	crowdBar,
	earlySnapshot,
	firstVotingStage,
	groupPortrait,
	isRevealDue,
	pickPractice,
	stageAfterSeedResult,
	toPracticeStatements,
	type PolisCommentRow,
	type PracticeStatement
} from './onboarding';
import type { PolisReportData, ReportComment } from '../reportTypes';

function comment(
	tid: number,
	is_seed: boolean,
	[agrees, disagrees, passes]: [number, number, number],
	groupVotes: Array<[groupId: number, agrees: number, disagrees: number]> = []
): ReportComment {
	return {
		tid,
		text: `statement ${tid}`,
		is_seed,
		overall_votes: { agrees, disagrees, passes },
		group_votes: groupVotes.map(([group_id, a, d]) => ({
			group_id,
			agrees: a,
			disagrees: d,
			passes: 0
		}))
	};
}

function report(overrides: Partial<PolisReportData> = {}): PolisReportData {
	return { comments: [], groups: [], participants: [], ...overrides };
}

function statement(
	tid: number,
	is_seed: boolean,
	[agrees, disagrees, passes]: [number, number, number]
): PracticeStatement {
	return { tid, text: `statement ${tid}`, is_seed, votes: { agrees, disagrees, passes } };
}

describe('toPracticeStatements', () => {
	const rows: PolisCommentRow[] = [
		{ tid: 1, txt: 'accepted', is_seed: true, mod: 1, agree_count: 2 },
		{ tid: 2, txt: 'unmoderated', is_seed: false, mod: 0 },
		{ tid: 3, txt: 'rejected', is_seed: false, mod: -1 },
		{ tid: 4, txt: 'meta', is_seed: false, mod: 1, is_meta: true },
		{ tid: 5, txt: 'no moderation field', is_seed: false }
	];

	it('keeps unmoderated statements unless the step moderates strictly', () => {
		expect(toPracticeStatements(rows, false).map((s) => s.tid)).toEqual([1, 2, 5]);
		expect(toPracticeStatements(rows, true).map((s) => s.tid)).toEqual([1, 5]);
	});

	it('carries the vote counts across', () => {
		expect(toPracticeStatements(rows, true)[0].votes).toEqual({
			agrees: 2,
			disagrees: 0,
			passes: 0
		});
	});
});

describe('pickPractice', () => {
	it('picks the most-voted seed and the most-voted participant statement', () => {
		const pick = pickPractice([
			statement(1, true, [1, 0, 0]),
			statement(2, true, [5, 2, 0]),
			statement(3, false, [0, 1, 0]),
			statement(4, false, [3, 3, 1])
		]);
		expect(pick.seed?.tid).toBe(2);
		expect(pick.participant?.tid).toBe(4);
	});

	it('still finds a participant statement nobody has voted on yet', () => {
		const pick = pickPractice([statement(1, true, [3, 0, 0]), statement(2, false, [0, 0, 0])]);
		expect(pick.participant?.tid).toBe(2);
	});

	it('leaves the participant statement out when there are only seeds', () => {
		const pick = pickPractice([statement(1, true, [0, 0, 0])]);
		expect(pick.seed?.tid).toBe(1);
		expect(pick.participant).toBeUndefined();
	});
});

describe('stages', () => {
	const seed = statement(1, true, [0, 0, 0]);
	const participant = statement(2, false, [0, 0, 0]);

	it('votes on the seed first, then the participant statement', () => {
		expect(firstVotingStage({ seed, participant })).toBe('seed');
		expect(stageAfterSeedResult({ seed, participant })).toBe('participant');
	});

	it('goes on to free voting when a statement is missing', () => {
		expect(stageAfterSeedResult({ seed, participant: undefined })).toBe('free');
		expect(firstVotingStage({ seed: undefined, participant })).toBe('participant');
		expect(firstVotingStage({ seed: undefined, participant: undefined })).toBe('free');
	});
});

describe('crowdBar', () => {
	it('adds the participant vote Polis has not counted yet', () => {
		const bar = crowdBar(statement(1, true, [1, 1, 1]), 'agree');
		expect(bar.voters).toBe(4);
		expect(bar.agreed).toBe(50);
		expect(bar.disagreed).toBe(25);
		expect(bar.passed).toBe(25);
	});

	it('is all zero with no voters', () => {
		expect(crowdBar(statement(1, true, [0, 0, 0]), undefined)).toEqual({
			agreed: 0,
			disagreed: 0,
			passed: 0,
			voters: 0
		});
	});
});

describe('groupPortrait', () => {
	const twoGroups = report({
		comments: [
			comment(
				1,
				true,
				[5, 5, 0],
				[
					[0, 5, 0],
					[1, 0, 5]
				]
			),
			comment(
				2,
				false,
				[5, 5, 0],
				[
					[0, 0, 5],
					[1, 5, 0]
				]
			)
		],
		groups: [
			{
				group_id: 0,
				members: [7],
				total_members: 1,
				representative_comments: [
					{ tid: 1, text: '' },
					{ tid: 2, text: '' }
				]
			},
			{ group_id: 1, members: [8], total_members: 1, representative_comments: [] }
		],
		participants: [
			{ pid: 7, group_id: 0 },
			{ pid: 8, group_id: 1 }
		]
	});

	it('splits the group key statements by which way the group leaned', () => {
		const portrait = groupPortrait(twoGroups, 7);
		expect(portrait?.agreed.map((c) => c.tid)).toEqual([1]);
		expect(portrait?.disagreed.map((c) => c.tid)).toEqual([2]);
	});

	it("lists every group with its size and marks the participant's own", () => {
		expect(groupPortrait(twoGroups, 8)?.groups).toEqual([
			{ groupId: 0, size: 1, isYours: false },
			{ groupId: 1, size: 1, isYours: true }
		]);
	});

	it('says nothing for someone not in a group yet', () => {
		expect(groupPortrait(twoGroups, 99)).toBeUndefined();
		expect(groupPortrait(twoGroups, undefined)).toBeUndefined();
	});

	it('says nothing while there is only one group', () => {
		expect(
			groupPortrait({ ...twoGroups, groups: twoGroups.groups.slice(0, 1) }, 7)
		).toBeUndefined();
	});
});

describe('earlySnapshot', () => {
	it('counts people and votes', () => {
		const snapshot = earlySnapshot(
			report({
				comments: [comment(1, true, [2, 2, 0]), comment(2, true, [3, 0, 1])],
				participants: [{ pid: 1 }, { pid: 2 }]
			})
		);
		expect(snapshot).toEqual({ people: 2, votes: 8 });
	});

	it('is empty without a report', () => {
		expect(earlySnapshot(undefined)).toEqual({ people: 0, votes: 0 });
	});
});

describe('isRevealDue', () => {
	it('fires on every fifteenth vote only', () => {
		expect(isRevealDue(0)).toBe(false);
		expect(isRevealDue(14)).toBe(false);
		expect(isRevealDue(15)).toBe(true);
		expect(isRevealDue(30)).toBe(true);
	});
});
