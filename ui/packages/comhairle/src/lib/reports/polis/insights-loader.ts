import type { ApiClient } from '@crownshy/api-client/api';
import { tryCatchAsync } from '$lib/utils/errorHandling';

/**
 * Pull a human-readable message out of whatever the api client throws. Zodios wraps
 * axios, so a failed call carries the API's JSON body at `response.data.err`; fall back
 * to the error's own `message`, then to `String(err)`.
 */
function errorText(err: unknown): string {
	if (typeof err === 'string') return err;
	if (err && typeof err === 'object') {
		const e = err as { response?: { data?: { err?: unknown } }; message?: unknown };
		if (typeof e.response?.data?.err === 'string') return e.response.data.err;
		if (typeof e.message === 'string') return e.message;
	}
	return String(err);
}

// A brand-new poll has no report yet: until Polis has enough vote data to run PCA, the
// report_data endpoint errors instead of returning empty. This matches that expected
// pre-report state so we can show a "waiting on votes" empty state for it, and a real
// "something broke" state for everything else (network, our-API 500s, auth). Tied to the
// current Polis error wording; if that changes we fall back to treating it as a genuine
// error, which is safe (shows the error state, which an admin can retry).
const PRE_REPORT_SIGNATURE = /parse PCA data|Failed to get comments/i;

export async function polisInsightsLoader(apiClient: ApiClient, workflowStepId: string) {
	const statementAux = await apiClient.PolisListStatementAux({
		queries: { workflow_step_id: workflowStepId }
	});

	// PolisReportData is WikiPollReport (the client's return type) plus the client-only
	// theme overlay, so this assigns without a cast.
	const reportResult = await tryCatchAsync(() =>
		apiClient.PolisGetReportData({ queries: { workflow_step_id: workflowStepId } })
	);

	if (reportResult.err === null) {
		return { polis: { statementAux, reportData: reportResult.ok, reportError: null } };
	}

	const message = errorText(reportResult.err);
	if (PRE_REPORT_SIGNATURE.test(message)) {
		// Expected pre-report state, not a failure: leave reportData + reportError null so
		// the UI shows the "no insights yet, waiting on votes" empty state.
		console.warn('Polis report not ready yet for step', workflowStepId, message);
		return { polis: { statementAux, reportData: null, reportError: null } };
	}

	// Genuine failure: surface a distinct error state in the UI.
	console.error('Failed to load Polis report for step', workflowStepId, reportResult.err);
	return { polis: { statementAux, reportData: null, reportError: message } };
}
