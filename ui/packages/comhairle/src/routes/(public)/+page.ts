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
	// TODO: the API has no "featured" flag yet, so the newest three open conversations stand in.
	// Fetched on their own so the carousel stays put when filters, search or page change.
	const featuredRequest = fetch(
		`/api/conversation?limit=3&offset=0&sort=created_at+desc${STATUS_FILTER_QUERY.open}`
	)
		.then((r) => r.json())
		.catch((e) =>
			console.log(`error fetching featured conversations ${JSON.stringify(e, null, 2)}`)
		);

	const resp = await fetch(
		`/api/conversation?limit=${PAGE_SIZE}&offset=${offset}&sort=${sortBy}&keyword=${search}${STATUS_FILTER_QUERY[status]}`
	)
		.then((r) => r.json())
		.catch((e) => console.log(`error fetching conversations ${JSON.stringify(e, null, 2)}`));

	const featured = await featuredRequest;

	return {
		featured: featured?.records ?? [],
		isCommunity: parentData.isCommunity,
		status,
		records: resp?.records ?? [],
		total: resp?.total ?? 0
	};
};
