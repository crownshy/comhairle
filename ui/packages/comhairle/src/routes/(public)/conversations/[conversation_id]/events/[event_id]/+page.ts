import { error, redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type { LocalizedEventDto, EventAttendanceDto } from '@crownshy/api-client/api';
import { apiErrorStatus, tryCatchAsync } from '$lib/utils/errorHandling';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async ({ parent, params, url, depends }) => {
	depends(key('public/event'));
	const { api, user } = await parent();
	const { conversation_id, event_id } = params;

	if (!user) {
		redirect(302, `/auth/login-otp/send?backTo=${encodeURIComponent(url.pathname)}`);
	}

	const loaded = await tryCatchAsync(() =>
		Promise.all([
			api.GetEvent({ params: { conversation_id, event_id } }),
			api.ListEventAttendances({
				params: { conversation_id, event_id },
				queries: { limit: 100 }
			})
		])
	);

	if (loaded.err !== null) {
		const status = apiErrorStatus(loaded.err) ?? 500;
		// A malformed id comes back as 400, and it can't match an event either.
		if (status !== 400 && status !== 404) {
			error(status, 'Could not load this event. Please try again.');
		}

		return {
			conversationId: conversation_id,
			event: null,
			attendances: [] as EventAttendanceDto[],
			totalAttendances: 0,
			user
		};
	}

	const [event, attendancesResult] = loaded.ok;
	return {
		conversationId: conversation_id,
		event: event as LocalizedEventDto,
		attendances: attendancesResult.records as EventAttendanceDto[],
		totalAttendances: attendancesResult.total as number,
		user
	};
};
