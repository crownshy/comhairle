import type { PageLoad } from "./$types"


export const load: PageLoad = async ({ params, parent }) => {
	let { api, conversation } = await parent();


	const invites = await api.ListInvitesForConversation({ params: { conversation_id: conversation.id } });

	return { invites, conversation };
}
