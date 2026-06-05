<script lang="ts">
	import { onMount } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { Sparkles, RotateCcw, Check, CornerDownRight, PlusCircle } from 'lucide-svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { saveRound } from './summary';
	import type { QuestionConfig, QuestionAnswers, SummaryRound } from './types';

	type Props = {
		topic: string;
		workflowStepId: string;
		questions: QuestionConfig[];
		answers: QuestionAnswers[];
		/**
		 * All summary rounds for this participant. The parent owns generation
		 * and appends to this array; Summary is a pure renderer over it.
		 */
		rounds: SummaryRound[];
		/**
		 * True while the parent is generating a round (initial or extension).
		 * Shows the loading skeleton below the existing stack.
		 */
		pendingNextRound?: boolean;
		/**
		 * True if the last generation attempt failed. Renders a Try again
		 * button alongside the existing stack (or instead of it, on first-gen
		 * failure).
		 */
		loadError?: boolean;
		/** Retry the last failed generation. Required when `loadError` is true. */
		onRetryGenerate?: () => void;
		/** Fired when the participant submits the latest round. */
		onDone?: () => void;
		/** Fired when the participant clicks "I want to answer more questions". */
		onAnswerMore?: () => void;
	};

	let {
		topic,
		questions,
		answers,
		workflowStepId,
		rounds,
		pendingNextRound = false,
		loadError = false,
		onRetryGenerate,
		onDone,
		onAnswerMore
	}: Props = $props();

	let submitting = $state(false);
	// Per-round edit drafts (by id). Edits to any round autosave on blur.
	let dirtyById = $state<Record<string, string>>({});
	let savingById = $state<Record<string, boolean>>({});

	const loadingMessages = [
		'Drawing your thoughts together…',
		'Looking for the threads that run through your answers…',
		'Almost there — building a statement that reflects what you shared.'
	];
	const skeletonLines: Array<{ first: string; second: string | null }> = [
		{ first: 'w-full', second: 'w-11/12' },
		{ first: 'w-full', second: 'w-2/3' },
		{ first: 'w-10/12', second: 'w-1/2' }
	];

	let messageIndex = $state(0);
	let fading = $state(false);

	onMount(() => {
		const interval = setInterval(() => {
			fading = true;
			setTimeout(() => {
				messageIndex = (messageIndex + 1) % loadingMessages.length;
				fading = false;
			}, 300);
		}, 3500);
		return () => clearInterval(interval);
	});

	// Only the newest round is editable. Prior rounds are frozen
	function editLatest(id: string, value: string) {
		dirtyById = { ...dirtyById, [id]: value };
	}

	async function persistEdit(id: string) {
		const draft = dirtyById[id];
		if (draft === undefined) return;
		const trimmed = draft.trim();
		if (!trimmed) return;
		savingById = { ...savingById, [id]: true };
		try {
			await saveRound({ workflowStepId, roundId: id, submittedText: trimmed });
			const { [id]: _, ...rest } = dirtyById;
			dirtyById = rest;
		} catch (e) {
			console.error(e);
			notifications.send({
				message: 'Could not save your edit. Please try again.',
				priority: 'ERROR'
			});
		} finally {
			const { [id]: _, ...rest } = savingById;
			savingById = rest;
		}
	}

	function valueFor(round: SummaryRound): string {
		return dirtyById[round.id] ?? round.submittedText;
	}

	async function submit() {
		const latest = rounds[rounds.length - 1];
		if (!latest) return;
		const value = valueFor(latest).trim();
		if (!value || submitting) return;
		submitting = true;
		try {
			if (dirtyById[latest.id] !== undefined) {
				await saveRound({ workflowStepId, roundId: latest.id, submittedText: value });
				const { [latest.id]: _, ...rest } = dirtyById;
				dirtyById = rest;
			}
			onDone?.();
		} catch (e) {
			console.error(e);
			notifications.send({
				message: 'Could not save your summary. Please try again.',
				priority: 'ERROR'
			});
		} finally {
			submitting = false;
		}
	}

	function latestLabel(total: number): string {
		if (total <= 1) return "Your latest thinking — edit anything that doesn't sound right";
		return `Round ${total} — your latest thinking`;
	}

	function frozenLabel(index: number): string {
		return `Round ${index + 1} thinking`;
	}

	let showFirstGenError = $derived(loadError && rounds.length === 0 && !pendingNextRound);
	let showRetryInline = $derived(loadError && rounds.length > 0 && !pendingNextRound);
</script>

<div class="mx-auto w-full max-w-2xl px-6 py-10">
	<header class="mb-8 text-center">
		{#if topic}
			<p class="text-muted-foreground text-xs font-medium tracking-wide uppercase">
				{topic}
			</p>
		{/if}
		<h2 class="text-foreground mt-1 text-3xl font-semibold tracking-tight">Where you stand</h2>
		<p class="text-muted-foreground mx-auto mt-2 max-w-md text-sm leading-relaxed">
			Here's everything you shared, and a short statement we've drafted from it. Edit the
			statement so it sounds like you — that's what you'll submit.
		</p>
	</header>

	<!-- Answers recap: read-only source material for the summaries below. -->
	<section>
		<h3 class="text-foreground text-lg font-semibold">Your answers</h3>
		<p class="text-muted-foreground mt-1 mb-6 text-sm">A recap of what you shared.</p>

		<div class="space-y-6">
			{#each questions as q (q.id)}
				{@const item = answers.find((x) => x.questionId === q.id)}
				{#if item}
					<div class="space-y-2">
						<h4 class="text-foreground text-base font-semibold">{q.text}</h4>
						<p class="text-foreground text-sm leading-relaxed whitespace-pre-wrap">
							{item.rootAnswer}
						</p>
						{#each item.followUps as followUp (followUp.id)}
							<div class="space-y-1 pl-4">
								<p
									class="text-muted-foreground flex items-center gap-1.5 text-sm leading-snug italic"
								>
									<CornerDownRight class="size-3.5 shrink-0" />
									{followUp.question}
								</p>
								<p
									class="text-foreground text-sm leading-relaxed whitespace-pre-wrap"
								>
									{followUp.answer}
								</p>
							</div>
						{/each}
					</div>
				{/if}
			{/each}
		</div>
	</section>

	<!-- Summary stack: one editable textarea per round. -->
	<section class="mt-12 space-y-8">
		{#each rounds as round, i (round.id)}
			{@const isLatest = i === rounds.length - 1}
			{#if isLatest}
				<div class="space-y-3">
					<div class="flex items-center gap-2">
						<Sparkles class="text-primary size-4 shrink-0" />
						<p
							class="text-muted-foreground text-xs font-semibold tracking-wide uppercase"
						>
							{latestLabel(rounds.length)}
						</p>
						{#if savingById[round.id]}
							<span class="text-muted-foreground text-xs">Saving…</span>
						{/if}
					</div>
					<Textarea
						value={valueFor(round)}
						oninput={(e) => editLatest(round.id, e.currentTarget.value)}
						onblur={() => persistEdit(round.id)}
						rows={10}
						class="text-base leading-relaxed"
						placeholder="Your latest thinking…"
					/>
				</div>
			{:else}
				<div class="space-y-2">
					<p class="text-muted-foreground text-xs font-semibold tracking-wide uppercase">
						{frozenLabel(i)}
					</p>
					<p class="text-foreground/80 text-sm leading-relaxed whitespace-pre-wrap">
						{round.submittedText}
					</p>
				</div>
			{/if}
		{/each}

		{#if pendingNextRound}
			<div class="space-y-3">
				<div class="flex items-start gap-2">
					<Sparkles class="text-primary mt-0.5 size-4 shrink-0 animate-pulse" />
					<p
						class="text-muted-foreground text-sm leading-relaxed transition-opacity duration-300"
						class:opacity-0={fading}
						class:opacity-100={!fading}
						aria-live="polite"
					>
						{loadingMessages[messageIndex]}
					</p>
				</div>
				<div
					class="border-border bg-card space-y-3 rounded-lg border px-4 py-4"
					aria-hidden="true"
				>
					{#each skeletonLines as layout, i (i)}
						<div>
							<Skeleton class="h-4 {layout.first}" />
							{#if layout.second}
								<Skeleton class="mt-2 h-4 {layout.second}" />
							{/if}
						</div>
					{/each}
				</div>
			</div>
		{/if}

		{#if showFirstGenError}
			<div class="space-y-3 text-center">
				<p class="text-muted-foreground text-sm">
					Couldn't generate your summary. Please try again.
				</p>
				<div class="flex justify-center">
					<Button
						variant="outline"
						size="sm"
						onclick={() => onRetryGenerate?.()}
						disabled={!onRetryGenerate}
					>
						<RotateCcw class="size-3.5" />
						Try again
					</Button>
				</div>
			</div>
		{:else if !pendingNextRound && rounds.length > 0}
			{#if showRetryInline}
				<div
					class="border-border flex items-center justify-between gap-3 rounded-lg border px-4 py-3"
				>
					<p class="text-muted-foreground text-sm">
						Couldn't generate the new summary round.
					</p>
					<Button
						variant="outline"
						size="sm"
						onclick={() => onRetryGenerate?.()}
						disabled={!onRetryGenerate}
					>
						<RotateCcw class="size-3.5" />
						Try again
					</Button>
				</div>
			{/if}
			<div class="flex flex-col gap-2 sm:flex-row sm:justify-end">
				<Button
					variant="outline"
					size="lg"
					class="w-full sm:w-auto"
					onclick={onAnswerMore}
					disabled={!onAnswerMore}
				>
					<PlusCircle class="size-4" />
					I want to answer more questions
				</Button>
				<Button
					size="lg"
					class="w-full sm:w-auto"
					onclick={submit}
					disabled={!valueFor(rounds[rounds.length - 1]).trim() || submitting}
				>
					<Check class="size-4" />
					{submitting ? 'Saving...' : 'Confirm & Save'}
				</Button>
			</div>
		{/if}
	</section>
</div>
