import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import { redirect } from '@sveltejs/kit';
import { HttpStatus } from '$lib/utils/constants';
import { resolve } from '$app/paths';

type Response = {
	question: string;
	answer: string;
};

export const load: PageLoad = async ({ parent, depends }) => {
	depends(key('public/settings/profile'));

	const { api, user } = await parent();

	if (!user) {
		redirect(HttpStatus.Found, resolve('/(public)'));
	}

	return {
		user,
		streamedMyResponses: tryCatchAsync(async (ok) => {
			const demographicAnswers = await api.ListMyDemographicAnswers();

			const questions = await Promise.all(
				demographicAnswers.map((a) =>
					api.GetDemographicQuestion({
						queries: { organization: user.organizationId },
						params: { question_id: a.answer.question_id }
					})
				)
			);

			const responses: Response[] = [];

			for (const a of demographicAnswers) {
				const question = questions.find((q) => q.id === a.answer.question_id);
				if (!question) {
					continue;
				}

				responses.push({
					question: question.latest_published?.text ?? '',
					answer: a.answer.value?.toString() ?? ''
				});
			}

			return ok(responses);
		})
	};
};
