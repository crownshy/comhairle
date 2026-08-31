<script lang="ts">
	import { tick, untrack, onMount } from 'svelte';
	import * as m from '$lib/paraglide/messages';
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import {
		CornerDownRight,
		Shuffle,
		Check,
		RotateCcw,
		ChevronLeft,
		ChevronRight
	} from 'lucide-svelte';
	import type { QuestionConfig, QuestionAnswers } from './types';
	import { QuestionFlowState, type FlowMode } from './questionFlowState.svelte';
	import FollowUpLoading from './FollowUpLoading.svelte';

	type Props = {
		topic: string;
		workflowStepId: string;
		questions: QuestionConfig<string>[];
		followUpCount: number;
		initialAnswers?: QuestionAnswers[];
		mode?: FlowMode;
		onComplete: (answers: QuestionAnswers[]) => void;
		/** Fraction complete, 0 to 1, for the chrome's progress bar. */
		onProgress?: (fraction: number) => void;
	};

	let {
		topic,
		workflowStepId,
		questions,
		followUpCount,
		initialAnswers = [],
		mode = 'initial',
		onComplete,
		onProgress
	}: Props = $props();

	// Reported up so the chrome's bar can show it. This is the only place that knows how far
	// through the root answer and follow-up rounds a participant is.
	$effect(() => {
		onProgress?.(flow.progress / 100);
	});

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
	<!-- Where you are in the rounds. The bar itself lives in the step chrome (ADR-0018). -->
	<div class="border-border bg-card/60 border-b px-6 py-3 backdrop-blur">
		<div class="mx-auto max-w-2xl">
			<div class="text-muted-foreground flex items-center justify-end gap-3 text-xs">
				<span>
					{#if inExtensionPicker}
						{m.thinking_space_pick_question()}
					{:else if inExtensionChain}
						{m.thinking_space_go_deeper()}
						{flow.currentQuestionIndex + 1}
						{m.of()}
						{questions.length}
					{:else}
						{m.question()}
						{flow.currentQuestionIndex + 1}
						{m.of()}
						{questions.length}
					{/if}
				</span>
			</div>
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
						{m.thinking_space_explore_further()}
					</h2>
					<p class="text-muted-foreground mt-2 text-sm leading-relaxed">
						{m.thinking_space_add_more()}
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
									{count}
									{#if count === 1}
										{m.thinking_space_answer_so_far()}
									{:else}
										{m.thinking_space_answers_so_far()}
									{/if}
								</p>
							</div>
							<ChevronRight class="text-muted-foreground size-4 shrink-0" />
						</button>
					{/each}
				</div>

				<div class="border-border flex justify-end border-t pt-6">
					<Button size="lg" onclick={() => flow.finishExtension()}>
						<Check class="size-4" />
						{m.thinking_space_finish()}
					</Button>
				</div>
			</div>
		{:else}
			<div class="mx-auto max-w-2xl space-y-6">
				<!-- Root question. Sized as the thing on the screen, not as a field label. -->
				<section>
					<p class="text-primary mb-2 text-base font-medium">
						{m.question()}
						{flow.currentQuestionIndex + 1}
					</p>
					<h2
						class="text-foreground text-2xl leading-9 font-semibold sm:text-3xl sm:leading-10"
					>
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
							placeholder={m.thinking_space_write_thoughts()}
							rows={6}
							class="rounded-2xl text-base"
						/>
						<div class="flex justify-end">
							<Button
								onclick={() => flow.submitRootAnswer()}
								disabled={!flow.currentState.rootAnswer.trim() || flow.submitting}
							>
								{flow.submitting ? `${m.saving()}…` : m.continue_()}
							</Button>
						</div>
					</section>
				{:else}
					<section class="border-primary/20 bg-primary/5 rounded-xl border p-4">
						<p class="text-primary mb-1 text-xs font-semibold tracking-wide uppercase">
							{m.thinking_space_you_answered()}
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
							{m.follow_up()}
							{followUpIndex + 1}
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
									{m.thinking_space_continue()}
								</Button>
								<p
									class="text-muted-foreground text-center text-xs leading-relaxed"
								>
									{m.thinking_space_keep_going()}
								</p>
							</div>
							<div class="flex flex-1 flex-col items-stretch gap-2">
								<Button
									size="lg"
									class="h-12 w-full text-base"
									onclick={() => flow.continueNow()}
								>
									<Check class="size-4" />
									{flow.isLastQuestion ? m.finish() : m.move_on()}
								</Button>
								<p
									class="text-muted-foreground text-center text-xs leading-relaxed"
								>
									{flow.isLastQuestion
										? m.thinking_space_happy_finish()
										: m.thinking_space_happy_move_on()}
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
							{flow.isLastQuestion ? m.finish() : m.move_on()}
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
						{#if inExtensionChain}
							<div class="flex justify-end pt-2">
								<Button
									variant="outline"
									size="sm"
									onclick={() => flow.doneWithRoot()}
								>
									<Check class="size-3.5" />
									{m.thinking_space_question_done()}
								</Button>
							</div>
						{/if}
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
									{m.try_again()}
								</Button>
								{#if inExtensionChain}
									<Button size="sm" onclick={() => flow.doneWithRoot()}>
										{m.thinking_space_question_done()}
									</Button>
								{:else if flow.minReached}
									<Button size="sm" onclick={() => flow.continueNow()}>
										{flow.isLastQuestion ? m.finish() : m.move_on()}
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
											? m.thinking_space_pick_deeper()
											: flow.minReached
												? m.thinking_space_pick_deeper()
												: m.thinking_space_pick_follow_up()}
									</p>
									{#if inExtensionChain}
										<p class="text-muted-foreground text-xs">
											{m.or()}
											<button
												type="button"
												class="text-primary underline-offset-2 hover:underline"
												onclick={() => flow.doneWithRoot()}
											>
												{m.thinking_space_pick_back()}
											</button>
											.
										</p>
									{:else if !flow.minReached}
										<p class="text-muted-foreground text-xs">
											{flow.followUpsRemaining}
											{flow.followUpsRemaining === 1
												? m.thinking_space_follow_to_go()
												: m.thinking_space_follows_to_go()}
										</p>
									{:else}
										<p class="text-muted-foreground text-xs">
											{m.or()}
											<button
												type="button"
												class="text-primary lowercase underline-offset-2 hover:underline"
												onclick={() => flow.continueNow()}
											>
												{flow.isLastQuestion ? m.finish() : m.move_on()}
											</button>
											{m.instead()}.
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
									{m.thinking_space_pick_for_me()}
								</Button>
							</div>
							<div class="space-y-2">
								{#each flow.currentState.picker.slice(0, 5) as followUpQuestion, i (followUpQuestion + i)}
									<button
										type="button"
										onclick={() => flow.pickFollowUp(followUpQuestion)}
										class="border-primary/20 bg-primary/5 hover:border-primary hover:bg-primary/10 w-full rounded-lg border px-4 py-3 text-left text-sm leading-relaxed transition-colors"
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
										{m.thinking_space_question_done()}
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
								{m.follow_up()}
								{flow.followUpsDone + 1}
							</p>
							<Button
								variant="ghost"
								size="sm"
								class="text-muted-foreground -my-1"
								onclick={() => flow.backToPicker()}
							>
								<ChevronLeft class="size-3.5" />
								{m.thinking_space_pick_different()}
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
							placeholder={m.thinking_space_write_thoughts()}
							rows={4}
							class="text-base"
						/>
						<div class="flex justify-end">
							<Button
								onclick={() => flow.submitFollowUp()}
								disabled={!flow.currentState.currentPickAnswer.trim() ||
									flow.submitting}
							>
								{flow.submitting ? `${m.saving()}…` : m.continue_()}
							</Button>
						</div>
					</section>
				{/if}

				<div bind:this={bottomEl} class="h-1"></div>
			</div>
		{/if}
	</div>
</div>
