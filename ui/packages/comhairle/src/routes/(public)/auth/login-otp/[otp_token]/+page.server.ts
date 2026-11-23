import type { PageServerLoad } from './$types.js';
import { backToFromUrl } from '$lib/utils/backTo';

export const load: PageServerLoad = async ({ params, url }) => {
	const backTo = backToFromUrl(url);

	const jwt = params.otp_token;

	return {
		backTo,
		jwt
	};
};
