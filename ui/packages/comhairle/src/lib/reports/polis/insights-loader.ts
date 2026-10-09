import type { ApiClient, WikiPollConversationSummary } from '@crownshy/api-client/api';

export async function polisInsightsLoader(apiClient: ApiClient, workflowStepId: string) {
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

	return { polis: { statementAux, reportData, summary } };
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
