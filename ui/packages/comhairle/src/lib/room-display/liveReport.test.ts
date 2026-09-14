import { describe, it, expect } from 'vitest';
import { nodesFromReport, displayStateFromReport, stageFromReport } from './liveReport';
import type { PolisReportData, ReportComment } from '$lib/tools/polis/reportTypes';

function comment(tid: number, votes = 3, divisiveness: number | null = null): ReportComment {
	return {
		tid,
		text: `Statement ${tid}`,
		is_seed: false,
		overall_votes: { agrees: votes, disagrees: 0, passes: 0 },
		group_votes: [],
		divisiveness,
		group_informed_consensus: null
	};
}

function report(overrides: Partial<PolisReportData> = {}): PolisReportData {
	return {
		comments: [comment(0), comment(1)],
		groups: [],
		participants: [
			{ pid: 0, group_id: 0, pca_position: { x: 2, y: -1 } },
			{ pid: 1, group_id: 1, pca_position: { x: -4, y: 0.5 } },
			{ pid: 2, group_id: null, pca_position: null }
		],
		...overrides
	};
}

describe('nodesFromReport', () => {
	it('scales the cloud by its largest coordinate so it fills the map', () => {
		const nodes = nodesFromReport(report());
		expect(nodes.map((n) => n.id)).toEqual([0, 1]);
		expect(nodes[1].x).toBeCloseTo(-0.9);
		expect(nodes[0].x).toBeCloseTo(0.45);
		expect(nodes[0].y).toBeCloseTo(-0.225);
	});

	it('leaves unplaced participants off the map rather than on the origin', () => {
		expect(nodesFromReport(report()).some((n) => n.id === 2)).toBe(false);
	});

	it('carries the group id through, null when unclustered', () => {
		const nodes = nodesFromReport(
			report({ participants: [{ pid: 5, group_id: null, pca_position: { x: 0, y: 0 } }] })
		);
		expect(nodes[0]).toMatchObject({ id: 5, groupId: null, memberCount: 1 });
	});
});

describe('displayStateFromReport', () => {
	it('orders statements newest first by tid and sums the votes', () => {
		const state = displayStateFromReport(report());
		expect(state.published.map((c) => c.tid)).toEqual([1, 0]);
		expect(state.totalVotes).toBe(6);
		expect(state.votesByTid.size).toBe(0);
	});
});

describe('stageFromReport', () => {
	it('is empty with no votes at all', () => {
		expect(stageFromReport(report({ comments: [comment(0, 0)] }))).toBe('empty');
	});

	it('is warming until Polis reports at least two groups', () => {
		const one = report({
			groups: [{ group_id: 0, members: [0], total_members: 1, representative_comments: [] }]
		});
		expect(stageFromReport(one)).toBe('warming');
	});

	it('is shaped once two groups exist, rich once enough statements are scored', () => {
		const groups = [0, 1].map((g) => ({
			group_id: g,
			members: [g],
			total_members: 1,
			representative_comments: []
		}));
		expect(stageFromReport(report({ groups }))).toBe('shaped');
		const scored = Array.from({ length: 12 }, (_, i) => comment(i, 3, i * 0.3));
		expect(stageFromReport(report({ groups, comments: scored }))).toBe('rich');
	});
});
