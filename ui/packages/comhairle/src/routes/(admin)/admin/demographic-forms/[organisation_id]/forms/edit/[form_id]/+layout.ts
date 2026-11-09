import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ parent, params }) => {
	const p = await parent();

	return {
		streamedForm: tryCatchAsync(() =>
			p.api.GetDemographicFormLatest({
				queries: { organization_id: params.organisation_id },
				params: { form_id: params.form_id }
			})
		)
	};
};
