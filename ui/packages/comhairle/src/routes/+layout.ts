import { createApiClient } from '@crown-shy/api-client/client';
import type { LayoutLoad } from './$types';
import { browser } from '$app/environment';

export const load: LayoutLoad = async ({ url, data }) => {
	const token = data.token;
	const user = data.user;
	const api = createApiClient(url.origin + "/api", token, browser ? "client" : "server");	
	
	try {
		const userRoles = await api.GetUserRoles();
		return { api, user, userRoles, isCommunity, themeName };
	}
	catch (e) {
		return { api, user, isCommunity, themeName };
	}

};
