export type StepStatus = 'completed' | 'completed-locked' | 'current' | 'upcoming';

/**
 * One row of the step dropdown. Carried over verbatim from the old `StepSelector` so
 * navigation permissions are unchanged: only completed, revisitable steps get an `href`.
 */
export type StepItem = {
	id: string;
	name: string;
	status: StepStatus;
	href?: string;
	/**
	 * A segment that is part of the journey but not one of the workflow's steps: today only
	 * the conversation landing page, which is Step zero (ADR-0021). Excluded from "Step N of
	 * M" so adding it cannot change the count participants are quoted.
	 */
	isIntro?: boolean;
};
