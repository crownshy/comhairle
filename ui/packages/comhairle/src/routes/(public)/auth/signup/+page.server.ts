import type { PageServerLoad } from './$types.js';
import { normaliseBackTo } from '$lib/utils/backTo';

export const load: PageServerLoad = async ({ url }) => {
	const backTo = normaliseBackTo(url.searchParams.get('backTo'));
	return {
		backTo
	};
};
