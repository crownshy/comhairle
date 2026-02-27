import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { conversation, workflows, workflowSteps } = await parent();

	return { conversation, workflows, workflowSteps };
};
