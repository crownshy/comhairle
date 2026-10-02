import type {
	OrganizationDto,
	ResourcePermission,
	UserWithPermissionDto
} from '@crownshy/api-client/api';

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
	return Array.from(recipients.values()).toSorted((first, second) => first.name.localeCompare(second.name));
}
