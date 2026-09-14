import { describe, it, expect } from 'vitest';
import { liveVoteBars, presentByGroup, reportVoteBars } from './liveVotes';
import type { DisplayState, MapNode, Vote } from './types';
import type { ReportGroup } from '$lib/tools/polis/reportTypes';

const nodes: MapNode[] = [
	{ id: 0, x: 0, y: 0, groupId: 0, memberCount: 1 },
	{ id: 1, x: 0, y: 0, groupId: 0, memberCount: 1 },
	{ id: 2, x: 0, y: 0, groupId: 1, memberCount: 1 },
	{ id: 3, x: 0, y: 0, groupId: 1, memberCount: 1 }
];

const groups: ReportGroup[] = [
	{ group_id: 0, members: [0, 1], total_members: 2, representative_comments: [] },
	{ group_id: 1, members: [2, 3], total_members: 2, representative_comments: [] }
];

function stateWith(present: MapNode[], votes: [number, Vote][]): DisplayState {
	return {
		atMs: 0,
		nodes: present,
		published: [],
		votesByTid: new Map([[7, new Map(votes)]]),
		totalVotes: votes.length
	};
}

describe('liveVoteBars', () => {
	it('shares over members present, so not-voted is the honest remainder', () => {
		const state = stateWith(nodes, [
			[0, 'agree'],
			[2, 'disagree']
		]);
		const bars = liveVoteBars(state, groups, 7);
		expect(bars.overall).toEqual({
			label: 'Overall',
			agreed: 25,
			disagreed: 25,
			passed: 0,
			notVoted: 50
		});
		expect(bars.groups[0]).toMatchObject({ label: 'Group A', agreed: 50, notVoted: 50 });
		expect(bars.groups[1]).toMatchObject({ label: 'Group B', disagreed: 50, notVoted: 50 });
	});

	it('ignores members who have not joined yet', () => {
		const state = stateWith(nodes.slice(0, 2), [[0, 'agree']]);
		const bars = liveVoteBars(state, groups, 7);
		expect(bars.overall.agreed).toBe(50);
		expect(bars.groups[1]).toMatchObject({ agreed: 0, disagreed: 0, passed: 0, notVoted: 0 });
	});

	it('is all zeros for a statement nobody has voted on', () => {
		const bars = liveVoteBars(stateWith(nodes, []), groups, 99);
		expect(bars.overall).toMatchObject({ agreed: 0, disagreed: 0, passed: 0, notVoted: 100 });
	});
});

describe('presentByGroup', () => {
	it('counts joined members per group and reports zero for empty groups', () => {
		const counts = presentByGroup(stateWith(nodes.slice(0, 3), []), groups);
		expect(counts.get(0)).toBe(2);
		expect(counts.get(1)).toBe(1);
	});

	it('lists every group even before anyone has joined', () => {
		const counts = presentByGroup(stateWith([], []), groups);
		expect([...counts.entries()]).toEqual([
			[0, 0],
			[1, 0]
		]);
	});
});

describe('reportVoteBars', () => {
	it('reads the report counts with the same labels the live bars use', () => {
		const comment = {
			tid: 1,
			text: 'x',
			is_seed: false,
			overall_votes: { agrees: 2, disagrees: 1, passes: 0 },
			group_votes: [
				{ group_id: 0, agrees: 2, disagrees: 0, passes: 0 },
				{ group_id: 1, agrees: 0, disagrees: 1, passes: 0 }
			],
			divisiveness: null,
			group_informed_consensus: null
		};
		const bars = reportVoteBars(comment, groups);
		expect(bars.overall).toMatchObject({
			label: 'Overall',
			agreed: 50,
			disagreed: 25,
			notVoted: 25
		});
		expect(bars.groups.map((b) => b.label)).toEqual(['Group A', 'Group B']);
		expect(bars.groups[0]).toMatchObject({ agreed: 100 });
	});
});
