import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { calcOffset, getPage } from '$lib/pagination';
import { conversationListQueries, searchFromUrl, statusFilterFromUrl } from './conversationFilters';

const PAGE_SIZE = 10;

export const load: PageLoad = async ({ parent, url, depends }) => {
	depends(key('admin/conversations'));
	const { api, user } = await parent();
	const filter = statusFilterFromUrl(url);
	const search = searchFromUrl(url);
	const userId = user?.id ?? '';

	const result = await tryCatchAsync(() =>
		api.GetPermittedConversations({
			queries: {
				...conversationListQueries(filter, userId),
				keyword: search.trim() || undefined,
				limit: PAGE_SIZE,
				offset: calcOffset({ page: getPage(url), pageSize: PAGE_SIZE })
			}
		})
	);

	return {
		filter,
		search,
		userId,
		pageSize: PAGE_SIZE,
		conversations: result.err === null ? result.ok : { records: [], total: 0 }
	};
};
