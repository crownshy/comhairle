import type { PageLoad } from './$types';
import { canPerformAction, loadUserActions } from '$lib/utils/permissions';
import { loadRoleManagement } from '$lib/components/permissions/roleAssignments';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async ({ parent, params, depends }) => {
	const { api, userOrganizations } = await parent();
	depends(key('admin/organization/details'));
	depends(key('admin/organization/permissions'));

	try {
		const organization = await api.GetOrganization({
			params: { organization_id: params.organization_id }
		});

		const access = (userOrganizations?.organizations ?? []).find(
			(entry) => entry.organization.id === params.organization_id
		);

		const organizationActions = await loadUserActions(api, 'organization', organization.id);
		const canManageTeam =
			canPerformAction(organizationActions, 'grant_permission') &&
			canPerformAction(organizationActions, 'revoke_permission');
		const roleManagement = canManageTeam
			? await loadRoleManagement(api, 'organization', organization.id)
			: null;

		const regionsResponse = await tryCatchAsync(() =>
			api.ListRegions({ queries: { limit: 500 } })
		);
		const regions = regionsResponse.err === null ? regionsResponse.ok.records : [];

		return {
			organization,
			regions,
			organizationActions,
			roleManagement,
			canEdit: access?.canUpdate ?? false,
			canDelete: access?.canDelete ?? false,
			canManageTeam
		};
	} catch (error) {
		return {
			organization: null,
			regions: [],
			organizationActions: null,
			roleManagement: null,
			canEdit: false,
			canDelete: false,
			canManageTeam: false
		};
	}
};
