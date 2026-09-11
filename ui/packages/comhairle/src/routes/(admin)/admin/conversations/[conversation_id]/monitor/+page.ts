import { notifications } from '$lib/notifications.svelte';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
	const { conversation_id } = params;
	const { api } = await parent();

	const workflows = await tryCatchAsync(() =>
		api.ListConversationWorkflows({ params: { conversation_id } })
	);

	if (workflows.err !== null) {
		console.error(workflows.err);
		notifications.addFlash({
			message: 'Problem loading workflows',
			priority: 'ERROR'
		});

		return {
			streamedWorkflowSteps: Promise.resolve({
				ok: null,
				err: 'Could not retrieve workflows'
			}),
			streamedWorkflowStats: Promise.resolve({
				ok: null,
				err: 'Could not retrieve workflows'
			})
		};
	}

	return {
		streamedWorkflowSteps: tryCatchAsync(() =>
			api.ListConversationWorkflowSteps({
				params: { conversation_id, workflow_id: workflows.ok[0].id }
			})
		),
		streamedWorkflowStats: tryCatchAsync(() =>
			api.GetConversationWorkflowStats({
				params: { conversation_id, workflow_id: workflows.ok[0].id }
			})
		)
	};
};
