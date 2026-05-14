/**
 * Prioritisation Tool — prototype data shapes.
 *
 * All state lives in `localStorage` keyed by workflow step id for the prototype.
 * No backend persistence yet; the backend stores only an opaque blob.
 */

export type PollState = 'draft' | 'published' | 'paused' | 'ended';

export type ProposalSortMode = 'by_proposal_id' | 'by_question';

export type Proposal = {
	id: string;
	order: number;
	title: string;
	/** Markdown body. */
	content: string;
	/** Data URL of an optional header image. */
	imageDataUrl?: string;
	/**
	 * Questions asked about this specific proposal. Each proposal owns its own
	 * question list — there is no poll-wide shared question set in the
	 * prototype. See `documentation/prioritisation-aggregation.md` for the
	 * implications (no cross-proposal ranking, no combined metrics).
	 */
	questions: Question[];
};

export type QuestionBase = {
	id: string;
	order: number;
	prompt: string;
	description?: string;
	optional: boolean;
};

export type SingleLineQuestion = QuestionBase & { type: 'single_line' };
export type LongTextQuestion = QuestionBase & { type: 'long_text' };
export type Choice = { id: string; label: string };
export type MultipleChoiceQuestion = QuestionBase & {
	type: 'multiple_choice';
	choices: Choice[];
};
export type FiveStarQuestion = QuestionBase & { type: 'five_star' };
export type RatingScaleQuestion = QuestionBase & {
	type: 'rating_scale';
	min: number;
	max: number;
	minLabel: string;
	maxLabel: string;
};

export type Question =
	| SingleLineQuestion
	| LongTextQuestion
	| MultipleChoiceQuestion
	| FiveStarQuestion
	| RatingScaleQuestion;

export type QuestionType = Question['type'];

export const QUESTION_TYPE_LABELS: Record<QuestionType, string> = {
	single_line: 'Single line text',
	long_text: 'Long text',
	multiple_choice: 'Multiple choice',
	five_star: '5 star rating',
	rating_scale: 'Rating scale'
};

export type PollSettings = {
	/** null = no timer (Forever). */
	timerSeconds: number | null;
	proposalSortMode: ProposalSortMode;
};

export type ReportPage = {
	id: string;
	order: number;
	/** Markdown body. */
	content: string;
};

export type Report = {
	pages: ReportPage[];
	publishedAt?: string;
};

export type Poll = {
	id: string;
	title: string;
	instruction: string;
	proposals: Proposal[];
	settings: PollSettings;
	state: PollState;
	joinCode: string;
	publishedAt?: string;
	pausedAt?: string;
	pausedAccumulatedSeconds: number;
	endedAt?: string;
	report: Report;
};

/** Per-participant draft answers for a single proposal. */
export type AnswerValue =
	| { kind: 'text'; value: string }
	| { kind: 'choice'; choiceId: string }
	| { kind: 'numeric'; value: number };

export type ProposalAnswers = Record<string /* questionId */, AnswerValue>;

export type ParticipantDraft = {
	participantId: string;
	/** proposalId -> { questionId -> AnswerValue } */
	byProposal: Record<string, ProposalAnswers>;
	startedAt: string;
	submittedAt?: string;
};

export type Submission = ParticipantDraft & { submittedAt: string };

export const QUESTION_TYPES: { type: QuestionType; label: string }[] = (
	Object.keys(QUESTION_TYPE_LABELS) as QuestionType[]
).map((t) => ({ type: t, label: QUESTION_TYPE_LABELS[t] }));

/** Letter prefix used for multiple-choice options in the UI (A, B, C, …). */
export function letterFor(index: number): string {
	return String.fromCharCode(65 + (index % 26));
}
