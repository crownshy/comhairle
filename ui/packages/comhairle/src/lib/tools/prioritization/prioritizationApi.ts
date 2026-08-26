import { invalidate } from '$app/navigation';
import { apiClient } from '@crownshy/api-client/client';
import type {
	PartialWorkflowStep,
	ProposalsListResponse,
	ProposalWithTranslationsDto as ApiProposalWithTranslations,
	Question as ApiQuestion
} from '@crownshy/api-client/api';
import type {
	LocalizedProposal,
	Proposal,
	ProposalResponse,
	Question,
	QuestionResponse,
	QuestionType,
	ToolConfig,
	DraftTranslatableJsonField,
	WorkflowStepInput
} from './types';
import { key } from '$lib/utils/invalidationKey';

/** API + DTO mapping */

/* ---------- Question type mapping ---------- */

/** Backend stores QuestionType as a key-tagged union ({ likert_scale: {...} }),
 * except the unit `Text` variant which serialises as the bare string "text".
 * The tool uses a `kind`-discriminated union. */
function normaliseQuestionType<TText>(raw: unknown): QuestionType<TText> {
	if (raw === 'text') return { kind: 'text' };
	if (raw && typeof raw === 'object') {
		const r = raw as Record<string, unknown>;
		if ('likert_scale' in r) {
			const ls = r.likert_scale as { categories?: { label: TText; value: number }[] };
			return { kind: 'likert', categories: ls.categories ?? [] };
		}
		if ('continuous' in r) {
			const c = r.continuous as {
				sub_steps?: number;
				min_value?: number;
				max_value?: number;
				min_label: TText;
				max_label: TText;
			};
			return {
				kind: 'continuous',
				subSteps: c.sub_steps ?? 10,
				minValue: c.min_value ?? 0,
				maxValue: c.max_value ?? 10,
				minLabel: c.min_label,
				maxLabel: c.max_label
			};
		}
	}
	return { kind: 'text' };
}

function normaliseQuestion<TText>(raw: unknown): Question<TText> {
	const r = (raw ?? {}) as { id?: string; text: TText; type?: unknown };
	if (!r.id) throw new Error('Question loaded from backend is missing an id.');
	return {
		id: r.id,
		text: r.text,
		type: normaliseQuestionType(r.type)
	};
}

function denormaliseQuestionType<TText>(
	type: QuestionType<TText>
): string | Record<string, unknown> {
	switch (type.kind) {
		case 'text':
			return 'text';
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

function denormaliseQuestion<TText>(q: Question<TText>): ApiQuestion {
	return {
		id: q.id,
		text: q.text,
		type: denormaliseQuestionType(q.type) as ApiQuestion['type']
	};
}

/** Resolve a workflow step's tool config into the tool's ToolConfig shape. Live
 * conversations read `toolConfig`; design/preview reads `previewToolConfig`. */
export function resolveToolConfig<TText>(
	workflowStep: WorkflowStepInput,
	isLive: boolean
): ToolConfig<TText> {
	const raw = (isLive ? workflowStep.toolConfig : workflowStep.previewToolConfig) as
		| {
				type?: string;
				questions?: unknown[];
				randomize_order?: boolean;
				alignment_question_id?: string;
				required_reviews?: number | null;
		  }
		| null
		| undefined;
	if (raw?.type !== 'prioritization')
		return {
			questions: [],
			sectionQuestions: [],
			randomizeOrder: false,
			alignmentQuestionId: ''
		};
	const withSections = raw as typeof raw & { section_questions?: unknown[] };
	return {
		questions: (raw.questions ?? []).map(normaliseQuestion<TText>),
		sectionQuestions: (withSections.section_questions ?? []).map(normaliseQuestion<TText>),
		randomizeOrder: Boolean(raw.randomize_order),
		alignmentQuestionId: raw.alignment_question_id,
		requiredReviews: raw.required_reviews ?? undefined
	};
}

/* ---------- Proposal mapping ---------- */

function mapWithTranslations(p: ApiProposalWithTranslations): Proposal {
	return {
		id: p.id,
		workflowStepId: p.workflowStepId,
		title: p.title,
		titleTranslations: p.titleTranslations,
		sections: [...p.sections]
			.sort((a, b) => a.position - b.position)
			.map((s) => ({
				id: s.id,
				position: s.position,
				body: s.body,
				bodyTranslations: s.bodyTranslations
			}))
	};
}

async function fetchWithTranslations(workflowStepId: string): Promise<Proposal[]> {
	const raw = (await apiClient.ListProposals({
		queries: { workflowStepId, withTranslations: true }
	})) as unknown as ProposalsListResponse;
	/** The endpoint returns an untagged union; admin callers always get the
	 * with-translations variant. The admin variant carries `titleTranslations`
	 * (the localized variant does not), so use that to tell them apart. Structural
	 * check + a single cast because TS can't narrow on `every()`. */
	const arr = raw as Array<{ titleTranslations?: unknown }>;
	if (!Array.isArray(arr) || arr.some((p) => !p?.titleTranslations)) {
		throw new Error(
			'Expected admin proposals with translations, got localized payload. Are you logged in as an admin?'
		);
	}
	return (arr as unknown as ApiProposalWithTranslations[]).map(mapWithTranslations);
}

/* ---------- Proposals ---------- */

/** Admin list — full translation envelopes for the editor. */
export function listProposals(workflowStepId: string): Promise<Proposal[]> {
	return fetchWithTranslations(workflowStepId);
}

/** Participant list — locale-resolved title + ordered section bodies. */
export async function listLocalizedProposals(workflowStepId: string): Promise<LocalizedProposal[]> {
	const raw = (await apiClient.ListProposals({
		queries: { workflowStepId, withTranslations: false }
	})) as unknown as Array<{
		id: string;
		workflowStepId: string;
		title: string;
		sections: Array<{ id: string; position: number; body: string }>;
	}>;
	return raw.map((p) => ({
		id: p.id,
		workflowStepId: p.workflowStepId,
		title: p.title,
		sections: [...(p.sections ?? [])]
			.sort((a, b) => a.position - b.position)
			.map((s) => ({ id: s.id, position: s.position, body: s.body }))
	}));
}

export async function createProposal(
	workflowStepId: string,
	input: { title: string; sections: string[] }
): Promise<Proposal> {
	const { id: createdId } = await apiClient.CreateProposal({
		title: input.title,
		sections: input.sections,
		workflow_step_id: workflowStepId
	});
	/** CreateProposal returns the raw ProposalDto (TextContent IDs only).
	 * Re-fetch with translations so the shape matches the rest of the list. */
	const list = await fetchWithTranslations(workflowStepId);
	const created = list.find((p) => p.id === createdId);
	if (!created) {
		throw new Error(`Proposal ${createdId} created but missing from list response.`);
	}
	return created;
}

export async function deleteProposal(id: string): Promise<void> {
	await apiClient.DeleteProposal(undefined, { params: { proposal_id: id } });
}

/** Append a new (optionally empty) section to a proposal. */
export async function addSection(proposalId: string, body: string): Promise<void> {
	await apiClient.CreateProposalSection({ body }, { params: { proposal_id: proposalId } });
}

export async function deleteSection(proposalId: string, sectionId: string): Promise<void> {
	await apiClient.DeleteProposalSection(undefined, {
		params: { proposal_id: proposalId, section_id: sectionId }
	});
}

/* ---------- Tool config ---------- */

export async function updateToolConfig(opts: {
	conversationId: string;
	workflowId: string;
	workflowStepId: string;
	toolConfig: ToolConfig;
	isLive: boolean;
}): Promise<void> {
	await putToolConfig({
		conversationId: opts.conversationId,
		workflowId: opts.workflowId,
		workflowStepId: opts.workflowStepId,
		isLive: opts.isLive,
		questions: opts.toolConfig.questions.map(denormaliseQuestion),
		sectionQuestions: opts.toolConfig.sectionQuestions.map(denormaliseQuestion),
		randomizeOrder: opts.toolConfig.randomizeOrder,
		alignmentQuestionId: opts.toolConfig.alignmentQuestionId,
		requiredReviews: opts.toolConfig.requiredReviews
	});
}

async function putToolConfig(opts: {
	conversationId: string;
	workflowId: string;
	workflowStepId: string;
	isLive: boolean;
	questions: ApiQuestion[];
	sectionQuestions: ApiQuestion[];
	randomizeOrder: boolean;
	alignmentQuestionId?: string;
	requiredReviews?: number;
}): Promise<void> {
	const payload = {
		type: 'prioritization' as const,
		questions: opts.questions,
		section_questions: opts.sectionQuestions,
		randomize_order: opts.randomizeOrder,
		...(opts.alignmentQuestionId && { alignment_question_id: opts.alignmentQuestionId }),
		...(opts.requiredReviews != null && { required_reviews: opts.requiredReviews })
	};
	const body: PartialWorkflowStep = opts.isLive
		? { tool_config: payload }
		: { preview_tool_config: payload };
	await apiClient.UpdateConversationWorkflowStep(body, {
		params: {
			conversation_id: opts.conversationId,
			workflow_id: opts.workflowId,
			workflow_step_id: opts.workflowStepId
		}
	});

	await invalidate(key('conversation/workflow'));
}

/* ---------- Responses ---------- */

export async function submitResponse(
	proposalId: string,
	responses: QuestionResponse[]
): Promise<void> {
	await apiClient.CreateProposalResponse(
		{
			question_responses: responses.map((r) => ({
				question_id: r.questionId,
				value: r.value,
				...(r.sectionId ? { section_id: r.sectionId } : {})
			}))
		},
		{ params: { proposal_id: proposalId } }
	);
}

export async function listResponses(proposalId: string): Promise<ProposalResponse[]> {
	const dtos = await apiClient.ListProposalResponses({ params: { proposal_id: proposalId } });
	return dtos.map((d) => ({
		id: d.id,
		proposalId: d.proposalId,
		userId: d.userId,
		responses: d.response.map((r) => {
			const sectionId = (r as { section_id?: string }).section_id;
			return {
				questionId: r.question_id,
				value: r.value,
				...(sectionId ? { sectionId } : {})
			};
		})
	}));
}
