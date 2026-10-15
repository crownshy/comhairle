import { error, redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { calcOffset, calcPageCount, getPage, setPage } from '$lib/pagination';
import { conversationListQueries, searchFromUrl, statusFilterFromUrl } from './conversationFilters';

const PAGE_SIZE = 10;

export const load: PageLoad = async ({ parent, url, depends }) => {
	depends(key('admin/conversations'));
	const { api, user } = await parent();
	const filter = statusFilterFromUrl(url);
	const search = searchFromUrl(url);
	const page = getPage(url);

	const result = await tryCatchAsync(() =>
		api.GetPermittedConversations({
			queries: {
				...conversationListQueries(filter, user?.id),
				keyword: search.trim() || undefined,
				limit: PAGE_SIZE,
				offset: calcOffset({ page, pageSize: PAGE_SIZE })
			}
		})
	);
	if (result.err !== null) error(502, 'Could not load your conversations');

	const conversations = result.ok;
	const lastPage = calcPageCount({ pageSize: PAGE_SIZE, total: conversations.total });
	if (page > lastPage) {
		const lastPageUrl = setPage(url, lastPage);
		redirect(307, `${lastPageUrl.pathname}${lastPageUrl.search}`);
	}

	return { filter, search, pageSize: PAGE_SIZE, conversations };
};
