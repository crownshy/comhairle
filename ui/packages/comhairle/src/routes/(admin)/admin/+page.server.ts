import { redirect } from '@sveltejs/kit';
import { resolve } from '$app/paths';
import { HttpStatus } from '$lib/utils/constants';
import type { PageServerLoad } from './$types';

// Temporary (302) rather than permanent (301): /admin may get its own workspace page later,
// and browsers would keep following a cached 301.
export const load: PageServerLoad = () => {
	redirect(HttpStatus.Found, resolve('/(admin)/admin/conversations'));
};
