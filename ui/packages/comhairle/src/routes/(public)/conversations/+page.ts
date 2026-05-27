import type { PageLoad } from './$types';

import { getSort, PAGE_SIZE, getSearch } from './utils.js';
import { getPage, calcOffset } from '$lib/pagination';

export const load: PageLoad = async (event) => {
	const { user, api } = await event.parent();

	const sortBy = getSort(event.url);
	const page = getPage(event.url);
	const search = getSearch(event.url);
	const offset = calcOffset({ page, pageSize: PAGE_SIZE });
	const resp = await event
		.fetch(
			`/api/conversation?limit=${PAGE_SIZE}&offset=${offset}&sort=${sortBy}&keyword=${search}&is_complete=false`
		)
		.then((r) => r.json())
		.catch((e) => console.log(`error fetching conversations ${JSON.stringify(e, null, 2)}`));

	if (user) {
		try {
			const participated = await api.GetConversationsUserIsParticipatingIn();
			const publicIds = new Set((resp?.records ?? []).map((c: { id: string }) => c.id));
			const extra = participated.filter((c) => !publicIds.has(c.id));
			if (extra.length > 0) {
				return {
					...resp,
					records: [...extra, ...(resp?.records ?? [])],
					total: (resp?.total ?? 0) + extra.length
				};
			}
		} catch (e) {
			console.log(`error fetching participated conversations ${JSON.stringify(e, null, 2)}`);
		}
	}

	return { ...resp };
};
