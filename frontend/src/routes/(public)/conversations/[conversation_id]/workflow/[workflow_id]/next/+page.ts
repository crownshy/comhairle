
import { redirect } from "@sveltejs/kit";
import type { PageLoad } from "./$types"
import { thank_you_page, workflow_step_url } from "$lib/urls";

export const ssr = false;
export const csr = true;

export const load: PageLoad = async ({ parent, params }) => {
	let { api, conversation } = await parent();
	let workflow_id = params.workflow_id;
	let conversation_id = params.conversation_id;
	let redirect_url = "/"
	try {
		let next_step = await api.NextWorkflowStepForUser({ params: { conversation_id: conversation.id, workflow_id: workflow_id } })

		if (next_step) {
			redirect_url = workflow_step_url(conversation.id, workflow_id, next_step.id)
		}
		else {
			redirect_url = thank_you_page(conversation.id, workflow_id)
		}
	}
	catch (e) {
		console.log("error ", e)
	}
	return redirect(302, redirect_url)
}
