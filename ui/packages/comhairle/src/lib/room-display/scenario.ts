/**
 * Folds a scripted scenario into the state the Room display renders.
 *
 * `stateAt` is pure and fully determined by `(scenario, atMs)`, which is what makes
 * scrubbing work: jumping backwards is the same operation as playing forwards, so the
 * facilitator can rewind to "twenty minutes in" mid-pitch and get exactly the frame
 * they had. At roundtable scale a run is a few hundred events, so folding from scratch
 * on every frame is cheaper than maintaining an incremental cursor and its
 * invalidation rules.
 *
 * Events are assumed sorted by `at`; `buildScenario` sorts once at construction so the
 * fold can stop at the first event past `atMs`.
 */

import { computeStage, ratchet, DEFAULT_THRESHOLDS, type RevealThresholds } from './revealStage';
import type {
	DisplayState,
	MapNode,
	RevealStage,
	Scenario,
	ScenarioEvent,
	Vote,
	VoteDistribution
} from './types';

/** Sorts events by time so `stateAt` can early-exit. Stable for equal timestamps. */
export function orderEvents(events: ScenarioEvent[]): ScenarioEvent[] {
	return [...events].sort((a, b) => a.at - b.at);
}

/** Folds the scenario up to and including `atMs`. */
export function stateAt(scenario: Scenario, atMs: number): DisplayState {
	const joined: MapNode[] = [];
	const seenNodes = new Set<number>();
	const publishedTids: number[] = [];
	const votesByTid = new Map<number, Map<number, Vote>>();
	let totalVotes = 0;

	for (const event of scenario.events) {
		if (event.at > atMs) break;

		switch (event.kind) {
			case 'participantJoined': {
				if (seenNodes.has(event.nodeId)) break;
				const node = scenario.nodes.find((n) => n.id === event.nodeId);
				if (!node) break;
				seenNodes.add(event.nodeId);
				joined.push(node);
				break;
			}
			case 'statementPublished': {
				if (!publishedTids.includes(event.tid)) publishedTids.push(event.tid);
				break;
			}
			case 'voteCast': {
				let byNode = votesByTid.get(event.tid);
				if (!byNode) {
					byNode = new Map();
					votesByTid.set(event.tid, byNode);
				}
				// A node revoting the same statement replaces rather than double-counts.
				if (!byNode.has(event.nodeId)) totalVotes += 1;
				byNode.set(event.nodeId, event.vote);
				break;
			}
		}
	}

	const byTid = new Map(scenario.comments.map((c) => [c.tid, c]));
	// Most recently published first, which is the order the statement ticker wants.
	const published = publishedTids
		.map((tid) => byTid.get(tid))
		.filter((c) => c !== undefined)
		.reverse();

	return { atMs, nodes: joined, published, votesByTid, totalVotes };
}

/**
 * How one map node split on one statement, for the cross-highlight.
 *
 * At roundtable scale every node holds one member, so exactly one count is 1 and the
 * dot renders as a single person's vote. Above Polis's 100-cluster cap a node holds
 * several and the same shape describes a split, which is why this returns counts
 * rather than a single `Vote`.
 */
export function nodeVoteDistribution(
	state: DisplayState,
	node: MapNode,
	tid: number
): VoteDistribution {
	const byNode = state.votesByTid.get(tid);
	const vote = byNode?.get(node.id);

	// The scenario records one vote per node, so a multi-member node is represented by
	// its single recorded answer standing for all its members. Real data will carry a
	// per-member breakdown here instead; the shape does not change.
	const agrees = vote === 'agree' ? node.memberCount : 0;
	const disagrees = vote === 'disagree' ? node.memberCount : 0;
	const passes = vote === 'pass' ? node.memberCount : 0;
	const notVoted = node.memberCount - agrees - disagrees - passes;

	return { agrees, disagrees, passes, notVoted };
}

/** Total participants represented by the joined nodes, not the node count. */
export function participantCount(state: DisplayState): number {
	return state.nodes.reduce((sum, n) => sum + n.memberCount, 0);
}

/**
 * The reveal stage at each sample point across a whole scenario, ratcheted forward.
 *
 * Precomputed once per scenario rather than folded per frame for two reasons. It is
 * far cheaper: a 4.5 hour run sampled every 30 seconds is 540 folds, which is fine
 * once and ruinous at 60fps. And it makes scrubbing correct: the stage at a playhead
 * is a property of the timeline up to that point, not of what the facilitator has
 * happened to watch, so rewinding to minute two shows the minute-two screen rather
 * than keeping an unlock the room has not seen yet.
 */
export function stageTimeline(
	scenario: Scenario,
	sampleMs = 30_000,
	thresholds: RevealThresholds = DEFAULT_THRESHOLDS
): { atMs: number; stage: RevealStage }[] {
	const samples: { atMs: number; stage: RevealStage }[] = [];
	let reached: RevealStage = 'empty';
	for (let at = 0; at <= scenario.durationMs; at += sampleMs) {
		reached = ratchet(reached, computeStage(stateAt(scenario, at), thresholds));
		samples.push({ atMs: at, stage: reached });
	}
	return samples;
}

/** The stage at `atMs`, from a timeline built by `stageTimeline`. */
export function stageAt(
	timeline: { atMs: number; stage: RevealStage }[],
	atMs: number
): RevealStage {
	let stage: RevealStage = 'empty';
	for (const sample of timeline) {
		if (sample.atMs > atMs) break;
		stage = sample.stage;
	}
	return stage;
}
