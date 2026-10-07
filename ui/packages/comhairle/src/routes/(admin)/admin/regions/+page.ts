import { error } from '@sveltejs/kit';
import { HttpStatus } from '$lib/utils/constants';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { key } from '$lib/utils/invalidationKey';
import { getPage, calcOffset } from '$lib/pagination';
import type { PageLoad } from './$types';

const PAGE_SIZE = 100;

export const load: PageLoad = async ({ parent, depends, url }) => {
	depends(key('admin/regions'));
	const { api } = await parent();
	const page = getPage(url);
	const name = url.searchParams.get('name') ?? undefined;
	const tag = url.searchParams.get('tag') ?? undefined;
	const offset = calcOffset({ page, pageSize: PAGE_SIZE });
	const response = await tryCatchAsync(() =>
		api.ListRegionAreas({ queries: { limit: PAGE_SIZE, offset, name, tag } })
	);
	if (response.err !== null)
		error(HttpStatus.InternalServerError, 'Could not load geographic areas');
	return { areas: response.ok, pageSize: PAGE_SIZE };
};
