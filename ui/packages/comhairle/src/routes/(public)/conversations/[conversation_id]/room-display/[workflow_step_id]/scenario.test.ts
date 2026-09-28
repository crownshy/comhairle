import { describe, it, expect } from 'vitest';
import {
	orderEvents,
	stateAt,
	nodeVoteDistribution,
	participantCount,
	stageTimeline,
	stageAt
} from './scenario';
import { buildScenario } from './buildScenario';
import { STAGE_ORDER } from './revealStage';
import type { MapNode, Scenario, ScenarioEvent } from './types';
import type { ReportComment } from '$lib/tools/polis/reportTypes';

function node(id: number, memberCount = 1): MapNode {
	return { id, x: id, y: id, groupId: 0, memberCount };
}

function comment(tid: number, divisiveness: number | null = 1): ReportComment {
	return {
		tid,
		text: `stmt ${tid}`,
		is_seed: false,
		overall_votes: { agrees: 0, disagrees: 0, passes: 0 },
		group_votes: [],
		divisiveness
	};
}

function scenario(events: ScenarioEvent[], nodes: MapNode[] = [node(0), node(1)]): Scenario {
	return {
		durationMs: 10_000,
		nodes,
		comments: [comment(0), comment(1)],
		groups: [],
		events: orderEvents(events)
	};
}

describe('stateAt', () => {
	it('includes events at exactly atMs and excludes later ones', () => {
		const s = scenario([
			{ at: 100, kind: 'participantJoined', nodeId: 0 },
			{ at: 200, kind: 'participantJoined', nodeId: 1 }
		]);
		expect(stateAt(s, 100).nodes.map((n) => n.id)).toEqual([0]);
		expect(stateAt(s, 200).nodes.map((n) => n.id)).toEqual([0, 1]);
		expect(stateAt(s, 199).nodes.map((n) => n.id)).toEqual([0]);
	});

	it('is pure, so scrubbing back gives the same frame as playing forward', () => {
		const s = scenario([
			{ at: 100, kind: 'participantJoined', nodeId: 0 },
			{ at: 150, kind: 'statementPublished', tid: 0 },
			{ at: 200, kind: 'voteCast', tid: 0, nodeId: 0, vote: 'agree' },
			{ at: 300, kind: 'participantJoined', nodeId: 1 }
		]);
		const played = stateAt(s, 250);
		stateAt(s, 400);
		expect(stateAt(s, 250)).toEqual(played);
		expect(stateAt(s, 400).nodes).toHaveLength(2);
	});

	it('orders published statements most recent first, for the ticker', () => {
		const s = scenario([
			{ at: 10, kind: 'statementPublished', tid: 0 },
			{ at: 20, kind: 'statementPublished', tid: 1 }
		]);
		expect(stateAt(s, 100).published.map((c) => c.tid)).toEqual([1, 0]);
	});

	it('counts a node revoting a statement once, taking the later answer', () => {
		const s = scenario([
			{ at: 10, kind: 'voteCast', tid: 0, nodeId: 0, vote: 'agree' },
			{ at: 20, kind: 'voteCast', tid: 0, nodeId: 0, vote: 'disagree' }
		]);
		const state = stateAt(s, 100);
		expect(state.totalVotes).toBe(1);
		expect(state.votesByTid.get(0)?.get(0)).toBe('disagree');
	});

	it('ignores joins for nodes the scenario does not define', () => {
		const s = scenario([{ at: 10, kind: 'participantJoined', nodeId: 99 }]);
		expect(stateAt(s, 100).nodes).toEqual([]);
	});

	it('ignores a duplicate join for the same node', () => {
		const s = scenario([
			{ at: 10, kind: 'participantJoined', nodeId: 0 },
			{ at: 20, kind: 'participantJoined', nodeId: 0 }
		]);
		expect(stateAt(s, 100).nodes.map((n) => n.id)).toEqual([0]);
	});

	it('drops published tids with no matching comment', () => {
		const s = scenario([{ at: 10, kind: 'statementPublished', tid: 42 }]);
		expect(stateAt(s, 100).published).toEqual([]);
	});
});

describe('nodeVoteDistribution', () => {
	it('puts a single-member node entirely in one bucket', () => {
		const s = scenario([{ at: 10, kind: 'voteCast', tid: 0, nodeId: 0, vote: 'agree' }]);
		expect(nodeVoteDistribution(stateAt(s, 100), node(0), 0)).toEqual({
			agrees: 1,
			disagrees: 0,
			passes: 0,
			notVoted: 0
		});
	});

	it('reports an unvoted node as entirely not-voted', () => {
		expect(nodeVoteDistribution(stateAt(scenario([]), 100), node(0), 0)).toEqual({
			agrees: 0,
			disagrees: 0,
			passes: 0,
			notVoted: 1
		});
	});

	it('scales to a multi-member node, so the counts still sum to memberCount', () => {
		const s = scenario(
			[{ at: 10, kind: 'voteCast', tid: 0, nodeId: 0, vote: 'disagree' }],
			[node(0, 7)]
		);
		const dist = nodeVoteDistribution(stateAt(s, 100), node(0, 7), 0);
		expect(dist.disagrees).toBe(7);
		expect(dist.agrees + dist.disagrees + dist.passes + dist.notVoted).toBe(7);
	});
});

describe('participantCount', () => {
	it('sums member counts rather than counting dots', () => {
		const s = scenario(
			[
				{ at: 10, kind: 'participantJoined', nodeId: 0 },
				{ at: 20, kind: 'participantJoined', nodeId: 1 }
			],
			[node(0, 5), node(1, 3)]
		);
		expect(participantCount(stateAt(s, 100))).toBe(8);
	});
});

describe('stageTimeline and stageAt', () => {
	it('never goes backwards within a run', () => {
		const s = buildScenario({
			statements: Array.from({ length: 12 }, (_, i) => `s${i}`),
			sourceComments: Array.from({ length: 12 }, (_, i) => comment(i, 2)),
			participantCount: 28,
			durationMs: 600_000,
			seed: 7
		});
		const timeline = stageTimeline(s, 10_000);
		const ordinals = timeline.map((t) => STAGE_ORDER.indexOf(t.stage));
		expect(ordinals).toEqual([...ordinals].sort((a, b) => a - b));
	});

	it('gives the same stage for a playhead whether reached forwards or by scrubbing back', () => {
		const s = buildScenario({
			statements: Array.from({ length: 12 }, (_, i) => `s${i}`),
			sourceComments: Array.from({ length: 12 }, (_, i) => comment(i, 2)),
			participantCount: 28,
			durationMs: 600_000,
			seed: 7
		});
		const timeline = stageTimeline(s, 10_000);
		const early = stageAt(timeline, 60_000);
		stageAt(timeline, 590_000);
		expect(stageAt(timeline, 60_000)).toBe(early);
	});

	it('is empty at the very start', () => {
		const s = scenario([]);
		expect(stageAt(stageTimeline(s, 1000), 0)).toBe('empty');
	});
});
