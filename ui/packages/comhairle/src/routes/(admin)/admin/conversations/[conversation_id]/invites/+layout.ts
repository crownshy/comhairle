import type { LayoutLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import { tryCatchAsync } from '$lib/utils/errorHandling';

export const load: LayoutLoad = async ({ parent, depends, params }) => {
	depends(key('conversation/invites'));

	const { api } = await parent();
	const { conversation_id } = params;

	return {
		streamedInvites: tryCatchAsync(() =>
			api
				.ListInvitesForConversation({
					params: { conversation_id }
				})
				.then((result) => {
					return {
						emailInvites: result.filter(
							(invite) =>
								typeof invite.inviteType !== 'string' &&
								'email' in invite.inviteType &&
								invite.inviteType.email
						),
						openInvites: result.filter((invite) => invite.inviteType === 'open')
					};
				})
		)
	};
};
