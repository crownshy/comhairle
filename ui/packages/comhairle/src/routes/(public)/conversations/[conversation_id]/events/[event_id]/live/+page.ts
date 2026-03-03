import type { PageLoad } from './$types';
import type { LocalizedEventDto, EventAttendanceDto } from '@crownshy/api-client/api';

export const load: PageLoad = async ({ parent, params }) => {
	const { api, user } = await parent();
	const { conversation_id, event_id } = params;

	try {
		const [event, attendancesResult] = await Promise.all([
			api.GetEvent({ params: { conversation_id, event_id } }),
			api.ListEventAttendances({
				params: { conversation_id, event_id },
				queries: { limit: 200 }
			})
		]);

		return {
			conversationId: conversation_id,
			eventId: event_id,
			event: event as LocalizedEventDto,
			attendances: attendancesResult.records as EventAttendanceDto[],
			user
		};
	} catch (e) {
		console.error('Failed to load live event:', e);
		return {
			conversationId: conversation_id,
			eventId: event_id,
			event: null,
			attendances: [] as EventAttendanceDto[],
			user
		};
	}
};
