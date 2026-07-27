import { apiClient } from '@crownshy/api-client/client';

export async function thinkingSpaceInsightsLoader(workflowStepId: string) {
	const insights = await apiClient.GetThinkingSpaceInsights({
		queries: { workflow_step_id: workflowStepId }
	});

	return { thinkingSpace: { insights } };
}
