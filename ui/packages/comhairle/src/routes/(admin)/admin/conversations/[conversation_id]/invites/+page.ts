import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async ({ parent, depends, params }) => {
	depends(key('conversation/invites'));
	const { api } = await parent();
	const { conversation_id } = params;

	const invites = await api.ListInvitesForConversation({
		params: { conversation_id }
	});

	return { invites };
};
