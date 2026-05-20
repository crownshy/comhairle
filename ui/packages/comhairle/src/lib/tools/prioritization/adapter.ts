import type {
	Draft,
	LocalizedProposal,
	Proposal,
	ProposalResponse,
	QuestionResponse,
	ToolConfig
} from './types';

/** The contract between the portable PrioritizationTool and whatever environment hosts it (comhairle backend, Storybook, tests).
 *
 * Only `comhairleAdapter` and `memoryAdapter` may import outside this folder. Everything else goes through this interface.
 */
export type PrioritizationAdapter = {
	/** Proposals — one list method. Returns resolved title/body so both admin and participant views share the same call. There is no localized list endpoint on the backend; locale resolution happens client-side via `pickLocalized` helpers in components. */
	listProposals(): Promise<Proposal[]>;
	/** Lightweight participant variant — no translation envelopes, no admin gating. */
	listLocalizedProposals(): Promise<LocalizedProposal[]>;

	createProposal(input: { title: string; body: string }): Promise<Proposal>;

	/** Stubbed in v1: the backend has no DeleteProposal endpoint. The comhairle implementation surfaces a toast and rejects. When the endpoint exists, swap the body of this method only. */
	deleteProposal(id: string): Promise<void>;

	/** Edits flow through the translations API. There is no UpdateProposal endpoint — title/body are TextContent entries and we mutate them per locale. */
	updateTranslation(textContentId: string, locale: string, value: string): Promise<void>;

	/** Tool config — questions + randomize_order. The portable tool stores these in `StepContext`, but writing them back requires the host to know how the config is persisted (workflow_step.tool_config in comhairle). The adapter hides that detail. */
	updateToolConfig(toolConfig: ToolConfig): Promise<void>;

	/** Responses */
	submitResponse(proposalId: string, response: QuestionResponse[]): Promise<void>;
	listResponses(proposalId: string): Promise<ProposalResponse[]>;

	/** Dev-mode reset: removes every response the calling user has submitted for the given proposal. Backed by DeleteMyProposalResponses on the real adapter; an in-memory no-op on the fake. */
	clearMyResponses(proposalId: string): Promise<void>;

	/** Drafts — purely local, no network */
	loadDraft(participantId: string): Draft | null;
	saveDraft(draft: Draft): void;
	clearDraft(participantId: string): void;
};
