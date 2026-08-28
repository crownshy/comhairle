import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';
import { notifications } from '$lib/notifications.svelte';

export const load: PageLoad = async ({ parent }) => {
	const { api, conversation } = await parent();

	if (!conversation.image) {
		notifications.addFlash({
			message: 'No media url found',
			priority: 'ERROR'
		});
		return {
			media: null
		};
	}

	return {
		media: tryCatchAsync(() => api.GetMedia({ params: { media_id: conversation.image! } }))
	};
};
