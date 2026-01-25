import { loginRedirect } from "$lib/urls";
import { isRedirect, redirect } from "@sveltejs/kit";
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ parent, params }) => {

	let { api, user } = await parent()
	let conversation_id = params.conversation_id
	let preview = params.preview === 'preview'

	try {
		let conversation = await api.GetConversation({ params: { conversation_id } })

		if (!conversation.is_live && !preview) {
			return redirect(302, "/");
		}

		let workflows = await api.ListWorkflows({ params: { conversation_id: conversation.id } });

		let participation

		if (user) {
			participation = await api.GetUserParticipation({ params: { conversation_id: conversation.id, workflow_id: workflows[0].id } })
		}
		else {
			participation = null
		}

		return { conversation, workflows, participation, api, user, preview }

	}
	catch (e) {
		if (isRedirect(e)) {
			throw e
		}
		return redirect(302, "/");
		loginRedirect(`/conversations/${params.conversation_id}`, "Login to take part")
	}
}
