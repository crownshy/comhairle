/** Types for the prioritization tool. The api module (prioritizationApi.ts)
 * maps backend DTOs into these shapes. */

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

/** One translatable field, in the shape the comhairle TranslatableField
 * component expects, so components can pass `proposal.titleTranslations` straight in. */
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

/** Locale-resolved variant shown to participants. Mirrors the backend's LocalizedProposalDto. */
export type LocalizedProposal = {
	id: string;
	workflowStepId: string;
	title: string;
	body: string;
};

/** Question definitions (from the workflow step's tool config) */

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

/** Backend payload only carries numeric values today; text-type questions are
 * dropped at submit time. See README "Deferred". */
export type QuestionResponse = { questionId: string; value: number };

export type ProposalResponse = {
	id: string;
	proposalId: string;
	userId: string;
	responses: QuestionResponse[];
};

/** Props the host page passes into the tool UIs. */

export type WorkflowStepInput = {
	id: string;
	toolConfig?: unknown;
	previewToolConfig?: unknown;
};

export type ConversationInput = {
	primaryLocale?: string;
	isLive?: boolean;
	supportedLanguages?: string[];
};
