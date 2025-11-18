import { isRedirect, redirect } from "@sveltejs/kit";
import type { PageLoad } from './$types';
import { notifications } from '$lib/notifications.svelte';
import { next_workflow_step_url } from "$lib/urls";

export const load: PageLoad = async (event) => {
	let { api, conversation } = await event.parent();

	let conversation_id = conversation.id;
	let { workflow_id, workflow_step_id } = event.params
	try {

		let current_step = await api.NextWorkflowStepForUser({ params: { conversation_id: conversation.id, workflow_id: workflow_id } })

		if (current_step && (current_step.id !== workflow_step_id)) {
			return redirect(302, next_workflow_step_url(conversation_id, workflow_id))
		}

		await api.SetUserProgress("in_progress", {
			params: { conversation_id, workflow_id, workflow_step_id }
			,

			headers: { 'Content-Type': 'application/json' }
		})

		let workflow_steps = await api.ListWorkflowSteps({ params: { conversation_id, workflow_id } })
		let workflow_step = await api.GetWorkflowStep({ params: { conversation_id: conversation_id, workflow_id: workflow_id, workflow_step_id: workflow_step_id } })

		return { conversation, workflow_step, api, workflow_steps }
	}
	//TODO figure out how to type this from the generated api
	catch (e: any) {
		/// Throw if error is a redirect
		if (isRedirect(e)) {
			console.log(e)
			throw e
		}
		//TODO we probably want some error codes to match on here
		// rather than the plain text
		if (e.response.data.err === "User Required for this route") {
			notifications.addFlash({ message: "Login or signup to take part in the conversation", priority: "INFO" })
			redirect(307, "/auth/login")
		}
		redirect(307, "/")
	}
}
