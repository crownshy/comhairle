<script lang="ts">
	import { tick, untrack, onMount } from 'svelte';
	import * as m from '$lib/paraglide/messages';
	import { Button } from '$lib/components/ui/button';
	import { Spinner } from '$lib/components/ui/spinner';
	import { CornerDownRight, Check, RotateCcw, ChevronRight, SendHorizontal } from 'lucide-svelte';
	import type { QuestionConfig, QuestionAnswers } from './types';
	import { QuestionFlowState, type FlowMode } from './questionFlowState.svelte';
	import type { ToolSequence } from '$lib/step-brief/toolSequence';
	import FollowUpLoading from './FollowUpLoading.svelte';

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

	// Reported up so the chrome's bar can show it, and so the pager's back arrow steps out of a
	// picked follow-up before it leaves the step. This is the only place that knows how far
	// through the root answer and follow-up rounds a participant is.
	$effect(() => {
		onSequence?.({
			progress: flow.progress / 100,
			prev: flow.currentState.phase === 'answering' ? () => flow.backToPicker() : undefined
		});
	});

	let answering = $derived(flow.currentState.phase !== 'picking');
	let isRoot = $derived(flow.currentState.phase === 'root');
	let draft = $derived(
		isRoot ? flow.currentState.rootAnswer : flow.currentState.currentPickAnswer
	);

	let composerEl = $state<HTMLTextAreaElement | null>(null);

	// Land the caret in the box on every new prompt: on a phone the keyboard coming up is the
	// signal that it is your turn.
	$effect(() => {
		const prompt = `${flow.currentQuestionIndex}:${flow.currentState.phase}:${flow.followUpsDone}`;
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

	function submitDraft() {
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
	{#if inExtensionPicker}
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
				<h2 class="text-foreground text-xl leading-snug font-semibold">
					{m.thinking_space_pick_follow_up()}
				</h2>
				<div class="space-y-3">
					{#each flow.currentState.picker as followUpQuestion (followUpQuestion)}
						<button
							type="button"
							onclick={() => flow.pickFollowUp(followUpQuestion)}
							class="border-border bg-card hover:border-primary hover:bg-accent w-full rounded-xl border px-4 py-4 text-left text-base leading-relaxed transition-colors"
						>
							{followUpQuestion}
						</button>
					{/each}
				</div>
				{#if inExtensionChain}
					<div class="flex justify-end">
						<Button variant="outline" onclick={() => flow.doneWithRoot()}>
							<Check class="size-4" />
							{m.thinking_space_question_done()}
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
		     over it. -->
		<div
			class="mx-auto flex w-full max-w-2xl flex-1 flex-col justify-end gap-8 py-8 sm:justify-center"
		>
			<div>
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
					class="text-foreground mt-2 text-2xl leading-snug font-semibold sm:text-3xl sm:leading-tight"
				>
					{#if isRoot}
						{flow.currentQuestion.text || '(unnamed question)'}
					{:else}
						{flow.currentState.currentPick}
					{/if}
				</h2>
			</div>

			<div
				class="border-input bg-background focus-within:border-ring focus-within:ring-ring/30 rounded-2xl border px-4 pt-3 pb-2 transition-colors focus-within:ring-2"
			>
				<textarea
					bind:this={composerEl}
					value={draft}
					oninput={(e) => updateDraft(e.currentTarget.value)}
					onkeydown={handleKeydown}
					placeholder={m.thinking_space_write_thoughts()}
					rows={2}
					class="text-foreground placeholder:text-muted-foreground w-full resize-none bg-transparent text-base leading-relaxed outline-none"
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
