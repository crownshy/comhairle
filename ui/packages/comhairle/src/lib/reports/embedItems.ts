import { apiClient } from '@crownshy/api-client/client';

/** Something a per-item section block can show, such as one HeyForm question. */
export type EmbedItem = { id: string; label: string };

/** How each per-item section block lists what a step could embed, for the add-chart dialog. */
const ITEM_LOADERS: Record<string, (workflowStepId: string) => Promise<EmbedItem[]>> = {
	'heyform-question': async (workflowStepId) => {
		const questions = await apiClient.HeyFormListEmbeddableQuestions({
			queries: { workflow_step_id: workflowStepId }
		});
		return questions.map((question) => ({ id: question.id, label: question.title }));
	}
};

export async function loadEmbedItems(
	sectionBlock: string,
	workflowStepId: string
): Promise<EmbedItem[]> {
	return (await ITEM_LOADERS[sectionBlock]?.(workflowStepId)) ?? [];
}
