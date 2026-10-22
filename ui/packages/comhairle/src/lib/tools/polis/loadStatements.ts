import type { ApiClient, PolisStatementAux } from '@crownshy/api-client/api';
import { tryCatchAsync } from '$lib/utils/errorHandling';

export async function loadPolisStatements(api: ApiClient, workflowStepId: string) {
	const statementAux = await tryCatchAsync(() =>
		api.PolisListStatementAux({ queries: { workflow_step_id: workflowStepId } })
	);
	const statements: PolisStatementAux[] = statementAux.err !== null ? [] : statementAux.ok;
	return { statements, statementsFailed: statementAux.err !== null };
}
