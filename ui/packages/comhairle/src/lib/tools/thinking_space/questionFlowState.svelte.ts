import { apiClient } from '@crownshy/api-client/client';
import { notifications } from '$lib/notifications.svelte';
import { fetchFollowUps } from './converse';
import type { QuestionConfig, QuestionAnswers, FollowUpAnswer } from './types';

export type Phase = 'root' | 'picking' | 'answering';

/**
 * Extension mode adds a top-level navigation hub: a picker over root questions.
 * 'root-picker' = participant is choosing which root to extend, or finishing.
 * 'in-chain'    = participant is inside a root's chain answering follow-ups.
 */
export type ExtensionPhase = 'root-picker' | 'in-chain';

export type LocalQuestionState = {
	rootAnswer: string;
	rootSubmitted: boolean;
	rootAnswerId: string | null;
	followUps: FollowUpAnswer[];
	picker: string[];
	pickerLoading: boolean;
	pickerError: boolean;
	currentPick: string;
	currentPickAnswer: string;
	phase: Phase;
	// Once the follow-up minimum is met, the participant is asked whether to
	// keep going deeper or move on. The picker is only revealed after they
	// opt in via `chooseMore()`. Reset to false after each follow-up answer
	// so we re-ask before each additional one.
	wantsMore: boolean;
};

/**
 * 'initial' — the participant's first pass. Each root question must be
 *   answered, then `followUpCount` follow-ups picked.
 * 'extension' — the participant returned via "answer more questions". They
 *   choose which root to extend from a picker; inside a chain the agent
 *   keeps generating follow-ups until they click "Done with this question"
 *   and return to the picker. No `other_questions` pool reuse — every
 *   follow-up is a fresh RAGFlow call.
 */
export type FlowMode = 'initial' | 'extension';

type Init = {
	questions: QuestionConfig[];
	followUpCount: number;
	workflowStepId: string;
	initialAnswers: QuestionAnswers[];
	onComplete: (answers: QuestionAnswers[]) => void;
	mode?: FlowMode;
};

/**
 * Reactive state container for the Thinking Space participant flow. Owns all
 * per-question state, the current question pointer, and the network calls that
 * mutate them. The component layer is a thin shell that holds one instance and
 * renders from its getters.
 */
export class QuestionFlowState {
	readonly questions: QuestionConfig[];
	readonly followUpCount: number;
	readonly workflowStepId: string;
	readonly mode: FlowMode;
	private readonly onComplete: (answers: QuestionAnswers[]) => void;

	states = $state<LocalQuestionState[]>([]);
	currentQuestionIndex = $state(0);
	transitioning = $state(false);
	submitting = $state(false);
	// Extension-mode navigation phase. Unused in initial mode.
	extensionPhase = $state<ExtensionPhase>('root-picker');

	constructor(init: Init) {
		this.questions = init.questions;
		this.followUpCount = init.followUpCount;
		this.workflowStepId = init.workflowStepId;
		this.mode = init.mode ?? 'initial';
		this.onComplete = init.onComplete;

		this.states = init.questions.map((_, i) => this.initialStateFor(i, init.initialAnswers));
		this.currentQuestionIndex = this.resumeIndex(init.initialAnswers);
	}

	private initialStateFor(
		questionIndex: number,
		initialAnswers: QuestionAnswers[]
	): LocalQuestionState {
		const stored = initialAnswers.find(
			(answer) => answer.questionId === this.questions[questionIndex].id
		);
		if (!stored) {
			return {
				rootAnswer: '',
				rootSubmitted: false,
				rootAnswerId: null,
				followUps: [],
				picker: [],
				pickerLoading: false,
				pickerError: false,
				currentPick: '',
				currentPickAnswer: '',
				phase: 'root',
				wantsMore: false
			};
		}
		return {
			rootAnswer: stored.rootAnswer,
			rootSubmitted: true,
			rootAnswerId: stored.rootAnswerId ?? null,
			followUps: stored.followUps,
			// Picker is fetched from the agent on mount / after each answer.
			picker: [],
			pickerLoading: false,
			pickerError: false,
			currentPick: '',
			currentPickAnswer: '',
			phase: 'picking',
			wantsMore: false
		};
	}

	// Resume on the first question not yet answered, or whose follow-up minimum
	// hasn't been reached. Extension mode doesn't use this — the root picker
	// drives navigation; this default is harmless until a root is chosen.
	private resumeIndex(initialAnswers: QuestionAnswers[]): number {
		if (this.mode === 'extension') return 0;
		for (let i = 0; i < this.questions.length; i++) {
			const stored = initialAnswers.find(
				(answer) => answer.questionId === this.questions[i].id
			);
			if (!stored) return i;
			if (stored.followUps.length < this.followUpCount) return i;
		}
		return Math.max(0, this.questions.length - 1);
	}

	get currentState(): LocalQuestionState {
		return this.states[this.currentQuestionIndex];
	}

	get currentQuestion(): QuestionConfig {
		return this.questions[this.currentQuestionIndex];
	}

	get followUpsDone(): number {
		return this.currentState.followUps.length;
	}

	get followUpsRemaining(): number {
		return Math.max(0, this.followUpCount - this.followUpsDone);
	}

	get isLastQuestion(): boolean {
		return this.currentQuestionIndex === this.questions.length - 1;
	}

	// Minimum follow-ups reached for the current question — Continue button
	// is revealed but the user may keep answering more follow-ups.
	get minReached(): boolean {
		return this.currentState.rootSubmitted && this.followUpsDone >= this.followUpCount;
	}

	get totalSteps(): number {
		return this.questions.length * (1 + this.followUpCount);
	}

	get completedSteps(): number {
		let steps = 0;
		for (let i = 0; i < this.currentQuestionIndex; i++) steps += 1 + this.followUpCount;
		if (this.currentState.rootSubmitted) steps += 1;
		steps += Math.min(this.followUpsDone, this.followUpCount);
		return Math.min(steps, this.totalSteps);
	}

	get progress(): number {
		return this.totalSteps > 0 ? (this.completedSteps / this.totalSteps) * 100 : 0;
	}

	private buildAnswers(): QuestionAnswers[] {
		return this.states.map((state, index) => ({
			questionId: this.questions[index].id,
			rootAnswer: state.rootAnswer,
			rootAnswerId: state.rootAnswerId,
			followUps: state.followUps
		}));
	}

	// Build the running Q/A history the agent uses to generate follow-ups.
	private buildHistory(questionIndex: number): string {
		const state = this.states[questionIndex];
		const lines: string[] = [];
		let turn = 1;
		lines.push(`Q${turn}: ${this.questions[questionIndex].text}`);
		lines.push(`A${turn}: ${state.rootAnswer}`);
		for (const followUp of state.followUps) {
			turn++;
			lines.push(`Q${turn}: ${followUp.question}`);
			lines.push(`A${turn}: ${followUp.answer}`);
		}
		return lines.join('\n');
	}

	// Participant has opted to keep going deeper after meeting the minimum.
	// Reveals the picker; loads it if it isn't already in flight or cached.
	chooseMore() {
		this.states[this.currentQuestionIndex] = {
			...this.currentState,
			wantsMore: true
		};
		const state = this.currentState;
		if (state.pickerLoading) return;
		if (state.pickerError || state.picker.length === 0) {
			this.loadPicker(this.currentQuestionIndex);
		}
	}

	continueNow() {
		if (this.isLastQuestion) {
			this.onComplete(this.buildAnswers());
			return;
		}
		this.transitioning = true;
		setTimeout(() => {
			this.currentQuestionIndex = this.currentQuestionIndex + 1;
			this.transitioning = false;
		}, 500);
	}

	async loadPicker(questionIndex: number) {
		this.states[questionIndex] = {
			...this.states[questionIndex],
			pickerLoading: true,
			pickerError: false
		};
		try {
			const question = this.questions[questionIndex];
			const followUps = await fetchFollowUps({
				workflowStepId: this.workflowStepId,
				startingQuestion: question.text,
				questionIntent: question.intent,
				history: this.buildHistory(questionIndex)
			});
			const picker = followUps.map((followUp) => followUp.question);
			this.states[questionIndex] = {
				...this.states[questionIndex],
				picker,
				pickerLoading: false,
				pickerError: picker.length === 0
			};
			// Persist this round of generated follow-ups so the backend has the
			// audit trail required to resume a participant later. Fire-and-forget:
			// failure must not block the picker UI. Skips when the root answer id
			// is missing (shouldn't happen post-root-submit, but defensive).
			const rootAnswerId = this.states[questionIndex].rootAnswerId;
			if (rootAnswerId && picker.length > 0) {
				apiClient
					.CreateThinkingSpaceFollowUpQuestions({
						workflow_step_id: this.workflowStepId,
						root_question_id: rootAnswerId,
						follow_up_questions: picker
					})
					.catch((err) => {
						console.error('Failed to persist follow-up questions', err);
					});
			}
		} catch (e) {
			console.error(e);
			this.states[questionIndex] = {
				...this.states[questionIndex],
				picker: [],
				pickerLoading: false,
				pickerError: true
			};
			notifications.send({
				message: 'Could not load follow-up questions. Please try again.',
				priority: 'ERROR'
			});
		}
	}

	retryPicker() {
		this.loadPicker(this.currentQuestionIndex);
	}

	async submitRootAnswer() {
		const value = this.currentState.rootAnswer.trim();
		if (!value || this.submitting) return;
		this.submitting = true;
		try {
			const saved = await apiClient.CreateThinkingSpaceAnswer({
				workflow_step_id: this.workflowStepId,
				question: this.currentQuestion.text,
				answer: value
			});
			this.states[this.currentQuestionIndex] = {
				...this.currentState,
				rootAnswer: value,
				rootSubmitted: true,
				rootAnswerId: saved.id,
				picker: [],
				phase: 'picking',
				wantsMore: false
			};
			if (this.followUpCount > 0) this.loadPicker(this.currentQuestionIndex);
		} catch (e) {
			console.error(e);
			notifications.send({
				message: 'Could not save your answer. Please try again.',
				priority: 'ERROR'
			});
		} finally {
			this.submitting = false;
		}
	}

	// Participant changed their mind about the question they picked — return
	// to the picker. Puts the abandoned pick back at the top of the list so
	// it's easy to find if they meant to choose it after all. Any draft text
	// is dropped (they're abandoning that pick by definition).
	backToPicker() {
		const pick = this.currentState.currentPick;
		if (!pick) return;
		this.states[this.currentQuestionIndex] = {
			...this.currentState,
			currentPick: '',
			currentPickAnswer: '',
			picker: [pick, ...this.currentState.picker.filter((other) => other !== pick)],
			phase: 'picking'
		};
	}

	pickFollowUp(question: string) {
		this.states[this.currentQuestionIndex] = {
			...this.currentState,
			currentPick: question,
			currentPickAnswer: '',
			picker: this.currentState.picker.filter((other) => other !== question),
			phase: 'answering'
		};
	}

	pickRandom() {
		const pool = this.currentState.picker;
		if (pool.length === 0) return;
		this.pickFollowUp(pool[Math.floor(Math.random() * pool.length)]);
	}

	async submitFollowUp() {
		const value = this.currentState.currentPickAnswer.trim();
		if (!value || this.submitting) return;
		this.submitting = true;
		try {
			const saved = await apiClient.CreateThinkingSpaceAnswer({
				workflow_step_id: this.workflowStepId,
				question: this.currentState.currentPick,
				answer: value,
				is_follow_up: true,
				root_question_id: this.currentState.rootAnswerId,
				other_questions: this.currentState.picker
			});
			const followUp: FollowUpAnswer = {
				id: saved.id,
				question: this.currentState.currentPick,
				answer: value,
				// Remaining pool at the moment of submit — alternatives the
				// participant didn't choose. Seeds the extension-mode picker
				// on a future "answer more" visit.
				otherQuestions: this.currentState.picker
			};
			// Always refetch the picker and stay in 'picking'. The participant
			// chooses when to move on via the Continue button (revealed once
			// followUpsDone >= followUpCount). We never force-quit them.
			this.states[this.currentQuestionIndex] = {
				...this.currentState,
				followUps: [...this.currentState.followUps, followUp],
				currentPick: '',
				currentPickAnswer: '',
				picker: [],
				phase: 'picking',
				wantsMore: false
			};
			if (this.followUpCount > 0) this.loadPicker(this.currentQuestionIndex);
		} catch (e) {
			console.error(e);
			notifications.send({
				message: 'Could not save your answer. Please try again.',
				priority: 'ERROR'
			});
		} finally {
			this.submitting = false;
		}
	}

	// ── Extension-mode navigation ──────────────────────────────────────────
	// In extension mode the participant chooses a root from the picker,
	// extends it indefinitely, then clicks "Done with this question" to
	// return to the picker. From the picker they finish via finishExtension.

	enterRoot(questionIndex: number) {
		this.currentQuestionIndex = questionIndex;
		this.extensionPhase = 'in-chain';
		// Reset picker so we always fetch fresh follow-ups using full chain.
		this.states[questionIndex] = {
			...this.states[questionIndex],
			picker: [],
			pickerLoading: false,
			pickerError: false,
			phase: 'picking',
			wantsMore: false
		};
		if (this.followUpCount > 0) this.loadPicker(questionIndex);
	}

	doneWithRoot() {
		this.extensionPhase = 'root-picker';
	}

	finishExtension() {
		this.onComplete(this.buildAnswers());
	}

	// Total answers under a root (root + follow-ups). Used by picker counts.
	answerCountFor(questionIndex: number): number {
		const state = this.states[questionIndex];
		if (!state) return 0;
		return (state.rootSubmitted ? 1 : 0) + state.followUps.length;
	}

	updateRootAnswerDraft(value: string) {
		this.states[this.currentQuestionIndex] = {
			...this.currentState,
			rootAnswer: value
		};
	}

	updateFollowUpDraft(value: string) {
		this.states[this.currentQuestionIndex] = {
			...this.currentState,
			currentPickAnswer: value
		};
	}
}
