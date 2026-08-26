import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type { LocalizedEventDto, EventAttendanceDto } from '@crownshy/api-client/api';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async ({ parent, params, url, depends }) => {
	depends(key('event'));
	const { api, user } = await parent();
	const { conversation_id, event_id } = params;

	if (!user) {
		redirect(302, `/auth/login-otp/send?backTo=${encodeURIComponent(url.pathname)}`);
	}

	try {
		const [event, attendancesResult] = await Promise.all([
			api.GetEvent({ params: { conversation_id, event_id } }),
			api.ListEventAttendances({
				params: { conversation_id, event_id },
				queries: { limit: 100 }
			})
		]);

		return {
			conversationId: conversation_id,
			event: event as LocalizedEventDto,
			attendances: attendancesResult.records as EventAttendanceDto[],
			totalAttendances: attendancesResult.total as number,
			user
		};
	} catch (e) {
		console.error('Failed to load event:', e);
		return {
			conversationId: conversation_id,
			event: null,
			attendances: [] as EventAttendanceDto[],
			totalAttendances: 0,
			user
		};
	}
};
