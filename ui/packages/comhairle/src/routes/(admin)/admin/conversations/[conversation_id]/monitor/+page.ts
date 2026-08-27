import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
	const { conversation_id } = params;
	const { api, workflows } = await parent();

	const workflowStepsResponse = tryCatchAsync(() =>
		api.GetConversationWorkflowStats({
			params: { conversation_id, workflow_id: workflows[0].id }
		})
	);

	const workflowStatsResponse = tryCatchAsync(() =>
		api.GetConversationWorkflowStats({
			params: { conversation_id, workflow_id: workflows[0].id }
		})
	);

	const workflowSteps = await workflowStepsResponse;
	if (workflowSteps !== null) {
		console.error(workflowSteps.err);
	}

	const workflowStats = await workflowStatsResponse;
	if (workflowStats !== null) {
		console.error(workflowStats.err);
	}

	return { workflowSteps: workflowSteps.ok ?? [], workflowStats: workflowStats.ok ?? [] };
};
