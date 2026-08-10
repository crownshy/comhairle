import { tryCatchAsync } from '$lib/utils/errorHandling';
import { apiClient } from '@crownshy/api-client/client';
import type { InsightQuestion } from '@crownshy/api-client/api';

export async function surveyInsightsLoader(workflowStepId: string) {
	const response = await tryCatchAsync(() =>
		apiClient.HeyFormGetInsights({
			params: { workflow_step_id: workflowStepId }
		})
	);

	if (response.err !== null) {
		return { survey: { insights: [] as InsightQuestion[] } };
	}

	return { survey: response.ok.questions };
}
