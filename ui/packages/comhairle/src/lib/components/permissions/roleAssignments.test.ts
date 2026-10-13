import { describe, expect, it, vi } from 'vitest';
import { loadRoleManagement, roleRecipients } from './roleAssignments';
import { SYSTEM_RESOURCE_ID } from '$lib/utils/permissions';

type RoleManagementApi = Parameters<typeof loadRoleManagement>[0];

function createRoleManagementApi() {
	return {
		GetPermissionRoles: vi
			.fn<RoleManagementApi['GetPermissionRoles']>()
			.mockResolvedValue(['observer']),
		ListResourcePermissions: vi
			.fn<RoleManagementApi['ListResourcePermissions']>()
			.mockResolvedValue({ records: [], total: 0 }),
		ListUsersWithPermission: vi
			.fn<RoleManagementApi['ListUsersWithPermission']>()
			.mockResolvedValue([]),
		ListOrganizations: vi
			.fn<RoleManagementApi['ListOrganizations']>()
			.mockResolvedValue({ records: [], total: 0 })
	};
}

describe('loadRoleManagement', () => {
	it('loads system roles and assignments for the canonical system resource', async () => {
		const api = createRoleManagementApi();
		api.GetPermissionRoles.mockResolvedValue(['super_admin', 'admin', 'translator']);
		const result = await loadRoleManagement(api, 'system', SYSTEM_RESOURCE_ID);

		expect(result.err).toBeNull();
		expect(result.ok?.roles).toEqual(['super_admin', 'admin', 'translator']);
		expect(api.GetPermissionRoles).toHaveBeenCalledWith({
			params: { resource_type: 'system' }
		});
		for (const load of [api.ListResourcePermissions, api.ListUsersWithPermission]) {
			expect(load).toHaveBeenCalledWith({
				params: { resource_type: 'system', resource_id: SYSTEM_RESOURCE_ID },
				queries: { limit: 100, offset: 0 }
			});
		}
	});

	it('returns the existing success shape and resource parameters', async () => {
		const api = createRoleManagementApi();
		const result = await loadRoleManagement(api, 'organization', 'organization');

		expect(result).toEqual({
			ok: { roles: ['observer'], assignments: [], users: [], organizations: [] },
			err: null
		});
		expect(api.GetPermissionRoles).toHaveBeenCalledWith({
			params: { resource_type: 'organization' }
		});
		for (const load of [api.ListResourcePermissions, api.ListUsersWithPermission]) {
			expect(load).toHaveBeenCalledWith({
				params: { resource_type: 'organization', resource_id: 'organization' },
				queries: { limit: 100, offset: 0 }
			});
		}
		expect(api.ListOrganizations).toHaveBeenCalledWith({ queries: { limit: 100, offset: 0 } });
	});

	it.each([
		['GetPermissionRoles', 'roles', 'Could not load available roles.'],
		['ListResourcePermissions', 'assignments', 'Could not load role assignments.'],
		['ListUsersWithPermission', 'users', 'Could not load users.'],
		['ListOrganizations', 'organizations', 'Could not load organizations.']
	] as const)(
		'identifies a failed %s load and preserves its cause',
		async (method, source, message) => {
			const api = createRoleManagementApi();
			const cause = new Error('Request failed');
			api[method].mockRejectedValue(cause);

			const result = await loadRoleManagement(api, 'conversation', 'conversation');

			expect(result.ok).toBeNull();
			expect(result.err).toEqual([{ source, message, cause }]);
			expect(result.err?.[0].cause).toBe(cause);
		}
	);

	it('reports every failed load, not just the first rejection', async () => {
		const api = createRoleManagementApi();
		const rolesError = new Error('Roles failed');
		const usersError = new Error('Users failed');
		api.GetPermissionRoles.mockRejectedValue(rolesError);
		api.ListUsersWithPermission.mockRejectedValue(usersError);

		const result = await loadRoleManagement(api, 'conversation', 'conversation');

		expect(result.ok).toBeNull();
		expect(result.err).toEqual([
			{ source: 'roles', message: 'Could not load available roles.', cause: rolesError },
			{ source: 'users', message: 'Could not load users.', cause: usersError }
		]);
	});

	it('collects subsequent pages without changing the result shape', async () => {
		const api = createRoleManagementApi();
		const users = Array.from({ length: 100 }, (_, index) => ({
			id: `user-${index}`,
			roleName: 'observer'
		}));
		const lastUser = { id: 'last-user', roleName: 'observer' };
		api.ListUsersWithPermission.mockResolvedValueOnce(users).mockResolvedValueOnce([lastUser]);

		const result = await loadRoleManagement(api, 'conversation', 'conversation');

		expect(result.err).toBeNull();
		expect(result.ok?.users).toEqual([...users, lastUser]);
		expect(api.ListUsersWithPermission).toHaveBeenNthCalledWith(2, {
			params: { resource_type: 'conversation', resource_id: 'conversation' },
			queries: { limit: 100, offset: 100 }
		});
	});

	it('reports a later-page failure without returning partial data', async () => {
		const api = createRoleManagementApi();
		const cause = new Error('Second page failed');
		api.ListUsersWithPermission.mockResolvedValueOnce(
			Array.from({ length: 100 }, (_, index) => ({
				id: `user-${index}`,
				roleName: 'observer'
			}))
		).mockRejectedValueOnce(cause);

		const result = await loadRoleManagement(api, 'conversation', 'conversation');

		expect(result.ok).toBeNull();
		expect(result.err).toEqual([{ source: 'users', message: 'Could not load users.', cause }]);
	});

	it('starts independent loads concurrently', async () => {
		const api = createRoleManagementApi();
		let resolveRoles!: (roles: string[]) => void;
		api.GetPermissionRoles.mockReturnValue(
			new Promise<string[]>((resolve) => {
				resolveRoles = resolve;
			})
		);

		const pending = loadRoleManagement(api, 'conversation', 'conversation');

		expect(api.ListResourcePermissions).toHaveBeenCalledOnce();
		expect(api.ListUsersWithPermission).toHaveBeenCalledOnce();
		expect(api.ListOrganizations).toHaveBeenCalledOnce();
		resolveRoles(['observer']);
		expect((await pending).err).toBeNull();
	});
});

describe('roleRecipients', () => {
	it('groups and deduplicates direct roles for a user', () => {
		const recipients = roleRecipients(
			[
				{ user_id: 'alice', role_name: 'observer' },
				{ user_id: 'alice', role_name: 'moderator' },
				{ user_id: 'alice', role_name: 'observer' }
			],
			[{ id: 'alice', username: 'Alice', email: 'alice@example.com' }],
			[]
		);
		expect(recipients).toEqual([
			{
				id: 'alice',
				type: 'user',
				kind: 'User',
				name: 'Alice',
				detail: 'alice@example.com',
				roles: ['observer', 'moderator']
			}
		]);
	});

	it('resolves an organization from its membership group, not its resource ID', () => {
		const recipients = roleRecipients(
			[{ group_id: 'membership', role_name: 'content_editor' }],
			[],
			[
				{
					id: 'organization',
					userGroupId: 'membership',
					name: 'Acme',
					contactEmail: 'acme@example.com'
				}
			]
		);
		expect(recipients[0]).toMatchObject({
			id: 'membership',
			type: 'group',
			kind: 'Organization',
			name: 'Acme'
		});
	});

	it('keeps user and group actors distinct even with the same ID', () => {
		const recipients = roleRecipients(
			[
				{ user_id: 'same', role_name: 'observer' },
				{ group_id: 'same', role_name: 'admin' }
			],
			[],
			[{ id: 'organization', userGroupId: 'same', name: 'Acme' }]
		);
		expect(recipients).toHaveLength(2);
		expect(recipients.map((recipient) => recipient.type)).toEqual(
			expect.arrayContaining(['user', 'group'])
		);
	});

	it('hides standalone group grants and ignores missing actors', () => {
		const recipients = roleRecipients(
			[{ group_id: 'group', role_name: 'translator' }, { role_name: 'admin' }],
			[],
			[]
		);
		expect(recipients).toEqual([]);
	});
});
