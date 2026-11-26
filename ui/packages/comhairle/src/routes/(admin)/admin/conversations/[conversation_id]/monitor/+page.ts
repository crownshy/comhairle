import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async ({ parent, params, depends }) => {
	depends(key('admin/conversation/monitor'));

	const { workflowSteps, workflows, api } = await parent();
	const { conversation_id } = params;

	const workflow = workflows[0];

	if (!workflow) return { workflowStats: null };

	const workflowStats = await api.GetConversationWorkflowStats({
		params: { conversation_id: conversation_id, workflow_id: workflow.id }
	});

	return { workflowSteps, workflowStats, conversationId: params.conversation_id };
};
