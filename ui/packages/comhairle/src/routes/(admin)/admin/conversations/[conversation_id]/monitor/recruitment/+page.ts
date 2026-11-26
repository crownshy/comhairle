import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import { tryCatchAsync } from '$lib/utils/errorHandling';

export const load: PageLoad = async ({ parent, params, depends }) => {
	depends(key('admin/conversation/monitor'));

	const { api } = await parent();
	const { conversation_id } = params;

	const invitesRes = await tryCatchAsync(() =>
		api.ListInvitesForConversation({ params: { conversation_id } })
	);

	if (invitesRes.err !== null) {
		return { invites: [] };
	}

	return { invites: invitesRes.ok };
};
