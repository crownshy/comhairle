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
};
