import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ parent, params }) => {
	const p = await parent();

	return {
		streamedQuestions: tryCatchAsync(() =>
			p.api.ListDemographicQuestionsLatest({
				queries: { organization_id: params.organisation_id }
			})
		)
	};
};
