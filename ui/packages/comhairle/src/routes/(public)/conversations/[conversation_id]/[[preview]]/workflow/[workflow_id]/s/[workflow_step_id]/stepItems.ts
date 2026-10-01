export type StepStatus = 'completed' | 'completed-locked' | 'current' | 'upcoming';

/** One step as the progress bar sees it. Only completed, revisitable steps get an `href`. */
export type StepItem = {
	id: string;
	name: string;
	status: StepStatus;
	href?: string;
};
