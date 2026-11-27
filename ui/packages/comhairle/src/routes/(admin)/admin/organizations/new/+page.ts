import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const p = await parent();

	const regions = await tryCatchAsync(() =>
		p.api.ListRegions({ queries: { limit: 500 } }).then((response) => response.records)
	);

	return {
		regions
	};
};
