import type { LayoutServerLoad } from './$types.js';
import { env } from '$env/dynamic/public';
import { serverApiBaseUrl } from '$lib/apiBaseUrl';

export const load: LayoutServerLoad = async (event) => {
	event.depends('user');

	const tk = event.cookies.get('auth-token');
	const common = {
		isCommunity: env.PUBLIC_IS_COMMUNITY === 'true'
	};

	if (!tk) {
		return {
			user: null,
			...common
		};
	}

	const resp = await event.fetch(`${serverApiBaseUrl(event.url)}/auth/current_user`, {
		method: 'GET',
		// An absolute internal URL is cross-origin as far as `event.fetch` is concerned, so it
		// stops forwarding the request's cookies and the token has to be passed by hand.
		headers: { Accept: 'application/json', Cookie: `auth-token=${tk}` }
	});

	if (!resp.ok) {
		return { user: null, ...common };
	}
	const body = await resp.json();
	if (!body.id) return { user: null, ...common };

	// console.log("Returning with token ", tk)
	return { user: body, token: tk, ...common };
};
