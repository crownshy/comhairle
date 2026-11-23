import { apiClient } from '@crownshy/api-client/client';
import type {
	PrioritizationPublicAlignmentQuestion,
	PrioritizationPublicKeyStats,
	PrioritizationPublicProposal,
	PrioritizationPublicRankedProposal,
	PrioritizationPublicRanking,
	PrioritizationPublicScale,
	PublicTheme
} from '@crownshy/api-client/api';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { EmbedItem } from '../embedItems';
import type { PrioritizationEmbeddableComponentType } from './embeddableComponents';

/** What one Prioritization section block draws, from its public report-data endpoint. */
export type PrioritizationEmbedData =
	| { componentType: 'prioritization-key-stats'; stats: PrioritizationPublicKeyStats }
	| { componentType: 'prioritization-ranking'; ranking: PrioritizationPublicRanking }
	| { componentType: 'prioritization-proposal'; proposal: PrioritizationPublicProposal }
	| { componentType: 'prioritization-themes'; themes: PublicTheme[] };

const ENDPOINTS = {
	public: {
		'prioritization-key-stats': apiClient.PrioritizationGetPublicKeyStats,
		'prioritization-ranking': apiClient.PrioritizationGetPublicRanking,
		'prioritization-proposal': apiClient.PrioritizationGetPublicProposal,
		'prioritization-themes': apiClient.PrioritizationGetPublicThemes
	},
	preview: {
		'prioritization-key-stats': apiClient.PrioritizationPreviewKeyStats,
		'prioritization-ranking': apiClient.PrioritizationPreviewRanking,
		'prioritization-proposal': apiClient.PrioritizationPreviewProposal,
		'prioritization-themes': apiClient.PrioritizationPreviewThemes
	}
} satisfies Record<'public' | 'preview', Record<PrioritizationEmbeddableComponentType, unknown>>;

/**
 * Load one section block's data. A preview falls back to the public endpoint, as
 * `loadPolisEmbedData` explains. Any failure throws, and the caller shows the embed's unavailable state.
 */
export async function loadPrioritizationEmbedData(
	workflowStepId: string,
	componentType: PrioritizationEmbeddableComponentType,
	item: string | undefined,
	preview = false
): Promise<PrioritizationEmbedData> {
	if (preview) {
		const previewed = await tryCatchAsync(() =>
			loadFrom(ENDPOINTS.preview, workflowStepId, componentType, item)
		);
		if (previewed.err === null) return previewed.ok;
	}
	return loadFrom(ENDPOINTS.public, workflowStepId, componentType, item);
}

async function loadFrom(
	endpoints: (typeof ENDPOINTS)['public'],
	workflowStepId: string,
	componentType: PrioritizationEmbeddableComponentType,
	item: string | undefined
): Promise<PrioritizationEmbedData> {
	const request = { queries: { workflow_step_id: workflowStepId } };
	switch (componentType) {
		case 'prioritization-key-stats':
			return { componentType, stats: await endpoints[componentType](request) };
		case 'prioritization-ranking':
			return { componentType, ranking: await endpoints[componentType](request) };
		case 'prioritization-proposal': {
			if (!item) throw new Error('A proposal embed needs a proposal');
			const proposal = await endpoints[componentType]({
				queries: { workflow_step_id: workflowStepId, proposal_id: item }
			});
			return { componentType, proposal };
		}
		case 'prioritization-themes':
			return { componentType, themes: (await endpoints[componentType](request)).themes };
	}
}

/** The step's proposals, for the add-chart dialog. */
export async function listEmbeddableProposals(workflowStepId: string): Promise<EmbedItem[]> {
	const proposals = await apiClient.PrioritizationListEmbeddableProposals({
		queries: { workflow_step_id: workflowStepId }
	});
	return proposals.map((proposal) => ({ id: proposal.id, label: proposal.title }));
}

/**
 * The ranking bars share one scale: an answer's range times the most reviews any proposal
 * got, so the proposal with the most support fills its bar.
 */
export function rankingDomain(
	question: PrioritizationPublicAlignmentQuestion,
	proposals: PrioritizationPublicRankedProposal[]
): [number, number] {
	const mostReviews = Math.max(1, ...proposals.map((proposal) => proposal.reviews));
	return [question.min * mostReviews, question.max * mostReviews];
}

/**
 * One value per answer in each bucket, placed at the bucket's middle, which is the shape the
 * admin distribution chart draws from.
 */
export function bucketAnswers(scale: PrioritizationPublicScale): number[] {
	const width = scale.buckets.length > 0 ? (scale.max - scale.min) / scale.buckets.length : 0;
	return scale.buckets.flatMap(({ from, count }) => Array<number>(count).fill(from + width / 2));
}
