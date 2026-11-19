import type { PageLoad } from './$types';
import { getPage, calcOffset } from '$lib/pagination';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import QueryParams from '$lib/interfaces/QueryParams.svelte';

export const load: PageLoad = async ({ parent, url }) => {
	const p = await parent();

	const CONVERSATION_PAGE_SIZE = 6;

	return {
		conversations: await tryCatchAsync(() =>
			p.api.ListConversations({
				queries: {
					limit: CONVERSATION_PAGE_SIZE,
					offset: calcOffset({ page: getPage(url), pageSize: CONVERSATION_PAGE_SIZE }),
					keyword: QueryParams.get('search', url),
					is_complete: false
				}
			})
		),
		CONVERSATION_PAGE_SIZE
	};
};
