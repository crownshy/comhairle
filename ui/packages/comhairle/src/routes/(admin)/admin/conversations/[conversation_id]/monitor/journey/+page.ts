import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async ({ parent, params, depends }) => {
	depends(key('admin/conversation/monitor'));

	// TODO: data for journey page

	return {};
};
