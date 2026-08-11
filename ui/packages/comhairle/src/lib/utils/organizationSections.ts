import type { LocalizedOrganizationDto } from '@crownshy/api-client/api';

export type OrganizationSection = {
	key: 'organizations';
	title: string;
	organizations: LocalizedOrganizationDto[];
};

export function getOrganizationSections(userOrganizations?: {
	organizations?: {
		organization: LocalizedOrganizationDto;
	}[];
}): OrganizationSection[] {
	const organizations = userOrganizations?.organizations ?? [];
	if (organizations.length === 0) {
		return [];
	}

	return [
		{
			key: 'organizations',
			title: 'Organizations',
			organizations: organizations.map((entry) => entry.organization)
		}
	];
}
