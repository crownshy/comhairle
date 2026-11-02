import type {
	ApiClient,
	ThemeCollectionDto,
	WikiPollConversationSummary
} from '@crownshy/api-client/api';

type StepPath = { conversationId: string; workflowId: string; workflowStepId: string };

export async function polisInsightsLoader(apiClient: ApiClient, stepPath: StepPath) {
	const { workflowStepId } = stepPath;
	const statementAux = await apiClient.PolisListStatementAux({
		queries: { workflow_step_id: workflowStepId }
	});
	// PolisReportData is WikiPollReport (the client's return type) plus the client-only
	// theme overlay, so this assigns without a cast. Fails (→ null) when the poll has no
	// votes yet.
	const reportData = await apiClient.PolisGetReportData({
		queries: { workflow_step_id: workflowStepId }
	});
	const summary = await getLatestSummary(apiClient, workflowStepId);
	const themeCollection = await getLatestThemeCollection(apiClient, stepPath);

	return { polis: { statementAux, reportData, summary, themeCollection } };
}

/** No summary has been generated yet is a normal, expected 404, not a load failure. */
async function getLatestSummary(
	apiClient: ApiClient,
	workflowStepId: string
): Promise<WikiPollConversationSummary | null> {
	try {
		return await apiClient.PolisGetLatestSummary({
			queries: { workflow_step_id: workflowStepId }
		});
	} catch (e) {
		if (e.response?.status === 404) return null;
		throw e;
	}
}

/** No themes have been generated yet is a normal, expected 404, not a load failure. */
async function getLatestThemeCollection(
	apiClient: ApiClient,
	{ conversationId, workflowId, workflowStepId }: StepPath
): Promise<ThemeCollectionDto | null> {
	try {
		return await apiClient.GetLatestThemeCollection({
			params: {
				conversation_id: conversationId,
				workflow_id: workflowId,
				workflow_step_id: workflowStepId
			},
			queries: { source_tool: 'polis' }
		});
	} catch (e) {
		if (e.response?.status === 404) return null;
		throw e;
	}
}
