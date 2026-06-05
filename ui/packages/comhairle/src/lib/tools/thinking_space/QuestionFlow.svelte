<script lang="ts">
	import { tick, untrack, onMount } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Progress } from '$lib/components/ui/progress';
	import {
		CornerDownRight,
		Shuffle,
		Check,
		RotateCcw,
		ChevronLeft,
		ChevronRight,
		ArrowLeft
	} from 'lucide-svelte';
	import type { QuestionConfig, QuestionAnswers } from './types';
	import { QuestionFlowState, type FlowMode } from './questionFlowState.svelte';
	import FollowUpLoading from './FollowUpLoading.svelte';

	type Props = {
		topic: string;
		workflowStepId: string;
		questions: QuestionConfig[];
		followUpCount: number;
		initialAnswers?: QuestionAnswers[];
		mode?: FlowMode;
		onComplete: (answers: QuestionAnswers[]) => void;
		/**
		 * Back-out affordance, currently only used in extension mode so the
		 * participant can return to the summary screen without finishing the
		 * full second pass. Any follow-ups they've already submitted are
		 * persisted; the parent decides whether to generate a new round based
		 * on whether new answers were actually added.
		 */
		onBack?: () => void;
	};

	let {
		topic,
		workflowStepId,
		questions,
		followUpCount,
		initialAnswers = [],
		mode = 'initial',
		onComplete,
		onBack
	}: Props = $props();

	const flow = new QuestionFlowState({
		questions,
		followUpCount,
		workflowStepId,
		initialAnswers,
		onComplete,
		mode
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
		// Re-scroll when current phase, follow-up count, or deepen choice changes
		const _ =
			flow.currentState.phase +
			':' +
			flow.currentState.followUps.length +
			':' +
			flow.currentQuestionIndex +
			':' +
			flow.currentState.wantsMore;
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
		// Extension mode starts in the root picker; loadPicker fires when the
		// participant enters a root via enterRoot().
		if (flow.mode === 'extension') return;
		if (flow.currentState.phase === 'picking' && flow.followUpCount > 0) {
			flow.loadPicker(flow.currentQuestionIndex);
		}
	});

	let inExtensionPicker = $derived(
		flow.mode === 'extension' && flow.extensionPhase === 'root-picker'
	);
	let inExtensionChain = $derived(
		flow.mode === 'extension' && flow.extensionPhase === 'in-chain'
	);

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
			<div class="text-muted-foreground mb-2 flex items-center justify-between gap-3 text-xs">
				{#if onBack}
					<button
						type="button"
						onclick={() => onBack?.()}
						class="text-muted-foreground hover:text-foreground inline-flex items-center gap-1 transition-colors"
					>
						<ArrowLeft class="size-3.5" />
						Back to summary
					</button>
				{:else}
					<span></span>
				{/if}
				<span>
					{#if inExtensionPicker}
						Pick a question to explore further
					{:else if inExtensionChain}
						Going deeper on question {flow.currentQuestionIndex + 1} of {questions.length}
					{:else}
						Question {flow.currentQuestionIndex + 1} of {questions.length} · {Math.round(
							flow.progress
						)}%
					{/if}
				</span>
			</div>
			{#if flow.mode !== 'extension'}
				<Progress value={flow.progress} class="h-1.5" />
			{/if}
		</div>
	</div>

	<!-- Scrollable content -->
	<div
		class="flex-1 overflow-y-auto px-6 py-8 transition-opacity duration-300"
		class:opacity-30={flow.transitioning}
	>
		{#if inExtensionPicker}
			<div class="mx-auto max-w-2xl space-y-6">
				<header>
					<h2 class="text-foreground text-2xl leading-snug font-semibold">
						What do you want to explore further?
					</h2>
					<p class="text-muted-foreground mt-2 text-sm leading-relaxed">
						Pick a question to add more thinking to. You can come back here to pick
						another, or finish to update your latest thinking.
					</p>
				</header>

				<div class="space-y-2">
					{#each questions as q, i (q.id)}
						{@const count = flow.answerCountFor(i)}
						<button
							type="button"
							onclick={() => flow.enterRoot(i)}
							class="border-border bg-card hover:border-primary hover:bg-primary/5 flex w-full items-center justify-between gap-3 rounded-lg border px-4 py-3 text-left transition-colors"
						>
							<div class="min-w-0">
								<p class="text-foreground text-sm leading-snug font-medium">
									{q.text || '(unnamed question)'}
								</p>
								<p class="text-muted-foreground mt-1 text-xs">
									{count} answer{count === 1 ? '' : 's'} so far
								</p>
							</div>
							<ChevronRight class="text-muted-foreground size-4 shrink-0" />
						</button>
					{/each}
				</div>

				<div class="border-border flex justify-end border-t pt-6">
					<Button size="lg" onclick={() => flow.finishExtension()}>
						<Check class="size-4" />
						Finish & update my latest thinking
					</Button>
				</div>
			</div>
		{:else}
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

				<!-- Choice — once the follow-up minimum is met, ask whether to keep
			     going deeper or move on. The picker stays hidden until the
			     participant opts in. Initial mode only — extension mode has a
			     single "Done with this question" exit instead. -->
				{#if !inExtensionChain && flow.minReached && flow.currentState.phase !== 'answering' && flow.followUpCount > 0 && !flow.currentState.wantsMore}
					<section bind:this={continueEl} class="border-border border-t pt-8">
						<div class="flex flex-col gap-4 sm:flex-row sm:items-start">
							<div class="flex flex-1 flex-col items-stretch gap-2">
								<Button
									variant="outline"
									size="lg"
									class="h-12 w-full text-base"
									onclick={() => flow.chooseMore()}
								>
									Continue exploring
								</Button>
								<p
									class="text-muted-foreground text-center text-xs leading-relaxed"
								>
									Keep going deeper with more follow-up questions.
								</p>
							</div>
							<div class="flex flex-1 flex-col items-stretch gap-2">
								<Button
									size="lg"
									class="h-12 w-full text-base"
									onclick={() => flow.continueNow()}
								>
									<Check class="size-4" />
									{flow.isLastQuestion ? 'Finish' : 'Move on'}
								</Button>
								<p
									class="text-muted-foreground text-center text-xs leading-relaxed"
								>
									{flow.isLastQuestion
										? "I'm happy with my response and ready to finish."
										: "I'm happy with my response and ready to move on to the next question."}
								</p>
							</div>
						</div>
					</section>
				{/if}

				<!-- Continue alone — when there are no optional follow-ups to offer.
			     Initial mode only. -->
				{#if !inExtensionChain && flow.minReached && flow.currentState.phase !== 'answering' && flow.followUpCount === 0}
					<section bind:this={continueEl} class="border-border border-t pt-8">
						<Button
							size="lg"
							class="h-12 w-full text-base"
							onclick={() => flow.continueNow()}
						>
							<Check class="size-4" />
							{flow.isLastQuestion ? 'Finish' : 'Move on'}
						</Button>
					</section>
				{/if}

				<!-- Picker visible when below the minimum, OR after opting in to
			     deepen. In extension mode the picker is always shown while
			     picking — there's no minimum and no opt-in gate. -->
				{#if flow.currentState.phase === 'picking' && (inExtensionChain || !flow.minReached || flow.currentState.wantsMore)}
					<!-- Picker: loading -->
					{#if flow.currentState.pickerLoading}
						<FollowUpLoading />
					{/if}

					<!-- Picker: failed to load -->
					{#if flow.currentState.pickerError && !flow.currentState.pickerLoading}
						<section class="border-border space-y-3 border-t pt-6">
							<p class="text-muted-foreground text-sm">
								Couldn't load follow-up questions.
							</p>
							<div class="flex flex-wrap gap-2">
								<Button
									variant="outline"
									size="sm"
									onclick={() => flow.retryPicker()}
								>
									<RotateCcw class="size-3.5" />
									Try again
								</Button>
								{#if inExtensionChain}
									<Button size="sm" onclick={() => flow.doneWithRoot()}>
										Done with this question
									</Button>
								{:else if flow.minReached}
									<Button size="sm" onclick={() => flow.continueNow()}>
										{flow.isLastQuestion ? 'Finish' : 'Move on'}
									</Button>
								{/if}
							</div>
						</section>
					{/if}

					<!-- Picker -->
					{#if flow.currentState.picker.length > 0 && !flow.currentState.pickerLoading}
						<section class="border-border space-y-3 border-t pt-6">
							<div class="flex items-start justify-between gap-3">
								<div class="min-w-0">
									<p class="text-foreground text-sm font-semibold">
										{inExtensionChain
											? 'Pick one to go deeper'
											: flow.minReached
												? 'Pick one to go deeper'
												: 'Pick a follow-up to continue'}
									</p>
									{#if inExtensionChain}
										<p class="text-muted-foreground text-xs">
											Or
											<button
												type="button"
												class="text-primary underline-offset-2 hover:underline"
												onclick={() => flow.doneWithRoot()}
											>
												go back to pick a different question
											</button>
											.
										</p>
									{:else if !flow.minReached}
										<p class="text-muted-foreground text-xs">
											{flow.followUpsRemaining} more follow-up{flow.followUpsRemaining ===
											1
												? ''
												: 's'} to go
										</p>
									{:else}
										<p class="text-muted-foreground text-xs">
											Or
											<button
												type="button"
												class="text-primary underline-offset-2 hover:underline"
												onclick={() => flow.continueNow()}
											>
												{flow.isLastQuestion ? 'finish' : 'move on'}
											</button>
											instead.
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
							{#if inExtensionChain}
								<div class="flex justify-end pt-2">
									<Button
										variant="outline"
										size="sm"
										onclick={() => flow.doneWithRoot()}
									>
										<Check class="size-3.5" />
										Done with this question
									</Button>
								</div>
							{/if}
						</section>
					{/if}
				{/if}

				<!-- Active follow-up answer input -->
				{#if flow.currentState.phase === 'answering'}
					<section class="border-border space-y-3 border-t pt-6">
						<div class="flex items-center justify-between gap-3">
							<p
								class="text-muted-foreground flex items-center gap-1.5 text-xs font-semibold tracking-wide uppercase"
							>
								<CornerDownRight class="size-3.5" />
								Follow-up {flow.followUpsDone + 1}
							</p>
							<Button
								variant="ghost"
								size="sm"
								class="text-muted-foreground -my-1"
								onclick={() => flow.backToPicker()}
							>
								<ChevronLeft class="size-3.5" />
								Pick a different question
							</Button>
						</div>
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
		{/if}
	</div>
</div>
