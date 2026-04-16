import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type { LocalizedEventDto, EventAttendanceDto } from '@crownshy/api-client/api';

export const load: PageLoad = async ({ parent, params }) => {
	const { api, user } = await parent();
	const { conversation_id, event_id } = params;

	let event: LocalizedEventDto;
	let attendances: EventAttendanceDto[];

	try {
		const [eventRes, attendancesResult] = await Promise.all([
			api.GetEvent({ params: { conversation_id, event_id } }),
			api.ListEventAttendances({
				params: { conversation_id, event_id },
				queries: { limit: 200 }
			})
		]);
		event = eventRes as LocalizedEventDto;
		attendances = attendancesResult.records as EventAttendanceDto[];
	} catch (e) {
		console.error('Failed to load live event:', e);
		redirect(302, `/conversations/${conversation_id}/events/${event_id}`);
	}

	// JWT may fail if user has no attendance yet — load page anyway
	let jwt: string | null = null;
	let isModerator = false;
	try {
		const authRes = await api.GetEventJWT({ params: { conversation_id, event_id } });
		jwt = authRes.jwt;
		isModerator = authRes.is_moderator ?? false;
	} catch (e) {
		console.warn('JWT not available (user may not be registered yet):', e);
	}

	return {
		conversationId: conversation_id,
		eventId: event_id,
		event,
		attendances,
		jwt,
		isModerator,
		user
	};
};
