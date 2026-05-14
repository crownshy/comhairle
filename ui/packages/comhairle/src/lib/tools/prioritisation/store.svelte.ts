import type {
	Poll,
	Proposal,
	Question,
	QuestionType,
	Submission,
	ParticipantDraft,
	ProposalAnswers,
	AnswerValue
} from './types';
import { letterFor } from './types';

/** localStorage namespace. */
const NS = 'comhairle:prioritisation';

const pollKey = (stepId: string) => `${NS}:${stepId}`;
const subsKey = (stepId: string) => `${NS}:${stepId}:submissions`;
const draftKey = (stepId: string, participantId: string) =>
	`${NS}:${stepId}:drafts:${participantId}`;
const participantKey = `${NS}:participant_id`;

/** Generate a 5-digit zero-padded join code. */
function generateJoinCode(): string {
	return Math.floor(Math.random() * 100000)
		.toString()
		.padStart(5, '0');
}

/** Stable random id helper (uses crypto when available). */
function uid(): string {
	if (typeof crypto !== 'undefined' && crypto.randomUUID) return crypto.randomUUID();
	return Math.random().toString(36).slice(2) + Date.now().toString(36);
}

function defaultQuestionConfig(type: QuestionType, order: number): Question {
	const base = {
		id: uid(),
		order,
		prompt: '',
		description: '',
		optional: false
	};
	switch (type) {
		case 'single_line':
			return { ...base, type };
		case 'long_text':
			return { ...base, type };
		case 'multiple_choice':
			return {
				...base,
				type,
				choices: [
					{ id: uid(), label: '' },
					{ id: uid(), label: '' }
				]
			};
		case 'five_star':
			return { ...base, type };
		case 'rating_scale':
			return {
				...base,
				type,
				min: 0,
				max: 10,
				minLabel: '',
				maxLabel: ''
			};
	}
}

function emptyPoll(stepId: string): Poll {
	return {
		id: stepId,
		title: '',
		instruction: '',
		proposals: [],
		settings: {
			timerSeconds: null,
			proposalSortMode: 'by_proposal_id'
		},
		state: 'draft',
		joinCode: generateJoinCode(),
		pausedAccumulatedSeconds: 0,
		report: { pages: [] }
	};
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
 * Reactive store for one prioritisation poll, keyed by workflow step id.
 *
 * Hydrates from localStorage on construction; persists on every mutation.
 */
export class PrioritisationStore {
	stepId: string;
	poll = $state<Poll>(emptyPoll('placeholder'));
	submissions = $state<Submission[]>([]);

	constructor(stepId: string) {
		this.stepId = stepId;
		const existing = loadJSON<Poll>(pollKey(stepId));
		this.poll = existing ?? emptyPoll(stepId);
		// Migrate older blobs missing fields:
		if (!this.poll.report) this.poll.report = { pages: [] };
		if (!this.poll.settings) {
			this.poll.settings = { timerSeconds: null, proposalSortMode: 'by_proposal_id' };
		}
		for (const p of this.poll.proposals) {
			if (!p.questions) p.questions = [];
		}
		this.submissions = loadJSON<Submission[]>(subsKey(stepId)) ?? [];
	}

	private persist(): void {
		saveJSON(pollKey(this.stepId), this.poll);
	}

	private persistSubmissions(): void {
		saveJSON(subsKey(this.stepId), this.submissions);
	}

	// ---- Poll metadata --------------------------------------------------

	setTitle(title: string): void {
		this.poll.title = title;
		this.persist();
	}

	setInstruction(instruction: string): void {
		this.poll.instruction = instruction;
		this.persist();
	}

	setTimer(seconds: number | null): void {
		this.poll.settings.timerSeconds = seconds;
		this.persist();
	}

	setProposalSortMode(mode: 'by_proposal_id'): void {
		this.poll.settings.proposalSortMode = mode;
		this.persist();
	}

	// ---- Proposals ------------------------------------------------------

	addProposal(): Proposal {
		const order = this.poll.proposals.length + 1;
		const p: Proposal = { id: uid(), order, title: '', content: '', questions: [] };
		this.poll.proposals = [...this.poll.proposals, p];
		this.persist();
		return p;
	}

	updateProposal(id: string, patch: Partial<Proposal>): void {
		this.poll.proposals = this.poll.proposals.map((p) =>
			p.id === id ? { ...p, ...patch } : p
		);
		this.persist();
	}

	removeProposal(id: string): void {
		this.poll.proposals = this.poll.proposals
			.filter((p) => p.id !== id)
			.map((p, i) => ({ ...p, order: i + 1 }));
		this.persist();
	}

	reorderProposal(id: string, direction: 'up' | 'down'): void {
		const arr = [...this.poll.proposals];
		const i = arr.findIndex((p) => p.id === id);
		if (i < 0) return;
		const j = direction === 'up' ? i - 1 : i + 1;
		if (j < 0 || j >= arr.length) return;
		[arr[i], arr[j]] = [arr[j], arr[i]];
		this.poll.proposals = arr.map((p, idx) => ({ ...p, order: idx + 1 }));
		this.persist();
	}

	// ---- Questions (per-proposal) --------------------------------------

	private mapProposal(proposalId: string, fn: (p: Proposal) => Proposal): void {
		this.poll.proposals = this.poll.proposals.map((p) => (p.id === proposalId ? fn(p) : p));
	}

	addQuestion(proposalId: string, type: QuestionType): Question | null {
		const proposal = this.poll.proposals.find((p) => p.id === proposalId);
		if (!proposal) return null;
		const q = defaultQuestionConfig(type, proposal.questions.length + 1);
		this.mapProposal(proposalId, (p) => ({ ...p, questions: [...p.questions, q] }));
		this.persist();
		return q;
	}

	updateQuestion(proposalId: string, questionId: string, patch: Partial<Question>): void {
		this.mapProposal(proposalId, (p) => ({
			...p,
			questions: p.questions.map((q) =>
				q.id === questionId ? ({ ...q, ...patch } as Question) : q
			)
		}));
		this.persist();
	}

	duplicateQuestion(proposalId: string, questionId: string): Question | null {
		const proposal = this.poll.proposals.find((p) => p.id === proposalId);
		const orig = proposal?.questions.find((q) => q.id === questionId);
		if (!orig || !proposal) return null;
		const copy: Question = JSON.parse(JSON.stringify(orig));
		copy.id = uid();
		copy.order = proposal.questions.length + 1;
		if (copy.type === 'multiple_choice') {
			copy.choices = copy.choices.map((c) => ({ ...c, id: uid() }));
		}
		this.mapProposal(proposalId, (p) => ({ ...p, questions: [...p.questions, copy] }));
		this.persist();
		return copy;
	}

	removeQuestion(proposalId: string, questionId: string): void {
		this.mapProposal(proposalId, (p) => ({
			...p,
			questions: p.questions
				.filter((q) => q.id !== questionId)
				.map((q, i) => ({ ...q, order: i + 1 }))
		}));
		this.persist();
	}

	addChoice(proposalId: string, questionId: string): void {
		this.mapProposal(proposalId, (p) => ({
			...p,
			questions: p.questions.map((q) => {
				if (q.id !== questionId || q.type !== 'multiple_choice') return q;
				return { ...q, choices: [...q.choices, { id: uid(), label: '' }] };
			})
		}));
		this.persist();
	}

	updateChoice(proposalId: string, questionId: string, choiceId: string, label: string): void {
		this.mapProposal(proposalId, (p) => ({
			...p,
			questions: p.questions.map((q) => {
				if (q.id !== questionId || q.type !== 'multiple_choice') return q;
				return {
					...q,
					choices: q.choices.map((c) => (c.id === choiceId ? { ...c, label } : c))
				};
			})
		}));
		this.persist();
	}

	removeChoice(proposalId: string, questionId: string, choiceId: string): void {
		this.mapProposal(proposalId, (p) => ({
			...p,
			questions: p.questions.map((q) => {
				if (q.id !== questionId || q.type !== 'multiple_choice') return q;
				return { ...q, choices: q.choices.filter((c) => c.id !== choiceId) };
			})
		}));
		this.persist();
	}

	// ---- Lifecycle ------------------------------------------------------

	publish(): { ok: true } | { ok: false; reason: string } {
		const issues = this.validatePublish();
		if (issues.length) return { ok: false, reason: issues.join('; ') };
		this.poll.state = 'published';
		this.poll.publishedAt = new Date().toISOString();
		this.poll.pausedAt = undefined;
		this.poll.pausedAccumulatedSeconds = 0;
		this.persist();
		return { ok: true };
	}

	unpublish(): void {
		this.poll.state = 'draft';
		this.poll.publishedAt = undefined;
		this.persist();
	}

	pause(): void {
		if (this.poll.state !== 'published') return;
		this.poll.state = 'paused';
		this.poll.pausedAt = new Date().toISOString();
		this.persist();
	}

	resume(): void {
		if (this.poll.state !== 'paused' || !this.poll.pausedAt) return;
		const elapsed = (Date.now() - new Date(this.poll.pausedAt).getTime()) / 1000;
		this.poll.pausedAccumulatedSeconds += elapsed;
		this.poll.pausedAt = undefined;
		this.poll.state = 'published';
		this.persist();
	}

	end(): void {
		this.poll.state = 'ended';
		this.poll.endedAt = new Date().toISOString();
		this.persist();
	}

	validatePublish(): string[] {
		const issues: string[] = [];
		if (!this.poll.title.trim()) issues.push('Title is required');
		if (this.poll.proposals.length < 2) issues.push('Add at least two proposals');
		for (const p of this.poll.proposals) {
			const label = p.title.trim() || `#${p.order}`;
			if (!p.title.trim()) issues.push(`Proposal ${p.order} needs a title`);
			if (p.questions.length < 1) {
				issues.push(`Proposal "${label}" needs at least one question`);
			}
			for (const q of p.questions) {
				if (!q.prompt.trim()) issues.push(`Proposal "${label}" Q${q.order} needs a prompt`);
				if (q.type === 'multiple_choice') {
					if (q.choices.length < 2)
						issues.push(`Proposal "${label}" Q${q.order} needs at least two choices`);
					if (q.choices.some((c) => !c.label.trim()))
						issues.push(`Proposal "${label}" Q${q.order} has an empty choice`);
				}
			}
		}
		return issues;
	}

	// ---- Timer ----------------------------------------------------------

	timeLeftSeconds(nowMs: number = Date.now()): number | null {
		const { timerSeconds } = this.poll.settings;
		if (timerSeconds === null) return null;
		if (!this.poll.publishedAt) return timerSeconds;
		const started = new Date(this.poll.publishedAt).getTime();
		let elapsed = (nowMs - started) / 1000 - this.poll.pausedAccumulatedSeconds;
		if (this.poll.state === 'paused' && this.poll.pausedAt) {
			const pausedFor = (nowMs - new Date(this.poll.pausedAt).getTime()) / 1000;
			elapsed -= pausedFor;
		}
		return Math.max(0, timerSeconds - elapsed);
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
		// Replace any previous submission from same participant.
		this.submissions = [
			...this.submissions.filter((s) => s.participantId !== participantId),
			submission
		];
		this.persistSubmissions();
		return submission;
	}

	draftIsComplete(draft: ParticipantDraft): boolean {
		for (const p of this.poll.proposals) {
			const ans: ProposalAnswers = draft.byProposal[p.id] ?? {};
			for (const q of p.questions) {
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
		for (const p of this.poll.proposals) {
			const ans: ProposalAnswers = draft.byProposal[p.id] ?? {};
			for (const q of p.questions) {
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
