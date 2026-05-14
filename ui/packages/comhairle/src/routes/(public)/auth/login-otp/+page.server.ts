import type { PageServerLoad } from './$types.js';

export const load: PageServerLoad = async ({ url }) => {
	const backTo = url.searchParams.get('backTo') ?? '/';

	return {
		backTo
	};
};
