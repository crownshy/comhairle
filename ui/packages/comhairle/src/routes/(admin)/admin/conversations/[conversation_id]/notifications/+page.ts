import type { PageLoad } from './$types';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async ({ parent, params, depends }) => {
	depends(key('conversation/notifications/recipients'));

	const { api } = await parent();

	return {
		streamedRecipients: tryCatchAsync(() =>
			api.GetNotificationRecipients({
				params: { conversation_id: params.conversation_id }
			})
		)
	};
};
