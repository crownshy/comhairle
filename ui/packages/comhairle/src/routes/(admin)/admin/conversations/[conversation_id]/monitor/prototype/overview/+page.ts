import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
	const { stats: workflowStats } = await parent();

	return { workflowStats, conversationId: params.conversation_id };
};
