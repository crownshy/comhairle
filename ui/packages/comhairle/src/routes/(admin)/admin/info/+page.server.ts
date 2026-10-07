import { redirect } from '@sveltejs/kit';
import { resolve } from '$app/paths';
import type { PageServerLoad } from './$types';
import { HttpStatus } from '$lib/utils/constants';

// The admin guide opens on the welcome page.
export const load: PageServerLoad = () => {
	redirect(HttpStatus.Found, resolve('/admin/info/welcome'));
};
