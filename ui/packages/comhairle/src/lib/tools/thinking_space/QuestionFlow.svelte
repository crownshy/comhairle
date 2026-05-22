<script lang="ts">
	import { tick, untrack, onMount } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Progress } from '$lib/components/ui/progress';
	import { CornerDownRight, Shuffle, Check, Loader2, RotateCcw } from 'lucide-svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { fetchFollowUps } from './converse';
	import type { QuestionConfig, QuestionAnswers, FollowUpAnswer } from './types';

	type Props = {
		topic: string;
		workflowStepId: string;
		questions: QuestionConfig[];
		followUpCount: number;
		initialAnswers?: QuestionAnswers[];
		onComplete: (answers: QuestionAnswers[]) => void;
	};

	let {
		topic,
		workflowStepId,
		questions,
		followUpCount,
		initialAnswers = [],
		onComplete
	}: Props = $props();

	type Phase = 'main' | 'picking' | 'answering';

	type LocalQuestionState = {
		mainAnswer: string;
		mainSubmitted: boolean;
		mainAnswerId: string | null;
		followUps: FollowUpAnswer[];
		picker: string[];
		pickerLoading: boolean;
		pickerError: boolean;
		currentPick: string;
		currentPickAnswer: string;
		phase: Phase;
	};

	function initialStateFor(qIdx: number): LocalQuestionState {
		const stored = initialAnswers.find((a) => a.questionId === questions[qIdx].id);
		if (!stored) {
			return {
				mainAnswer: '',
				mainSubmitted: false,
				mainAnswerId: null,
				followUps: [],
				picker: [],
				pickerLoading: false,
				pickerError: false,
				currentPick: '',
				currentPickAnswer: '',
				phase: 'main'
			};
		}
		return {
			mainAnswer: stored.mainAnswer,
			mainSubmitted: true,
			mainAnswerId: stored.mainAnswerId ?? null,
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
	let currentQIdx = $state(
		(() => {
			for (let i = 0; i < questions.length; i++) {
				const stored = initialAnswers.find((a) => a.questionId === questions[i].id);
				if (!stored) return i;
				if (stored.followUps.length < followUpCount) return i;
			}
			return Math.max(0, questions.length - 1);
		})()
	);

	let states = $state<LocalQuestionState[]>(questions.map((_, i) => initialStateFor(i)));
	let transitioning = $state(false);
	let submitting = $state(false);

	let bottomEl = $state<HTMLDivElement | null>(null);
	let continueEl = $state<HTMLElement | null>(null);
	let mainTextareaEl = $state<HTMLTextAreaElement | null>(null);
	let followUpTextareaEl = $state<HTMLTextAreaElement | null>(null);

	let currentState = $derived(states[currentQIdx]);
	let currentQuestion = $derived(questions[currentQIdx]);
	let followUpsDone = $derived(currentState.followUps.length);
	let followUpsRemaining = $derived(Math.max(0, followUpCount - followUpsDone));
	let isLastQuestion = $derived(currentQIdx === questions.length - 1);
	// Minimum follow-ups reached for the current question — Continue button
	// is revealed but the user may keep answering more follow-ups.
	let minReached = $derived(currentState.mainSubmitted && followUpsDone >= followUpCount);

	let totalSteps = $derived(questions.length * (1 + followUpCount));
	let completedSteps = $derived.by(() => {
		let n = 0;
		for (let i = 0; i < currentQIdx; i++) n += 1 + followUpCount;
		if (currentState.mainSubmitted) n += 1;
		n += Math.min(followUpsDone, followUpCount);
		return Math.min(n, totalSteps);
	});
	let progress = $derived(totalSteps > 0 ? (completedSteps / totalSteps) * 100 : 0);

	async function scrollToFocus() {
		await tick();
		if (minReached && currentState.phase !== 'answering') {
			continueEl?.scrollIntoView({ behavior: 'smooth', block: 'center' });
		} else {
			bottomEl?.scrollIntoView({ behavior: 'smooth', block: 'end' });
		}
	}

	$effect(() => {
		// Re-scroll when current phase or follow-up count changes
		const _ = currentState.phase + ':' + currentState.followUps.length + ':' + currentQIdx;
		untrack(() => {
			scrollToFocus();
		});
		// Focus textareas
		untrack(async () => {
			await tick();
			if (currentState.phase === 'main') mainTextareaEl?.focus();
			if (currentState.phase === 'answering') followUpTextareaEl?.focus();
		});
		// reference _ to satisfy linter
		void _;
	});

	function buildAnswers(): QuestionAnswers[] {
		return states.map((s, i) => ({
			questionId: questions[i].id,
			mainAnswer: s.mainAnswer,
			mainAnswerId: s.mainAnswerId,
			followUps: s.followUps
		}));
	}

	function continueNow() {
		if (isLastQuestion) {
			onComplete(buildAnswers());
			return;
		}
		transitioning = true;
		setTimeout(() => {
			currentQIdx = currentQIdx + 1;
			transitioning = false;
		}, 500);
	}

	// Build the running Q/A history the agent uses to generate follow-ups.
	function buildHistory(qIdx: number): string {
		const s = states[qIdx];
		const lines: string[] = [];
		let n = 1;
		lines.push(`Q${n}: ${questions[qIdx].text}`);
		lines.push(`A${n}: ${s.mainAnswer}`);
		for (const fu of s.followUps) {
			n++;
			lines.push(`Q${n}: ${fu.question}`);
			lines.push(`A${n}: ${fu.answer}`);
		}
		return lines.join('\n');
	}

	async function loadPicker(qIdx: number) {
		states[qIdx] = { ...states[qIdx], pickerLoading: true, pickerError: false };
		try {
			const followUps = await fetchFollowUps({
				workflowStepId,
				startingQuestion: questions[qIdx].text,
				// No dedicated intent field yet — the question text is the
				// best proxy until the config schema gains one.
				questionIntent: questions[qIdx].text,
				history: buildHistory(qIdx)
			});
			const picker = followUps.map((f) => f.question);
			states[qIdx] = {
				...states[qIdx],
				picker,
				pickerLoading: false,
				pickerError: picker.length === 0
			};
		} catch (e) {
			console.error(e);
			states[qIdx] = {
				...states[qIdx],
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

	function retryPicker() {
		loadPicker(currentQIdx);
	}

	onMount(() => {
		if (currentState.phase === 'picking' && followUpCount > 0) {
			loadPicker(currentQIdx);
		}
	});

	async function submitMainAnswer() {
		const value = currentState.mainAnswer.trim();
		if (!value || submitting) return;
		submitting = true;
		try {
			const saved = await apiClient.CreateThinkingSpaceAnswer({
				workflow_step_id: workflowStepId,
				question: currentQuestion.text,
				answer: value
			});
			states[currentQIdx] = {
				...currentState,
				mainAnswer: value,
				mainSubmitted: true,
				mainAnswerId: saved.id,
				picker: [],
				phase: 'picking'
			};
			if (followUpCount > 0) loadPicker(currentQIdx);
		} catch (e) {
			console.error(e);
			notifications.send({
				message: 'Could not save your answer. Please try again.',
				priority: 'ERROR'
			});
		} finally {
			submitting = false;
		}
	}

	function pickFollowUp(q: string) {
		states[currentQIdx] = {
			...currentState,
			currentPick: q,
			currentPickAnswer: '',
			picker: currentState.picker.filter((x) => x !== q),
			phase: 'answering'
		};
	}

	function pickRandom() {
		const pool = currentState.picker;
		if (pool.length === 0) return;
		pickFollowUp(pool[Math.floor(Math.random() * pool.length)]);
	}

	async function submitFollowUp() {
		const value = currentState.currentPickAnswer.trim();
		if (!value || submitting) return;
		submitting = true;
		try {
			const saved = await apiClient.CreateThinkingSpaceAnswer({
				workflow_step_id: workflowStepId,
				question: currentState.currentPick,
				answer: value,
				is_follow_up: true,
				root_question_id: currentState.mainAnswerId,
				other_questions: currentState.picker
			});
			const fu: FollowUpAnswer = {
				id: saved.id,
				question: currentState.currentPick,
				answer: value
			};
			const updatedFollowUps = [...currentState.followUps, fu];
			// Always refetch the picker and stay in 'picking'. The participant
			// chooses when to move on via the Continue button (revealed once
			// followUpsDone >= followUpCount). We never force-quit them.
			states[currentQIdx] = {
				...currentState,
				followUps: updatedFollowUps,
				currentPick: '',
				currentPickAnswer: '',
				picker: [],
				phase: 'picking'
			};
			if (followUpCount > 0) loadPicker(currentQIdx);
		} catch (e) {
			console.error(e);
			notifications.send({
				message: 'Could not save your answer. Please try again.',
				priority: 'ERROR'
			});
		} finally {
			submitting = false;
		}
	}

	function handleMainKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			submitMainAnswer();
		}
	}

	function handleFollowUpKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			submitFollowUp();
		}
	}
</script>

<div class="flex h-full flex-col">
	<!-- Header / progress -->
	<div class="border-border bg-card/60 border-b px-6 py-4 backdrop-blur">
		<div class="mx-auto max-w-2xl">
			<div class="text-muted-foreground mb-2 flex items-center justify-between text-xs">
				<span class="font-medium tracking-wide uppercase">
					Thinking space · {topic}
				</span>
				<span>
					Question {currentQIdx + 1} of {questions.length} · {Math.round(progress)}%
				</span>
			</div>
			<Progress value={progress} class="h-1.5" />
		</div>
	</div>

	<!-- Scrollable content -->
	<div
		class="flex-1 overflow-y-auto px-6 py-8 transition-opacity duration-300"
		class:opacity-30={transitioning}
	>
		<div class="mx-auto max-w-2xl space-y-6">
			<!-- Main question -->
			<section>
				<p class="text-primary mb-2 text-xs font-semibold tracking-wide uppercase">
					Question {currentQIdx + 1}
				</p>
				<h2 class="text-foreground text-2xl leading-snug font-semibold">
					{currentQuestion.text || '(unnamed question)'}
				</h2>
			</section>

			{#if !currentState.mainSubmitted}
				<section class="space-y-3">
					<Textarea
						bind:ref={mainTextareaEl}
						bind:value={states[currentQIdx].mainAnswer}
						onkeydown={handleMainKeydown}
						placeholder="Write your thoughts here…"
						rows={4}
						class="text-base"
					/>
					<div class="flex justify-end">
						<Button
							onclick={submitMainAnswer}
							disabled={!currentState.mainAnswer.trim() || submitting}
						>
							{submitting ? 'Saving…' : 'Continue'}
						</Button>
					</div>
				</section>
			{:else}
				<section class="border-primary/20 bg-primary/5 rounded-xl border p-4">
					<p class="text-primary mb-1 text-xs font-semibold tracking-wide uppercase">
						You answered
					</p>
					<p class="text-foreground text-sm leading-relaxed whitespace-pre-wrap">
						{currentState.mainAnswer}
					</p>
				</section>
			{/if}

			<!-- Completed follow-ups -->
			{#each currentState.followUps as fu, fi (fi)}
				<section class="border-border space-y-2 border-t pt-6">
					<p
						class="text-muted-foreground flex items-center gap-1.5 text-xs font-semibold tracking-wide uppercase"
					>
						<CornerDownRight class="size-3.5" />
						Follow-up {fi + 1}
					</p>
					<p class="text-foreground/80 text-base leading-snug italic">
						{fu.question}
					</p>
					<div class="border-primary/20 bg-primary/5 rounded-lg border p-3">
						<p class="text-foreground text-sm leading-relaxed whitespace-pre-wrap">
							{fu.answer}
						</p>
					</div>
				</section>
			{/each}

			<!-- Continue — primary action once the follow-up minimum is met -->
			{#if minReached && currentState.phase !== 'answering'}
				<section bind:this={continueEl} class="border-border border-t pt-8">
					<Button size="lg" class="h-12 w-full text-base" onclick={continueNow}>
						<Check class="size-4" />
						{isLastQuestion ? 'Finish' : 'Continue to the next question'}
					</Button>
				</section>
			{/if}

			<!-- Picker: loading -->
			{#if currentState.phase === 'picking' && currentState.pickerLoading}
				<section class="border-border flex items-center gap-2 border-t pt-6">
					<Loader2 class="text-primary size-4 animate-spin" />
					<p class="text-muted-foreground text-sm">Generating follow-up questions…</p>
				</section>
			{/if}

			<!-- Picker: failed to load -->
			{#if currentState.phase === 'picking' && currentState.pickerError && !currentState.pickerLoading}
				<section class="border-border space-y-3 border-t pt-6">
					<p class="text-muted-foreground text-sm">Couldn't load follow-up questions.</p>
					<Button variant="outline" size="sm" onclick={retryPicker}>
						<RotateCcw class="size-3.5" />
						Try again
					</Button>
				</section>
			{/if}

			<!-- Picker -->
			{#if currentState.phase === 'picking' && currentState.picker.length > 0 && !currentState.pickerLoading}
				<section class="border-border space-y-3 border-t pt-6">
					<div class="flex items-baseline justify-between gap-3">
						<div>
							<p class="text-foreground text-sm font-semibold">
								{minReached
									? 'Or keep deepening your views'
									: 'Pick a follow-up to continue'}
							</p>
							{#if !minReached}
								<p class="text-muted-foreground text-xs">
									{followUpsRemaining} more follow-up{followUpsRemaining === 1
										? ''
										: 's'} to go
								</p>
							{:else}
								<p class="text-muted-foreground text-xs">
									Optional. Pick another follow-up to go deeper, or continue
									above.
								</p>
							{/if}
						</div>
					</div>
					<div class="space-y-2">
						{#each currentState.picker.slice(0, 5) as fq, i (fq + i)}
							<button
								type="button"
								onclick={() => pickFollowUp(fq)}
								class="border-border bg-card hover:border-primary hover:bg-primary/5 w-full rounded-lg border px-4 py-3 text-left text-sm leading-relaxed transition-colors"
							>
								{fq}
							</button>
						{/each}
					</div>
					<Button variant="secondary" size="sm" onclick={pickRandom}>
						<Shuffle class="size-3.5" />
						Pick one for me
					</Button>
				</section>
			{/if}

			<!-- Active follow-up answer input -->
			{#if currentState.phase === 'answering'}
				<section class="border-border space-y-3 border-t pt-6">
					<p
						class="text-muted-foreground flex items-center gap-1.5 text-xs font-semibold tracking-wide uppercase"
					>
						<CornerDownRight class="size-3.5" />
						Follow-up {followUpsDone + 1}
					</p>
					<p class="text-foreground text-lg leading-snug italic">
						{currentState.currentPick}
					</p>
					<Textarea
						bind:ref={followUpTextareaEl}
						bind:value={states[currentQIdx].currentPickAnswer}
						onkeydown={handleFollowUpKeydown}
						placeholder="Write your thoughts…"
						rows={4}
						class="text-base"
					/>
					<div class="flex justify-end">
						<Button
							onclick={submitFollowUp}
							disabled={!currentState.currentPickAnswer.trim() || submitting}
						>
							{submitting ? 'Saving…' : 'Continue'}
						</Button>
					</div>
				</section>
			{/if}

			<div bind:this={bottomEl} class="h-1"></div>
		</div>
	</div>
</div>
