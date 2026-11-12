import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { loadDemographics } from './demographicApi';

export const load: PageLoad = async ({ parent, depends, url }) => {
	depends(key('admin/demographic-forms'));
	const { api, userOrganizations, isSuperAdmin } = await parent();

	const organizations = (userOrganizations?.organizations ?? [])
		.filter((access) => isSuperAdmin || access.isAssociated)
		.map((access) => ({ id: access.organization.id, name: access.organization.name }));
	const requestedId = url.searchParams.get('organization');
	const organization = organizations.find((org) => org.id === requestedId) ?? null;
	const managingDefaults = organization === null && isSuperAdmin === true;
	if (organization === null && !managingDefaults) {
		return {
			organizations,
			organizationId: null,
			canEdit: false,
			managingDefaults: false,
			questions: [],
			forms: [],
			tags: []
		};
	}

	const organizationId = organization?.id ?? null;
	const result = await tryCatchAsync(() => loadDemographics(api, organizationId));
	if (result.err !== null) {
		error(500, 'Could not load demographics');
	}
	return { ...result.ok, organizations, organizationId, canEdit: true, managingDefaults };
};
