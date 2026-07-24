import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type { EventResponse, AudioRecordingDto } from '@crownshy/api-client/api';

export const load: PageLoad = async ({ parent, params, url }) => {
	const { api, user } = await parent();
	const { conversation_id, event_id } = params;

	if (!user) {
		redirect(302, `/auth/login-otp/send?backTo=${encodeURIComponent(url.pathname)}`);
	}

	let event: EventResponse;
	try {
		event = await api.GetEvent({ params: { conversation_id, event_id } });
	} catch {
		redirect(302, `/conversations/${conversation_id}/events/${event_id}`);
	}

	try {
		const attendances = await api.ListEventAttendances({
			params: { conversation_id, event_id },
			queries: { limit: 200 }
		});
		const isRegistered = attendances.records.some(
			(attendance) => attendance.userId === user.id
		);
		if (!isRegistered) {
			redirect(
				302,
				`/conversations/${conversation_id}/events/${event_id}?error=not-registered`
			);
		}
	} catch {
		redirect(302, `/conversations/${conversation_id}/events/${event_id}`);
	}

	let recordings: AudioRecordingDto[] = [];
	try {
		recordings = await api.ListAudioRecordings({ params: { conversation_id, event_id } });
	} catch {
		// Participant flows can continue without the name list.
	}

	return {
		conversationId: conversation_id,
		eventId: event_id,
		event,
		recordings,
		user
	};
};
