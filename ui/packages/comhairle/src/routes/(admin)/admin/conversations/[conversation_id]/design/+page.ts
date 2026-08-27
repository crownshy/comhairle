import type { PageLoad } from './$types';
import { tryCatchAsync } from '$lib/utils/errorHandling';

export const load: PageLoad = async ({ parent }) => {
	const { api, conversation } = await parent();
	const { image } = conversation;

	if (!image) {
		return;
	}

	const result = await tryCatchAsync(() => api.GetMedia({ params: { media_id: image } }));
	if (result.err !== null) {
		return;
	}

	return {
		media: result.ok
	};
};
