import { fail, redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';

export const actions = {
	default: async (evt) => {
		const resp = await evt.fetch('/api/auth/signup_annon', {
			method: 'POST',
			headers: {}
		});

		if (!resp.ok) {
			console.log(resp);
			//TODO: proper error message here, probably need form
			return fail(400, {});
		}

		redirect(301, '/auth/anonymous-signup/code');
	}
};
