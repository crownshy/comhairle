/**
 * Prioritization Tool — prototype data shapes.
 *
 * State lives in `localStorage` keyed by workflow step id for the prototype.
 * Mirrors the backend tool_config / poll / proposal / proposal_question
 * schema as closely as makes sense for a frontend mock.
 */

/** Default min/max for a continuous question slider. */
export const DEFAULT_CONTINUOUS_MIN = 0;
export const DEFAULT_CONTINUOUS_MAX = 10;

/**
 * Reserved enum for combining per-question scores into a single ranking.
 * Empty for now (MVP) — backend will populate when defined.
 */
export type CombinationMetric = never;

export type Proposal = {
	id: string;
	order: number;
	/** Translatable in backend; plain string in this prototype. */
	title: string;
	/** Translatable in backend; rich-text JSON in this prototype. */
	body: string;
	/** Data URL of an optional header image. */
	imageDataUrl?: string;
};

export type QuestionBase = {
	id: string;
	order: number;
	/** Question prompt (a.k.a. "text" / "title" in the backend). */
	prompt: string;
	description?: string;
	optional: boolean;
};

export type LikertCategory = { value: number; label: string };

export type LikertScaleQuestion = QuestionBase & {
	type: 'likert_scale';
	categories: LikertCategory[];
};

export type ContinuousQuestion = QuestionBase & {
	type: 'continuous';
	/** Numeric value at the left end of the slider. */
	minValue: number;
	/** Numeric value at the right end of the slider. */
	maxValue: number;
	/** Label shown at the left end (e.g. "No support"). */
	minLabel: string;
	/** Label shown at the right end (e.g. "Full support"). */
	maxLabel: string;
};

export type TextQuestion = QuestionBase & { type: 'text' };

export type Question = LikertScaleQuestion | ContinuousQuestion | TextQuestion;

export type QuestionType = Question['type'];

export const QUESTION_TYPE_LABELS: Record<QuestionType, string> = {
	likert_scale: 'Likert scale',
	continuous: 'Continuous',
	text: 'Text'
};

export const QUESTION_TYPES: { type: QuestionType; label: string }[] = (
	Object.keys(QUESTION_TYPE_LABELS) as QuestionType[]
).map((t) => ({ type: t, label: QUESTION_TYPE_LABELS[t] }));

/**
 * Tool config (`tool_config` JSONB on the backend). The questions array is
 * shared across all proposals in the poll.
 */
export type ToolConfig = {
	randomizeOrder: boolean;
	/** Reserved; not used in the MVP. */
	combinationMetric?: CombinationMetric;
	questions: Question[];
};

export type ReportPage = {
	id: string;
	order: number;
	/** Rich-text JSON body. */
	content: string;
};

export type Report = {
	pages: ReportPage[];
	publishedAt?: string;
};

export type Poll = {
	id: string;
	/** Translatable in backend; plain string in this prototype. */
	title: string;
	/** Translatable in backend; plain string in this prototype. */
	description: string;
	toolConfig: ToolConfig;
	proposals: Proposal[];
	report: Report;
};

/** Single answer to a single question. */
export type AnswerValue = { kind: 'text'; value: string } | { kind: 'numeric'; value: number };

/** Map of questionId -> AnswerValue for a single proposal. */
export type ProposalAnswers = Record<string, AnswerValue>;

export type ParticipantDraft = {
	participantId: string;
	/** proposalId -> { questionId -> AnswerValue } */
	byProposal: Record<string, ProposalAnswers>;
	startedAt: string;
	submittedAt?: string;
};

export type Submission = ParticipantDraft & { submittedAt: string };

/** Letter prefix used for ordered options in the UI (A, B, C, …). */
export function letterFor(index: number): string {
	return String.fromCharCode(65 + (index % 26));
}

/** Default categories for a fresh likert_scale question (5-point agree/disagree). */
export function defaultLikertCategories(): LikertCategory[] {
	return [
		{ value: 1, label: 'Strongly disagree' },
		{ value: 2, label: 'Disagree' },
		{ value: 3, label: 'Neutral' },
		{ value: 4, label: 'Agree' },
		{ value: 5, label: 'Strongly agree' }
	];
}
