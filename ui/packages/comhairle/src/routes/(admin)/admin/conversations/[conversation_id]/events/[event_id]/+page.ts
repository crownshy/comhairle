import type { PageLoad } from './$types';
import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import { key } from '$lib/utils/invalidationKey';
import { HttpStatus } from '$lib/utils/constants';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { resolve } from '$app/paths';

export const load: PageLoad = async ({ params, parent, depends }) => {
	depends(key('conversation/events'));
	const { api, conversation } = await parent();
	const { conversation_id, event_id } = params;

	const event = await tryCatchAsync(() =>
		api.GetEvent({
			params: { conversation_id, event_id },
			queries: { withTranslations: true }
		})
	);

	if (event.err !== null) {
		notifications.addFlash({ priority: 'WARNING', message: 'Problem loading event' });
		redirect(
			HttpStatus.Found,
			resolve('/(admin)/admin/conversations/[conversation_id]/events', { conversation_id })
		);
	}

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
		attendees: attendees.records,
		invites,
		recordings
	};
};
