import type { PageLoad } from './$types';
import { tryCatchAsync } from '$lib/utils/errorHandling';

export const load: PageLoad = async ({ parent, params }) => {
	const { api } = await parent();

	return {
		streamedRecipients: tryCatchAsync(() =>
			api.GetNotificationRecipients({
				params: { conversation_id: params.conversation_id }
			})
		)
	};
};
