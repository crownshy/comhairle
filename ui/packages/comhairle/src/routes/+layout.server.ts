import type { LayoutServerLoad } from './$types.js';
import { env } from '$env/dynamic/public';
import { resolveThemeName } from '$lib/types/theme';

export const load: LayoutServerLoad = async (event) => {
	event.depends('user');

	const common = {
		themeName: resolveThemeName(env.PUBLIC_THEME),
		isCommunity: env.PUBLIC_IS_COMMUNITY === 'true'
	};

	const resp = await event.fetch(`/api/auth/current_user`, {
		method: 'GET',
		headers: { Accept: 'application/json' }
	});

	// Keep extraction of `auth-token` cookie after `/api/auth/current_user`
	// request.
	//
	// This ensures `tk` passed down to `+layout.ts` (where `api` client is
	// constructed) always contains a fresh `auth-token`, which may have been
	// updated as part of the refresh flow in `handleFetch` (see
	// `hooks.server.ts`).
	const tk = event.cookies.get('auth-token');

	if (!tk || !resp.ok) {
		return { user: null, ...common };
	}
	const body = await resp.json();
	if (!body.id) return { user: null, ...common };

	// console.log("Returning with token ", tk)
	return { user: body, token: tk, ...common };
};
