import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
	const { api } = await parent();

	const formsResponse = tryCatchAsync(() =>
		api
			.ListDemographicFormsLatest({ queries: { organization_id: params.organization_id } })
			.then((form) => form.filter((f) => !f.archived_at))
	);

	const questionsResponse = tryCatchAsync(() =>
		api
			.ListDemographicQuestionsLatest({
				queries: { organization_id: params.organization_id }
			})
			.then((question) => question.filter((q) => !q.archived_at))
	);

	const tagsResponse = tryCatchAsync(() =>
		api.ListDemographicTags({ queries: { organization_id: params.organization_id } })
	);

	const forms = await formsResponse;
	if (forms.err !== null) {
		error(HttpStatus.InternalServerError, 'Could not load forms');
	}

	const questions = await questionsResponse;
	if (questions.err !== null) {
		error(HttpStatus.InternalServerError, 'Could not load questions');
	}

	const tags = await tagsResponse;
	if (tags.err !== null) {
		error(HttpStatus.InternalServerError, 'Could not load tags');
	}

	return {
		organizations,
		canEdit: true,
		forms: forms.ok,
		questions: questions.ok,
		tags: tags.ok
	};
};
