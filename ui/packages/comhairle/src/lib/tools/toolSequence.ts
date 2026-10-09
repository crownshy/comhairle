/**
 * What a tool's `UserUI` can report so the pager and progress bar act inside it (ADR-0047).
 * A tool that reports nothing is a single page, and the arrows act at the step boundary.
 */
export type ToolSequence = {
	/** Undefined at the tool's last page, so forward completes the step. */
	next?: () => void;
	/** Undefined at the tool's first page, so back leaves the step. */
	previous?: () => void;
	/** Fill of this step's progress segment, 0 to 1. */
	progress?: number;
	/** Why forward is closed, such as "Vote on 3 more statements", shown when it is pressed. */
	blockedReason?: string;
};

export type OnSequenceChange = (sequence: ToolSequence) => void;
