import type { PageServerLoad } from './$types.js';
import { fail, redirect } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms';
import { loginFormSchema } from '$lib/profile';
import { zod } from 'sveltekit-superforms/adapters';

export const load: PageServerLoad = async () => {
	return {
		form: await superValidate(zod(loginFormSchema))
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

		return redirect(302, '/');
	}
};
