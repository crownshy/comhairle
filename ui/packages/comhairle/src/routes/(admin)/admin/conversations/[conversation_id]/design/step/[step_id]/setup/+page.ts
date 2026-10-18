import type { PolisStatementAux } from '@crownshy/api-client/api';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';

// Polis Setup lists the step's seed statements, so it loads the statements here.
export const load: PageLoad = async ({ parent, depends }) => {
	depends('polis:statement-aux');
	const { api, toolConfig, step_id } = await parent();
	if (toolConfig?.type !== 'polis') return { statements: [] as PolisStatementAux[] };

	const statementAux = await tryCatchAsync(() =>
		api.PolisListStatementAux({ queries: { workflow_step_id: step_id } })
	);
	return { statements: statementAux.err !== null ? [] : statementAux.ok };
};
