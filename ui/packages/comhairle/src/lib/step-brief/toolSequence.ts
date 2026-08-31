/**
 * The optional contract a tool `UserUI` implements so the pager and the progress bar can
 * speak for it (ADR-0018).
 *
 * A tool that reports nothing is a single page whose progress is step-granular, which is
 * how six of the seven tools behaved before this existed. HeyForm is a cross-origin iframe
 * and can never implement it.
 */
export type ToolSequence = {
	/** Advance within the tool. Undefined means the tool is at its end, so forward completes the step. */
	next?: () => void;
	/** Go back within the tool. Undefined means the tool is at its start, so back leaves the step. */
	prev?: () => void;
	/** Fill of this step's progress segment, 0 to 1. Undefined means not reported. */
	progress?: number;
	/**
	 * A within-step position shown beside the step label, e.g. Polis's "Opinion 3 of 12".
	 * Undefined hides it. This is where `show_remaining_statements` now lands.
	 */
	count?: string;
};

export type OnSequenceChange = (sequence: ToolSequence) => void;

/** Clamped so a tool reporting a bad fraction cannot bleed into a neighbouring segment. */
export function clampProgress(value: number | undefined): number | undefined {
	if (typeof value !== 'number' || Number.isNaN(value)) return undefined;
	return Math.min(1, Math.max(0, value));
}
