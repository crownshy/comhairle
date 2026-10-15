import type { PageLoad } from './$types';
import type { createApiClient } from '@crownshy/api-client/client';
import { key } from '$lib/utils/invalidationKey';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { conversationListQueries } from './conversations/conversationFilters';

type ConversationListApi = Pick<ReturnType<typeof createApiClient>, 'GetPermittedConversations'>;
type ListQueries = Parameters<ConversationListApi['GetPermittedConversations']>[0];

/** A page of the conversations you can see, or null when the request fails. */
async function permittedConversations(api: ConversationListApi, queries: ListQueries) {
	const result = await tryCatchAsync(() => api.GetPermittedConversations(queries));
	return result.err === null ? result.ok : null;
}

// The permitted list includes your own conversations and comes newest created first, so
// `limit` means "the latest" and `total` is the count for each tile.
export const load: PageLoad = async ({ parent, depends }) => {
	depends(key('admin/conversations'));
	const { api, user } = await parent();

	const [latest, live, drafts, completed, shared] = await Promise.all([
		permittedConversations(api, { queries: { limit: 1 } }),
		permittedConversations(api, {
			queries: { ...conversationListQueries('live', undefined), limit: 4 }
		}),
		permittedConversations(api, {
			queries: { ...conversationListQueries('draft', undefined), limit: 3 }
		}),
		permittedConversations(api, {
			queries: { ...conversationListQueries('completed', undefined), limit: 1 }
		}),
		permittedConversations(api, {
			queries: { ...conversationListQueries('shared', user?.id), limit: 1 }
		})
	]);

	return {
		pickUp: latest?.records[0] ?? null,
		live,
		drafts,
		completedCount: completed?.total ?? null,
		sharedCount: shared?.total ?? null
	};
};
