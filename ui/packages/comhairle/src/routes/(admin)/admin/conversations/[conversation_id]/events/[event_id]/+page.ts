import type { PageLoad } from '../$types';
import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';

export const load: PageLoad = async ({ params, parent }) => {
	const conversation_id = params.conversation_id;
	const event_id = params.event_id;
	const { api, conversation } = await parent();

	try {
		const event = await api.GetEvent({
			params: { conversation_id, event_id },
			queries: { withTranslations: true }
		});

		const facilitators = await api.ListEventAttendances({
			params: { conversation_id, event_id },
			queries: { role: 'facilitator' }
		});
		const moderators = await api.ListEventAttendances({
			params: { conversation_id, event_id },
			queries: { role: 'moderator' }
		});

		const invites = await api.ListInvitesForEvent({
			params: { conversation_id: conversation.id, event_id }
		});

		const recordings = await api.ListAudioRecordings({
			params: { conversation_id, event_id }
		});

		return {
			event,
			conversation,
			facilitators: facilitators.records,
			moderators: moderators.records,
			invites,
			recordings
		};
	} catch (e) {
		console.error(e);
		notifications.addFlash({ priority: 'WARNING', message: 'Problem loading event' });
		redirect(302, `/admin/conversations/${conversation_id}/events`);
	}
};
