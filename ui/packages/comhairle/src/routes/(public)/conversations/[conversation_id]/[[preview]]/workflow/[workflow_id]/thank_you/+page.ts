import type { PageLoad } from './$types';
import type { FeedbackSurveyDto } from '@crownshy/api-client/api';
import { tryCatchAsync } from '$lib/utils/errorHandling';

export const load: PageLoad = async ({ parent, params, depends }) => {
	const {
		api,
		conversation,
		user,
		workflows,
		workflowSteps,
		preview,
		sealed,
		availableDocuments,
		hasKnowledgeBaseDocs
	} = await parent();

	depends('app:feedback-survey');

	const steps = [...workflowSteps].sort((a, b) => a.stepOrder - b.stepOrder);

	// A sealed participant has nothing to return to, so the page's revisit section drops out
	// on its own (it renders only when this list is non-empty). Filtering here rather than in
	// the markup keeps this page and the step gate reading the same flag.
	const revisitableSteps = sealed ? [] : steps.filter((s) => s.canRevisit);

	// The survey is the one thing on this page that is fetched for it alone. A failed fetch
	// costs the participant the invitation, not the page, so it degrades to "no survey".
	const surveyResult = await tryCatchAsync(
		(): Promise<FeedbackSurveyDto | null> =>
			api.GetFeedbackSurvey({ params: { conversation_id: params.conversation_id } })
	);
	const feedbackSurvey: FeedbackSurveyDto | null =
		surveyResult.err === null ? surveyResult.ok : null;

	return {
		conversation,
		user,
		workflow: workflows[0],
		steps,
		revisitableSteps,
		preview,
		availableDocuments,
		hasKnowledgeBaseDocs,
		feedbackSurvey,
		// The end of the flow keeps the flow's chrome, which carries its own preview marker.
		// Flags off the conversation layout's full-width banner, as the step pages do.
		participantChrome: true
	};
};
