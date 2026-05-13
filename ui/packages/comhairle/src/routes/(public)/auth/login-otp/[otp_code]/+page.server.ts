import type { PageServerLoad } from './$types.js';

export const load: PageServerLoad = async ({ params, url }) => {
	const backTo = url.searchParams.get('backTo') ?? '/';
	const email = url.searchParams.get('email');

	return {
		backTo,
		email,
		otpCode: params.otp_code
	};
};
