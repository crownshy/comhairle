import type { PageLoad } from './$types';
import { PAGE_SIZE, getSearch } from './conversations/utils.js';
import { getPage, calcOffset } from '$lib/pagination';
import { tryCatchAsync } from '$lib/utils/errorHandling';

export const load: PageLoad = async ({ parent, url }) => {
	const p = await parent();

	return {
		conversations: await tryCatchAsync(() =>
			p.api.ListConversations({
				queries: {
					limit: PAGE_SIZE,
					offset: calcOffset({ page: getPage(url), pageSize: PAGE_SIZE }),
					keyword: getSearch(url),
					is_complete: false
				}
			})
		)
	};
};
