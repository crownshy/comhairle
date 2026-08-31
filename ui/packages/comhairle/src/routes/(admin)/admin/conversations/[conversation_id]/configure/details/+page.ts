import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { api, conversation } = await parent();

	if (!conversation.image) {
		return {
			media: null
		};
	}

	return {
		media: tryCatchAsync(() => api.GetMedia({ params: { media_id: conversation.image! } }))
	};
};
