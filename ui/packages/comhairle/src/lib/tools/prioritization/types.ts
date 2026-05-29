/** Types for the prioritization tool. Re-exports generated DTOs from the
 * api-client where shapes match the UI's needs, and defines local shapes
 * (Proposal, QuestionType, ToolConfig, responses) where the UI deliberately
 * differs from the backend (kind-tagged unions, camelCase, etc.). The api
 * module (prioritizationApi.ts) maps between them. */

import type {
	Category,
	LocalizedProposalDto,
	TextContentDto,
	TextFormat as ApiTextFormat,
	TextTranslationDto,
	Translation
} from '@crownshy/api-client/api';

export type Locale = string;

export type TextFormat = ApiTextFormat;

export type TextContent = TextContentDto;

export type TextTranslation = TextTranslationDto;

/** One translatable field, in the shape the comhairle TranslatableField
 * component expects. Same shape as the backend `Translation` DTO. */
export type TextContentWithTranslations = Translation;

export type Proposal = {
	id: string;
	workflowStepId: string;
	title: string;
	body: string;
	titleTranslations: TextContentWithTranslations;
	bodyTranslations: TextContentWithTranslations;
};

/** Locale-resolved variant shown to participants. */
export type LocalizedProposal = LocalizedProposalDto;

/** Question definitions (from the workflow step's tool config) */

export type LikertCategory = Category;

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

/** Numeric for likert/continuous; string for free-text. The backend
 * `value` field is an untagged enum so JSON is `value: 4.5` or `value: "..."`. */
export type QuestionResponse = { questionId: string; value: number | string };

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
