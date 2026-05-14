/**
 * Backend currently only persists `topic` on the workflow step `toolConfig`.
 * Everything below (questions list, follow-up count, captured answers and
 * claims) is currently held client-side in localStorage. See
 * THINKING_SPACE_TODO.md for the planned backend integration.
 */

export interface QuestionConfig {
	id: string;
	text: string;
}

export interface ThinkingSpaceConfig {
	questions: QuestionConfig[];
	/**
	 * Minimum number of AI-generated follow-ups a participant must answer
	 * for the current main question before the "move on" affordance
	 * appears. They can keep answering more follow-ups if they want.
	 * 0 = no follow-ups required.
	 */
	followUpCount: number;
}

export interface FollowUpAnswer {
	question: string;
	answer: string;
}

export interface QuestionAnswers {
	questionId: string;
	mainAnswer: string;
	followUps: FollowUpAnswer[];
}

export type ClaimStatus = 'pending' | 'approved' | 'removed';

export interface ParticipantClaim {
	id: string;
	content: string;
	sourceQuestionId: string;
	sourceQuestionText: string;
	status: ClaimStatus;
}

export type ThinkingSpacePhase = 'welcome' | 'questions' | 'review' | 'submitted';
