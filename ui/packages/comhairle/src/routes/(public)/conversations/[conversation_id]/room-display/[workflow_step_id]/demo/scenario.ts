/**
 * Replays a scripted scenario up to a point in time. Recomputed from scratch each
 * frame, which is cheap at a few hundred events and makes seeking backwards trivial.
 */

import { computeStage, ratchet, DEFAULT_THRESHOLDS, type RevealThresholds } from '../revealStage';
import type {
	DisplayState,
	MapNode,
	RevealStage,
	Scenario,
	ScenarioEvent,
	Vote,
	VoteDistribution
} from '../types';

/** Sorts events by time, which `stateAt` relies on. Stable for equal timestamps. */
export function orderEvents(events: ScenarioEvent[]): ScenarioEvent[] {
	return [...events].sort((a, b) => a.at - b.at);
}

/** The display state after every event up to and including `atMs`. */
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
				// A second vote on the same statement replaces the first.
				if (!byNode.has(event.nodeId)) totalVotes += 1;
				byNode.set(event.nodeId, event.vote);
				break;
			}
		}
	}

	const byTid = new Map(scenario.comments.map((c) => [c.tid, c]));
	// Newest first.
	const published = publishedTids
		.map((tid) => byTid.get(tid))
		.filter((c) => c !== undefined)
		.reverse();

	return { atMs, nodes: joined, published, votesByTid, totalVotes };
}

/**
 * How one map node voted on one statement. Returns counts rather than a single `Vote`
 * because above 100 participants Polis groups several people into one node.
 */
export function nodeVoteDistribution(
	state: DisplayState,
	node: MapNode,
	tid: number
): VoteDistribution {
	const byNode = state.votesByTid.get(tid);
	const vote = byNode?.get(node.id);

	// The scenario has one vote per node, so it stands for every member of the node.
	const agrees = vote === 'agree' ? node.memberCount : 0;
	const disagrees = vote === 'disagree' ? node.memberCount : 0;
	const passes = vote === 'pass' ? node.memberCount : 0;
	const notVoted = node.memberCount - agrees - disagrees - passes;

	return { agrees, disagrees, passes, notVoted };
}

/** Total members across joined nodes, which can exceed the node count. */
export function participantCount(state: DisplayState): number {
	return state.nodes.reduce((sum, n) => sum + n.memberCount, 0);
}

/**
 * The reveal stage sampled across the whole scenario, computed once rather than per
 * frame. Stages only move forward, so seeking backwards shows the earlier stage.
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
