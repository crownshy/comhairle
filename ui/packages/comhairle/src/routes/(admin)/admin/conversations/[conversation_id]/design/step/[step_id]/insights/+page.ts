import { redirect } from '@sveltejs/kit';
import type { PolisStatementAux } from '@crownshy/api-client/api';
import type { PolisReportData } from '$lib/tools/polis/reportTypes';
import type { PageLoad } from './$types';

/**
 * Insights only exists for Polis steps; non-polis steps are bounced to Setup. Both the
 * statement_aux and report/vote-export fetches live here so they run only when this tab is
 * open. `polis:statement-aux` and `polis:report` are the invalidation keys the insights
 * actions re-run after sync.
 */
export const load: PageLoad = async (event) => {
	const { api, toolConfig, step_id } = await event.parent();

	if (toolConfig?.type !== 'polis') {
		redirect(
			307,
			`/admin/conversations/${event.params.conversation_id}/design/step/${event.params.step_id}/setup`
		);
	}

	event.depends('polis:statement-aux');
	event.depends('polis:report');

	let statementAux: PolisStatementAux[] = [];
	let reportData: PolisReportData | null = null;
	try {
		statementAux = await api.PolisListStatementAux({ queries: { workflow_step_id: step_id } });
	} catch (e) {
		console.error('Failed to load Polis statement aux', e);
	}
	try {
		// PolisReportData is WikiPollReport (the client's return type) plus the client-only
		// theme overlay, so this assigns without a cast. Fails (→ null) when the poll has no
		// votes yet.
		reportData = await api.PolisGetReportData({ queries: { workflow_step_id: step_id } });
	} catch (e) {
		console.error('Failed to load Polis report data', e);
	}

	return { statementAux, reportData };
};
