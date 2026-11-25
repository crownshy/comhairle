import type { PageLoad } from './$types';
import { getPage, calcOffset } from '$lib/pagination';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import QueryParams, { type ConversationsParams } from '$lib/interfaces/QueryParams.svelte';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async ({ parent, url, depends }) => {
	depends(key('public/conversations'));
	const p = await parent();

	const CONVERSATION_PAGE_SIZE = 12;
	const filter = QueryParams.get<ConversationsParams>('filter', url);
	const search = QueryParams.get<ConversationsParams>('search', url);

	return {
		conversations: await tryCatchAsync(() =>
			p.api.ListConversations({
				queries: {
					limit: CONVERSATION_PAGE_SIZE,
					offset: calcOffset({ page: getPage(url), pageSize: CONVERSATION_PAGE_SIZE }),
					keyword: search === undefined ? undefined : String(search),
					// Each flag only goes when its filter is picked, so All lists finished ones too.
					is_live: filter === 'open' ? true : undefined,
					is_complete: filter === 'open' ? false : filter === 'ended' ? true : undefined
				}
			})
		),
		CONVERSATION_PAGE_SIZE
	};
};
