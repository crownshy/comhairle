/** The cover's share of a step's progress segment. The body owns the rest. */
const COVER_SHARE = 0.12;

/**
 * How full the current step's progress segment is, 0 to 1.
 *
 * The cover creeps across a small leading share so the bar responds while a participant
 * pages through the brief, then the body takes over. A body that reports no progress holds
 * at the handover point rather than jumping, which is the honest reading for a tool that
 * cannot say where it is (HeyForm's iframe, ADR-0018). The completion screen is the one
 * place the segment is unambiguously full, whatever the tool last reported.
 */
export function segmentFill(options: {
	phase: 'cover' | 'body' | 'done';
	slideIndex: number;
	slideCount: number;
	toolProgress?: number;
}): number {
	const { phase, slideIndex, slideCount, toolProgress } = options;

	if (phase === 'done') return 1;

	if (phase === 'cover') {
		const steps = Math.max(1, slideCount);
		return (COVER_SHARE * Math.min(slideIndex + 1, steps)) / steps;
	}

	if (typeof toolProgress !== 'number' || Number.isNaN(toolProgress)) {
		return COVER_SHARE;
	}
	const clamped = Math.min(1, Math.max(0, toolProgress));
	return COVER_SHARE + (1 - COVER_SHARE) * clamped;
}
