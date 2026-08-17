/** Types for the prioritization tool. Re-exports generated DTOs from the
 * api-client where shapes match the UI's needs, and defines local shapes
 * (Proposal, QuestionType, ToolConfig, responses) where the UI deliberately
 * differs from the backend (kind-tagged unions, camelCase, etc.). The api
 * module (prioritizationApi.ts) maps between them. */

import type {
	LocalizedProposalDto,
	TextContentDto,
	TextFormat as ApiTextFormat,
	TextTranslationDto,
	Translation,
	TranslationDto
} from '@crownshy/api-client/api';

export type Locale = string;

export type TextFormat = ApiTextFormat;

export type TextContent = TextContentDto;

export type TextTranslation = TextTranslationDto;

/** One translatable field, in the shape the comhairle TranslatableField
 * component expects. Same shape as the backend `Translation` DTO. */
export type TextContentWithTranslations = Translation;

/** One translatable body section of a proposal, in the admin editor's shape. */
export type ProposalSection = {
	id: string;
	position: number;
	body: string;
	bodyTranslations: TextContentWithTranslations;
};

export type Proposal = {
	id: string;
	workflowStepId: string;
	title: string;
	titleTranslations: TextContentWithTranslations;
	sections: ProposalSection[];
};

/** Locale-resolved variant shown to participants (title + ordered section bodies). */
export type LocalizedProposal = LocalizedProposalDto;

/** Form state for a question being created or edited.  */
export type DraftFields = { text: DraftTranslatableJsonField; type: DraftQuestionType };

export type QuestionType<TText> =
	| { kind: 'text' }
	| { kind: 'likert'; categories: LikertCategory<TText>[] }
	| {
			kind: 'continuous';
			subSteps: number;
			minValue: number;
			maxValue: number;
			maxLabel: TText;
			minLabel: TText;
	  };

export type Question<TText> = {
	id: string;
	text: TText;
	type: QuestionType<TText>;
};

/** Question definitions (from the workflow step's tool config) */

export type LikertCategory<TText> = {
	label: TText;
	value: number;
};

export type ToolConfig<TText> = {
	/** Questions asked once about the proposal as a whole. */
	questions: Question<TText>[];
	/** Questions asked about each section (same set for every section). */
	sectionQuestions: Question<TText>[];
	randomizeOrder: boolean;
	alignmentQuestionId: string;
	/** Minimum proposals a participant must review before they can continue to the
	 * next step. Unset means every proposal must be reviewed, which is the default;
	 * an admin sets a number only to loosen that. Clamped to the proposal count at
	 * gate time so the bar is never impossible. */
	requiredReviews?: number;
};

/** ---------------------------- **/
/** WITH TRANSLATIONS PRIORITIZATION TOOL TYPES FOR ADMIN UI **/
/** ---------------------------- **/

/** Mirror type of the backend JsonFieldWithTranslations with optional `translations`
 * field to allow creating new translatable fields on questions
 */
export type DraftTranslatableJsonField = {
	localized: string;
	translations?: TranslationDto;
};

export type DraftQuestion = Question<DraftTranslatableJsonField>;
export type DraftQuestionType = QuestionType<DraftTranslatableJsonField>;
export type DraftLikertCategory = LikertCategory<DraftTranslatableJsonField>;

/** Responses */

/** Numeric for likert/continuous; string for free-text. The backend
 * `value` field is an untagged enum so JSON is `value: 4.5` or `value: "..."`.
 * `sectionId` is set for section-question answers and omitted for proposal-wide ones. */
export type QuestionResponse = { questionId: string; value: number | string; sectionId?: string };

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
