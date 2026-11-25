import { redirect } from '@sveltejs/kit';
import { HttpStatus } from '$lib/utils/constants';
import type { PageServerLoad } from './$types';

// Temporary (302) rather than permanent (301): the landing depends on the user's roles, and
// browsers would keep following a cached 301.
export const load: PageServerLoad = async ({ parent }) => {
	const { landingPath } = await parent();
	redirect(HttpStatus.Found, landingPath);
};
