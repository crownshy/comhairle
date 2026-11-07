import type { LayoutLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';

export const load: LayoutLoad = async ({ parent, depends }) => {
	depends(key('admin/demographic-forms'));
	const p = await parent();

	return {
		userOrganisations: p.userOrganisations.organizations.filter((org) => org.isAssociated),
		canEdit: true
	};
};
