import { apiClient } from '@crownshy/api-client/client';
import { notifications } from '$lib/notifications.svelte';
import { fetchFollowUps } from './converse';
import type { QuestionConfig, QuestionAnswers, FollowUpAnswer } from './types';

export type Phase = 'root' | 'picking' | 'answering';

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
};

type Init = {
	questions: QuestionConfig[];
	followUpCount: number;
	workflowStepId: string;
	initialAnswers: QuestionAnswers[];
	onComplete: (answers: QuestionAnswers[]) => void;
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
	private readonly onComplete: (answers: QuestionAnswers[]) => void;

	states = $state<LocalQuestionState[]>([]);
	currentQuestionIndex = $state(0);
	transitioning = $state(false);
	submitting = $state(false);

	constructor(init: Init) {
		this.questions = init.questions;
		this.followUpCount = init.followUpCount;
		this.workflowStepId = init.workflowStepId;
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
				phase: 'root'
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
			phase: 'picking'
		};
	}

	// Resume on the first question not yet answered, or whose follow-up minimum
	// hasn't been reached. If everything is complete, land on the last question.
	private resumeIndex(initialAnswers: QuestionAnswers[]): number {
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
			const followUps = await fetchFollowUps({
				workflowStepId: this.workflowStepId,
				startingQuestion: this.questions[questionIndex].text,
				// The root question config has no separate `intent` field yet,
				// so we send the question text here as a proxy. If the config
				// schema grows an explicit intent, use that instead.
				questionIntent: this.questions[questionIndex].text,
				history: this.buildHistory(questionIndex)
			});
			const picker = followUps.map((followUp) => followUp.question);
			this.states[questionIndex] = {
				...this.states[questionIndex],
				picker,
				pickerLoading: false,
				pickerError: picker.length === 0
			};
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
				phase: 'picking'
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
				answer: value
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
				phase: 'picking'
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
