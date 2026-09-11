import { tryCatchAsync } from '$lib/utils/errorHandling';
import { notifications } from '$lib/notifications.svelte';
import type { LayoutLoad } from './$types';
import type { WorkflowStepWithTranslationsDto } from '@crownshy/api-client/api';
import { key } from '$lib/utils/invalidationKey';

export const load: LayoutLoad = async ({ parent, params, depends }) => {
	depends(key('conversation/design/workflow'));

	const { api } = await parent();
	const { conversation_id } = params;

	return {
		streamedWorkflows: tryCatchAsync(async (ok, err) => {
			const workflows = await tryCatchAsync(() =>
				api.ListConversationWorkflows({ params: { conversation_id } })
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
					params: { conversation_id, workflow_id: workflows.ok[0].id }
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

			return ok({
				current: {
					workflow: workflows.ok[0],
					steps: (workflowSteps.ok as WorkflowStepWithTranslationsDto[]).toSorted(
						(a, b) => a.stepOrder - b.stepOrder
					)
				}
			});
		})
	};
};
