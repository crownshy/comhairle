/** The rule deciding when a participant may leave the prioritization step.
 *
 * Kept out of the component so it can be tested directly: it is the whole
 * contract between the admin's `required_reviews` config and the step's
 * top-nav "Next", and getting it wrong either strands participants or lets
 * them skip the step entirely.
 */

/**
 * How many proposals the participant must review before they can continue.
 *
 * An unset minimum means every proposal, which is the default (ADR-0015). A
 * configured minimum only ever loosens that, and is clamped to the proposals
 * that actually exist so an over-set minimum can never strand the participant
 * on a step they cannot finish.
 *
 * A step with no proposals requires nothing. Callers must not read that as
 * "the gate is open": while proposals are still loading the count is also 0,
 * which is why `canLeaveStep` takes `proposalsLoaded` separately.
 */
export function requiredReviewCount(
	configured: number | null | undefined,
	proposalCount: number
): number {
	if (proposalCount === 0) return 0;
	if (configured == null) return proposalCount;
	if (configured < 1) return 1;
	return Math.min(configured, proposalCount);
}

/**
 * Whether the participant has met the minimum and may move on.
 *
 * `proposalsLoaded` is load-bearing. Before proposals arrive the required
 * count is 0, so a plain `reviewed >= required` would open the gate on mount
 * and let a fast participant click straight past the step.
 */
export function canLeaveStep({
	reviewedCount,
	requiredReviews,
	proposalsLoaded
}: {
	reviewedCount: number;
	requiredReviews: number;
	proposalsLoaded: boolean;
}): boolean {
	if (!proposalsLoaded) return false;
	return reviewedCount >= requiredReviews;
}
