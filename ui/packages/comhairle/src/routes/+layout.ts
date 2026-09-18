import { createApiClient } from '@crownshy/api-client/client';
import type { LayoutLoad } from './$types';
import { browser } from '$app/environment';

export const load: LayoutLoad = async ({ url, data }) => {
	const token = data.token;
	const user = data.user;
	const { isCommunity, themeName } = data;
	const api = createApiClient(url.origin + '/api', token, browser ? 'client' : 'server');

	try {
		const userSystemActions = await api.GetUserSystemActions();
		return { api, user, userSystemActions, isCommunity, themeName };
	} catch (e) {
		return { api, user, isCommunity, themeName };
	}
};
