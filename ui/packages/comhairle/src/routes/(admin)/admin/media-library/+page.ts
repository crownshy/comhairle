import { tryCatchAsync } from '$lib/utils/errorHandling';
import { key } from '$lib/utils/invalidationKey';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, depends }) => {
	depends(key('admin/media-library/media'));

	const p = await parent();

	return {
		media: await tryCatchAsync(() => p.api.ListMedia().then((res) => res.records))
	};
};
