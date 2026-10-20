import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import { redirect } from '@sveltejs/kit';
import { HttpStatus } from '$lib/utils/constants';
import { resolve } from '$app/paths';

export const load: PageLoad = async ({ parent, depends }) => {
	depends(key('public/settings/profile'));

	const { api, user } = await parent();

	if (!user) {
		redirect(HttpStatus.Found, resolve('/(public)'));
	}

	const questions = tryCatchAsync(() =>
		api.GetDemographicsQuestions({ queries: { limit: 100 } })
	);
	const responses = tryCatchAsync(() =>
		api.GetDemographicsResponses({ queries: { user_id: user.id, limit: 100 } })
	);

	return {
		user,
		demographicQuestions: (await questions).ok?.records ?? [],
		demographicResponses: (await responses).ok?.records ?? []
	};
};
