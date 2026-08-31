import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
	const { api } = await parent();
	const { conversation_id } = params;

	const events = await tryCatchAsync(() =>
		api.ListEvents({ params: { conversation_id }, queries: { created_at: 'desc' } })
	);

	return {
		events: events.ok?.records ?? []
	};
};
