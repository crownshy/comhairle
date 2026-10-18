import { redirect } from '@sveltejs/kit';
import type { PolisStatementAux } from '@crownshy/api-client/api';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, parent, depends }) => {
	// Re-run after sync, moderate, split and seed.
	depends('polis:statement-aux');
	const { api, conversation, workflowSteps } = await parent();

	const step = workflowSteps.find((s) => s.id === params.step_id);
	const toolConfig = step && (conversation.isLive ? step.toolConfig : step.previewToolConfig);
	if (!step || toolConfig?.type !== 'polis') {
		redirect(307, `/admin/conversations/${params.conversation_id}/moderate`);
	}

	const statementAux = await tryCatchAsync(() =>
		api.PolisListStatementAux({ queries: { workflow_step_id: step.id } })
	);
	const statements: PolisStatementAux[] = statementAux.err !== null ? [] : statementAux.ok;

	return { step, toolConfig, statements, statementsFailed: statementAux.err !== null };
};
