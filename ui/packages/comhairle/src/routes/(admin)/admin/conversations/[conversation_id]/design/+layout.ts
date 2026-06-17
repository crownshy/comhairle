import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ parent }) => {
	const { conversation, workflows, workflowSteps } = await parent();
	return { conversation, workflows, workflowSteps };
};
