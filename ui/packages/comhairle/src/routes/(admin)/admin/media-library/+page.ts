import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async ({ parent, depends }) => {
	depends(key('admin/media'));

	const { api } = await parent();

	// TODO: Add in limit & pagination
	return {
		streamedMedia: tryCatchAsync(() => api.ListMedia().then((res) => res.records))
	};
};
