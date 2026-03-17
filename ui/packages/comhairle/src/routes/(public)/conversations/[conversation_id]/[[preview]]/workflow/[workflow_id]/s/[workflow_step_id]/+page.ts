import { isRedirect, redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { notifications } from '$lib/notifications.svelte';
import { next_workflow_step_url } from '$lib/urls';
import type { LocalizedWorkflowStepDto, UserProgressDto } from '@crownshy/api-client/api';

export const load: PageLoad = async (event) => {
	const { api, conversation, preview } = await event.parent();

	const conversation_id = conversation.id;
	const { workflow_id, workflow_step_id } = event.params;
	try {
		let userProgress: UserProgressDto[] = [];
		try {
			userProgress = await api.GetUserProgress({
				params: { conversation_id, workflow_id }
			});
		} catch {
			// Progress may not be available
		}

		const stepProgress = userProgress.find((p) => p.workflowStepId === workflow_step_id);
		const isStepAlreadyDone = stepProgress?.status === 'done';

		// If we are in preview mode then let the user see this step regardless of if it
		// is next. Also dont capture progress
		if (conversation.isLive && !isStepAlreadyDone) {
			const current_step = await api.NextConversationWorkflowStepForUser({
				params: { conversation_id: conversation.id, workflow_id: workflow_id }
			});

			if (current_step && current_step.id !== workflow_step_id) {
				return redirect(302, next_workflow_step_url(conversation_id, workflow_id, preview));
			}

			await api.SetUserProgress('in_progress', {
				params: { conversation_id, workflow_id, workflow_step_id },
				headers: { 'Content-Type': 'application/json' }
			});
		}
		const workflowSteps: LocalizedWorkflowStepDto[] = await api.ListConversationWorkflowSteps({
			params: { conversation_id, workflow_id }
		});
		const workflowStep: LocalizedWorkflowStepDto = await api.GetConversationWorkflowStep({
			params: {
				conversation_id: conversation_id,
				workflow_id: workflow_id,
				workflow_step_id: workflow_step_id
			}
		});

		return { conversation, workflowStep, api, workflowSteps, workflow_id, userProgress };
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
