import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import type { InviteDto } from '@crownshy/api-client/api';
import { typed } from '$lib/utils/types';

type PendingInvite = Pick<InviteDto, 'id' | 'status'> & { email: string };

export const load: PageLoad = async ({ parent, params, depends }) => {
	depends(key('event/facilitators'));

	const { api } = await parent();
	const { conversation_id, event_id } = params;

	return {
		streamedAttendees: tryCatchAsync(() =>
			api
				.ListEventAttendances({
					params: { conversation_id, event_id },
					queries: { limit: 1000 }
				})
				.then((result) => result.records)
		),
		streamedPendingInvites: tryCatchAsync(() =>
			api
				.ListInvitesForEvent({
					params: { conversation_id, event_id }
				})
				.then((result) =>
					typed<PendingInvite[]>(
						result
							.filter(
								(invite) =>
									typeof invite.inviteType !== 'string' &&
									'email' in invite.inviteType &&
									invite.inviteType.email &&
									(invite.status === 'pending' || invite.status === 'open')
							)
							.map((invite) => ({
								id: invite.id,
								email: (invite.inviteType as { email: string }).email,
								status: invite.status
							}))
					)
				)
		)
	};
};
