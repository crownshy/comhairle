import { tryCatchAsync, type ErrorType } from '$lib/utils/errorHandling';
import type { DemographicFormLatestDto, QuestionLatestDto } from '@crownshy/api-client/api';
import type { PageLoad } from './$types';
import { typed } from '$lib/utils/types';
import Demographics from '$lib/interfaces/Demographics';

type LoadType = {
	form: DemographicFormLatestDto;
	questions: QuestionLatestDto[];
};

export const load: PageLoad = async ({ parent, params }) => {
	if (!params.form_id) {
		// If the form doesn't exist make a new one
		return {
			streamed: tryCatchAsync<LoadType, ErrorType>(async () => {
				return {
					form: Demographics.newForm(params.organisation_id),
					questions: typed<QuestionLatestDto[]>([])
				};
			})
		};
	}

	const p = await parent();

	return {
		streamed: tryCatchAsync<LoadType, ErrorType>(async () => {
			const form = await p.api.GetDemographicFormLatest({
				queries: { organization_id: params.organisation_id },
				params: { form_id: params.form_id! }
			});

			const questionRequests = [];

			for (const entry of form.entries) {
				questionRequests.push(
					p.api.GetDemographicsQuestionLatest({
						queries: { organization_id: params.organisation_id },
						params: { question_id: entry.question_id }
					})
				);
			}

			const questions = await Promise.all(questionRequests);

			return {
				form,
				questions
			};
		})
	};
};
