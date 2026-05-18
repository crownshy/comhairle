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

/** localStorage namespace. */
const NS = 'comhairle:prioritization';

const pollKey = (stepId: string) => `${NS}:${stepId}`;
const subsKey = (stepId: string) => `${NS}:${stepId}:submissions`;
const draftKey = (stepId: string, participantId: string) =>
	`${NS}:${stepId}:drafts:${participantId}`;
const participantKey = `${NS}:participant_id`;

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
	const proposals = Array.isArray(stored.proposals)
		? (stored.proposals as unknown as Array<Record<string, unknown>>).map((p, i) => ({
				id: typeof p.id === 'string' ? p.id : `proposal_${i}`,
				order: typeof p.order === 'number' ? p.order : i + 1,
				title: typeof p.title === 'string' ? p.title : '',
				body:
					(typeof p.body === 'string' && p.body) ||
					(typeof p.content === 'string' && p.content) ||
					'',
				imageDataUrl:
					typeof p.imageDataUrl === 'string' ? (p.imageDataUrl as string) : undefined
			}))
		: [];
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
		proposals,
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
 * Reactive store backing the prioritization tool's prototype state.
 *
 * Persists to `localStorage` keyed by workflow step id. The backend will
 * eventually own this state via `tool_config` JSONB plus `proposal` /
 * `proposal_question` / `question_response` rows; for now the entire blob
 * lives in the browser.
 */
export class PrioritizationStore {
	stepId: string;
	poll = $state<Poll>(emptyPoll('unknown'));
	submissions = $state<Submission[]>([]);

	constructor(stepId: string) {
		this.stepId = stepId;
		const stored = loadJSON<Partial<Poll>>(pollKey(stepId));
		this.poll = migratePoll(stepId, stored);
		this.submissions = loadJSON<Submission[]>(subsKey(stepId)) ?? [];
	}

	private persist(): void {
		saveJSON(pollKey(this.stepId), this.poll);
	}

	private persistSubmissions(): void {
		saveJSON(subsKey(this.stepId), this.submissions);
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

	// ---- PollEditor ------------------------------------------------------

	addProposal(): Proposal {
		const order = this.poll.proposals.length + 1;
		const p: Proposal = { id: uid(), order, title: '', body: '' };
		this.poll.proposals = [...this.poll.proposals, p];
		this.persist();
		return p;
	}

	updateProposal(proposalId: string, patch: Partial<Proposal>): void {
		this.poll.proposals = this.poll.proposals.map((p) =>
			p.id === proposalId ? { ...p, ...patch } : p
		);
		this.persist();
	}

	duplicateProposal(proposalId: string): Proposal | null {
		const original = this.poll.proposals.find((p) => p.id === proposalId);
		if (!original) return null;
		const copy: Proposal = {
			...original,
			id: uid(),
			order: this.poll.proposals.length + 1
		};
		this.poll.proposals = [...this.poll.proposals, copy];
		this.persist();
		return copy;
	}

	removeProposal(proposalId: string): void {
		this.poll.proposals = this.poll.proposals
			.filter((p) => p.id !== proposalId)
			.map((p, i) => ({ ...p, order: i + 1 }));
		this.persist();
	}

	/** Reorder proposals to match the given list of ids. Missing ids dropped. */
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
		this.persist();
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

	submitDraft(participantId: string): Submission | null {
		const draft = this.loadDraft(participantId);
		if (!this.draftIsComplete(draft)) return null;
		const submission: Submission = { ...draft, submittedAt: new Date().toISOString() };
		this.submissions = [
			...this.submissions.filter((s) => s.participantId !== participantId),
			submission
		];
		this.persistSubmissions();
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
