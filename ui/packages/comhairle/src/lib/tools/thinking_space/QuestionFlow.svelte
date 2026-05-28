<script lang="ts">
	import { tick, untrack, onMount } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Progress } from '$lib/components/ui/progress';
	import { CornerDownRight, Shuffle, Check, RotateCcw } from 'lucide-svelte';
	import type { QuestionConfig, QuestionAnswers } from './types';
	import { QuestionFlowState } from './questionFlowState.svelte';
	import FollowUpLoading from './FollowUpLoading.svelte';

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

	const flow = new QuestionFlowState({
		questions,
		followUpCount,
		workflowStepId,
		initialAnswers,
		onComplete
	});

	let bottomEl = $state<HTMLDivElement | null>(null);
	let continueEl = $state<HTMLElement | null>(null);
	let rootTextareaEl = $state<HTMLTextAreaElement | null>(null);
	let followUpTextareaEl = $state<HTMLTextAreaElement | null>(null);

	async function scrollToFocus() {
		await tick();
		if (flow.minReached && flow.currentState.phase !== 'answering') {
			continueEl?.scrollIntoView({ behavior: 'smooth', block: 'center' });
		} else {
			bottomEl?.scrollIntoView({ behavior: 'smooth', block: 'end' });
		}
	}

	$effect(() => {
		// Re-scroll when current phase or follow-up count changes
		const _ =
			flow.currentState.phase +
			':' +
			flow.currentState.followUps.length +
			':' +
			flow.currentQuestionIndex;
		untrack(() => {
			scrollToFocus();
		});
		// Focus textareas
		untrack(async () => {
			await tick();
			if (flow.currentState.phase === 'root') rootTextareaEl?.focus();
			if (flow.currentState.phase === 'answering') followUpTextareaEl?.focus();
		});
		// reference _ to satisfy linter
		void _;
	});

	onMount(() => {
		if (flow.currentState.phase === 'picking' && flow.followUpCount > 0) {
			flow.loadPicker(flow.currentQuestionIndex);
		}
	});

	function handleRootKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			flow.submitRootAnswer();
		}
	}

	function handleFollowUpKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			flow.submitFollowUp();
		}
	}
</script>

<div class="flex h-full flex-col">
	<!-- Header / progress -->
	<div class="border-border bg-card/60 border-b px-6 py-4 backdrop-blur">
		<div class="mx-auto max-w-2xl">
			<div class="text-muted-foreground mb-2 flex items-center justify-end text-xs">
				<span>
					Question {flow.currentQuestionIndex + 1} of {questions.length} · {Math.round(
						flow.progress
					)}%
				</span>
			</div>
			<Progress value={flow.progress} class="h-1.5" />
		</div>
	</div>

	<!-- Scrollable content -->
	<div
		class="flex-1 overflow-y-auto px-6 py-8 transition-opacity duration-300"
		class:opacity-30={flow.transitioning}
	>
		<div class="mx-auto max-w-2xl space-y-6">
			<!-- Root question -->
			<section>
				<p class="text-primary mb-2 text-xs font-semibold tracking-wide uppercase">
					Question {flow.currentQuestionIndex + 1}
				</p>
				<h2 class="text-foreground text-2xl leading-snug font-semibold">
					{flow.currentQuestion.text || '(unnamed question)'}
				</h2>
			</section>

			{#if !flow.currentState.rootSubmitted}
				<section class="space-y-3">
					<Textarea
						bind:ref={rootTextareaEl}
						value={flow.currentState.rootAnswer}
						oninput={(e) => flow.updateRootAnswerDraft(e.currentTarget.value)}
						onkeydown={handleRootKeydown}
						placeholder="Write your thoughts here…"
						rows={4}
						class="text-base"
					/>
					<div class="flex justify-end">
						<Button
							onclick={() => flow.submitRootAnswer()}
							disabled={!flow.currentState.rootAnswer.trim() || flow.submitting}
						>
							{flow.submitting ? 'Saving…' : 'Continue'}
						</Button>
					</div>
				</section>
			{:else}
				<section class="border-primary/20 bg-primary/5 rounded-xl border p-4">
					<p class="text-primary mb-1 text-xs font-semibold tracking-wide uppercase">
						You answered
					</p>
					<p class="text-foreground text-sm leading-relaxed whitespace-pre-wrap">
						{flow.currentState.rootAnswer}
					</p>
				</section>
			{/if}

			<!-- Completed follow-ups -->
			{#each flow.currentState.followUps as followUp, followUpIndex (followUpIndex)}
				<section class="border-border space-y-2 border-t pt-6">
					<p
						class="text-muted-foreground flex items-center gap-1.5 text-xs font-semibold tracking-wide uppercase"
					>
						<CornerDownRight class="size-3.5" />
						Follow-up {followUpIndex + 1}
					</p>
					<p class="text-foreground/80 text-base leading-snug italic">
						{followUp.question}
					</p>
					<div class="border-primary/20 bg-primary/5 rounded-lg border p-3">
						<p class="text-foreground text-sm leading-relaxed whitespace-pre-wrap">
							{followUp.answer}
						</p>
					</div>
				</section>
			{/each}

			<!-- Continue — primary action once the follow-up minimum is met -->
			{#if flow.minReached && flow.currentState.phase !== 'answering'}
				<section bind:this={continueEl} class="border-border border-t pt-8">
					<Button
						size="lg"
						class="h-12 w-full text-base"
						onclick={() => flow.continueNow()}
					>
						<Check class="size-4" />
						{flow.isLastQuestion ? 'Finish' : 'Continue to the next question'}
					</Button>
				</section>
			{/if}

			<!-- Picker: loading -->
			{#if flow.currentState.phase === 'picking' && flow.currentState.pickerLoading}
				<FollowUpLoading />
			{/if}

			<!-- Picker: failed to load -->
			{#if flow.currentState.phase === 'picking' && flow.currentState.pickerError && !flow.currentState.pickerLoading}
				<section class="border-border space-y-3 border-t pt-6">
					<p class="text-muted-foreground text-sm">Couldn't load follow-up questions.</p>
					<Button variant="outline" size="sm" onclick={() => flow.retryPicker()}>
						<RotateCcw class="size-3.5" />
						Try again
					</Button>
				</section>
			{/if}

			<!-- Picker -->
			{#if flow.currentState.phase === 'picking' && flow.currentState.picker.length > 0 && !flow.currentState.pickerLoading}
				<section class="border-border space-y-3 border-t pt-6">
					<div class="flex items-start justify-between gap-3">
						<div class="min-w-0">
							<p class="text-foreground text-sm font-semibold">
								{flow.minReached
									? 'Or keep deepening your views'
									: 'Pick a follow-up to continue'}
							</p>
							{#if !flow.minReached}
								<p class="text-muted-foreground text-xs">
									{flow.followUpsRemaining} more follow-up{flow.followUpsRemaining ===
									1
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
						<Button
							variant="secondary"
							size="sm"
							class="shrink-0"
							onclick={() => flow.pickRandom()}
						>
							<Shuffle class="size-3.5" />
							Pick one for me
						</Button>
					</div>
					<div class="space-y-2">
						{#each flow.currentState.picker.slice(0, 5) as followUpQuestion, i (followUpQuestion + i)}
							<button
								type="button"
								onclick={() => flow.pickFollowUp(followUpQuestion)}
								class="border-border bg-card hover:border-primary hover:bg-primary/5 w-full rounded-lg border px-4 py-3 text-left text-sm leading-relaxed transition-colors"
							>
								{followUpQuestion}
							</button>
						{/each}
					</div>
				</section>
			{/if}

			<!-- Active follow-up answer input -->
			{#if flow.currentState.phase === 'answering'}
				<section class="border-border space-y-3 border-t pt-6">
					<p
						class="text-muted-foreground flex items-center gap-1.5 text-xs font-semibold tracking-wide uppercase"
					>
						<CornerDownRight class="size-3.5" />
						Follow-up {flow.followUpsDone + 1}
					</p>
					<p class="text-foreground text-lg leading-snug italic">
						{flow.currentState.currentPick}
					</p>
					<Textarea
						bind:ref={followUpTextareaEl}
						value={flow.currentState.currentPickAnswer}
						oninput={(e) => flow.updateFollowUpDraft(e.currentTarget.value)}
						onkeydown={handleFollowUpKeydown}
						placeholder="Write your thoughts…"
						rows={4}
						class="text-base"
					/>
					<div class="flex justify-end">
						<Button
							onclick={() => flow.submitFollowUp()}
							disabled={!flow.currentState.currentPickAnswer.trim() ||
								flow.submitting}
						>
							{flow.submitting ? 'Saving…' : 'Continue'}
						</Button>
					</div>
				</section>
			{/if}

			<div bind:this={bottomEl} class="h-1"></div>
		</div>
	</div>
</div>
