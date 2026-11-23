import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { calcOffset, getPage } from '$lib/pagination';
import { conversationListQueries, statusFilterFromUrl } from './conversationFilters';
import QueryParams, { type ConversationsParams } from '$lib/interfaces/QueryParams.svelte';

const PAGE_SIZE = 10;

export const load: PageLoad = async ({ parent, url, depends }) => {
	depends(key('admin/conversations'));

	const { api, user } = await parent();
	const filter = statusFilterFromUrl(url);
	const page = getPage(url);

	return {
		conversations: await tryCatchAsync(() =>
			api.GetPermittedConversations({
				queries: {
					...conversationListQueries(filter, user?.id),
					keyword: QueryParams.get<ConversationsParams>('search', url),
					limit: PAGE_SIZE,
					offset: calcOffset({ page, pageSize: PAGE_SIZE })
				}
			})
		),
		PAGE_SIZE
	};
};
