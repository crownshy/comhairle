import { redirect } from "@sveltejs/kit"
import type { PageLoad } from "./$types"

export const load: PageLoad = async ({ parent }) => {
	let { user, api } = await parent()

	if (!user) {
		redirect(307, "/")
	}
	try {
		let participation = await api.GetConversationsUserIsParticipatingIn()
		let conversation_settings = await api.GetAllUserConversationPreferences()

		return { participation, conversation_settings, user }
	}
	catch (e) {
		return { error: e }
	}
}
