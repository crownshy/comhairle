import type {
	OrganizationDto,
	PermissionResourceType,
	ResourcePermission,
	UserWithPermissionDto
} from '@crownshy/api-client/api';
import type { createApiClient } from '@crownshy/api-client/client';
import { tryCatchAsync, type ErrorType, type Result } from '$lib/utils/errorHandling';

export type RoleResourceType = Extract<PermissionResourceType, 'conversation' | 'organization'>;

type RoleManagementApi = Pick<
	ReturnType<typeof createApiClient>,
	| 'GetPermissionRoles'
	| 'ListResourcePermissions'
	| 'ListUsersWithPermission'
	| 'ListOrganizations'
>;

type RoleManagementData = {
	roles: Awaited<ReturnType<RoleManagementApi['GetPermissionRoles']>>;
	assignments: ResourcePermission[];
	users: UserWithPermissionDto[];
	organizations: OrganizationDto[];
};

type RoleManagementLoadError = {
	source: keyof RoleManagementData;
	message: string;
	cause: ErrorType;
};

async function collectPages<RecordType>(loadPage: (offset: number) => Promise<RecordType[]>) {
	const records: RecordType[] = [];
	for (let offset = 0; ; offset += 100) {
		const page = await loadPage(offset);
		records.push(...page);
		if (page.length < 100) return records;
	}
}

/** Loads all role-management data in parallel, retaining each failed load's cause. */
export async function loadRoleManagement(
	api: RoleManagementApi,
	resourceType: RoleResourceType,
	resourceId: string
): Promise<Result<'ok', RoleManagementData, RoleManagementLoadError[]>> {
	const params = { resource_type: resourceType, resource_id: resourceId };
	const [roles, assignments, users, organizations] = await Promise.all([
		tryCatchAsync(() => api.GetPermissionRoles({ params: { resource_type: resourceType } })),
		tryCatchAsync(() =>
			collectPages((offset) =>
				api
					.ListResourcePermissions({ params, queries: { limit: 100, offset } })
					.then((page) => page.records)
			)
		),
		tryCatchAsync(() =>
			collectPages((offset) =>
				api.ListUsersWithPermission({ params, queries: { limit: 100, offset } })
			)
		),
		tryCatchAsync(() =>
			collectPages((offset) =>
				api
					.ListOrganizations({ queries: { limit: 100, offset } })
					.then((page) => page.records)
			)
		)
	]);
	const errors: RoleManagementLoadError[] = [];
	if (roles.err !== null)
		errors.push({
			source: 'roles',
			message: 'Could not load available roles.',
			cause: roles.err
		});
	if (assignments.err !== null)
		errors.push({
			source: 'assignments',
			message: 'Could not load role assignments.',
			cause: assignments.err
		});
	if (users.err !== null)
		errors.push({
			source: 'users',
			message: 'Could not load users.',
			cause: users.err
		});
	if (organizations.err !== null)
		errors.push({
			source: 'organizations',
			message: 'Could not load organizations.',
			cause: organizations.err
		});
	if (
		roles.err !== null ||
		assignments.err !== null ||
		users.err !== null ||
		organizations.err !== null
	) {
		return { ok: null, err: errors };
	}
	return {
		ok: {
			roles: roles.ok,
			assignments: assignments.ok,
			users: users.ok,
			organizations: organizations.ok
		},
		err: null
	};
}

export type RoleManagement = Awaited<ReturnType<typeof loadRoleManagement>>;

export type RoleRecipient = {
	id: string;
	type: 'user' | 'group';
	kind: 'User' | 'Organization';
	name: string;
	detail: string;
	roles: string[];
};

export function roleRecipients(
	assignments: Pick<ResourcePermission, 'user_id' | 'group_id' | 'role_name'>[],
	users: Pick<UserWithPermissionDto, 'id' | 'email' | 'username'>[],
	organizations: Pick<OrganizationDto, 'id' | 'name' | 'contactEmail' | 'userGroupId'>[]
): RoleRecipient[] {
	const recipients = new Map<string, RoleRecipient>();
	for (const assignment of assignments) {
		const id = assignment.user_id ?? assignment.group_id;
		if (!id) continue;
		const type = assignment.user_id ? 'user' : 'group';
		const organization =
			type === 'group'
				? organizations.find((organization) => organization.userGroupId === id)
				: undefined;
		if (type === 'group' && !organization) continue;
		const key = `${type}:${id}`;
		let recipient = recipients.get(key);
		if (!recipient) {
			const user = type === 'user' ? users.find((user) => user.id === id) : undefined;
			recipient = {
				id,
				type,
				kind: organization ? 'Organization' : 'User',
				name: organization?.name ?? user?.username ?? user?.email ?? id,
				detail: organization?.contactEmail ?? user?.email ?? id,
				roles: []
			};
			recipients.set(key, recipient);
		}
		if (!recipient.roles.includes(assignment.role_name))
			recipient.roles.push(assignment.role_name);
	}
	return Array.from(recipients.values()).toSorted((first, second) =>
		first.name.localeCompare(second.name)
	);
}
