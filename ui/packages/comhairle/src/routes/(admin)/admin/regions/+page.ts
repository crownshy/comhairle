import { error } from '@sveltejs/kit';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { getPage, calcOffset } from '$lib/pagination';
import type { PageLoad } from './$types';

const PAGE_SIZE = 100;

export const load: PageLoad = async ({ parent, depends, url }) => {
	depends('admin:regions');
	const { api, userRoles } = await parent();
	if (!userRoles?.some((role) => role.resource === 'Site' && role.roles.includes('SuperAdmin'))) {
		error(403, 'Super-admin access required');
	}
	const page = getPage(url);
	const name = url.searchParams.get('name') ?? undefined;
	const tag = url.searchParams.get('tag') ?? undefined;
	const offset = calcOffset({ page, pageSize: PAGE_SIZE });
	const response = await tryCatchAsync(() =>
		api.ListRegionAreas({ queries: { limit: PAGE_SIZE, offset, name, tag } })
	);
	if (response.err !== null) error(503, 'Could not load geographic areas');
	return { areas: response.ok, pageSize: PAGE_SIZE };
};
