<script lang="ts">
	import { tick, untrack, onMount } from 'svelte';
	import * as m from '$lib/paraglide/messages';
	import { Button } from '$lib/components/ui/button';
	import { Spinner } from '$lib/components/ui/spinner';
	import {
		CornerDownRight,
		Check,
		RotateCcw,
		ChevronRight,
		ArrowLeft,
		SendHorizontal
	} from 'lucide-svelte';
	import type { QuestionConfig, QuestionAnswers } from './types';
	import { QuestionFlowState, type FlowMode } from './questionFlowState.svelte';
	import type { ToolSequence } from '$lib/step-brief/toolSequence';
	import FollowUpLoading from './FollowUpLoading.svelte';
	import QuestionHandoff from './QuestionHandoff.svelte';
	import QuestionCrossroads from './QuestionCrossroads.svelte';

	type Props = {
		workflowStepId: string;
		questions: QuestionConfig<string>[];
		followUpCount: number;
		initialAnswers?: QuestionAnswers[];
		mode?: FlowMode;
		onComplete: (answers: QuestionAnswers[]) => void;
		/** Progress and the tool-internal back arrow, for the chrome and the pager (ADR-0018). */
		onSequence?: (sequence: ToolSequence) => void;
	};

	let {
		workflowStepId,
		questions,
		followUpCount,
		initialAnswers = [],
		mode = 'initial',
		onComplete,
		onSequence
	}: Props = $props();

	const flow = new QuestionFlowState({
		questions,
		followUpCount,
		workflowStepId,
		initialAnswers,
		onComplete,
		mode
	});

	// The back arrow always undoes the last forward move: out of a picked follow-up, or off the
	// handoff card back onto the question it followed.
	let stepBack = $derived.by(() => {
		if (flow.handoff === 'next') return () => flow.backToPreviousQuestion();
		if (flow.atCrossroads) return undefined;
		if (flow.currentState.phase === 'answering') return () => flow.backToPicker();
		// A picker past the minimum was reached by choosing to go deeper, so undoing that
		// choice puts the fork back.
		if (flow.followUpMinimumMet) return () => flow.backToCrossroads();
		return undefined;
	});

	// Reported up so the chrome's bar can show it, and so the pager's back arrow can step
	// inside the tool before it leaves the step. This is the only place that knows how far
	// through the root answer and follow-up rounds a participant is.
	$effect(() => {
		onSequence?.({ progress: flow.progress / 100, prev: stepBack });
	});

	let answering = $derived(
		flow.handoff === null && !flow.atCrossroads && flow.currentState.phase !== 'picking'
	);
	let isRoot = $derived(flow.currentState.phase === 'root');
	let draft = $derived(
		isRoot ? flow.currentState.rootAnswer : flow.currentState.currentPickAnswer
	);

	let composerEl = $state<HTMLTextAreaElement | null>(null);

	// The box grows with the answer rather than scrolling two fixed rows: on a phone what you
	// typed a second ago was disappearing off the top of a box that still looked half empty.
	// Past the cap it scrolls, so a long answer can't push the question off the screen.
	$effect(() => {
		const el = composerEl;
		if (!el) return;
		void draft;
		el.style.height = 'auto';
		el.style.height = `${el.scrollHeight}px`;
	});

	// Land the caret in the box on every new prompt: on a phone the keyboard coming up is the
	// signal that it is your turn.
	$effect(() => {
		const prompt = `${flow.currentQuestionIndex}:${flow.currentState.phase}:${flow.followUpsDone}:${flow.handoff}:${flow.atCrossroads}`;
		untrack(async () => {
			await tick();
			if (answering) composerEl?.focus();
		});
		void prompt;
	});

	onMount(() => {
		// Extension mode starts in the root picker; loadPicker fires when the
		// participant enters a root via enterRoot().
		if (flow.mode === 'extension') return;
		if (flow.currentState.phase === 'picking' && flow.followUpCount > 0 && !flow.handoff) {
			flow.loadPicker(flow.currentQuestionIndex);
		}
	});

	let inExtensionPicker = $derived(
		flow.mode === 'extension' && flow.extensionPhase === 'root-picker'
	);
	let inExtensionChain = $derived(
		flow.mode === 'extension' && flow.extensionPhase === 'in-chain'
	);

	// An empty picker that isn't in flight yet is still on its way: the fetch is kicked off from
	// onMount and from each answer, so it starts a tick after the state says 'picking'.
	let pickerPending = $derived(
		flow.currentState.pickerLoading ||
			(flow.currentState.picker.length === 0 && !flow.currentState.pickerError)
	);

	function updateDraft(value: string) {
		if (isRoot) flow.updateRootAnswerDraft(value);
		else flow.updateFollowUpDraft(value);
	}

	// Blur before the textarea is torn down. Submitting swaps the composer for the picker
	// while the box still has focus, and iOS Chrome then keeps the editing session alive
	// over a page with nothing to edit: the next drag selects text instead of scrolling.
	function submitDraft() {
		composerEl?.blur();
		if (isRoot) flow.submitRootAnswer();
		else flow.submitFollowUp();
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			submitDraft();
		}
	}
</script>

<div class="flex min-h-0 w-full flex-1 flex-col">
	{#if flow.handoff}
		<QuestionHandoff
			variant={flow.handoff}
			questionNumber={flow.currentQuestionIndex + 1}
			total={questions.length}
			question={flow.currentQuestion.text || '(unnamed question)'}
			followUpCount={flow.followUpCount}
			onStart={() => flow.startQuestion()}
		/>
	{:else if flow.atCrossroads}
		<QuestionCrossroads
			mode={flow.mode}
			questionNumber={flow.currentQuestionIndex + 1}
			total={questions.length}
			followUpsDone={flow.followUpsDone}
			isLastQuestion={flow.isLastQuestion}
			onDeeper={() => flow.goDeeper()}
			onMoveOn={() => (inExtensionChain ? flow.doneWithRoot() : flow.continueNow())}
		/>
	{:else if inExtensionPicker}
		<div class="mx-auto flex w-full max-w-2xl flex-1 flex-col gap-6 py-8">
			<header>
				<h2 class="text-foreground text-2xl leading-snug font-semibold">
					{m.thinking_space_explore_further()}
				</h2>
				<p class="text-muted-foreground mt-2 text-base leading-relaxed">
					{m.thinking_space_add_more()}
				</p>
			</header>

			<div class="space-y-2">
				{#each questions as q, i (q.id)}
					{@const count = flow.answerCountFor(i)}
					<button
						type="button"
						onclick={() => flow.enterRoot(i)}
						class="border-border bg-card hover:border-primary hover:bg-accent flex w-full items-center justify-between gap-3 rounded-xl border px-4 py-3 text-left transition-colors"
					>
						<div class="min-w-0">
							<p class="text-foreground text-base leading-snug">
								{q.text || '(unnamed question)'}
							</p>
							<p class="text-muted-foreground mt-1 text-sm">
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

			<div class="flex justify-end pt-2">
				<Button size="lg" onclick={() => flow.finishExtension()}>
					<Check class="size-4" />
					{m.thinking_space_finish()}
				</Button>
			</div>
		</div>
	{:else if !answering}
		<!-- Pick a follow-up: three questions and nothing else to weigh them against. -->
		<div class="mx-auto flex w-full max-w-2xl flex-1 flex-col gap-6 py-8">
			{#if pickerPending}
				<FollowUpLoading />
			{:else if flow.currentState.pickerError}
				<div class="space-y-4">
					<p class="text-foreground text-base">
						{m.thinking_space_follow_up_failure()}
					</p>
					<div class="flex flex-wrap gap-2">
						<Button variant="outline" onclick={() => flow.retryPicker()}>
							<RotateCcw class="size-4" />
							{m.try_again()}
						</Button>
						{#if inExtensionChain}
							<Button onclick={() => flow.doneWithRoot()}>
								{m.thinking_space_question_done()}
							</Button>
						{/if}
					</div>
				</div>
			{:else}
				{@const optional = flow.followUpMinimumMet}
				<header>
					<h2 class="text-foreground text-xl leading-snug font-semibold">
						{optional
							? m.thinking_space_more_heading()
							: m.thinking_space_pick_follow_up()}
					</h2>
					<p class="text-muted-foreground mt-2 text-base leading-relaxed">
						{optional ? m.thinking_space_more_desc() : m.thinking_space_pick_desc()}
					</p>
				</header>
				<div class="flex min-h-0 flex-1 flex-col gap-3">
					{#each flow.currentState.picker as followUpQuestion (followUpQuestion)}
						<button
							type="button"
							onclick={() => flow.pickFollowUp(followUpQuestion)}
							class="border-border bg-card hover:border-primary hover:bg-accent flex flex-1 items-center rounded-xl border px-4 py-4 text-left text-base leading-relaxed transition-colors"
						>
							{followUpQuestion}
						</button>
					{/each}
				</div>
				{#if optional}
					<div class="flex justify-end">
						<Button variant="outline" onclick={() => flow.backToCrossroads()}>
							<ArrowLeft class="size-4" />
							{m.back()}
						</Button>
					</div>
				{/if}
			{/if}
		</div>
	{:else}
		<!-- One prompt at a time: the question, and the box you answer it in. What you already
		     said is recapped on the summary, not here. On a phone the pair sits at the bottom so
		     the composer stays under the thumb and above the keyboard; on a wide screen there is
		     no thumb to reach and no keyboard to dodge, so it centres instead of leaving a void
		     over it.

		     The keyboard leaves a short strip of screen and the question has to stay in it, so
		     the phone sizes are the tight ones: smaller type, less air, and a question that
		     scrolls inside its own box rather than pushing itself off the top. The cap is that
		     strip, near enough half the screen, less the height of the box itself. -->
		<div
			class="mx-auto flex w-full max-w-2xl flex-1 flex-col justify-end gap-4 py-4 sm:justify-center sm:gap-8 sm:py-8"
		>
			<div class="max-h-[calc(50svh-8rem)] min-h-0 shrink overflow-y-auto sm:max-h-none">
				{#if isRoot}
					<p class="text-primary text-base font-medium">
						{m.question()}
						{flow.currentQuestionIndex + 1}
					</p>
				{:else}
					<p class="text-primary flex items-center gap-1.5 text-base font-medium">
						<CornerDownRight class="size-4" />
						{m.follow_up()}
						{flow.followUpsDone + 1}
					</p>
				{/if}
				<h2
					class="text-foreground mt-1.5 text-xl leading-snug font-semibold sm:mt-2 sm:text-3xl sm:leading-tight"
				>
					{#if isRoot}
						{flow.currentQuestion.text || '(unnamed question)'}
					{:else}
						{flow.currentState.currentPick}
					{/if}
				</h2>
			</div>

			<div
				class="border-input bg-background focus-within:border-ring focus-within:ring-ring/30 shrink-0 rounded-2xl border px-4 pt-3 pb-2 transition-colors focus-within:ring-2"
			>
				<textarea
					bind:this={composerEl}
					value={draft}
					oninput={(e) => updateDraft(e.currentTarget.value)}
					onkeydown={handleKeydown}
					placeholder={m.thinking_space_write_thoughts()}
					rows={2}
					class="text-foreground placeholder:text-muted-foreground max-h-40 min-h-13 w-full resize-none overflow-y-auto bg-transparent text-base leading-relaxed outline-none sm:max-h-56"
				></textarea>
				<div class="flex justify-end">
					<button
						type="button"
						onclick={submitDraft}
						disabled={!draft.trim() || flow.submitting}
						class="text-foreground -mr-1 inline-flex size-9 items-center justify-center rounded-full transition-opacity disabled:opacity-30"
						aria-label={m.thinking_space_send()}
					>
						{#if flow.submitting}
							<Spinner class="size-5" />
						{:else}
							<SendHorizontal class="size-5" />
						{/if}
					</button>
				</div>
			</div>
		</div>
	{/if}
</div>
