import type { PageLoad } from './$types';
import type { ComhairleDocument, FeedbackSurveyDto } from '@crownshy/api-client/api';
import type { SurveyQuestion } from '$lib/reports/survey/insights-loader';
import { feedbackSurveyInsightsLoader } from '$lib/reports/survey/insights-loader';
import { tryCatchAsync } from '$lib/utils/errorHandling';

/**
 * The Content tab's rich fields (FAQ, thank-you, privacy policy, short privacy policy) offer an
 * "Insert Source Document" control. It needs the conversation's parsed knowledge base documents to
 * populate the picker, and it needs them again on the render path to resolve each badge's name/size
 * and download link. We fetch them once here (page-scoped so unrelated conversation sub-pages don't
 * pay for it) and only surface the DONE-parsed ones, matching the Learn step path. A failed fetch
 * falls back to an empty list, so the picker shows its empty state rather than a raw backend error.
 *
 * The Feedback tab's survey is fetched on every tab (one cheap read, and the tab strip is
 * static). Its insights go to HeyForm three times over, so they are only fetched when that tab
 * is the one being shown.
 */
export const load: PageLoad = async ({
	parent,
	params,
	url,
	depends
}): Promise<{
	availableDocuments: ComhairleDocument[];
	feedbackSurvey: FeedbackSurveyDto | null;
	feedbackSurveyInsights: SurveyQuestion[] | null;
}> => {
	depends('conversation:documents');
	depends('conversation:feedback-survey');
	const { api } = await parent();
	const conversation_id = params.conversation_id;

	let availableDocuments: ComhairleDocument[] = [];
	try {
		const documents = await api.ListDocuments({ params: { conversation_id } });
		availableDocuments = documents.filter((d: ComhairleDocument) => d.parse_status === 'DONE');
	} catch (e) {
		console.warn('failed to load conversation documents', e);
	}

	const surveyResult = await tryCatchAsync(
		(): Promise<FeedbackSurveyDto | null> =>
			api.GetFeedbackSurvey({ params: { conversation_id } })
	);
	const feedbackSurvey: FeedbackSurveyDto | null =
		surveyResult.err === null ? surveyResult.ok : null;

	let feedbackSurveyInsights: SurveyQuestion[] | null = null;
	if (feedbackSurvey && url.searchParams.get('tab') === 'feedback') {
		const insights = await feedbackSurveyInsightsLoader(api, conversation_id);
		feedbackSurveyInsights = insights.survey;
	}

	return { availableDocuments, feedbackSurvey, feedbackSurveyInsights };
};
