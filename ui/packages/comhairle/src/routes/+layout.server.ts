import type { LayoutServerLoad } from './$types.js';
import { env } from '$env/dynamic/public';
import { resolveThemeName } from '$lib/types/theme';
import { key } from '$lib/utils/invalidationKey';

export const load: LayoutServerLoad = async (event) => {
	event.depends(key('user'));

	const common = {
		themeName: resolveThemeName(env.PUBLIC_THEME),
		isCommunity: env.PUBLIC_IS_COMMUNITY === 'true'
	};

	// Keep extraction of `auth-token` cookie after `/api/auth/current_user`
	// request.
	//
	// This ensures `tk` passed down to `+layout.ts` (where `api` client is
	// constructed) always contains a fresh `auth-token`, which may have been
	// updated as part of the refresh flow in `handleFetch` (see
	// `hooks.server.ts`).
	const tk = event.cookies.get('kc-access-token');

	let body: { id?: string } | undefined;
	try {
		const resp = await event.fetch(`/api/auth/current_user`, {
			method: 'GET',
			headers: { Accept: 'application/json' }
		});

		if (!tk || !resp.ok) {
			return { user: null, ...common };
		}
		body = await resp.json();
	} catch (e) {
		// Network-level failure, including a connection reset mid-body-read
		// (resp.ok can be true before the stream errors out) — degrade to
		// anonymous instead of letting this throw crash every route that
		// shares this root layout. Still pass the cookie through: the
		// session itself is fine, only this one fetch failed, so child loads
		// should keep authenticating rather than silently going anonymous too.
		return { user: null, token: tk, ...common };
	}

	if (!body?.id) return { user: null, ...common };

	// console.log("Returning with token ", tk)
	return { user: body, token: tk, ...common };
};
