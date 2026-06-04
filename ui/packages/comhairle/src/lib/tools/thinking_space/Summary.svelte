<script lang="ts">
	import { onMount } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { Sparkles, RotateCcw, Check, CornerDownRight, PlusCircle } from 'lucide-svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { fetchSummary, saveSummary } from './summary';
	import type { QuestionConfig, QuestionAnswers } from './types';

	type Props = {
		topic: string;
		workflowStepId: string;
		questions: QuestionConfig[];
		answers: QuestionAnswers[];
		/**
		 * Previously submitted summary, if any. When provided, the screen
		 * renders it directly, no AI re-call on revisit.
		 */
		initialSummary?: string | null;
		onDone?: () => void;
	};

	let {
		topic,
		workflowStepId,
		questions,
		answers,
		initialSummary = null,
		onDone
	}: Props = $props();

	let loading = $state(initialSummary === null);
	let loadError = $state(false);
	let submitting = $state(false);
	let summary = $state(initialSummary ?? '');

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
		if (initialSummary === null) void load();
		const interval = setInterval(() => {
			fading = true;
			setTimeout(() => {
				messageIndex = (messageIndex + 1) % loadingMessages.length;
				fading = false;
			}, 300);
		}, 3500);
		return () => clearInterval(interval);
	});

	async function load() {
		loading = true;
		loadError = false;
		try {
			summary = await fetchSummary({ workflowStepId, topic, questions, answers });
		} catch (e) {
			console.error(e);
			loadError = true;
		} finally {
			loading = false;
		}
	}

	async function submit() {
		const value = summary.trim();
		if (!value || submitting) return;
		submitting = true;
		try {
			await saveSummary({ workflowStepId, summary: value });
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

	function answerMore() {
		// TODO: wire up once the "answer more" pool / flow is finalised with the team.
		notifications.send({
			message: 'Answering more questions is coming soon.',
			priority: 'INFO'
		});
	}
</script>

<div class="mx-auto w-full max-w-2xl px-6 py-10">
	<!-- <header class="mb-8 text-center">
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
	</header> -->

	<!-- Answers recap first: the source material for the summary below. -->
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

	<!-- Summary: the editable artifact submitted as the participant's position statement. -->
	<section class="mt-12 space-y-4">
		<div class="flex items-center gap-2">
			<Sparkles class="text-primary size-4 shrink-0" />
			<p class="text-muted-foreground text-xs font-semibold tracking-wide uppercase">
				Drafted from your answers — edit anything that doesn't sound right
			</p>
		</div>

		{#if loading}
			<div class="space-y-4">
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
		{:else if loadError}
			<div class="space-y-3 text-center">
				<p class="text-muted-foreground text-sm">
					Couldn't generate your summary. Please try again.
				</p>
				<div class="flex justify-center">
					<Button variant="outline" size="sm" onclick={load}>
						<RotateCcw class="size-3.5" />
						Try again
					</Button>
				</div>
			</div>
		{:else}
			<Textarea
				bind:value={summary}
				rows={12}
				class="text-base leading-relaxed"
				placeholder="Your statement…"
			/>

			<div class="flex flex-col gap-2 sm:flex-row sm:justify-end">
				<Button variant="outline" size="lg" class="w-full sm:w-auto" onclick={answerMore}>
					<PlusCircle class="size-4" />
					I want to answer more questions
				</Button>
				<Button
					size="lg"
					class="w-full sm:w-auto"
					onclick={submit}
					disabled={!summary.trim() || submitting}
				>
					<Check class="size-4" />
					{submitting ? 'Saving...' : 'Confirm & Save'}
				</Button>
			</div>
		{/if}
	</section>
</div>
