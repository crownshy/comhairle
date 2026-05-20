import { apiClient } from '@crownshy/api-client/client';
import type { LocalizedProposalDto, ProposalResponseDto } from '@crownshy/api-client/api';

import {
	DEFAULT_CONTINUOUS_MAX,
	DEFAULT_CONTINUOUS_MIN,
	defaultLikertCategories,
	letterFor
} from './types';
import type {
	AnswerValue,
	ParticipantDraft,
	Poll,
	Proposal,
	ProposalAnswers,
	Question,
	QuestionType,
	Report,
	Submission,
	ToolConfig
} from './types';

/**
 * Local-only state still lives in `localStorage` keyed by workflow step id:
 *   - poll-wide questions + randomize flag (`tool_config` will move server-side
 *     when the backend gains an update endpoint)
 *   - report pages
 *   - per-participant in-progress drafts
 *
 * Proposals and submitted responses are owned by the backend
 * (`/tools/prioritization/proposals` and `.../responses`).
 */
const NS = 'comhairle:prioritization';

const pollKey = (stepId: string) => `${NS}:${stepId}`;
const draftKey = (stepId: string, participantId: string) =>
	`${NS}:${stepId}:drafts:${participantId}`;
const participantKey = `${NS}:participant_id`;

/** Debounce window for syncing proposal title/body edits to the backend. */
const SAVE_DEBOUNCE_MS = 500;

/** Stable random id helper (uses crypto when available). */
function uid(): string {
	if (typeof crypto !== 'undefined' && crypto.randomUUID) return crypto.randomUUID();
	return Math.random().toString(36).slice(2) + Date.now().toString(36);
}

/** Build a default question of the given type. */
function defaultQuestionConfig(type: QuestionType, order: number): Question {
	const base = {
		id: uid(),
		order,
		prompt: '',
		description: '',
		optional: false
	};
	switch (type) {
		case 'likert_scale':
			return { ...base, type, categories: defaultLikertCategories() };
		case 'continuous':
			return {
				...base,
				type,
				minValue: DEFAULT_CONTINUOUS_MIN,
				maxValue: DEFAULT_CONTINUOUS_MAX,
				minLabel: '',
				maxLabel: ''
			};
		case 'text':
			return { ...base, type };
	}
}

function emptyToolConfig(): ToolConfig {
	return { randomizeOrder: false, questions: [] };
}

function emptyPoll(stepId: string): Poll {
	return {
		id: stepId,
		title: '',
		description: '',
		toolConfig: emptyToolConfig(),
		proposals: [],
		report: { pages: [] }
	};
}

/**
 * Tolerate older localStorage payloads (instruction → description, missing
 * toolConfig, per-proposal questions, etc.) by mapping them into the current
 * shape. Anything unrecognised is dropped — this is best-effort, since the
 * tool is still pre-release.
 */
function migratePoll(stepId: string, stored: Partial<Poll> | null): Poll {
	if (!stored) return emptyPoll(stepId);
	const anyStored = stored as Record<string, unknown>;
	const description =
		(typeof stored.description === 'string' && stored.description) ||
		(typeof anyStored.instruction === 'string' && (anyStored.instruction as string)) ||
		'';
	const toolConfigRaw = (stored.toolConfig as ToolConfig | undefined) ?? emptyToolConfig();
	const toolConfig: ToolConfig = {
		randomizeOrder: Boolean(toolConfigRaw.randomizeOrder),
		questions: Array.isArray(toolConfigRaw.questions) ? toolConfigRaw.questions : []
	};
	return {
		id: typeof stored.id === 'string' ? stored.id : stepId,
		title: typeof stored.title === 'string' ? stored.title : '',
		description,
		toolConfig,
		// Proposals are owned by the backend now; ignore any cached copy.
		proposals: [],
		report: migrateReport(stored.report)
	};
}

function migrateReport(report: unknown): Report {
	if (!report || typeof report !== 'object') return { pages: [] };
	const r = report as Partial<Report>;
	if (!Array.isArray(r.pages)) return { pages: [] };
	return { pages: r.pages, publishedAt: r.publishedAt };
}

function loadJSON<T>(key: string): T | null {
	if (typeof localStorage === 'undefined') return null;
	const raw = localStorage.getItem(key);
	if (!raw) return null;
	try {
		return JSON.parse(raw) as T;
	} catch {
		return null;
	}
}

function saveJSON(key: string, value: unknown): void {
	if (typeof localStorage === 'undefined') return;
	localStorage.setItem(key, JSON.stringify(value));
}

/**
 * Map a server-side proposal response into the shape the aggregation code
 * expects. Each response becomes a single-proposal pseudo-submission keyed by
 * the response id, so identity grouping by participant is not preserved — the
 * aggregation only cares about the bag of answers per (proposal, question),
 * which this representation preserves.
 */
function responseToSubmission(r: ProposalResponseDto): Submission {
	const answers: ProposalAnswers = {};
	for (const a of r.response) {
		answers[a.question_id] = { kind: 'numeric', value: a.value };
	}
	const now = new Date().toISOString();
	return {
		participantId: r.id,
		byProposal: { [r.proposalId]: answers },
		startedAt: now,
		submittedAt: now
	};
}

/** Stable participant id for this browser, used for the prototype's "current user". */
export function getOrCreateParticipantId(): string {
	if (typeof localStorage === 'undefined') return 'anon';
	let id = localStorage.getItem(participantKey);
	if (!id) {
		id = uid();
		localStorage.setItem(participantKey, id);
	}
	return id;
}

/**
 * Reactive store backing the prioritization tool.
 *
 * Backend-owned (server):
 *   - proposals (`/tools/prioritization/proposals`)
 *   - submitted answers (`/tools/prioritization/proposals/{id}/responses`)
 *
 * Client-owned (localStorage, scoped by step id):
 *   - poll title/description (stub — backend doesn't yet have a slot)
 *   - tool config: questions, randomize_order (stub — `tool_config` is in DB
 *     but there is no update endpoint yet)
 *   - report pages
 *   - per-participant in-progress drafts
 */
export class PrioritizationStore {
	stepId: string;
	poll = $state<Poll>(emptyPoll('unknown'));
	submissions = $state<Submission[]>([]);
	proposalsLoading = $state(false);
	submissionsLoading = $state(false);

	private saveTimers = new Map<string, ReturnType<typeof setTimeout>>();
	private pendingPatches = new Map<string, { title?: string; body?: string }>();

	constructor(stepId: string) {
		this.stepId = stepId;
		const stored = loadJSON<Partial<Poll>>(pollKey(stepId));
		this.poll = migratePoll(stepId, stored);
		void this.loadProposals();
		void this.loadAllResponses();
	}

	private persist(): void {
		saveJSON(pollKey(this.stepId), this.poll);
	}

	// ---- Server I/O ----------------------------------------------------

	private proposalFromDto(dto: LocalizedProposalDto, order: number): Proposal {
		return {
			id: dto.id,
			order,
			title: dto.title,
			body: dto.body
		};
	}

	async loadProposals(): Promise<void> {
		this.proposalsLoading = true;
		try {
			const list = await apiClient.ListProposals({
				queries: { workflow_step_id: this.stepId }
			});
			this.poll.proposals = list.map((p, i) => this.proposalFromDto(p, i + 1));
		} catch (err) {
			console.error('Failed to load prioritization proposals', err);
		} finally {
			this.proposalsLoading = false;
		}
	}

	async loadAllResponses(): Promise<void> {
		this.submissionsLoading = true;
		try {
			if (this.poll.proposals.length === 0) {
				// Wait briefly for proposals to load if they haven't yet.
				await this.loadProposals();
			}
			const all = await Promise.all(
				this.poll.proposals.map((p) =>
					apiClient.ListProposalResponses({ params: { proposal_id: p.id } })
				)
			);
			this.submissions = all.flat().map((r) => responseToSubmission(r));
		} catch (err) {
			console.error('Failed to load prioritization responses', err);
		} finally {
			this.submissionsLoading = false;
		}
	}

	// ---- Top-level poll fields -----------------------------------------

	setTitle(title: string): void {
		this.poll.title = title;
		this.persist();
	}

	setDescription(description: string): void {
		this.poll.description = description;
		this.persist();
	}

	// ---- Tool config ----------------------------------------------------

	setRandomizeOrder(randomize: boolean): void {
		this.poll.toolConfig.randomizeOrder = randomize;
		this.persist();
	}

	// ---- Questions (poll-wide) -----------------------------------------

	addQuestion(type: QuestionType): Question {
		const order = this.poll.toolConfig.questions.length + 1;
		const q = defaultQuestionConfig(type, order);
		this.poll.toolConfig.questions = [...this.poll.toolConfig.questions, q];
		this.persist();
		return q;
	}

	removeQuestion(questionId: string): void {
		this.poll.toolConfig.questions = this.poll.toolConfig.questions
			.filter((q) => q.id !== questionId)
			.map((q, i) => ({ ...q, order: i + 1 }));
		this.persist();
	}

	updateQuestion(questionId: string, patch: Partial<Question>): void {
		this.poll.toolConfig.questions = this.poll.toolConfig.questions.map((q) => {
			if (q.id !== questionId) return q;
			// `type` changes are handled by replacing with a new default.
			if (patch.type && patch.type !== q.type) {
				return defaultQuestionConfig(patch.type, q.order);
			}
			return { ...q, ...patch } as Question;
		});
		this.persist();
	}

	duplicateQuestion(questionId: string): Question | null {
		const original = this.poll.toolConfig.questions.find((q) => q.id === questionId);
		if (!original) return null;
		const copy: Question = {
			...original,
			id: uid(),
			order: this.poll.toolConfig.questions.length + 1
		};
		this.poll.toolConfig.questions = [...this.poll.toolConfig.questions, copy];
		this.persist();
		return copy;
	}

	/** Reorder questions to match the given list of ids. Missing ids dropped. */
	reorderQuestions(orderedIds: string[]): void {
		const map = new Map(this.poll.toolConfig.questions.map((q) => [q.id, q]));
		const next: Question[] = [];
		orderedIds.forEach((id, i) => {
			const q = map.get(id);
			if (q) next.push({ ...q, order: i + 1 });
		});
		// Preserve any questions that weren't in orderedIds (defensive).
		for (const q of this.poll.toolConfig.questions) {
			if (!orderedIds.includes(q.id)) next.push({ ...q, order: next.length + 1 });
		}
		this.poll.toolConfig.questions = next;
		this.persist();
	}

	// ---- Likert scale category editing ---------------------------------

	addLikertCategory(questionId: string): void {
		this.updateLikert(questionId, (cats) => [...cats, { value: cats.length + 1, label: '' }]);
	}

	updateLikertCategory(
		questionId: string,
		index: number,
		patch: Partial<{ value: number; label: string }>
	): void {
		this.updateLikert(questionId, (cats) =>
			cats.map((c, i) => (i === index ? { ...c, ...patch } : c))
		);
	}

	removeLikertCategory(questionId: string, index: number): void {
		this.updateLikert(questionId, (cats) => cats.filter((_, i) => i !== index));
	}

	private updateLikert(
		questionId: string,
		fn: (cats: { value: number; label: string }[]) => { value: number; label: string }[]
	): void {
		this.poll.toolConfig.questions = this.poll.toolConfig.questions.map((q) => {
			if (q.id !== questionId || q.type !== 'likert_scale') return q;
			return { ...q, categories: fn(q.categories) };
		});
		this.persist();
	}

	// ---- Proposals (server-backed) -------------------------------------

	/**
	 * Create a new proposal on the server and append it locally. The returned
	 * proposal contains the server-assigned id; callers can synchronously open
	 * an editor on the result. On failure the local state is left unchanged.
	 */
	async addProposal(): Promise<Proposal | null> {
		try {
			const created = await apiClient.CreateProposal({
				workflow_step_id: this.stepId,
				title: '',
				body: ''
			});
			const p: Proposal = {
				id: created.id,
				order: this.poll.proposals.length + 1,
				title: '',
				body: ''
			};
			this.poll.proposals = [...this.poll.proposals, p];
			return p;
		} catch (err) {
			console.error('Failed to create proposal', err);
			return null;
		}
	}

	/**
	 * Patch a proposal locally, then debounce a PUT to the server. Multiple
	 * patches inside the debounce window are coalesced into a single request.
	 * `order` is local-only (not yet supported by the backend) and is skipped
	 * when syncing.
	 */
	updateProposal(proposalId: string, patch: Partial<Proposal>): void {
		this.poll.proposals = this.poll.proposals.map((p) =>
			p.id === proposalId ? { ...p, ...patch } : p
		);

		const sync: { title?: string; body?: string } = {};
		if (typeof patch.title === 'string') sync.title = patch.title;
		if (typeof patch.body === 'string') sync.body = patch.body;
		if (Object.keys(sync).length === 0) return;

		const merged = { ...(this.pendingPatches.get(proposalId) ?? {}), ...sync };
		this.pendingPatches.set(proposalId, merged);

		const existing = this.saveTimers.get(proposalId);
		if (existing) clearTimeout(existing);
		const timer = setTimeout(() => {
			void this.flushPendingPatch(proposalId);
		}, SAVE_DEBOUNCE_MS);
		this.saveTimers.set(proposalId, timer);
	}

	private async flushPendingPatch(proposalId: string): Promise<void> {
		const patch = this.pendingPatches.get(proposalId);
		this.pendingPatches.delete(proposalId);
		this.saveTimers.delete(proposalId);
		if (!patch) return;
		try {
			await apiClient.UpdateProposal(patch, { params: { proposal_id: proposalId } });
		} catch (err) {
			console.error('Failed to update proposal', err);
		}
	}

	async duplicateProposal(proposalId: string): Promise<Proposal | null> {
		const original = this.poll.proposals.find((p) => p.id === proposalId);
		if (!original) return null;
		try {
			const created = await apiClient.CreateProposal({
				workflow_step_id: this.stepId,
				title: original.title,
				body: original.body
			});
			const copy: Proposal = {
				id: created.id,
				order: this.poll.proposals.length + 1,
				title: original.title,
				body: original.body
			};
			this.poll.proposals = [...this.poll.proposals, copy];
			return copy;
		} catch (err) {
			console.error('Failed to duplicate proposal', err);
			return null;
		}
	}

	async removeProposal(proposalId: string): Promise<void> {
		const before = this.poll.proposals;
		this.poll.proposals = before
			.filter((p) => p.id !== proposalId)
			.map((p, i) => ({ ...p, order: i + 1 }));
		try {
			await apiClient.DeleteProposal(undefined, { params: { proposal_id: proposalId } });
		} catch (err) {
			console.error('Failed to delete proposal', err);
			this.poll.proposals = before; // rollback on failure
		}
	}

	/**
	 * Reorder proposals visually. The backend doesn't yet persist proposal
	 * order, so this only affects in-memory state and won't survive a reload.
	 */
	reorderProposals(orderedIds: string[]): void {
		const map = new Map(this.poll.proposals.map((p) => [p.id, p]));
		const next: Proposal[] = [];
		orderedIds.forEach((id, i) => {
			const p = map.get(id);
			if (p) next.push({ ...p, order: i + 1 });
		});
		for (const p of this.poll.proposals) {
			if (!orderedIds.includes(p.id)) next.push({ ...p, order: next.length + 1 });
		}
		this.poll.proposals = next;
	}

	// ---- Participant drafts --------------------------------------------

	loadDraft(participantId: string): ParticipantDraft {
		const existing = loadJSON<ParticipantDraft>(draftKey(this.stepId, participantId));
		return (
			existing ?? {
				participantId,
				byProposal: {},
				startedAt: new Date().toISOString()
			}
		);
	}

	saveDraft(draft: ParticipantDraft): void {
		saveJSON(draftKey(this.stepId, draft.participantId), draft);
	}

	setDraftAnswer(
		participantId: string,
		proposalId: string,
		questionId: string,
		value: AnswerValue
	): ParticipantDraft {
		const draft = this.loadDraft(participantId);
		const existing: ProposalAnswers = draft.byProposal[proposalId] ?? {};
		existing[questionId] = value;
		draft.byProposal[proposalId] = existing;
		this.saveDraft(draft);
		return draft;
	}

	/**
	 * Submit the participant's draft answers to the backend, one POST per
	 * proposal. Text answers are dropped silently — backend `Response` only
	 * carries a numeric `value`, so text questions are not yet round-tripped.
	 */
	async submitDraft(participantId: string): Promise<Submission | null> {
		const draft = this.loadDraft(participantId);
		if (!this.draftIsComplete(draft)) return null;

		try {
			for (const proposal of this.poll.proposals) {
				const answers = draft.byProposal[proposal.id] ?? {};
				const question_responses = Object.entries(answers)
					.filter(([, v]) => v.kind === 'numeric')
					.map(([question_id, v]) => ({
						question_id,
						value: (v as Extract<AnswerValue, { kind: 'numeric' }>).value
					}));
				if (question_responses.length === 0) continue;
				await apiClient.CreateProposalResponse(
					{ question_responses },
					{ params: { proposal_id: proposal.id } }
				);
			}
		} catch (err) {
			console.error('Failed to submit proposal responses', err);
			return null;
		}

		const submission: Submission = { ...draft, submittedAt: new Date().toISOString() };
		this.submissions = [
			...this.submissions.filter((s) => s.participantId !== participantId),
			submission
		];
		return submission;
	}

	draftIsComplete(draft: ParticipantDraft): boolean {
		const questions = this.poll.toolConfig.questions;
		for (const p of this.poll.proposals) {
			const ans: ProposalAnswers = draft.byProposal[p.id] ?? {};
			for (const q of questions) {
				if (q.optional) continue;
				const a = ans[q.id];
				if (!a) return false;
				if (a.kind === 'text' && !a.value.trim()) return false;
			}
		}
		return true;
	}

	missingAnswers(draft: ParticipantDraft): Array<{ proposalId: string; questionId: string }> {
		const out: Array<{ proposalId: string; questionId: string }> = [];
		const questions = this.poll.toolConfig.questions;
		for (const p of this.poll.proposals) {
			const ans: ProposalAnswers = draft.byProposal[p.id] ?? {};
			for (const q of questions) {
				if (q.optional) continue;
				const a = ans[q.id];
				const missing = !a || (a.kind === 'text' && !a.value.trim());
				if (missing) out.push({ proposalId: p.id, questionId: q.id });
			}
		}
		return out;
	}

	// ---- Report ---------------------------------------------------------

	addReportPage(): void {
		const order = this.poll.report.pages.length + 1;
		this.poll.report.pages = [...this.poll.report.pages, { id: uid(), order, content: '' }];
		this.persist();
	}

	updateReportPage(id: string, content: string): void {
		this.poll.report.pages = this.poll.report.pages.map((p) =>
			p.id === id ? { ...p, content } : p
		);
		this.persist();
	}

	removeReportPage(id: string): void {
		this.poll.report.pages = this.poll.report.pages
			.filter((p) => p.id !== id)
			.map((p, i) => ({ ...p, order: i + 1 }));
		this.persist();
	}

	publishReport(): void {
		this.poll.report.publishedAt = new Date().toISOString();
		this.persist();
	}
}

export { letterFor };
