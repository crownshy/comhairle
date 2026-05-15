import { isRedirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type { InviteDto } from '@crownshy/api-client/api';

export const load: PageLoad = async ({ params, parent }) => {
	const { api, user } = await parent();
	const { invite_id, event_id, conversation_id } = params;

	try {
		const invite: InviteDto = await api.GetInvite({
			params: { conversation_id, invite_id }
		});

		if (!invite.eventId) throw new Error('Wrong invite type');

		return { user, invite, eventId: event_id, conversationId: conversation_id };
	} catch (e) {
		if (isRedirect(e)) {
			throw e;
		}
		console.error(e);
		return { error: e.response?.data?.err };
	}
};
