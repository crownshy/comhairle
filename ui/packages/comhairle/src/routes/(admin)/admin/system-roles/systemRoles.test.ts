import { describe, expect, it, vi } from 'vitest';
import { createApiClient } from '@crownshy/api-client/client';
import { load } from './+page.server';
import { loadRoleManagement } from '$lib/components/permissions/roleAssignments';
import { key } from '$lib/utils/invalidationKey';
import { SYSTEM_RESOURCE_ID } from '$lib/utils/permissions';

vi.mock('$lib/components/permissions/roleAssignments', () => ({
	loadRoleManagement: vi.fn()
}));

function loadEvent(isSuperAdmin: boolean): Parameters<typeof load>[0] {
	return {
		parent: vi.fn().mockResolvedValue({ isSuperAdmin }),
		locals: { api: createApiClient('http://localhost/api', undefined, 'server') },
		depends: vi.fn()
	} as Parameters<typeof load>[0];
}

describe('system role management access', () => {
	it('rejects non-super-admins before loading assignments', async () => {
		vi.mocked(loadRoleManagement).mockClear();
		await expect(load(loadEvent(false))).rejects.toMatchObject({ status: 403 });
		expect(loadRoleManagement).not.toHaveBeenCalled();
	});

	it('loads system assignments for super-admins and declares its refresh key', async () => {
		const event = loadEvent(true);
		const roleManagement = {
			ok: { roles: ['super_admin'], assignments: [], users: [], organizations: [] },
			err: null
		};
		vi.mocked(loadRoleManagement).mockResolvedValue(roleManagement);

		await expect(load(event)).resolves.toEqual({ roleManagement });
		expect(loadRoleManagement).toHaveBeenCalledWith(
			event.locals.api,
			'system',
			SYSTEM_RESOURCE_ID
		);
		expect(event.depends).toHaveBeenCalledWith(key('admin/system/permissions'));
	});
});
