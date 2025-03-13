import { getSort, PAGE_SIZE } from './utils.js';
import { getPage, calcOffset } from '$lib/pagination';

export async function load(event) {
	// const offset = event.url.searchParams.get('offset') || '0';
	const sortBy = getSort(event.url);
	const page = getPage(event.url);
	const offset = calcOffset({ page, pageSize: PAGE_SIZE });
	const resp = await event
		.fetch(`/api/conversation?limit=${PAGE_SIZE}&offset=${offset}&sort=${sortBy}`)
		.then((r) => r.json());
	console.log('conversations ', resp);
	return { ...resp };
}
