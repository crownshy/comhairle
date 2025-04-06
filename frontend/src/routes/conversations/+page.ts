import type {PageLoad} from "./$types"

import { getSort, PAGE_SIZE, getSearch } from './utils.js';
import { getPage, calcOffset } from '$lib/pagination';

export const load: PageLoad = async (event)=> {
	// const offset = event.url.searchParams.get('offset') || '0';
	const sortBy = getSort(event.url);
	const page = getPage(event.url);
	const search = getSearch(event.url);
	const offset = calcOffset({ page, pageSize: PAGE_SIZE });
	const resp = await event
		.fetch(`/api/conversation?limit=${PAGE_SIZE}&offset=${offset}&sort=${sortBy}&title=${search}`)
		.then((r) => r.json());
	return { ...resp };
}
