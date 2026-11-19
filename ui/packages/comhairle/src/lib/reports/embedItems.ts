import { listEmbeddableQuestions } from './heyform/embedData';
import { listEmbeddableProposals } from './prioritization/embedData';

/** Something a per-item section block can show, such as one HeyForm question. */
export type EmbedItem = { id: string; label: string };

/**
 * How each per-item section block lists what a step could embed, for the add-chart dialog.
 * Adding a per-item block means one entry here, next to its entry in `embeds.ts`.
 */
const ITEM_LOADERS: Record<string, (workflowStepId: string) => Promise<EmbedItem[]>> = {
	'heyform-question': listEmbeddableQuestions,
	'prioritization-proposal': listEmbeddableProposals
};

export async function loadEmbedItems(
	sectionBlock: string,
	workflowStepId: string
): Promise<EmbedItem[]> {
	return (await ITEM_LOADERS[sectionBlock]?.(workflowStepId)) ?? [];
}
