import { fail, redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import { page } from '$app/state';

export const load:PageServerLoad = ({url})=>{
	let backTo = url.searchParams.get("backTo") ?? "/"
	return {backTo}
}

export const actions = {
	default: async (evt) => {
		const resp = await evt.fetch('/api/auth/signup_annon', {
			method: 'POST',
			headers: {}
		});

		if (!resp.ok) {
			//TODO: proper error message here, probably need form
			return fail(400, {});
		}

		let backTo = evt.url.searchParams.get("backTo") ?? "/"
		redirect(302, `/auth/anonymous-signup/code?backTo=${backTo}`);
	}
};
