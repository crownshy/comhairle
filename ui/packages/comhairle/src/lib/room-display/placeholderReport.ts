/**
 * Stand-in Polis report comments for the prototype, until a real export is wired in.
 *
 * Lives here rather than inline in the route so the tests exercise the same data the
 * screen does. The scores matter more than they look:
 *
 *  - `divisiveness` is the consensus continuum's x axis. It has to vary per statement.
 *    Repeating a couple of values stacks every dot on two columns and the swarm reads
 *    as broken, which is exactly how the first version of this looked. Real Polis
 *    extremity is a magnitude roughly in 0..5, spread unevenly with most statements
 *    nearer consensus, so the distribution is curved rather than evenly spaced.
 *  - The per-group vote split tracks `divisiveness`, because a statement scored as
 *    divisive that the groups happen to agree on would make the map contradict the
 *    plot sitting under it.
 */

import type { ReportComment } from '$lib/tools/polis/reportTypes';

/** Largest extremity the spread reaches, near the top of Polis's usual range. */
const MAX_DIVISIVENESS = 4.6;

/** Deterministic jitter in 0..1, so the swarm is uneven but identical every render. */
function jitter(i: number): number {
	const x = Math.sin(i * 12.9898) * 43758.5453;
	return Math.abs(x - Math.floor(x));
}

export function buildPlaceholderComments(statements: readonly string[]): ReportComment[] {
	return statements.map((text, i) => {
		const t = i / Math.max(1, statements.length - 1);
		const divisiveness = Math.max(
			0.05,
			0.2 + Math.pow(t, 1.6) * (MAX_DIVISIVENESS - 0.6) + jitter(i) * 0.35
		);

		const split = Math.min(1, divisiveness / MAX_DIVISIVENESS);
		const agreeA = Math.round(35 + split * 30);
		const agreeB = Math.round(35 - split * 28);

		return {
			tid: i,
			text,
			is_seed: false,
			overall_votes: {
				agrees: agreeA + agreeB,
				disagrees: 140 - agreeA - agreeB,
				passes: 22
			},
			group_votes: [
				{ group_id: 0, agrees: agreeA, disagrees: 70 - agreeA, passes: 11 },
				{ group_id: 1, agrees: agreeB, disagrees: 70 - agreeB, passes: 11 }
			],
			divisiveness,
			group_informed_consensus: Math.max(0.02, 0.7 - split * 0.65)
		};
	});
}
