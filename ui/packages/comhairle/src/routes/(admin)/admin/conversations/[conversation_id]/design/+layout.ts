import { tryCatchAsync } from '$lib/utils/errorHandling';
import { notifications } from '$lib/notifications.svelte';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ parent, params }) => {
	const { api, workflows } = await parent();
	const { conversation_id } = params;

	const workflowSteps = await tryCatchAsync(() =>
		api.ListConversationWorkflowSteps({
			params: { conversation_id, workflow_id: workflows[0].id }
		})
	);

	if (workflowSteps.err !== null) {
		notifications.addFlash({
			message: 'Could not load workflow steps',
			priority: 'WARNING'
		});
		console.error(workflowSteps.err);
	}

	return {
		workflowSteps: workflowSteps.ok ?? []
	};
};
