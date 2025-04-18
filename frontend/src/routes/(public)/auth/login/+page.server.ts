import type { PageServerLoad } from './$types.js';
import { fail, redirect } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms';
import { loginFormSchema } from '$lib/profile';
import { zod } from 'sveltekit-superforms/adapters';
import { page } from '$app/state';

export const load: PageServerLoad = async ({url}) => {
	let backTo = url.searchParams.get("backTo") ?? "/"
	return {
		form: await superValidate(zod(loginFormSchema)),
		backTo
	};
};

export const actions = {
	default: async (evt) => {
		const form = await superValidate(evt, zod(loginFormSchema));
		if (!form.valid) {
			return fail(400, {
				form
			});
		}
		const resp = await evt.fetch('/api/auth/login', {
			method: 'POST',
			body: JSON.stringify(form.data),
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (!resp.ok) {
			const body = await resp.json();
			if (body.err) {
				form.message = body.err;
			}
			return fail(400, {
				form
			});
		}

		let backTo = evt.url.searchParams.get('backTo');
		let redirectUrl = backTo ?? "/"
		return redirect(302, redirectUrl);
	}
};
