import { tryCatchAsync } from '$lib/utils/errorHandling';
import { error } from '@sveltejs/kit';
import { SPECIAL_CATEGORY_TAG } from '../../demographicModel';
import type { PageLoad } from './$types';
import { HttpStatus } from '$lib/utils/constants';

export const load: PageLoad = async ({ parent, params, url }) => {
	const p = await parent();

	const filter = url.searchParams.get('filter');
	const search = url.searchParams.get('search');

	const questionsRequest = tryCatchAsync(() =>
		p.api
			.ListDemographicQuestionsLatest({
				queries: { organization_id: params.organisation_id }
			})
			.then((questions) => {
				// TODO: Move this to the backend at some point
				if (filter) {
					questions = questions.filter((question) => {
						if (filter === SPECIAL_CATEGORY_TAG) {
							return question.specialCategory;
						}
						return question.tags.findIndex((tag) => tag.name === filter) > -1;
					});
				}

				if (search) {
					questions = questions.filter((question) =>
						question.text.toLowerCase().includes(search.toLowerCase())
					);
				}

				return questions;
			})
	);
	const tagsRequest = tryCatchAsync(() =>
		p.api.ListDemographicTags({ queries: { organization_id: params.organisation_id } })
	);

	const questions = await questionsRequest;
	const tags = await tagsRequest;

	if (questions.err !== null) {
		error(HttpStatus.InternalServerError, 'Could not load questions');
	}

	if (tags.err !== null) {
		error(HttpStatus.InternalServerError, 'Could not load tags');
	}

	return {
		questions: questions.ok,
		tags: tags.ok
	};
};
