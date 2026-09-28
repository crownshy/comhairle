/**
 * Placeholder report comments for the demo. Each statement gets its own
 * `divisiveness` (the strip's x axis) so the dots spread out, and the group vote
 * split follows that score so the map agrees with the strip.
 */

import type { ReportComment } from '$lib/tools/polis/reportTypes';

/** Near the top of Polis's usual 0 to 5 range. */
const MAX_DIVISIVENESS = 4.6;

/** Deterministic jitter in 0..1, so every render is identical. */
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
