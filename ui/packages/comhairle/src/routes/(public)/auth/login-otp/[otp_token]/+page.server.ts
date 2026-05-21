import type { PageServerLoad } from './$types.js';

export const load: PageServerLoad = async ({ params, url }) => {
	const backTo = url.searchParams.get('backTo') ?? '/';

	const jwt = params.otp_token;

	return {
		backTo,
		jwt
	};
};
