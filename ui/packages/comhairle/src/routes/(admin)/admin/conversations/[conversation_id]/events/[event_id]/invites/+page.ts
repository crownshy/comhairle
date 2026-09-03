import type { PageLoad } from './$types';
import { tryCatchAsync } from '$lib/utils/errorHandling';

export const load: PageLoad = async ({ parent, params }) => {
	const { api } = await parent();
	const { conversation_id, event_id } = params;

	const streamedEmailInvites = tryCatchAsync(() =>
		api
			.ListInvitesForEvent({
				params: { conversation_id, event_id }
			})
			.then((result) =>
				result.filter(
					(invite) =>
						typeof invite.inviteType !== 'string' &&
						'email' in invite.inviteType &&
						invite.inviteType.email
				)
			)
	);

	return {
		streamedEmailInvites
	};
};
