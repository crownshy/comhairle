import type {
	ApiClient,
	PolisStatementAux,
	PolisStatementTranslation
} from '@crownshy/api-client/api';
import { tryCatchAsync } from '$lib/utils/errorHandling';

export async function loadPolisStatements(api: ApiClient, workflowStepId: string) {
	const statementAux = await tryCatchAsync(() =>
		api.PolisListStatementAux({ queries: { workflow_step_id: workflowStepId } })
	);
	const statements: PolisStatementAux[] = statementAux.err !== null ? [] : statementAux.ok;
	return { statements, statementsFailed: statementAux.err !== null };
}

/** Every translation in the step, so the moderation queue needs one request, not one per row. */
export async function loadStepTranslations(api: ApiClient, workflowStepId: string) {
	const result = await tryCatchAsync(() =>
		api.PolisListStepStatementTranslations({ queries: { workflow_step_id: workflowStepId } })
	);
	const translations: PolisStatementTranslation[] = result.err !== null ? [] : result.ok;
	return { translations };
}
