import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
	const { api } = await parent();
	const { conversation_id } = params;

	const result = await tryCatchAsync(() =>
		api.ListEvents({ params: { conversation_id }, queries: { created_at: 'desc' } })
	);
	if (result.err !== null) {
		return;
	}

	return {
		events: result.ok.records
	};
};
