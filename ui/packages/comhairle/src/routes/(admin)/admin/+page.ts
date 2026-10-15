import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import { tryCatchAsync } from '$lib/utils/errorHandling';

const LATEST_CONVERSATION_COUNT = 5;

export const load: PageLoad = async ({ parent, depends }) => {
	depends(key('admin/conversations'));
	const { api } = await parent();

	// The permitted list includes your own conversations and comes newest created first.
	const latest = await tryCatchAsync(() =>
		api.GetPermittedConversations({ queries: { limit: LATEST_CONVERSATION_COUNT } })
	);

	return { latestConversations: latest.err === null ? latest.ok.records : null };
};
