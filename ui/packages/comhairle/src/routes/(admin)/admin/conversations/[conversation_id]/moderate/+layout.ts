import { notifications } from '$lib/notifications.svelte';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ params, parent }) => {
	const { api } = await parent();

	return {
		streamedWorkflowSteps: tryCatchAsync(async (ok, err) => {
			const workflows = await tryCatchAsync(() =>
				api.ListConversationWorkflows({
					params: { conversation_id: params.conversation_id }
				})
			);

			if (workflows.err !== null) {
				console.error(workflows.err);
				notifications.addFlash({
					message: 'Problem loading workflows',
					priority: 'WARNING'
				});
				throw err(workflows.err);
			}

			const workflowSteps = await tryCatchAsync(() =>
				api.ListConversationWorkflowSteps({
					params: {
						conversation_id: params.conversation_id,
						workflow_id: workflows.ok[0].id
					}
				})
			);

			if (workflowSteps.err !== null) {
				notifications.addFlash({
					message: 'Could not load workflow steps',
					priority: 'WARNING'
				});
				console.error(workflowSteps.err);
				throw err(workflowSteps.err);
			}

			return ok(workflowSteps.ok.toSorted((a, b) => a.stepOrder - b.stepOrder));
		})
	};
};
