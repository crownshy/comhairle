import type { PageServerLoad } from './$types.js';
import { fail, redirect } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms';
import { signupFormSchema, type SignupForm } from '$lib/profile';
import { zod } from 'sveltekit-superforms/adapters';

export const load: PageServerLoad = async () => {
	return {
		form: await superValidate(zod(signupFormSchema))
	};
};

export const actions = {
	default: async (evt) => {
		const form = await superValidate(evt, zod(signupFormSchema));
		if (!form.valid) {
			return fail(400, { form });
		}

		const { username, email, password } = form.data;
		const resp = await evt.fetch(`/api/auth/signup`, {
			method: 'POST',
			body: JSON.stringify({ username, email, password }),
			headers: {
				'Content-Type': 'application/json'
			}
		});
		console.log(resp);
		if (!resp.ok) {
			const body = await resp.json();
			if (body.err) {
				form.message = body.err;
			}
			return fail(resp.status, { form });
		}

		return redirect(302, '/');
	}
};
