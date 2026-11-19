import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { stats: workflowStats } = await parent();
	return { workflowStats };
};
