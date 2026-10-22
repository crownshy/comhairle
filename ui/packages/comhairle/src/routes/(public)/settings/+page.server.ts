import { resolve } from '$app/paths';
import { HttpStatus } from '$lib/utils/constants';
import { redirect } from '@sveltejs/kit';

export const load = () => {
	redirect(HttpStatus.PermanentRedirect, resolve('/(public)/settings/profile'));
};
