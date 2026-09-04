import { describe, it, expect } from 'vitest';
import { buildScenario, type BuildScenarioOptions } from './buildScenario';
import { stateAt } from './scenario';
import { computeStage, ratchet } from './revealStage';
import type { ReportComment } from '$lib/tools/polis/reportTypes';
import type { RevealStage } from './types';

function source(tid: number): ReportComment {
	return {
		tid,
		text: `source ${tid}`,
		is_seed: false,
		overall_votes: { agrees: 40, disagrees: 30, passes: 10 },
		group_votes: [
			{ group_id: 0, agrees: 30, disagrees: 5, passes: 5 },
			{ group_id: 1, agrees: 10, disagrees: 25, passes: 5 }
		],
		divisiveness: 2.5,
		group_informed_consensus: 0.4
	};
}

const options: BuildScenarioOptions = {
	statements: Array.from({ length: 12 }, (_, i) => `statement ${i}`),
	sourceComments: Array.from({ length: 12 }, (_, i) => source(i)),
	participantCount: 28,
	durationMs: 600_000,
	seed: 42
};

describe('buildScenario', () => {
	it('is deterministic for a seed, so a run can be rehearsed', () => {
		expect(buildScenario(options)).toEqual(buildScenario(options));
	});

	it('produces a different run for a different seed', () => {
		const a = buildScenario(options);
		const b = buildScenario({ ...options, seed: 43 });
		expect(a.events).not.toEqual(b.events);
	});

	it('makes one single-member node per participant', () => {
		const s = buildScenario(options);
		expect(s.nodes).toHaveLength(28);
		expect(s.nodes.every((n) => n.memberCount === 1)).toBe(true);
	});

	it('splits the room into the requested number of groups', () => {
		const s = buildScenario({ ...options, groupCount: 3 });
		expect(s.groups).toHaveLength(3);
		expect(s.groups.reduce((sum, g) => sum + g.total_members, 0)).toBe(28);
	});

	it('keeps the borrowed scores from the source comments', () => {
		const s = buildScenario(options);
		expect(s.comments.every((c) => c.divisiveness === 2.5)).toBe(true);
	});

	it('uses the supplied statement text, not the source text', () => {
		const s = buildScenario(options);
		expect(s.comments[0].text).toBe('statement 0');
	});

	it('never votes before both the voter and the statement exist', () => {
		const s = buildScenario(options);
		const joinAt = new Map<number, number>();
		const publishAt = new Map<number, number>();
		for (const e of s.events) {
			if (e.kind === 'participantJoined' && !joinAt.has(e.nodeId)) joinAt.set(e.nodeId, e.at);
			if (e.kind === 'statementPublished' && !publishAt.has(e.tid))
				publishAt.set(e.tid, e.at);
		}
		for (const e of s.events) {
			if (e.kind !== 'voteCast') continue;
			expect(joinAt.get(e.nodeId)).toBeLessThanOrEqual(e.at);
			expect(publishAt.get(e.tid)).toBeLessThanOrEqual(e.at);
		}
	});

	it('leaves some statements unvoted by some nodes, so notVoted is real', () => {
		const s = buildScenario(options);
		const end = stateAt(s, s.durationMs);
		const everyoneVotedOnEverything = s.comments.every(
			(c) => (end.votesByTid.get(c.tid)?.size ?? 0) === s.nodes.length
		);
		expect(everyoneVotedOnEverything).toBe(false);
	});

	it('emits events in time order', () => {
		const s = buildScenario(options);
		const times = s.events.map((e) => e.at);
		expect(times).toEqual([...times].sort((a, b) => a - b));
	});

	it('walks the room from empty through to a clustered display', () => {
		const s = buildScenario(options);
		let reached: RevealStage = 'empty';
		const seen: RevealStage[] = [];
		for (let t = 0; t <= s.durationMs; t += s.durationMs / 60) {
			reached = ratchet(reached, computeStage(stateAt(s, t)));
			if (seen.at(-1) !== reached) seen.push(reached);
		}
		expect(seen[0]).toBe('empty');
		expect(reached === 'shaped' || reached === 'rich').toBe(true);
	});
});
