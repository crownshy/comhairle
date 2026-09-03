import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
	const { conversation_id } = params;
	const { api, workflows } = await parent();

	return {
		streamedWorkflowSteps: tryCatchAsync(() =>
			api.ListConversationWorkflowSteps({
				params: { conversation_id, workflow_id: workflows[0].id }
			})
		),
		streamedWorkflowStats: tryCatchAsync(() =>
			api.GetConversationWorkflowStats({
				params: { conversation_id, workflow_id: workflows[0].id }
			})
		)
	};
};
