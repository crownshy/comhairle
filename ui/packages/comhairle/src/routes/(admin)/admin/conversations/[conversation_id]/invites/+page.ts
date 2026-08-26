import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async ({ parent, depends }) => {
	depends(key('conversation/invites'));
	const { api, conversation } = await parent();

	const invites = await api.ListInvitesForConversation({
		params: { conversation_id: conversation.id }
	});

	return { invites, conversation };
};
