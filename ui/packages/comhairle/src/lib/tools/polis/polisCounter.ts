/**
 * Compute the "Opinion X of Y" counter shown to participants while voting.
 *
 * `total` is the number of statements in the conversation and `remaining` is how
 * many the participant still has to vote on. The returned `current` is 1-based
 * (the position being shown) and is clamped so it can never exceed `total` — i.e.
 * once every statement has been voted on we show "Opinion 4 of 4", never "5 of 4".
 *
 * Inputs are defended against out-of-range values (negative, or remaining larger
 * than total) so a jittery Polis response can't produce a nonsensical counter.
 */
export function opinionCounter(
	total: number,
	remaining: number
): { current: number; total: number } {
	const safeTotal = Math.max(0, Math.floor(total));
	if (safeTotal === 0) {
		return { current: 0, total: 0 };
	}

	const safeRemaining = Math.min(safeTotal, Math.max(0, Math.floor(remaining)));
	const voted = Math.min(safeTotal - safeRemaining, safeTotal - 1);

	return { current: voted + 1, total: safeTotal };
}
