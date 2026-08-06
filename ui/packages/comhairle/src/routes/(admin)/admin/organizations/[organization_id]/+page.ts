import type { PageLoad } from './$types';
import { apiClient } from '@crownshy/api-client/client';

export const load: PageLoad = async ({ parent, params, depends, fetch }) => {
	const { api, userOrganizations } = await parent();
	depends('organization:details');
	depends('organization:team');
	depends('organization:regions');

	try {
		const organization = await api.GetOrganization({
			params: { organization_id: params.organization_id }
		});

		const access = (userOrganizations?.organizations ?? []).find(
			(entry) => entry.organization.id === params.organization_id
		);

		let team: {
			members: {
				id: string;
				username?: string | null;
				email?: string | null;
				role: 'member' | 'admin';
			}[];
		} = {
			members: []
		};

		if (access?.canManageTeam) {
			const teamResponse = await fetch(`/api/organizations/${params.organization_id}/team`);
			if (teamResponse.ok) {
				team = await teamResponse.json();
			}
		}

		const regionsResponse = await apiClient.ListRegions({ queries: { limit: 500 } });
		const regions = regionsResponse.records;

		return {
			organization,
			regions,
			team,
			canEdit: access?.canUpdate ?? false,
			canDelete: access?.canDelete ?? false,
			canManageTeam: access?.canManageTeam ?? false
		};
	} catch (error) {
		return {
			organization: null,
			regions: [],
			team: { members: [] },
			canEdit: false,
			canDelete: false,
			canManageTeam: false
		};
	}
};
