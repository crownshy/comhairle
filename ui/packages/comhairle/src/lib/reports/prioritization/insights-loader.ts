import type { ApiClient } from '@crownshy/api-client/api';

export async function prioritizationInsightsLoader(apiClient: ApiClient, workflowStepId: string) {
	const insights = await apiClient.GetPrioritizationInsights({
		queries: { workflow_step_id: workflowStepId }
	});

	return { prioritization: { insights } };
}
