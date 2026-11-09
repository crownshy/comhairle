import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ parent, params }) => {
	const p = await parent();

	return {
		streamed: tryCatchAsync(async () => {
			const form = await p.api.GetDemographicFormLatest({
				queries: { organization_id: params.organisation_id },
				params: { form_id: params.form_id }
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
