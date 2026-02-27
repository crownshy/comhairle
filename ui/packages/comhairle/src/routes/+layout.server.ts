import type { LayoutServerLoad } from './$types.js';
import { env } from '$env/dynamic/public';

export const load: LayoutServerLoad = async (event) => {
	const tk = event.cookies.get('auth-token');
	const common = {
		themeName: env.PUBLIC_THEME ?? 'comhairle',
		isCommunity: env.PUBLIC_IS_COMMUNITY === 'true'
	};

	if (!tk) {
		return {
			user: null,
			...common
		};
	}

	const resp = await event.fetch(`/api/auth/current_user`, {
		method: 'GET',
		headers: { Accept: 'application/json' }
	});

	if (!resp.ok) {
		return { user: null, ...common };
	}
	const body = await resp.json();
	if (!body.id) return { user: null, ...common };

	// console.log("Returning with token ", tk)
	return { user: body, token: tk, ...common };
};
