import type { PrioritizationAdapter } from '../adapter';
import type {
	Draft,
	Proposal,
	ProposalResponse,
	QuestionResponse,
	TextContentWithTranslations,
	ToolConfig
} from '../types';

/** In-memory adapter for Storybook, tests, and offline dev. No network, no localStorage by default. */

let idCounter = 0;
const id = (prefix: string) => `${prefix}-${++idCounter}`;

export function createMemoryAdapter(opts?: {
	primaryLocale?: string;
	seedProposals?: Array<{ title: string; body: string }>;
}): PrioritizationAdapter {
	const primaryLocale = opts?.primaryLocale ?? 'en';
	const proposals = new Map<string, Proposal>();
	const responses = new Map<string, ProposalResponse[]>();
	const drafts = new Map<string, Draft>();

	function makeEnvelope(content: string, format: 'plain' | 'rich'): TextContentWithTranslations {
		const contentId = id('text');
		return {
			textContent: { id: contentId, primaryLocale, format },
			textTranslations: [
				{
					id: id('translation'),
					contentId,
					locale: primaryLocale,
					content,
					aiGenerated: false,
					requiresValidation: false
				}
			]
		};
	}

	const makeProposal = (title: string, body: string): Proposal => ({
		id: id('proposal'),
		workflowStepId: 'memory-step',
		title,
		body,
		titleTranslations: makeEnvelope(title, 'plain'),
		bodyTranslations: makeEnvelope(body, 'rich')
	});

	for (const seed of opts?.seedProposals ?? []) {
		const p = makeProposal(seed.title, seed.body);
		proposals.set(p.id, p);
	}

	return {
		async listProposals() {
			return [...proposals.values()];
		},
		async listLocalizedProposals() {
			return [...proposals.values()].map((p) => ({
				id: p.id,
				workflowStepId: p.workflowStepId,
				title: p.title,
				body: p.body
			}));
		},
		async createProposal(input) {
			const p = makeProposal(input.title, input.body);
			proposals.set(p.id, p);
			return p;
		},
		async deleteProposal(idToDelete) {
			proposals.delete(idToDelete);
		},
		async updateTranslation(textContentId, locale, value) {
			for (const p of proposals.values()) {
				for (const slot of [p.titleTranslations, p.bodyTranslations]) {
					if (slot.textContent.id !== textContentId) continue;
					const existing = slot.textTranslations.find((t) => t.locale === locale);
					if (existing) existing.content = value;
					else
						slot.textTranslations.push({
							id: id('translation'),
							contentId: textContentId,
							locale,
							content: value,
							aiGenerated: false,
							requiresValidation: true
						});
				}
			}
		},
		async updateToolConfig(_toolConfig: ToolConfig) {
			/** No-op for the in-memory fake. */
		},
		async submitResponse(proposalId, response: QuestionResponse[]) {
			const list = responses.get(proposalId) ?? [];
			list.push({
				id: id('response'),
				proposalId,
				userId: 'memory-user',
				responses: response
			});
			responses.set(proposalId, list);
		},
		async listResponses(proposalId) {
			return responses.get(proposalId) ?? [];
		},
		async clearMyResponses(proposalId) {
			/** Memory fake has no concept of "current user" — just drop everything for the proposal to keep behaviour predictable in tests. */
			responses.delete(proposalId);
		},
		loadDraft(participantId) {
			return drafts.get(participantId) ?? null;
		},
		saveDraft(draft) {
			drafts.set(draft.participantId, draft);
		},
		clearDraft(participantId) {
			drafts.delete(participantId);
		}
	};
}
