import type { PolisStatementAux } from '@crownshy/api-client/api';
import type { PolisReportData } from '$lib/tools/polis/reportTypes';
import type { PageLoad } from './$types';

export const load: PageLoad = async (event) => {
	const step_id = event.params.step_id;
	const { api, conversation, workflows, workflowSteps } = await event.parent();

	// Polis Moderation/Insights subtabs read the statement_aux table for this
	// step. Declared here so the tabs can invalidate just this after
	// sync/moderate/seed, and so both subtabs share one fetch. `polis:report`
	// is a separate key for the Insights vote/report export.
	event.depends('polis:statement-aux');
	event.depends('polis:report');

	const step = workflowSteps?.find((s) => s.id === step_id);
	const toolConfig = step
		? conversation.isLive
			? step.toolConfig
			: step.previewToolConfig
		: null;

	let statementAux: PolisStatementAux[] = [];
	let reportData: PolisReportData | null = null;
	if (toolConfig?.type === 'polis') {
		try {
			statementAux = await api.PolisListStatementAux({
				queries: { workflow_step_id: step_id }
			});
		} catch (e) {
			console.error('Failed to load Polis statement aux', e);
		}
		try {
			// Typed as WikiPollReport by the client; structurally PolisReportData.
			// Fails (→ null) when the poll has no votes yet.
			reportData = (await api.PolisGetReportData({
				queries: { workflow_step_id: step_id }
			})) as unknown as PolisReportData;
		} catch (e) {
			console.error('Failed to load Polis report data', e);
		}
	}

	return { step_id, conversation, workflows, workflowSteps, statementAux, reportData };
};
