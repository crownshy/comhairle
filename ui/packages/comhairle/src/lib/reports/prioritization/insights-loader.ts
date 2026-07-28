import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { ApiClient } from '@crownshy/api-client/api';

export async function prioritizationInsightsLoader(apiClient: ApiClient, workflowStepId: string) {
	const result = await tryCatchAsync(() =>
		apiClient.GetPrioritizationInsights({
			queries: { workflow_step_id: workflowStepId }
		})
	);

	if (result.err !== null) {
		const message = 'Something went wrong retrieving insights data.';
		if (result.err.response?.status === 422) {
			return {
				prioritization: {
					insights: null,
					error:
						message + ' Please ensure an alignment question is selected for this step.'
				}
			};
		}
		return { prioritization: { insights: null, error: message } };
	}

	return { prioritization: { insights: result.ok, error: null } };
}
