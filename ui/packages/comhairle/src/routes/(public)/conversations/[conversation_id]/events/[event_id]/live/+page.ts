import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type { LocalizedEventDto, EventAttendanceDto } from '@crownshy/api-client/api';

export const load: PageLoad = async ({ parent, params }) => {
	const { api, user } = await parent();
	const { conversation_id, event_id } = params;

	try {
		const [event, attendancesResult, authRes] = await Promise.all([
			api.GetEvent({ params: { conversation_id, event_id } }),
			api.ListEventAttendances({
				params: { conversation_id, event_id },
				queries: { limit: 200 }
			}),
			api.GetEventJWT({ params: { conversation_id, event_id } })
		]);

		return {
			conversationId: conversation_id,
			eventId: event_id,
			event: event as LocalizedEventDto,
			attendances: attendancesResult.records as EventAttendanceDto[],
			jwt: authRes.jwt,
			isModerator: authRes.is_moderator ?? false,
			user
		};
	} catch (e) {
		console.error('Failed to load live event:', e);
		redirect(302, `/conversations/${conversation_id}/events/${event_id}`);
	}
};
