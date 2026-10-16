import type { PageServerLoad } from './$types.js';

export const load: PageServerLoad = async ({ url }) => {
	let backTo = url.searchParams.get('backTo') ?? '/';
	return {
		backTo,
		inviteToken: url.searchParams.get('invite'),
		inviteEmail: url.searchParams.get('email')
	};
};
