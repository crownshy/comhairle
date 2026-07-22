import type { ApiClient } from '@crownshy/api-client/api';

export async function thinkingSpaceInsightsLoader(apiClient: ApiClient, workflowStepId: string) {
	const insights = await apiClient.GetThinkingSpaceInsights({
		queries: { workflow_step_id: workflowStepId }
	});

	return { thinkingSpace: { insights } };
}
