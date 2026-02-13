import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, parent }) => {
	const { api, conversation } = await parent();

	const invites = await api.ListInvitesForConversation({
		params: { conversation_id: conversation.id }
	});

	return { invites, conversation };
};
