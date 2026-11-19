import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { api } = await parent();

	return {
		streamedParticipation: tryCatchAsync(() => api.GetConversationsUserIsParticipatingIn())
	};
};
