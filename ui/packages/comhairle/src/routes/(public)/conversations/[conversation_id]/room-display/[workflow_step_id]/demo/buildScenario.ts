/**
 * Builds the scripted demo scenario. Vote splits and scores come from real report
 * comments; arrival times and which person cast which vote are generated from `seed`,
 * so the same seed always replays the same run.
 */

import type { ReportComment, ReportGroup } from '$lib/tools/polis/reportTypes';
import { orderEvents } from './scenario';
import type { MapNode, Scenario, ScenarioEvent, Vote } from '../types';

export interface BuildScenarioOptions {
	statements: readonly string[];
	/** Real comments whose vote splits and scores are copied, index for index. */
	sourceComments: ReportComment[];
	participantCount: number;
	durationMs: number;
	seed: number;
	groupCount?: number;
	/**
	 * Statements published at the start as moderator seeds. With too few, the display
	 * unlocks the consensus strip while it still has almost nothing to plot.
	 */
	seedCount?: number;
	/**
	 * Median delay between someone being able to vote on a statement and voting on it.
	 * Defaults to a share of `durationMs` so short scenarios are paced the same way.
	 */
	medianResponseMs?: number;
}

/** Seeded random number generator (Mulberry32), so a demo run can be replayed exactly. */
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
	const medianResponseMs = options.medianResponseMs ?? durationMs * 0.03;
	const seedCount = options.seedCount ?? Math.ceil(statements.length * 0.48);
	const random = makeRandom(seed);

	// One member per node matches what Polis produces for a room under 100 people.
	const nodes: MapNode[] = Array.from({ length: participantCount }, (_, i) => {
		const groupId = i % groupCount;
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
			is_seed: i < seedCount
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

	// Most people join early, as a room scanning a QR code does, with a few latecomers.
	const joinWindow = durationMs * 0.18;
	const joinTimes = nodes.map((node, i) => {
		const early = i < participantCount * 0.8;
		const at = early ? random() * joinWindow : joinWindow + random() * (durationMs * 0.5);
		events.push({ at, kind: 'participantJoined', nodeId: node.id });
		return at;
	});

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
			// A vote can only happen after both the statement and the person exist.
			const earliest = Math.max(publishedAt, joinedAt);
			if (earliest >= durationMs) continue;
			// Skip some votes so the map shows people who have not voted yet.
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

			// Exponential delay: most votes land soon, a few much later.
			const delay = -Math.log(1 - random()) * (medianResponseMs / Math.LN2);
			const at = earliest + delay;
			if (at >= durationMs) continue;

			events.push({ at, kind: 'voteCast', tid: comment.tid, nodeId: node.id, vote });
		}
	}

	return { durationMs, nodes, comments, groups, events: orderEvents(events) };
}
