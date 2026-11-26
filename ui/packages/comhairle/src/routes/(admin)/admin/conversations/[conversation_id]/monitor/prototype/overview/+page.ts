import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
	const { workflowSteps, workflowStats } = await parent();

	return { workflowSteps, workflowStats, conversationId: params.conversation_id };
};
