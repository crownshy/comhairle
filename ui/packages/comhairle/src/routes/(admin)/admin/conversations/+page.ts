import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import { filterConversations, statusFilterFromUrl } from './conversationFilters';

// Filters the lists the admin layout already loads for the sidebar, so a chip costs no request.
export const load: PageLoad = async ({ parent, url, depends }) => {
	depends(key('admin/conversations'));
	const { ownedConversations, permittedConversations } = await parent();
	const filter = statusFilterFromUrl(url);
	const sharedRecords = permittedConversations?.records ?? [];

	return {
		filter,
		conversations: filterConversations(
			ownedConversations?.records ?? [],
			sharedRecords,
			filter
		),
		sharedConversationIds: new Set(sharedRecords.map((conversation) => conversation.id))
	};
};
