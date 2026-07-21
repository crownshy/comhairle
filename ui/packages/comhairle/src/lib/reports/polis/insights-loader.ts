import type { PolisStatementAux } from '@crownshy/api-client/api';
import type { PolisReportData } from '$lib/tools/polis/reportTypes';

export async function polisInsightsLoader(apiClient: any, workflowStepId: string) {
	let statementAux: PolisStatementAux[] = [];
	let reportData: PolisReportData | null = null;
	try {
		statementAux = await apiClient.PolisListStatementAux({
			queries: { workflow_step_id: workflowStepId }
		});
	} catch (e) {
		console.error('Failed to load Polis statement aux', e);
	}
	try {
		// PolisReportData is WikiPollReport (the client's return type) plus the client-only
		// theme overlay, so this assigns without a cast. Fails (→ null) when the poll has no
		// votes yet.
		reportData = await apiClient.PolisGetReportData({
			queries: { workflow_step_id: workflowStepId }
		});
	} catch (e) {
		console.error('Failed to load Polis report data', e);
	}

	return { polis: { statementAux, reportData } };
}
