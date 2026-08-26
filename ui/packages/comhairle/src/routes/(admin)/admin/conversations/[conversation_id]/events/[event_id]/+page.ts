import type { PageLoad } from '../$types';
import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async ({ params, parent, depends }) => {
	depends(key('conversation/events'));
	const conversation_id = params.conversation_id;
	const event_id = params.event_id;
	const { api, conversation } = await parent();

	try {
		const event = await api.GetEvent({
			params: { conversation_id, event_id },
			queries: { withTranslations: true }
		});

		// All registered attendees (any role), so the facilitators tab can list
		// everyone and let an admin promote/demote them per person. Large limit to
		// pull the whole roster in one page.
		const attendees = await api.ListEventAttendances({
			params: { conversation_id, event_id },
			queries: { limit: 1000 }
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
			attendees: attendees.records,
			invites,
			recordings
		};
	} catch (e) {
		console.error(e);
		notifications.addFlash({ priority: 'WARNING', message: 'Problem loading event' });
		redirect(302, `/admin/conversations/${conversation_id}/events`);
	}
};
