import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { thank_you_page, workflow_step_url } from '$lib/urls';

export const ssr = false;
export const csr = true;

export const load: PageLoad = async ({ parent, params, url }) => {
	const { api, conversation, preview, workflowSteps } = await parent();
	const workflow_id = params.workflow_id;

	// Preserve query parameters for redirects
	const queryString = url.search;

	let redirect_url = '/';
	try {
		if (conversation.isComplete) {
			redirect_url = `/conversations/${conversation.id}`;
		} else if (!preview && conversation.isLive) {
			const next_step = await api.NextConversationWorkflowStepForUser({
				params: { conversation_id: conversation.id, workflow_id: workflow_id }
			});

			if (next_step) {
				redirect_url = workflow_step_url(
					conversation.id,
					workflow_id,
					next_step.id,
					preview
				);
			} else {
				redirect_url = thank_you_page(conversation.id, workflow_id, preview);
			}
		} else {
			const firstStep = workflowSteps.find((s) => s.stepOrder === 1);
			redirect_url = workflow_step_url(conversation.id, workflow_id, firstStep.id, preview);
		}
	} catch (e) {}
	return redirect(302, redirect_url + queryString);
};
