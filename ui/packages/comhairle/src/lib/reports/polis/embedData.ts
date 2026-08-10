import { apiClient } from '@crownshy/api-client/client';
import type { PolisStatementAux } from '@crownshy/api-client/api';
import type { PolisReportData } from '$lib/tools/polis/reportTypes';
import { tryCatchAsync } from '$lib/utils/errorHandling';

export type PolisEmbedData = {
	reportData: PolisReportData;
	statementAux: PolisStatementAux[];
};

/**
 * Load the data a live Polis embed needs (ADR-0012). `reportData` is required and is
 * public (`PolisGetReportData` has no auth), so it works for anonymous report viewers.
 * `statementAux` needs an authenticated user (`RequiredUser`), so it is best-effort: a
 * logged-in viewer gets themes + moderation counts, an anonymous one gets `[]` and the
 * components degrade gracefully.
 *
 * Throws when `reportData` can't load (no votes yet, or the Step/poll is gone) — the caller
 * turns that into the embed's empty / unavailable state.
 */
export async function loadPolisEmbedData(workflowStepId: string): Promise<PolisEmbedData> {
	// PolisGetReportData returns WikiPollReport, which is PolisReportData minus the client-only
	// theme overlay; it assigns without a cast (same as the insights loader).
	const reportData: PolisReportData = await apiClient.PolisGetReportData({
		queries: { workflow_step_id: workflowStepId }
	});

	const auxResult = await tryCatchAsync(() =>
		apiClient.PolisListStatementAux({ queries: { workflow_step_id: workflowStepId } })
	);

	return {
		reportData,
		statementAux: auxResult.err === null ? auxResult.ok : []
	};
}
