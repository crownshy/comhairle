import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, parent }) => {
	const { conversation, workflowSteps } = await parent();

	return { conversation, workflowSteps };
};
