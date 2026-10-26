import type { PageServerLoad } from './$types.js';
import { normaliseBackTo } from '$lib/utils/backTo';

export const load: PageServerLoad = async ({ params, url }) => {
	const backTo = normaliseBackTo(url.searchParams.get('backTo'));

	const jwt = params.otp_token;

	return {
		backTo,
		jwt
	};
};
