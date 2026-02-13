import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { thank_you_page, workflow_step_url } from '$lib/urls';

export const ssr = false;
export const csr = true;

export const load: PageLoad = async ({ parent, params }) => {
	const { api, conversation, preview } = await parent();
	const workflow_id = params.workflow_id;
	let redirect_url = '/';
	try {
		if (conversation.is_live) {
			const next_step = await api.NextWorkflowStepForUser({
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
			const steps = await api.ListWorkflowSteps({
				params: { conversation_id: conversation.id, workflow_id: workflow_id }
			});
			const first_step = steps.find((s) => s.step_order === 1);
			redirect_url = workflow_step_url(conversation.id, workflow_id, first_step.id, preview);
		}
	} catch (e) {}
	return redirect(302, redirect_url);
};
