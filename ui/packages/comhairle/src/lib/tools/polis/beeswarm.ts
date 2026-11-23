/**
 * Data helpers for the "Consensus continuum" beeswarm.
 *
 * The dot packing itself is done by d3-force (via layerchart's `ForceSimulation`)
 * in `ConsensusContinuum.svelte`; this module only massages the report comments
 * that feed it, kept pure so it can be unit-tested.
 *
 * The x axis is Polis `divisiveness`, which the backend fills straight from the
 * Polis PCA math's `comment-extremity` (see `polis_service.rs`): the magnitude of
 * a statement's position in the opinion space. Low = broadly-agreed (consensus),
 * high = strongly separates the opinion groups (divisive). A comment only has a
 * score once Polis has enough votes to place it, so unscored comments are dropped.
 */
/** A statement that may carry a divisiveness score. */
type Placeable = { tid: number; divisiveness?: number | null };

/** A statement Polis has scored, so it is safe to place on the continuum. */
export type ScoredComment<T extends Placeable = Placeable> = T & { divisiveness: number };

function hasScore<T extends Placeable>(c: T): c is ScoredComment<T> {
	return typeof c.divisiveness === 'number' && Number.isFinite(c.divisiveness);
}

/** Only the comments Polis gave a numeric divisiveness (extremity) score. */
export function scoredComments<T extends Placeable>(comments: T[]): ScoredComment<T>[] {
	return comments.filter(hasScore);
}

/**
 * The `tid` of the most divisive scored comment (highest extremity), or null if
 * none are scored. Used as the continuum's default focus so the vote breakdown
 * below the plot is always populated with a meaningful (divisive) statement.
 */
export function mostDivisiveTid(comments: Placeable[]): number | null {
	const scored = scoredComments(comments);
	if (scored.length === 0) return null;
	return scored.reduce((a, b) => (b.divisiveness > a.divisiveness ? b : a)).tid;
}
