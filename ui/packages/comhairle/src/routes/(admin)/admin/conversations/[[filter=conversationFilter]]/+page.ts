import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import {
	filterConversations,
	isConversationFilterSegment,
	type ConversationStatusFilter
} from '$lib/utils/conversationFilters';

// Filters the lists the admin layout already loads for the sidebar, so a chip costs no request.
export const load: PageLoad = async ({ parent, params, depends }) => {
	depends(key('admin/conversations'));
	const { ownedConversations, permittedConversations } = await parent();
	const filter: ConversationStatusFilter =
		params.filter && isConversationFilterSegment(params.filter) ? params.filter : 'all';

	return {
		filter,
		conversations: filterConversations(
			ownedConversations?.records ?? [],
			permittedConversations?.records ?? [],
			filter
		)
	};
};
