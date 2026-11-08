import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params, url }) => {
	const p = await parent();

	const filter = url.searchParams.get('filter');
	const search = url.searchParams.get('search');

	return {
		streamedQuestions: tryCatchAsync(() =>
			p.api
				.ListDemographicQuestionsLatest({
					queries: { organization_id: params.organisation_id }
				})
				.then((questions) => {
					// TODO: Move this to the backend at some point
					if (filter) {
						questions = questions.filter(
							(question) => question.tags.findIndex((tag) => tag.name === filter) > -1
						);
					}

					if (search) {
						questions = questions.filter((question) =>
							question.text.toLowerCase().includes(search.toLowerCase())
						);
					}

					return questions;
				})
		),
		streamedTags: tryCatchAsync(() =>
			p.api.ListDemographicTags({ queries: { organization_id: params.organisation_id } })
		)
	};
};
