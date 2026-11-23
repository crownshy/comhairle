import type { PageServerLoad } from './$types.js';
import { backToFromUrl } from '$lib/utils/backTo';

export const load: PageServerLoad = async ({ url }) => {
	const backTo = backToFromUrl(url);
	return {
		backTo
	};
};
