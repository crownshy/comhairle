import type { LayoutServerLoad } from './$types.js';

export const load: LayoutServerLoad = async (event) => {
	const tk = event.cookies.get('auth-token');
	if (!tk) {
		return {
			user: null
		};
	}
	
	const resp = await event.fetch(`/api/auth/current_user`, {
		method: 'GET',
		headers: { Accept: 'application/json' }
	});

	if (!resp.ok) {
		return { user: null };
	}
	const body = await resp.json();
	if (!body.id) return { user: null };

	// console.log("Returning with token ", tk)
	return { user: body, token:tk };
};
