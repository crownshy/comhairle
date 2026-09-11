import { tryCatchAsync } from '$lib/utils/errorHandling';
import { key } from '$lib/utils/invalidationKey';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params, depends }) => {
	depends(key('conversation/events'));

	const { api } = await parent();
	const { conversation_id } = params;

	const events = await tryCatchAsync(() =>
		api.ListEvents({ params: { conversation_id }, queries: { created_at: 'desc' } })
	);

	return {
		events: events.ok?.records ?? []
	};
};
