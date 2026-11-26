import type { PageLoad } from './overview/$typesew/$types';
import { key } from '$lib/utils/invalidationKey';
import { tryCatchAsync } from '$lib/utils/errorHandling';

export const load: PageLoad = async ({ parent, params, depends }) => {
	depends(key('admin/conversation/monitor'));

	const { workflowSteps, workflows, api } = await parent();
	const { conversation_id } = params;

	const workflow = workflows[0];

	if (!workflow) return { workflowStats: null };

	const result = await tryCatchAsync(() =>
		api.GetConversationWorkflowStats({
			params: { conversation_id: conversation_id, workflow_id: workflow.id }
		})
	);

	if (result.err !== null) return { workflowStats: null };

	return { workflowSteps, workflowStats: result.ok, conversationId: params.conversation_id };
};
