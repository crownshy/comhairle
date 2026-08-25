/**
 * The backend is the source of truth for Thinking Space: the tool config
 * (topic, root questions, follow-up count) lives on the workflow step, and
 * every answer is persisted via the `thinking_space/answers` endpoints.
 * Participant progress is rebuilt from those saved answers on load — nothing
 * is stored client-side.
 */

export interface QuestionConfig<TText> {
	id: string;
	text: TText;
	intent: TText;
	// Matches the api-client's `passthrough()` shape so this type is assignable
	// when sent back as part of an UpdateConversationWorkflowStep payload.
	[key: string]: unknown;
}

export interface FollowUpAnswer {
	/** Backend id of the saved follow-up answer — needed to edit it later. */
	id: string | null;
	question: string;
	answer: string;
	/**
	 * Alternatives the agent offered at the moment this follow-up was submitted,
	 * minus the one the participant actually chose. Persisted for research /
	 * audit only — not reused on extension
	 */
	otherQuestions: string[];
}

export interface QuestionAnswers {
	questionId: string;
	rootAnswer: string;
	/** Backend id of the saved root answer — the root_question_id for follow-ups. */
	rootAnswerId: string | null;
	followUps: FollowUpAnswer[];
}

export type ThinkingSpacePhase = 'questions' | 'summary';

/**
 * One round's summary for one participant.
 *
 * A participant may stack multiple summaries within a single Thinking Space
 * step by choosing "answer more questions" — each round produces its own row.
 */
export interface SummaryRound {
	/** Backend row id. Stable across edits; used for upsert. */
	id: string;
	/** The AI-generated text for this round. Immutable once written. */
	aiDraft: string;
	/** The participant's submitted text. Equals aiDraft until they edit. */
	submittedText: string;
	/** ISO timestamp; rounds are ordered by this on display. */
	createdAt: string;
}
