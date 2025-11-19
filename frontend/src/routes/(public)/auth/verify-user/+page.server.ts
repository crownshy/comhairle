import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url }) => {
	const token = url.searchParams.get('token') ?? '';

	return {
		token
	};
};
