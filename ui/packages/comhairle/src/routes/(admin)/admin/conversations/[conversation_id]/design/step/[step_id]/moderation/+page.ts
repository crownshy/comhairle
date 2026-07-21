import { redirect } from '@sveltejs/kit';
import type { PolisStatementAux } from '@crownshy/api-client/api';
import type { PageLoad } from './$types';

/**
 * Moderation only exists for Polis steps. Non-polis steps are bounced to Setup. The
 * statement_aux fetch lives here (not the step's shared load) so it runs only when this tab
 * is actually open. `polis:statement-aux` is the invalidation key the moderation actions
 * re-run after sync/moderate/seed.
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

	let statementAux: PolisStatementAux[] = [];
	try {
		statementAux = await api.PolisListStatementAux({ queries: { workflow_step_id: step_id } });
	} catch (e) {
		console.error('Failed to load Polis statement aux', e);
	}

	return { statementAux };
};
