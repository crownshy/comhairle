import { createApiClient } from '@crownshy/api-client/client';
import type { LayoutLoad } from './$types';
import { browser } from '$app/environment';
import { hasSuperAdminRole, loadUserActions } from '$lib/utils/permissions';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { key } from '$lib/utils/invalidationKey';

export const load: LayoutLoad = async ({ url, data, depends }) => {
	depends(key('user'));
	const token = data.token;
	const user = data.user;
	const { isCommunity, themeName } = data;
	const api = createApiClient(url.origin + '/api', token, browser ? 'client' : 'server');

	const [rolesResult, systemActions] = await Promise.all([
		tryCatchAsync(() => api.GetUserRoles()),
		loadUserActions(api, 'system')
	]);
	const userRoles = rolesResult.err === null ? rolesResult.ok : undefined;
	const isSuperAdmin = hasSuperAdminRole(userRoles);
	return { api, user, isSuperAdmin, userRoles, systemActions, isCommunity, themeName };
};
