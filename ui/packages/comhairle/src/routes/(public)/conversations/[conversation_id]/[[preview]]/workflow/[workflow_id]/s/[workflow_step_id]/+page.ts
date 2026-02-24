import { isRedirect, redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { notifications } from '$lib/notifications.svelte';
import { next_workflow_step_url } from '$lib/urls';
import type { LocalizedWorkflowStepDto } from '$lib/api/api';

export const load: PageLoad = async (event) => {
	const { api, conversation, preview } = await event.parent();

	const conversation_id = conversation.id;
	const { workflow_id, workflow_step_id } = event.params;
	try {
		const current_step = await api.NextWorkflowStepForUser({
			params: { conversation_id: conversation.id, workflow_id: workflow_id }
		});
		// If we are in preview mode then let the user see this step regardless of if it
		// is next. Also dont capture progress
		if (conversation.isLive) {
			if (current_step && current_step.id !== workflow_step_id) {
				return redirect(302, next_workflow_step_url(conversation_id, workflow_id, preview));
			}

			await api.SetUserProgress('in_progress', {
				params: { conversation_id, workflow_id, workflow_step_id },
				headers: { 'Content-Type': 'application/json' }
			});
		}
		const workflowSteps: LocalizedWorkflowStepDto[] = await api.ListWorkflowSteps({
			params: { conversation_id, workflow_id }
		});
		const workflowStep: LocalizedWorkflowStepDto = await api.GetWorkflowStep({
			params: {
				conversation_id: conversation_id,
				workflow_id: workflow_id,
				workflow_step_id: workflow_step_id
			}
		});

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
