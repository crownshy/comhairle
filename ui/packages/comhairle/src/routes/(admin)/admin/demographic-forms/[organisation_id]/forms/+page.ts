import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
	const p = await parent();

	return {
		streamedForms: tryCatchAsync(() =>
			p.api.ListDemographicFormsLatest({
				queries: { organization_id: params.organisation_id }
			})
		),
		streamedQuestions: tryCatchAsync(() =>
			p.api.ListDemographicQuestionsLatest({
				queries: { organization_id: params.organisation_id }
			})
		)
	};
};
