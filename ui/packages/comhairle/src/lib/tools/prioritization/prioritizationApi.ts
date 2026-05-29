import { invalidateAll } from '$app/navigation';
import { apiClient } from '@crownshy/api-client/client';
import type {
	ProposalsListResponse,
	ProposalWithTranslations as ApiProposalWithTranslations
} from '@crownshy/api-client/api';
import type {
	LocalizedProposal,
	Proposal,
	ProposalResponse,
	Question,
	QuestionResponse,
	QuestionType,
	ToolConfig,
	WorkflowStepInput
} from './types';

/** API + DTO mapping */

/* ---------- Question type mapping ---------- */

/** Backend stores QuestionType as a key-tagged union ({ likert_scale: {...} }),
 * except the unit `Text` variant which serialises as the bare string "text".
 * The tool uses a `kind`-discriminated union. */
function normaliseQuestionType(raw: unknown): QuestionType {
	if (raw === 'text') return { kind: 'text' };
	if (raw && typeof raw === 'object') {
		const r = raw as Record<string, unknown>;
		if ('likert_scale' in r) {
			const ls = r.likert_scale as { categories?: { label: string; value: number }[] };
			return { kind: 'likert', categories: ls.categories ?? [] };
		}
		if ('continuous' in r) {
			const c = r.continuous as {
				sub_steps?: number;
				min_value?: number;
				max_value?: number;
				min_label?: string;
				max_label?: string;
			};
			return {
				kind: 'continuous',
				subSteps: c.sub_steps ?? 10,
				minValue: c.min_value ?? 0,
				maxValue: c.max_value ?? 10,
				minLabel: c.min_label ?? '',
				maxLabel: c.max_label ?? ''
			};
		}
	}
	return { kind: 'text' };
}

function normaliseQuestion(raw: unknown): Question {
	const r = (raw ?? {}) as { id?: string; text?: string; type?: unknown };
	return {
		id: r.id ?? crypto.randomUUID(),
		text: r.text ?? '',
		type: normaliseQuestionType(r.type)
	};
}

function denormaliseQuestionType(type: QuestionType): string | Record<string, unknown> {
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

function denormaliseQuestion(q: Question): Record<string, unknown> {
	return { id: q.id, text: q.text, type: denormaliseQuestionType(q.type) as unknown };
}

/** Resolve a workflow step's tool config into the tool's ToolConfig shape. Live
 * conversations read `toolConfig`; design/preview reads `previewToolConfig`. */
export function resolveToolConfig(workflowStep: WorkflowStepInput, isLive: boolean): ToolConfig {
	const raw = (isLive ? workflowStep.toolConfig : workflowStep.previewToolConfig) as
		| { type?: string; questions?: unknown[]; randomize_order?: boolean }
		| null
		| undefined;
	if (raw?.type !== 'prioritization') return { questions: [], randomizeOrder: false };
	return {
		questions: (raw.questions ?? []).map(normaliseQuestion),
		randomizeOrder: Boolean(raw.randomize_order)
	};
}

/* ---------- Proposal mapping ---------- */

function mapWithTranslations(p: ApiProposalWithTranslations): Proposal {
	return {
		id: p.id,
		workflowStepId: p.workflowStepId,
		title: p.title,
		body: p.body,
		titleTranslations: p.translations.title,
		bodyTranslations: p.translations.body
	};
}

async function fetchWithTranslations(workflowStepId: string): Promise<Proposal[]> {
	const raw = (await apiClient.ListProposals({
		queries: { workflowStepId, withTranslations: true }
	})) as unknown as ProposalsListResponse;
	/** The endpoint returns an untagged union; admin callers always get the
	 * with-translations variant. Structural check + a single cast because TS
	 * can't narrow on `every()`. */
	const arr = raw as Array<{ translations?: unknown }>;
	if (!Array.isArray(arr) || arr.some((p) => !p?.translations)) {
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

/** Participant list — locale-resolved title/body only. */
export async function listLocalizedProposals(workflowStepId: string): Promise<LocalizedProposal[]> {
	const raw = (await apiClient.ListProposals({
		queries: { workflowStepId, withTranslations: false }
	})) as unknown as Array<{ id: string; workflowStepId: string; title: string; body: string }>;
	return raw.map((p) => ({
		id: p.id,
		workflowStepId: p.workflowStepId,
		title: p.title,
		body: p.body
	}));
}

export async function createProposal(
	workflowStepId: string,
	input: { title: string; body: string }
): Promise<Proposal> {
	const { id: createdId } = await apiClient.CreateProposal({
		title: input.title,
		body: input.body,
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

/* ---------- Tool config ---------- */

export async function updateToolConfig(opts: {
	conversationId: string;
	workflowId: string;
	workflowStepId: string;
	toolConfig: ToolConfig;
	isLive: boolean;
}): Promise<void> {
	const payload = {
		type: 'prioritization' as const,
		questions: opts.toolConfig.questions.map(denormaliseQuestion),
		randomize_order: opts.toolConfig.randomizeOrder
	};
	await apiClient.UpdateConversationWorkflowStep(
		(opts.isLive
			? { tool_config: payload }
			: { preview_tool_config: payload }) as unknown as Parameters<
			typeof apiClient.UpdateConversationWorkflowStep
		>[0],
		{
			params: {
				conversation_id: opts.conversationId,
				workflow_id: opts.workflowId,
				workflow_step_id: opts.workflowStepId
			}
		}
	);

	await invalidateAll();
}

/* ---------- Responses ---------- */

export async function submitResponse(
	proposalId: string,
	responses: QuestionResponse[]
): Promise<void> {
	/** Backend `value` is an untagged enum accepting number | string, but the
	 * generated zod schema still types it as number. Cast until the api-client
	 * is regenerated. */
	await apiClient.CreateProposalResponse(
		{
			question_responses: responses.map((r) => ({
				question_id: r.questionId,
				value: r.value as number
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
		/** Backend `value` is number | string at runtime; the generated zod schema
		 * narrows to number. Widen here until the api-client is regenerated. */
		responses: d.response.map((r) => ({
			questionId: r.question_id,
			value: r.value as number | string
		}))
	}));
}
