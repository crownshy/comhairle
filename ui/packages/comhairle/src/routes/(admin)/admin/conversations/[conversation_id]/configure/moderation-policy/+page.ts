import { notifications } from '$lib/notifications.svelte';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { HttpStatus } from '$lib/utils/constants';
import { resolve } from '$app/paths';

export const load: PageLoad = async ({ parent, params }) => {
	const { api } = await parent();
	const { conversation_id } = params;

	const [policies, defaultReasons] = await Promise.all([
		tryCatchAsync(() =>
			api.ListConversationModerationPolicies({ params: { conversation_id } })
		),
		tryCatchAsync(() =>
			api.GetDefaultModerationPolicyReasons({
				params: { conversation_id }
			})
		)
	]);

	if (policies.err !== null || defaultReasons.err !== null) {
		notifications.addFlash({
			message: 'Problem loading for this page, please try again',
			priority: 'ERROR'
		});

		console.error(policies.err, defaultReasons.err);

		redirect(
			HttpStatus.Found,
			resolve('/(admin)/admin/conversations/[conversation_id]/configure/details', {
				conversation_id
			})
		);
	}

	return {
		policies: policies.ok,
		defaultReasons: defaultReasons.ok,
		streamedWorkflow: tryCatchAsync(async (ok, err) => {
			const workflows = await tryCatchAsync(() =>
				api.ListConversationWorkflows({ params: { conversation_id } })
			);
			if (workflows.err !== null) {
				throw err('Unable to load workflows:' + workflows.err);
			}

			const workflowSteps = await tryCatchAsync(() =>
				api.ListConversationWorkflowSteps({
					params: { conversation_id, workflow_id: workflows.ok[0].id },
					queries: { withTranslations: true }
				})
			);
			if (workflowSteps.err !== null) {
				throw err('Unable to load workflows:' + workflowSteps.err);
			}

			return ok({ id: workflows.ok[0].id, steps: workflowSteps.ok });
		})
	};
};
