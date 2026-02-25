import type { PageLoad } from './$types';

export const load: PageLoad = async ({ url }) => {
	const token = url.searchParams.get('token') ?? '';

	return {
		token
	};
};
