import { error } from '@sveltejs/kit';
import { loadRoleManagement } from '$lib/components/permissions/roleAssignments';
import { HttpStatus } from '$lib/utils/constants';
import { key } from '$lib/utils/invalidationKey';
import { SYSTEM_RESOURCE_ID } from '$lib/utils/permissions';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ parent, locals, depends }) => {
	const { isSuperAdmin } = await parent();
	if (!isSuperAdmin) error(HttpStatus.Forbidden, 'Super-admin access required');
	depends(key('admin/system/permissions'));
	const roleManagement = await loadRoleManagement(locals.api, 'system', SYSTEM_RESOURCE_ID);
	return { roleManagement };
};
