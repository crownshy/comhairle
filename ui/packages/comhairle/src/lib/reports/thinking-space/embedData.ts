import { apiClient } from '@crownshy/api-client/client';
import type {
	PublicTheme,
	ThinkingSpacePublicKeyStats,
	ThinkingSpacePublicQuestion
} from '@crownshy/api-client/api';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { ThinkingSpaceEmbeddableComponentType } from './embeddableComponents';

/** What one Thinking Space section block draws, from its public report-data endpoint. */
export type ThinkingSpaceEmbedData =
	| { componentType: 'thinkingspace-key-stats'; stats: ThinkingSpacePublicKeyStats }
	| { componentType: 'thinkingspace-questions'; questions: ThinkingSpacePublicQuestion[] }
	| { componentType: 'thinkingspace-themes'; themes: PublicTheme[] };

const ENDPOINTS = {
	public: {
		'thinkingspace-key-stats': apiClient.ThinkingSpaceGetPublicKeyStats,
		'thinkingspace-questions': apiClient.ThinkingSpaceGetPublicQuestions,
		'thinkingspace-themes': apiClient.ThinkingSpaceGetPublicThemes
	},
	preview: {
		'thinkingspace-key-stats': apiClient.ThinkingSpacePreviewKeyStats,
		'thinkingspace-questions': apiClient.ThinkingSpacePreviewQuestions,
		'thinkingspace-themes': apiClient.ThinkingSpacePreviewThemes
	}
} satisfies Record<'public' | 'preview', Record<ThinkingSpaceEmbeddableComponentType, unknown>>;

/**
 * Load one section block's data. A preview falls back to the public endpoint, as the Polis
 * embeds do. Any failure throws, and the caller shows the embed's unavailable state.
 */
export async function loadThinkingSpaceEmbedData(
	workflowStepId: string,
	componentType: ThinkingSpaceEmbeddableComponentType,
	preview = false
): Promise<ThinkingSpaceEmbedData> {
	if (preview) {
		const previewed = await tryCatchAsync(() =>
			loadFrom(ENDPOINTS.preview, workflowStepId, componentType)
		);
		if (previewed.err === null) return previewed.ok;
	}
	return loadFrom(ENDPOINTS.public, workflowStepId, componentType);
}

async function loadFrom(
	endpoints: (typeof ENDPOINTS)['public'],
	workflowStepId: string,
	componentType: ThinkingSpaceEmbeddableComponentType
): Promise<ThinkingSpaceEmbedData> {
	const request = { queries: { workflow_step_id: workflowStepId } };
	switch (componentType) {
		case 'thinkingspace-key-stats':
			return { componentType, stats: await endpoints[componentType](request) };
		case 'thinkingspace-questions':
			return {
				componentType,
				questions: (await endpoints[componentType](request)).questions
			};
		case 'thinkingspace-themes':
			return { componentType, themes: (await endpoints[componentType](request)).themes };
	}
}
