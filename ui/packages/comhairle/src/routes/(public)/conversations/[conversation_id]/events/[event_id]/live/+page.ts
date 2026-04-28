import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type { EventResponse, EventAttendanceDto } from '@crownshy/api-client/api';

export const load: PageLoad = async ({ parent, params }) => {
	const { api, user } = await parent();
	const { conversation_id, event_id } = params;

	let event: EventResponse;
	let attendances: EventAttendanceDto[];

	try {
		const [eventRes, attendancesResult] = await Promise.all([
			api.GetEvent({ params: { conversation_id, event_id } }),
			api.ListEventAttendances({
				params: { conversation_id, event_id },
				queries: { limit: 200 }
			})
		]);
		console.log({ eventRes, attendancesResult });
		event = eventRes;
		attendances = attendancesResult.records as EventAttendanceDto[];
	} catch (e) {
		console.error('Failed to load live event:', e);
		redirect(302, `/conversations/${conversation_id}/events/${event_id}`);
	}

	// No JWT → user not registered, send back to event detail page
	let jwt: string;
	let isModerator = false;
	try {
		console.log('Attemptng to get JWT');
		const authRes = await api.GetEventJWT({ params: { conversation_id, event_id } });
		jwt = authRes.jwt;
		isModerator = authRes.isModerator ?? false;
	} catch (e) {
		console.warn('JWT not available (user may not be registered yet):', e);
		redirect(302, `/conversations/${conversation_id}/events/${event_id}?error=not-registered`);
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
