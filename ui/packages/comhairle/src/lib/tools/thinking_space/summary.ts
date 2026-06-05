/**
 * Thinking Space summary client. Wraps the backend summary endpoints (built on
 * branch 431/thinking-space-summary, merged in) and adapts the DTO into the
 * frontend's `SummaryRound` shape.
 *
 * Endpoints:
 *   - POST /tools/thinking_space/summaries/generate  (creates an AI-drafted row)
 *   - POST /tools/thinking_space/summaries           (upsert: edit if summary_id, else create)
 *   - GET  /tools/thinking_space/summaries           (list by workflow_step_id)
 *
 * See documentation/adr/0002-thinking-space-summary-storage.md.
 *
 * Known gaps in the current DTO that this file works around:
 *   - `createdAt` is not exposed, so `SummaryRound.createdAt` is best-effort
 *     (real time on freshly generated rounds; epoch placeholder on hydration).
 *     Order is taken from list response order.
 *   - The backend stores a single `summary` column, so the original AI draft
 *     is overwritten when the participant submits an edit. After hydration
 *     `aiDraft` and `submittedText` are necessarily the same.
 */

import { apiClient } from '@crownshy/api-client/client';

import type { QuestionConfig, QuestionAnswers, SummaryRound } from './types';

export interface GenerateRoundParams {
	workflowStepId: string;
	// The backend reads topic/questions/answers from the DB; these fields are
	// kept on the interface so callers can pass what they already have without
	// the type complaining.
	topic?: string;
	questions?: QuestionConfig[];
	answers?: QuestionAnswers[];
}

interface SummaryDto {
	id: string;
	summary: string;
	isAiGenerated: boolean;
	workflowStepId: string;
	userId: string;
}

function dtoToRound(dto: SummaryDto): SummaryRound {
	return {
		id: dto.id,
		aiDraft: dto.summary,
		submittedText: dto.summary,
		createdAt: new Date().toISOString()
	};
}

export async function generateNextRound(params: GenerateRoundParams): Promise<SummaryRound> {
	const dto = await apiClient.GenerateThinkingSpaceSummary({
		workflow_step_id: params.workflowStepId
	});
	return dtoToRound(dto as SummaryDto);
}

export async function saveRound(params: {
	workflowStepId: string;
	roundId: string;
	submittedText: string;
}): Promise<void> {
	await apiClient.UpdateOrCreateThinkingSpaceSummary({
		workflow_step_id: params.workflowStepId,
		summary_id: params.roundId,
		summary: params.submittedText
	});
}

export async function hydrateRounds(params: { workflowStepId: string }): Promise<SummaryRound[]> {
	const dtos = (await apiClient.ListThinkingSpaceSummaries({
		queries: { workflow_step_id: params.workflowStepId }
	})) as SummaryDto[];

	// Backend does not return a timestamp on the DTO yet, so trust list order
	// (insertion order in practice). Stamp a fixed placeholder for createdAt;
	// the field is unused for ordering here.
	return dtos.map((dto) => ({
		id: dto.id,
		aiDraft: dto.summary,
		submittedText: dto.summary,
		createdAt: new Date(0).toISOString()
	}));
}
