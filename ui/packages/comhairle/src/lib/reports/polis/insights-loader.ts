import type { ApiClient } from '@crownshy/api-client/api';

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

	return { polis: { statementAux, reportData } };
}
