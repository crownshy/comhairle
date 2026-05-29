<script lang="ts">
	import { onMount } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { Sparkles, RotateCcw, Check } from 'lucide-svelte';
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
		 * renders it directly — no AI re-call on revisit.
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
			Based on your responses, here's a short summary of where you stand on this topic. Edit
			anything that doesn't sound right — this will be your submitted statement.
		</p>
	</header>

	{#if loading}
		<section class="space-y-4">
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
		</section>
	{:else if loadError}
		<section class="space-y-3 text-center">
			<p class="text-muted-foreground text-sm">
				Couldn't generate your summary. Please try again.
			</p>
			<div class="flex justify-center">
				<Button variant="outline" size="sm" onclick={load}>
					<RotateCcw class="size-3.5" />
					Try again
				</Button>
			</div>
		</section>
	{:else}
		<section class="space-y-4">
			<div class="flex items-center gap-2">
				<Sparkles class="text-primary size-4 shrink-0" />
				<p class="text-muted-foreground text-xs font-semibold tracking-wide uppercase">
					Drafted from your answers
				</p>
			</div>
			<Textarea
				bind:value={summary}
				rows={12}
				class="text-base leading-relaxed"
				placeholder="Your statement…"
			/>
			<div class="flex justify-end">
				<Button
					size="lg"
					class="w-full sm:w-auto"
					onclick={submit}
					disabled={!summary.trim() || submitting}
				>
					<Check class="size-4" />
					{submitting ? 'Submitting…' : 'Submit my statement'}
				</Button>
			</div>
		</section>
	{/if}
</div>
