/**
 * Calls the Thinking Space converse endpoint (`POST /tools/thinking_space`)
 * which generates up to 5 candidate follow-up questions for a root answer.
 * The backend rebuilds the Q/A chain from the stored `ThinkingSpaceAnswer`
 * rows itself, so the caller only needs to identify the root answer.
 */

import { apiClient } from '@crownshy/api-client/client';

export interface ConverseParams {
	workflowStepId: string;
	rootQuestionId: string;
	questionIntent: string;
}

export async function fetchFollowUps(params: ConverseParams): Promise<string[]> {
	const { questions } = await apiClient.ConverseThinkingSpace({
		workflow_step_id: params.workflowStepId,
		root_question_id: params.rootQuestionId,
		question_intent: params.questionIntent
	});
	return questions;
}
