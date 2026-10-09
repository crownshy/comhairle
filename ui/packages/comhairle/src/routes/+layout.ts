import { createApiClient } from '@crownshy/api-client/client';
import type { LayoutLoad } from './$types';
import { browser } from '$app/environment';
import { loadUserActions } from '$lib/utils/permissions';
import { tryCatchAsync } from '$lib/utils/errorHandling';

export const load: LayoutLoad = async ({ url, data, depends }) => {
	const token = data.token;
	const user = data.user;
	const { isCommunity, themeName } = data;
	const api = createApiClient(url.origin + '/api', token, browser ? 'client' : 'server');

	const [rolesResult, systemActions] = await Promise.all([
		tryCatchAsync(() => api.GetUserRoles()),
		loadUserActions(api, 'system')
	]);
	const userRoles = rolesResult.err === null ? rolesResult.ok : undefined;
	const isSuperAdmin = userRoles.some(
		(role) => role.resource === 'Site' && role.roles.includes('SuperAdmin')
	);
	return { api, user, isSuperAdmin, userRoles, systemActions, isCommunity, themeName };
};
