import { createApiClient } from '@crownshy/api-client/client';
import type { LayoutLoad } from './$types';
import { browser } from '$app/environment';
import { serverApiBaseUrl } from '$lib/apiBaseUrl';

export const load: LayoutLoad = async ({ url, data }) => {
	const token = data.token;
	const user = data.user;
	const { isCommunity, themeName } = data;
	const api = browser
		? createApiClient(`${url.origin}/api`, token, 'client')
		: createApiClient(serverApiBaseUrl(url), token, 'server');

	try {
		const userRoles = await api.GetUserRoles();
		return { api, user, userRoles, isCommunity, themeName };
	} catch (e) {
		return { api, user, isCommunity, themeName };
	}
};
