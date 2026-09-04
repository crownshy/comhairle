/**
 * Builds a scripted scenario for the Room display prototype.
 *
 * Vote *shapes* are borrowed from a real Polis export rather than invented: the
 * proportion agreeing, the per-group split, and Polis's own `divisiveness` and
 * `group_informed_consensus` scores all come from statements that real people really
 * voted on. What is synthesised is the arrival order and the assignment of votes to
 * individual nodes, neither of which the report payload carries.
 *
 * Deterministic given a seed, so a rehearsed run replays exactly. That matters more
 * than it sounds: the point of the driver is that you can practise the same five
 * minutes until it lands.
 */

import type { ReportComment, ReportGroup } from '$lib/tools/polis/reportTypes';
import { orderEvents } from './scenario';
import type { MapNode, Scenario, ScenarioEvent, Vote } from './types';

export interface BuildScenarioOptions {
	/** Statement text, in order. Usually PLACEHOLDER_STATEMENTS. */
	statements: readonly string[];
	/** Real comments whose vote shapes and scores are borrowed, index for index. */
	sourceComments: ReportComment[];
	/** People in the room. 28 for the roundtable. */
	participantCount: number;
	durationMs: number;
	seed: number;
	/** Opinion groups to split the room into. */
	groupCount?: number;
}

/**
 * Mulberry32. A seeded PRNG is the whole point here, so `Math.random` is not an
 * option: an unrepeatable demo cannot be rehearsed.
 */
function makeRandom(seed: number): () => number {
	let a = seed >>> 0;
	return () => {
		a = (a + 0x6d2b79f5) >>> 0;
		let t = Math.imul(a ^ (a >>> 15), 1 | a);
		t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
		return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
	};
}

/** Draws a vote from a group's observed agree/disagree/pass split. */
function drawVote(random: () => number, agrees: number, disagrees: number, passes: number): Vote {
	const total = agrees + disagrees + passes;
	if (total <= 0) return 'pass';
	const roll = random() * total;
	if (roll < agrees) return 'agree';
	if (roll < agrees + disagrees) return 'disagree';
	return 'pass';
}

export function buildScenario(options: BuildScenarioOptions): Scenario {
	const { statements, sourceComments, participantCount, durationMs, seed } = options;
	const groupCount = options.groupCount ?? 2;
	const random = makeRandom(seed);

	// Nodes are base clusters holding one member each, which is what Polis produces
	// below its 100-cluster cap and therefore what a 28-person room looks like.
	const nodes: MapNode[] = Array.from({ length: participantCount }, (_, i) => {
		const groupId = i % groupCount;
		// Group centres spread along x, with jitter, so the map has visible clusters
		// without pretending to be a real PCA projection.
		const centreX = groupCount === 1 ? 0 : -1 + (2 * groupId) / (groupCount - 1);
		return {
			id: i,
			x: centreX + (random() - 0.5) * 0.7,
			y: (random() - 0.5) * 1.4,
			groupId,
			memberCount: 1
		};
	});

	const comments: ReportComment[] = statements.map((text, i) => {
		const source = sourceComments[i % Math.max(1, sourceComments.length)];
		return {
			...source,
			tid: i,
			text,
			is_seed: i < 6
		};
	});

	const groups: ReportGroup[] = Array.from({ length: groupCount }, (_, groupId) => {
		const members = nodes.filter((n) => n.groupId === groupId).map((n) => n.id);
		return {
			group_id: groupId,
			members,
			total_members: members.length,
			representative_comments: comments
				.slice(groupId * 2, groupId * 2 + 3)
				.map((c) => ({ tid: c.tid, text: c.text }))
		};
	});

	const events: ScenarioEvent[] = [];

	// Joins cluster in the opening stretch, the way a room scanning a QR code does,
	// with a thin tail for latecomers.
	const joinWindow = durationMs * 0.18;
	const joinTimes = nodes.map((node, i) => {
		const early = i < participantCount * 0.8;
		const at = early ? random() * joinWindow : joinWindow + random() * (durationMs * 0.5);
		events.push({ at, kind: 'participantJoined', nodeId: node.id });
		return at;
	});

	// Seeds are published at the start; participant statements arrive across the run.
	const publishTimes = comments.map((comment, i) => {
		const at = comment.is_seed
			? 0
			: (i / comments.length) * durationMs * 0.75 + random() * 20_000;
		events.push({ at, kind: 'statementPublished', tid: comment.tid });
		return at;
	});

	for (const comment of comments) {
		const publishedAt = publishTimes[comment.tid];
		for (const node of nodes) {
			const joinedAt = joinTimes[node.id];
			// A node can only vote on a statement that exists, and only after it has
			// arrived. Anything else would let the fold produce votes from nobody.
			const earliest = Math.max(publishedAt, joinedAt);
			if (earliest >= durationMs) continue;
			// Not everyone votes on everything, which is what makes `notVoted` a real
			// state on the map rather than a theoretical one.
			if (random() < 0.22) continue;

			const groupVote = comment.group_votes.find((g) => g.group_id === node.groupId);
			const vote = groupVote
				? drawVote(random, groupVote.agrees, groupVote.disagrees, groupVote.passes)
				: drawVote(
						random,
						comment.overall_votes.agrees,
						comment.overall_votes.disagrees,
						comment.overall_votes.passes
					);

			events.push({
				at: earliest + random() * (durationMs - earliest),
				kind: 'voteCast',
				tid: comment.tid,
				nodeId: node.id,
				vote
			});
		}
	}

	return { durationMs, nodes, comments, groups, events: orderEvents(events) };
}
