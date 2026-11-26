import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ params, parent, depends }) => {
	const { api, workflows, workflowSteps } = await parent();
	const conversation_id = params.conversation_id;

	const workflow = workflows[0];

	if (!workflow) return { workflowStats: null };

	const workflowStats = await api.GetConversationWorkflowStats({
		params: { conversation_id: conversation_id, workflow_id: workflow.id }
	});

	return { workflowSteps, workflowStats, conversationId: conversation_id };
};
