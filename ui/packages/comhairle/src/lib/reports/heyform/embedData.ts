import { apiClient } from '@crownshy/api-client/client';
import type { HeyFormPublicQuestion, HeyFormPublicScale } from '@crownshy/api-client/api';
import { tryCatchAsync } from '$lib/utils/errorHandling';

/**
 * Load one question's public results (ADR-0061). A preview tries the facilitator endpoint first
 * and falls back to the public one, as the Polis embeds do. Any failure throws, and the caller
 * shows the embed's unavailable state.
 */
export async function loadHeyFormQuestion(
	workflowStepId: string,
	questionId: string,
	preview = false
): Promise<HeyFormPublicQuestion> {
	const request = { queries: { workflow_step_id: workflowStepId, question_id: questionId } };
	if (preview) {
		const previewed = await tryCatchAsync(() => apiClient.HeyFormPreviewQuestion(request));
		if (previewed.err === null) return previewed.ok;
	}
	return apiClient.HeyFormGetPublicQuestion(request);
}

/** One answer per person who picked each point, the shape the admin scale chart draws from. */
export function scaleAnswers(scale: HeyFormPublicScale): number[] {
	return scale.points.flatMap(({ point, count }) => Array<number>(count).fill(point));
}
