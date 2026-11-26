import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
	const { api, workflows } = await parent();
	const conversation_id = params.conversation_id;

	const workflow = workflows[0];

	if (!workflow) return { workflowStats: null };

	const workflowStats = await api.GetConversationWorkflowStats({
		params: { conversation_id: conversation_id, workflow_id: workflow.id }
	});

	return { workflowStats, conversationId: conversation_id };
};
