import type { PageLoad } from './$types';
import { getPage, calcOffset } from '$lib/pagination';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import QueryParams, { type ConversationsParams } from '$lib/interfaces/QueryParams.svelte';
import { key } from '$lib/utils/invalidationKey';
import Conversation, { type ConversationStatus } from '$lib/interfaces/Conversation';

const FILTER_STATUS = {
	open: 'live',
	ended: 'completed'
} as const satisfies Record<ConversationsParams['filter'], ConversationStatus>;

export const load: PageLoad = async ({ parent, url, depends }) => {
	depends(key('public/conversations'));
	const p = await parent();

	const CONVERSATION_PAGE_SIZE = 12;
	const filter = QueryParams.get<ConversationsParams>('filter', url) as
		ConversationsParams['filter'] | undefined;
	const search = QueryParams.get<ConversationsParams>('search', url);

	return {
		conversations: await tryCatchAsync(() =>
			p.api.ListConversations({
				queries: {
					...(filter ? Conversation.getParams(FILTER_STATUS[filter]) : {}),
					keyword: search === undefined ? undefined : String(search),
					limit: CONVERSATION_PAGE_SIZE,
					offset: calcOffset({ page: getPage(url), pageSize: CONVERSATION_PAGE_SIZE })
				}
			})
		),
		CONVERSATION_PAGE_SIZE
	};
};
