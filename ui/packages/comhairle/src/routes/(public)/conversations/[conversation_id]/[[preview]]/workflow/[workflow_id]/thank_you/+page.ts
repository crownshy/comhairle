import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { conversation, user, workflows, workflowSteps, preview } = await parent();

	const revisitableSteps = workflowSteps
		.filter((s) => s.canRevisit)
		.sort((a, b) => a.stepOrder - b.stepOrder);

	return { conversation, user, workflow: workflows[0], revisitableSteps, preview };
};
