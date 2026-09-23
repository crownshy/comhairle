import { describe, it, expect } from 'vitest';
import { apportion, apportionVotes } from './apportionedVotes';
import type { PolisReportData, ReportComment, ReportGroup } from '$lib/tools/polis/reportTypes';
import type { Vote } from './types';

function comment(overrides: Partial<ReportComment> = {}): ReportComment {
	return {
		tid: 7,
		text: 'Statement 7',
		is_seed: false,
		overall_votes: { agrees: 0, disagrees: 0, passes: 0 },
		group_votes: [],
		divisiveness: null,
		group_informed_consensus: null,
		...overrides
	};
}

/** `size` placed participants in `groupId`, numbered from `firstPid`. */
function members(groupId: number | null, size: number, firstPid: number) {
	return Array.from({ length: size }, (_, i) => ({
		pid: firstPid + i,
		group_id: groupId,
		pca_position: { x: i, y: -i }
	}));
}

function group(groupId: number, size: number, firstPid: number): ReportGroup {
	return {
		group_id: groupId,
		members: Array.from({ length: size }, (_, i) => firstPid + i),
		total_members: size,
		representative_comments: []
	};
}

/** Two groups of 12, with the given per-group counts on tid 7. */
function report(
	groupVotes: { group_id: number; agrees: number; disagrees: number; passes: number }[],
	overrides: Partial<PolisReportData> = {}
): PolisReportData {
	const overall = groupVotes.reduce(
		(sum, g) => ({
			agrees: sum.agrees + g.agrees,
			disagrees: sum.disagrees + g.disagrees,
			passes: sum.passes + g.passes
		}),
		{ agrees: 0, disagrees: 0, passes: 0 }
	);
	return {
		comments: [comment({ group_votes: groupVotes, overall_votes: overall })],
		groups: [group(0, 12, 0), group(1, 12, 12)],
		participants: [...members(0, 12, 0), ...members(1, 12, 12)],
		...overrides
	};
}

/** Votes on tid 7 for the pids in `pids`, tallied by kind. */
function tally(matrix: Map<number, Map<number, Vote>>, pids: number[]) {
	const votes = matrix.get(7)!;
	const count = (vote: Vote) => pids.filter((pid) => votes.get(pid) === vote).length;
	return {
		agree: count('agree'),
		disagree: count('disagree'),
		pass: count('pass'),
		notVoted: pids.filter((pid) => votes.get(pid) === undefined).length
	};
}

const range = (from: number, to: number) => Array.from({ length: to - from }, (_, i) => from + i);

describe('apportion', () => {
	it('hands out exactly the weights when the seats match their total', () => {
		expect(apportion([9, 2, 1, 0], 12)).toEqual([9, 2, 1, 0]);
	});

	it('always fills every seat, so no dot is left without a colour', () => {
		expect(apportion([1, 1, 1, 0], 10).reduce((s, n) => s + n, 0)).toBe(10);
		expect(apportion([5, 3, 1, 4], 7).reduce((s, n) => s + n, 0)).toBe(7);
	});

	it('gives the odd seats to the largest remainders', () => {
		// Thirds of 10: 3.33 each, so the first two remainders (tied, lowest index
		// first) take the spare seats.
		expect(apportion([1, 1, 1], 10)).toEqual([4, 3, 3]);
	});

	it('hands out nothing when there is nothing to hand out', () => {
		expect(apportion([0, 0, 0, 0], 5)).toEqual([0, 0, 0, 0]);
		expect(apportion([3, 1, 0, 0], 0)).toEqual([0, 0, 0, 0]);
	});
});

describe('apportionVotes', () => {
	it("reproduces each group's counts exactly, dot for dot", () => {
		const matrix = apportionVotes(
			report([
				{ group_id: 0, agrees: 9, disagrees: 2, passes: 1 },
				{ group_id: 1, agrees: 1, disagrees: 8, passes: 0 }
			])
		);
		expect(tally(matrix, range(0, 12))).toEqual({
			agree: 9,
			disagree: 2,
			pass: 1,
			notVoted: 0
		});
		expect(tally(matrix, range(12, 24))).toEqual({
			agree: 1,
			disagree: 8,
			pass: 0,
			notVoted: 3
		});
	});

	it('repaints identically for an unchanged report, so polling does not reshuffle', () => {
		const data = report([
			{ group_id: 0, agrees: 5, disagrees: 4, passes: 2 },
			{ group_id: 1, agrees: 7, disagrees: 1, passes: 0 }
		]);
		expect([...apportionVotes(data).get(7)!]).toEqual([...apportionVotes(data).get(7)!]);
	});

	it('nudges the boundaries when a vote arrives, rather than scattering the group', () => {
		// One new agree shifts each colour boundary along by one dot, so the churn is
		// one dot per colour at most. Sixty members rather than twelve, to show it does
		// not grow with the group.
		const votes = (agrees: number) =>
			apportionVotes(
				report([{ group_id: 0, agrees, disagrees: 20, passes: 5 }], {
					groups: [group(0, 60, 0)],
					participants: members(0, 60, 0)
				})
			);
		const before = votes(20);
		const after = votes(21);
		const changed = range(0, 60).filter(
			(pid) => before.get(7)!.get(pid) !== after.get(7)!.get(pid)
		);
		expect(changed.length).toBeLessThanOrEqual(3);
	});

	it('deals a different statement to different dots, so no dot reads as a persistent voter', () => {
		const data = report([{ group_id: 0, agrees: 6, disagrees: 6, passes: 0 }]);
		const twoStatements = {
			...data,
			comments: [data.comments[0], { ...data.comments[0], tid: 8 }]
		};
		const matrix = apportionVotes(twoStatements);
		const agreedOn = (tid: number) =>
			range(0, 12).filter((pid) => matrix.get(tid)!.get(pid) === 'agree');
		expect(agreedOn(7)).not.toEqual(agreedOn(8));
	});

	it('gives the unclustered dots whatever the groups did not cast', () => {
		const data = report([{ group_id: 0, agrees: 3, disagrees: 0, passes: 0 }], {
			groups: [group(0, 12, 0)],
			participants: [...members(0, 12, 0), ...members(null, 4, 100)]
		});
		data.comments[0].overall_votes = { agrees: 5, disagrees: 2, passes: 0 };

		expect(tally(apportionVotes(data), range(100, 104))).toEqual({
			agree: 2,
			disagree: 2,
			pass: 0,
			notVoted: 0
		});
	});

	it('keeps the proportions when Polis has placed fewer dots than it counts members', () => {
		// Six of the group's twelve members have a PCA position, so six dots carry a
		// 6/3/3 split: halves, quarters, quarters.
		const data = report([{ group_id: 0, agrees: 6, disagrees: 3, passes: 3 }], {
			groups: [group(0, 12, 0)],
			participants: members(0, 6, 0)
		});
		expect(tally(apportionVotes(data), range(0, 6))).toEqual({
			agree: 3,
			disagree: 2,
			pass: 1,
			notVoted: 0
		});
	});

	it('deals the overall counts across every dot before Polis has clustered', () => {
		// The first minutes: no groups, no per-group counts, and eight dots to spread
		// six votes over.
		const data: PolisReportData = {
			comments: [comment({ overall_votes: { agrees: 4, disagrees: 1, passes: 1 } })],
			groups: [],
			participants: members(null, 8, 0)
		};
		expect(tally(apportionVotes(data), range(0, 8))).toEqual({
			agree: 4,
			disagree: 1,
			pass: 1,
			notVoted: 2
		});
	});

	it('leaves a statement nobody voted on entirely uncoloured', () => {
		const matrix = apportionVotes(
			report([{ group_id: 0, agrees: 0, disagrees: 0, passes: 0 }])
		);
		expect(matrix.get(7)!.size).toBe(0);
	});
});
