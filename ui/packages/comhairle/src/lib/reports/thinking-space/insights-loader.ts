// TODO: type
export async function thinkingSpaceInsightsLoader(apiClient: any, workflowStepId: string) {
	const insights = await apiClient.GetThinkingSpaceInsights({
		queries: { workflow_step_id: workflowStepId }
	});

	return { thinkingSpace: { insights } };
}
