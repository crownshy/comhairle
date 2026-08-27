import type { LayoutServerLoad } from './$types.js';
import { env } from '$env/dynamic/public';
import { resolveThemeName } from '$lib/types/theme';
import { key } from '$lib/utils/invalidationKey';

export const load: LayoutServerLoad = async (event) => {
	event.depends(key('user'));

	const token = event.cookies.get('auth-token');
	const common = {
		themeName: resolveThemeName(env.PUBLIC_THEME),
		isCommunity: env.PUBLIC_IS_COMMUNITY === 'true'
	};

	if (!token) {
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

	return { user: body, token, ...common };
};
