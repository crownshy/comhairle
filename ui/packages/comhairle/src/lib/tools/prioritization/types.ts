/** Portable types for the prioritization tool. No knowledge of comhairle or @crownshy/api-client lives here. The adapter is responsible for mapping backend DTOs into these shapes. */

export type Locale = string;

export type TextFormat = 'plain' | 'rich' | 'markdown';

export type TextContent = {
	id: string;
	primaryLocale: Locale;
	format: TextFormat;
};

export type TextTranslation = {
	id: string;
	contentId: string;
	locale: Locale;
	content: string;
	aiGenerated: boolean;
	requiresValidation: boolean;
};

/** One translatable field, in the exact shape the comhairle TranslatableField component expects under the hood. Keeping our internal shape aligned with it means components can pass `proposal.titleTranslations` straight in. */
export type TextContentWithTranslations = {
	textContent: TextContent;
	textTranslations: TextTranslation[];
};

export type Proposal = {
	id: string;
	workflowStepId: string;
	title: string;
	body: string;
	titleTranslations: TextContentWithTranslations;
	bodyTranslations: TextContentWithTranslations;
};

/** Lightweight variant used by participants — no translation envelopes, just the locale-resolved strings. Mirrors the backend's LocalizedProposalDto. */
export type LocalizedProposal = {
	id: string;
	workflowStepId: string;
	title: string;
	body: string;
};

/** Question definitions (from workflow_step.tool_config) */

export type LikertCategory = { label: string; value: number };

export type QuestionType =
	| { kind: 'text' }
	| { kind: 'likert'; categories: LikertCategory[] }
	| {
			kind: 'continuous';
			subSteps: number;
			minValue: number;
			maxValue: number;
			minLabel: string;
			maxLabel: string;
	  };

export type Question = {
	id: string;
	text: string;
	type: QuestionType;
};

export type ToolConfig = {
	questions: Question[];
	randomizeOrder: boolean;
};

/** Responses */

/** Backend payload only carries numeric values today; text-type questions are dropped at submit time. See README "Deferred / not built". */
export type QuestionResponse = { questionId: string; value: number };

export type ProposalResponse = {
	id: string;
	proposalId: string;
	userId: string;
	responses: QuestionResponse[];
};

/** Drafts (localStorage only) */
export type Draft = {
	stepId: string;
	participantId: string;
	/** Keyed by proposalId -> questionId -> value */
	answers: Record<string, Record<string, number>>;
	updatedAt: number;
};

export type Mode = 'manage' | 'user' | 'report';
