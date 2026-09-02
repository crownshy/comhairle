import type { PageLoad } from './$types';
import { tryCatchAsync } from '$lib/utils/errorHandling';

export const load: PageLoad = async ({ parent, params }) => {
	const { api } = await parent();
	const { conversation_id, event_id } = params;

	return {
		attendees: tryCatchAsync(() =>
			api
				.ListEventAttendances({
					params: { conversation_id, event_id },
					queries: { limit: 1000 }
				})
				.then((result) => result.records)
		)
	};
};
