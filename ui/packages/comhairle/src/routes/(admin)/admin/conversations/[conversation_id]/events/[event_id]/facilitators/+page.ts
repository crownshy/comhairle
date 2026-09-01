import { tryCatchAsync } from '$lib/utils/errorHandling';
import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { HttpStatus } from '$lib/utils/constants';
import { resolve } from '$app/paths';
import { notifications } from '$lib/notifications.svelte';
import type { PendingInvite } from './types';

export const load: PageLoad = async ({ parent, params }) => {
	const { api } = await parent();
	const { conversation_id, event_id } = params;

	const attendees = await tryCatchAsync(() =>
		api.ListEventAttendances({
			params: { conversation_id, event_id },
			queries: { limit: 1000 }
		})
	);

	if (attendees.err !== null) {
		notifications.addFlash({
			message: 'Could not load attendees, please try again',
			priority: 'ERROR'
		});
		redirect(
			HttpStatus.TemporaryRedirect,
			resolve('/(admin)/admin/conversations/[conversation_id]/events', { conversation_id })
		);
	}

	const invites = await tryCatchAsync(() =>
		api.ListInvitesForEvent({
			params: { conversation_id, event_id }
		})
	);

	if (invites.err !== null) {
		notifications.addFlash({
			message: 'Could not load invites, please try again',
			priority: 'ERROR'
		});
		redirect(
			HttpStatus.TemporaryRedirect,
			resolve('/(admin)/admin/conversations/[conversation_id]/events', { conversation_id })
		);
	}

	return {
		attendees: attendees.ok.records,
		pendingInvites: invites.ok.reduce<PendingInvite[]>((acc, invite) => {
			if (
				typeof invite.inviteType !== 'string' &&
				'email' in invite.inviteType &&
				invite.inviteType.email &&
				(invite.status === 'pending' || invite.status === 'open')
			) {
				acc.push({
					id: invite.id,
					email: invite.inviteType.email,
					status: invite.status
				});
			}
			return acc;
		}, [])
	};
};
