import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async ({ parent, depends }) => {
	depends(key('public/settings/profile'));

	const { api, user } = await parent();

	const questions = tryCatchAsync(() =>
		api.GetDemographicsQuestions({ queries: { limit: 100 } })
	);
	const responses = tryCatchAsync(() =>
		api.GetDemographicsResponses({ queries: { user_id: user.id, limit: 100 } })
	);

	return {
		demographicQuestions: (await questions).ok?.records ?? [],
		demographicResponses: (await responses).ok?.records ?? []
	};
};
