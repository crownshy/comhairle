/**
 * The backend is the source of truth for Thinking Space: the tool config
 * (topic, root questions, follow-up count) lives on the workflow step, and
 * every answer is persisted via the `thinking_space/answers` endpoints.
 * Participant progress is rebuilt from those saved answers on load — nothing
 * is stored client-side.
 */

export interface QuestionConfig {
	id: string;
	text: string;
	intent: string;
	// Matches the api-client's `passthrough()` shape so this type is assignable
	// when sent back as part of an UpdateConversationWorkflowStep payload.
	[key: string]: unknown;
}

export interface FollowUpAnswer {
	/** Backend id of the saved follow-up answer — needed to edit it later. */
	id: string | null;
	question: string;
	answer: string;
}

export interface QuestionAnswers {
	questionId: string;
	rootAnswer: string;
	/** Backend id of the saved root answer — the root_question_id for follow-ups. */
	rootAnswerId: string | null;
	followUps: FollowUpAnswer[];
}

export type ThinkingSpacePhase = 'questions' | 'overview' | 'summary';
