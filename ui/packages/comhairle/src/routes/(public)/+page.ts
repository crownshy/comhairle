import type { PageLoad } from './$types';
import { getSort, PAGE_SIZE, getSearch } from './conversations/utils.js';
import { getPage, calcOffset } from '$lib/pagination';

export const load: PageLoad = async ({ parent, fetch, url }) => {
	const parentData = await parent();
	const sortBy = getSort(url);
	const page = getPage(url);
	const search = getSearch(url);
	const offset = calcOffset({ page, pageSize: PAGE_SIZE });
	const resp = await fetch(
		`/api/conversation?limit=${PAGE_SIZE}&offset=${offset}&sort=${sortBy}&keyword=${search}&is_complete=false`
	)
		.then((r) => r.json())
		.catch((e) => console.log(`error fetching conversations ${JSON.stringify(e, null, 2)}`));
	return {
		isCommunity: parentData.isCommunity,
		records: resp?.records ?? [],
		total: resp?.total ?? 0
	};
};
