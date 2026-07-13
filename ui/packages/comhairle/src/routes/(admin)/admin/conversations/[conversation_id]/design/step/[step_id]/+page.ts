import type { PolisStatementAux } from '@crownshy/api-client/api';
import type { PageLoad } from './$types';

export const load: PageLoad = async (event) => {
	const step_id = event.params.step_id;
	const { api, conversation, workflows, workflowSteps } = await event.parent();

	// Polis Moderation/Insights subtabs read the statement_aux table for this
	// step. Declared here so the tabs can invalidate just this after
	// sync/moderate/seed, and so both subtabs share one fetch.
	event.depends('polis:statement-aux');

	const step = workflowSteps?.find((s) => s.id === step_id);
	const toolConfig = step
		? conversation.isLive
			? step.toolConfig
			: step.previewToolConfig
		: null;

	let statementAux: PolisStatementAux[] = [];
	if (toolConfig?.type === 'polis') {
		try {
			statementAux = await api.PolisListStatementAux({
				queries: { workflow_step_id: step_id }
			});
		} catch (e) {
			console.error('Failed to load Polis statement aux', e);
		}
	}

	return { step_id, conversation, workflows, workflowSteps, statementAux };
};
