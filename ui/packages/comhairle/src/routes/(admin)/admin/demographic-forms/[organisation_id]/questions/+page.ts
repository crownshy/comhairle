import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
	const p = await parent();

	return {
		streamedTags: tryCatchAsync(() =>
			p.api.ListDemographicTags({ queries: { organization_id: params.organisation_id } })
		)
	};
};
