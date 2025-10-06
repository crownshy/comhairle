import { createApiClient } from '$lib/api/client';
import type { LayoutLoad } from './$types';
import { browser } from '$app/environment';

export const load: LayoutLoad = async ({ url, data }) => {
	let token = data.token;
	let user = data.user;
	const api = createApiClient(url.origin + "/api", token, browser ? "client" : "server");
	const userRoles = await api.GetUserRoles();

	return { api, user, userRoles };
};
