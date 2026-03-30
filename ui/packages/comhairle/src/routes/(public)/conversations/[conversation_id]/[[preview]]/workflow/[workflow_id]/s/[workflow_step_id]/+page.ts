import { isRedirect, redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { notifications } from '$lib/notifications.svelte';
import { next_workflow_step_url } from '$lib/urls';
import type {
	LocalizedWorkflowStepDto,
	LocalizedWorkflowStepWithProgressDto
} from '@crownshy/api-client/api';

export const load: PageLoad = async (event) => {
	const { api, conversation, preview } = await event.parent();

	const conversation_id = conversation.id;
	const { workflow_id, workflow_step_id } = event.params;
	try {
		let workflowSteps: LocalizedWorkflowStepWithProgressDto[];

		if (conversation.isLive) {
			workflowSteps = (await api.ListConversationWorkflowSteps({
				params: { conversation_id, workflow_id },
				queries: { withUserProgress: true }
			})) as LocalizedWorkflowStepWithProgressDto[];
		} else {
			const steps = (await api.ListConversationWorkflowSteps({
				params: { conversation_id, workflow_id }
			})) as LocalizedWorkflowStepDto[];
			workflowSteps = steps.map((s) => ({
				...s,
				progressStatus: 'not_started' as const
			}));
		}

		const thisStep = workflowSteps.find((s) => s.id === workflow_step_id);
		const isStepAlreadyDone = thisStep?.progressStatus === 'done';
		const isRevisitable = thisStep?.canRevisit ?? false;

		// If we are in preview mode then let the user see this step regardless of if it
		// is next. Also dont capture progress
		if (conversation.isLive) {
			if (isStepAlreadyDone && !isRevisitable) {
				return redirect(302, next_workflow_step_url(conversation_id, workflow_id, preview));
			}

			if (!isStepAlreadyDone) {
				const current_step = await api.NextConversationWorkflowStepForUser({
					params: { conversation_id: conversation.id, workflow_id: workflow_id }
				});

				if (current_step && current_step.id !== workflow_step_id) {
					return redirect(
						302,
						next_workflow_step_url(conversation_id, workflow_id, preview)
					);
				}

				await api.SetUserProgress('in_progress', {
					params: { conversation_id, workflow_id, workflow_step_id },
					headers: { 'Content-Type': 'application/json' }
				});
			}
		}

		const workflowStep = thisStep!;

		return { conversation, workflowStep, api, workflowSteps, workflow_id };
	} catch (e: any) {
		// TODO: figure out how to type this from the generated api
		/// Throw if error is a redirect
		if (isRedirect(e)) {
			console.log(e);
			throw e;
		}
		// TODO: we probably want some error codes to match on here
		// rather than the plain text
		if (e.response.data.err === 'User Required for this route') {
			notifications.addFlash({
				message: 'Login or signup to take part in the conversation',
				priority: 'INFO'
			});
			redirect(307, '/auth/login');
		}
		redirect(307, '/');
	}
};
