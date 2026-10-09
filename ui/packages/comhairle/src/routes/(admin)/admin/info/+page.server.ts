import { redirect } from '@sveltejs/kit';
import { resolve } from '$app/paths';
import type { PageServerLoad } from './$types';
import { HttpStatus } from '$lib/utils/constants';

// The Handbook opens on its welcome page.
export const load: PageServerLoad = () => {
	redirect(HttpStatus.Found, resolve('/admin/info/welcome'));
};
