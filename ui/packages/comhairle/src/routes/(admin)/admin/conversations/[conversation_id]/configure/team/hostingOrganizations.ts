import type { UserOrganizationAccess } from '@crownshy/api-client/api';

export type HostingOrganizationOption = { id: string; name: string };

/** Only organisations the user can update may host a conversation. */
export function hostingOrganizationOptions(
	organizations: UserOrganizationAccess[]
): HostingOrganizationOption[] {
	return organizations
		.filter((access) => access.canUpdate)
		.map((access) => ({ id: access.organization.id, name: access.organization.name }))
		.sort((a, b) => a.name.localeCompare(b.name));
}

/** The user may not be able to update the current host, so look it up among all their organisations. */
export function currentHostingOrganizationName(
	organizations: UserOrganizationAccess[],
	organizationId: string | null
): string | null {
	if (organizationId === null) return null;
	const match = organizations.find((access) => access.organization.id === organizationId);
	return match?.organization.name ?? 'Another organisation';
}
