import type {
	ApiClient,
	ThemeCollectionDto,
	WikiPollConversationSummary
} from '@crownshy/api-client/api';
import { tryCatchAsync } from '$lib/utils/errorHandling';

type StepPath = { conversationId: string; workflowId: string; workflowStepId: string };

export async function polisInsightsLoader(apiClient: ApiClient, stepPath: StepPath) {
	const { workflowStepId } = stepPath;
	const statementAux = await apiClient.PolisListStatementAux({
		queries: { workflow_step_id: workflowStepId }
	});
	const reportData = await getReportData(apiClient, workflowStepId);
	const summary = await getLatestSummary(apiClient, workflowStepId);
	const themeCollection = await getLatestThemeCollection(apiClient, stepPath);

	return { polis: { statementAux, reportData, summary, themeCollection } };
}

/**
 * Null when the poll has no votes yet, so PolisInsights shows its "No insights yet" state.
 * The API answers that case with a 500, so it can't be told apart from other failures.
 */
async function getReportData(apiClient: ApiClient, workflowStepId: string) {
	const result = await tryCatchAsync(() =>
		apiClient.PolisGetReportData({ queries: { workflow_step_id: workflowStepId } })
	);
	return result.err === null ? result.ok : null;
}

/** No summary has been generated yet is a normal, expected 404, not a load failure. */
async function getLatestSummary(
	apiClient: ApiClient,
	workflowStepId: string
): Promise<WikiPollConversationSummary | null> {
	try {
		return await apiClient.PolisGetLatestSummary({
			queries: { workflow_step_id: workflowStepId }
		});
	} catch (e) {
		if (e.response?.status === 404) return null;
		throw e;
	}
}

/** No themes have been generated yet is a normal, expected 404, not a load failure. */
async function getLatestThemeCollection(
	apiClient: ApiClient,
	{ conversationId, workflowId, workflowStepId }: StepPath
): Promise<ThemeCollectionDto | null> {
	try {
		return await apiClient.GetLatestThemeCollection({
			params: {
				conversation_id: conversationId,
				workflow_id: workflowId,
				workflow_step_id: workflowStepId
			},
			queries: { source_tool: 'polis' }
		});
	} catch (e) {
		if (e.response?.status === 404) return null;
		throw e;
	}
}
