import { tryCatchAsync } from '$lib/utils/errorHandling';
import { error } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';
import { HttpStatus } from '$lib/utils/constants';

export const load: LayoutLoad = async ({ parent, params }) => {
	const p = await parent();

	const forms = await tryCatchAsync(() =>
		p.api.ListDemographicFormsLatest({
			queries: { organization_id: params.organisation_id }
		})
	);

	if (forms.err !== null) {
		error(HttpStatus.InternalServerError, "Couldn't load forms");
	}

	const questions = await tryCatchAsync(() =>
		p.api.ListDemographicQuestionsLatest({
			queries: { organization_id: params.organisation_id }
		})
	);

	if (questions.err !== null) {
		error(HttpStatus.InternalServerError, "Couldn't load questions");
	}

	return {
		forms: forms.ok,
		questions: questions.ok
	};
};
