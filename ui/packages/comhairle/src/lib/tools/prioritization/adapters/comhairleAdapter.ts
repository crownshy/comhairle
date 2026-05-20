import { apiClient } from '@crownshy/api-client/client';
import { invalidateAll } from '$app/navigation';
import type {
	ProposalsListResponse,
	ProposalWithTranslations as ApiProposalWithTranslations
} from '@crownshy/api-client/api';
import type { PrioritizationAdapter } from '../adapter';
import type {
	Draft,
	LocalizedProposal,
	Proposal,
	ProposalResponse,
	Question,
	QuestionResponse,
	QuestionType,
	TextContentWithTranslations,
	ToolConfig
} from '../types';

/** Backend QuestionType is a key-tagged union; convert from our portable `kind`-discriminated shape before sending. */
function denormaliseQuestionType(type: QuestionType): Record<string, unknown> {
	switch (type.kind) {
		case 'text':
			return { text: '' };
		case 'likert':
			return { likert_scale: { categories: type.categories } };
		case 'continuous':
			return {
				continuous: {
					sub_steps: type.subSteps,
					min_value: type.minValue,
					max_value: type.maxValue,
					min_label: type.minLabel,
					max_label: type.maxLabel
				}
			};
	}
}

function denormaliseQuestion(q: Question): Record<string, unknown> {
	return { id: q.id, text: q.text, type: denormaliseQuestionType(q.type) };
}

function toTranslationEnvelope(
	t: ApiProposalWithTranslations['translations']['title']
): TextContentWithTranslations {
	return {
		textContent: {
			id: t.textContent.id,
			primaryLocale: t.textContent.primaryLocale,
			format: t.textContent.format as TextContentWithTranslations['textContent']['format']
		},
		textTranslations: t.textTranslations.map((tt) => ({
			id: tt.id,
			contentId: tt.contentId,
			locale: tt.locale,
			content: tt.content,
			aiGenerated: tt.aiGenerated,
			requiresValidation: tt.requiresValidation
		}))
	};
}

function mapWithTranslations(p: ApiProposalWithTranslations): Proposal {
	return {
		id: p.id,
		workflowStepId: p.workflowStepId,
		title: p.title,
		body: p.body,
		titleTranslations: toTranslationEnvelope(p.translations.title),
		bodyTranslations: toTranslationEnvelope(p.translations.body)
	};
}

const draftKey = (stepId: string, participantId: string) =>
	`prioritization.draft.${stepId}.${participantId}`;

export function createComhairleAdapter(opts: {
	conversationId: string;
	workflowId: string;
	workflowStepId: string;
	stepId: string; // same as workflowStepId but kept explicit for draft keying
}): PrioritizationAdapter {
	const { conversationId, workflowId, workflowStepId, stepId } = opts;

	async function fetchWithTranslations(): Promise<Proposal[]> {
		const raw = (await apiClient.ListProposals({
			queries: { workflowStepId, withTranslations: true }
		})) as unknown as ProposalsListResponse;
		/** The endpoint returns an untagged union; admin callers always get the with-translations variant. Use a structural check + a single cast because TS can't narrow on `every()`. */
		const arr = raw as Array<{ translations?: unknown }>;
		if (!Array.isArray(arr) || arr.some((p) => !p?.translations)) {
			throw new Error(
				'Expected admin proposals with translations, got localized payload. Are you logged in as an admin?'
			);
		}
		return (arr as unknown as ApiProposalWithTranslations[]).map(mapWithTranslations);
	}

	return {
		async listProposals(): Promise<Proposal[]> {
			return fetchWithTranslations();
		},

		async listLocalizedProposals(): Promise<LocalizedProposal[]> {
			const raw = (await apiClient.ListProposals({
				queries: { workflowStepId, withTranslations: false }
			})) as unknown as Array<{
				id: string;
				workflowStepId: string;
				title: string;
				body: string;
			}>;
			return raw.map((p) => ({
				id: p.id,
				workflowStepId: p.workflowStepId,
				title: p.title,
				body: p.body
			}));
		},

		async createProposal(input) {
			await apiClient.CreateProposal({
				title: input.title,
				body: input.body,
				workflow_step_id: workflowStepId
			});
			/** CreateProposal returns the raw ProposalDto (TextContent IDs only). Easier and safer to re-fetch with translations so the shape matches the rest of the list. */
			const list = await fetchWithTranslations();
			const created = list[list.length - 1];
			if (!created) {
				throw new Error('Proposal created but missing from list response.');
			}
			return created;
		},

		async deleteProposal(id) {
			await apiClient.DeleteProposal(undefined, { params: { proposal_id: id } });
		},

		async updateTranslation(textContentId, locale, value) {
			/** PUT updates an existing translation; if none exists for this locale we fall back to POST which create-or-updates. */
			try {
				await apiClient.UpdateTextTranslation(
					{ content: value },
					{ params: { text_content_id: textContentId, locale } }
				);
			} catch {
				await apiClient.CreateOrUpdateTextTranslation(
					{ content: value },
					{ params: { text_content_id: textContentId, locale } }
				);
			}
		},

		async updateToolConfig(toolConfig: ToolConfig) {
			/** Following the Polis pattern: writes go to preview_tool_config. Live tool_config picks the latest preview the next time the conversation is launched. */
			const payload = {
				type: 'prioritization' as const,
				questions: toolConfig.questions.map(denormaliseQuestion),
				randomize_order: toolConfig.randomizeOrder
			};
			await apiClient.UpdateConversationWorkflowStep(
				{ preview_tool_config: payload } as unknown as Parameters<
					typeof apiClient.UpdateConversationWorkflowStep
				>[0],
				{
					params: {
						conversation_id: conversationId,
						workflow_id: workflowId,
						workflow_step_id: workflowStepId
					}
				}
			);
			/**
			 * The wrapper reads tool_config off the workflow_step prop.
			 * Force a page-level reload so the updated config flows back through props → StepContext → Manage view.
			 */
			await invalidateAll();
		},

		/**
		 * Submit a response.
		 */
		async submitResponse(proposalId, responses) {
			await apiClient.CreateProposalResponse(
				{
					question_responses: responses.map((r) => ({
						question_id: r.questionId,
						value: r.value
					}))
				},
				{ params: { proposal_id: proposalId } }
			);
		},

		async clearMyResponses(proposalId) {
			await apiClient.DeleteMyProposalResponses(undefined, {
				params: { proposal_id: proposalId }
			});
		},

		async listResponses(proposalId): Promise<ProposalResponse[]> {
			const dtos = await apiClient.ListProposalResponses({
				params: { proposal_id: proposalId }
			});
			return dtos.map((d) => ({
				id: d.id,
				proposalId: d.proposalId,
				userId: d.userId,
				responses: d.response.map((r) => ({ questionId: r.question_id, value: r.value }))
			}));
		},

		loadDraft(participantId): Draft | null {
			if (typeof window === 'undefined') return null;
			const raw = window.localStorage.getItem(draftKey(stepId, participantId));
			if (!raw) return null;
			try {
				return JSON.parse(raw) as Draft;
			} catch {
				return null;
			}
		},

		saveDraft(draft) {
			if (typeof window === 'undefined') return;
			window.localStorage.setItem(
				draftKey(draft.stepId, draft.participantId),
				JSON.stringify(draft)
			);
		},

		clearDraft(participantId) {
			if (typeof window === 'undefined') return;
			window.localStorage.removeItem(draftKey(stepId, participantId));
		}
	};
}

export type { QuestionResponse };
