import type { ApiClient } from '@crownshy/api-client/api';
import { tryCatchAsync } from '$lib/utils/errorHandling';

export async function polisInsightsLoader(apiClient: ApiClient, workflowStepId: string) {
	const statementAux = await apiClient.PolisListStatementAux({
		queries: { workflow_step_id: workflowStepId }
	});

	// PolisReportData is WikiPollReport (the client's return type) plus the client-only
	// theme overlay, so this assigns without a cast.
	//
	// A brand-new poll has no report yet: until Polis has enough vote data to run PCA,
	// this call errors ("Failed to parse PCA data: error decoding response body") rather
	// than returning empty. That's the expected pre-vote state, not a failure we want to
	// surface, so we swallow it to null and let the insights UI render its empty state.
	// A real outage lands here too; it shows the same "no insights yet" state, and the
	// underlying error is logged for debugging.
	const reportResult = await tryCatchAsync(() =>
		apiClient.PolisGetReportData({ queries: { workflow_step_id: workflowStepId } })
	);
	if (reportResult.err !== null) {
		console.warn(
			'Polis report data not available yet for step',
			workflowStepId,
			reportResult.err
		);
	}
	const reportData = reportResult.err === null ? reportResult.ok : null;

	return { polis: { statementAux, reportData } };
}
