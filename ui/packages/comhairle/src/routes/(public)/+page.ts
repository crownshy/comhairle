import type { PageLoad } from './$types';
import {
	getSort,
	PAGE_SIZE,
	getSearch,
	getStatusFilter,
	STATUS_FILTER_QUERY
} from './conversations/utils.js';
import { getPage, calcOffset } from '$lib/pagination';

export const load: PageLoad = async ({ parent, fetch, url }) => {
	const parentData = await parent();
	const sortBy = getSort(url);
	const page = getPage(url);
	const search = getSearch(url);
	const status = getStatusFilter(url);
	const offset = calcOffset({ page, pageSize: PAGE_SIZE });
	const resp = await fetch(
		`/api/conversation?limit=${PAGE_SIZE}&offset=${offset}&sort=${sortBy}&keyword=${search}${STATUS_FILTER_QUERY[status]}`
	)
		.then((r) => r.json())
		.catch((e) => console.log(`error fetching conversations ${JSON.stringify(e, null, 2)}`));

	return {
		isCommunity: parentData.isCommunity,
		status,
		records: resp?.records ?? [],
		total: resp?.total ?? 0
	};
};
