import { redirect } from '@sveltejs/kit';
import { HttpStatus } from '$lib/utils/constants';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { adminLandingPath } from '../adminAccess';
import type { PageServerLoad } from './$types';

// Temporary (302) rather than permanent (301): the landing depends on the user's roles, and
// browsers would keep following a cached 301.
export const load: PageServerLoad = async ({ locals }) => {
	const roles = await tryCatchAsync(() => locals.api.GetUserRoles());
	redirect(HttpStatus.Found, adminLandingPath(roles.err === null ? roles.ok : null));
};
