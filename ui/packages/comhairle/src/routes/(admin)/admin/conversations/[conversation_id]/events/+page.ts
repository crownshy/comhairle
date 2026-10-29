import { tryCatchAsync } from '$lib/utils/errorHandling';
import { key } from '$lib/utils/invalidationKey';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params, depends }) => {
	depends(key('admin/conversation/events'));

	const { api } = await parent();

	return {
		streamedEvents: tryCatchAsync(() =>
			api
				.ListEvents({
					params: { conversation_id: params.conversation_id },
					queries: { created_at: 'desc' }
				})
				.then((res) => res.records)
		)
	};
};
