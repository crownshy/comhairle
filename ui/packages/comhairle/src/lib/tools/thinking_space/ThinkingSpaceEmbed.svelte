<script lang="ts">
	import { onMount } from 'svelte';
	import { Loader2, RotateCcw } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import QuestionFlow from './QuestionFlow.svelte';
	import ReviewPage from './ReviewPage.svelte';
	import Submitted from './Submitted.svelte';
	import {
		emptyState,
		loadParticipantState,
		saveParticipantState,
		clearParticipantState,
		type ParticipantState
	} from './participantStorage';
	import type { QuestionConfig, ParticipantClaim } from './types';

	type Props = {
		conversationId: string;
		workflowId: string;
		workflowStepId: string;
		userId: string;
		topic?: string;
		rootQuestions?: QuestionConfig[];
		followUpRoundsCount?: number;
		onDone?: () => void;
		onCanContinueChange?: (canContinue: boolean) => void;
	};

	let {
		conversationId,
		workflowStepId,
		userId,
		topic = '',
		rootQuestions = [],
		followUpRoundsCount = 2,
		onDone,
		onCanContinueChange
	}: Props = $props();

	let loaded = $state(false);
	let progress = $state<ParticipantState>(emptyState());

	let canContinue = $derived(progress.phase === 'submitted');

	$effect(() => {
		onCanContinueChange?.(canContinue);
	});

	onMount(() => {
		progress = loadParticipantState(workflowStepId, conversationId, userId);
		loaded = true;
	});

	function persist() {
		saveParticipantState(workflowStepId, conversationId, userId, $state.snapshot(progress));
	}

	function handleProgress(snapshot: {
		answers: ParticipantState['answers'];
		claims: ParticipantClaim[];
	}) {
		progress.answers = snapshot.answers;
		progress.claims = snapshot.claims;
		persist();
	}

	function handleQuestionsComplete(result: {
		answers: ParticipantState['answers'];
		claims: ParticipantClaim[];
	}) {
		progress.answers = result.answers;
		progress.claims = result.claims;
		progress.phase = 'review';
		persist();
	}

	function handleClaimsChange(next: ParticipantClaim[]) {
		progress.claims = next;
		persist();
	}

	function handleSubmit() {
		progress.phase = 'submitted';
		persist();
	}

	let configIncomplete = $derived(
		rootQuestions.length === 0 || rootQuestions.every((q) => q.text.trim().length === 0)
	);

	let claimCount = $derived(progress.claims.length);

	const isDev = import.meta.env.DEV;

	function devReset() {
		clearParticipantState(workflowStepId, conversationId, userId);
		progress = emptyState();
	}
</script>

{#if !loaded}
	<div class="flex h-96 items-center justify-center">
		<Loader2 class="text-primary size-6 animate-spin" />
		<span class="text-muted-foreground ml-2 text-sm">Loading…</span>
	</div>
{:else if configIncomplete}
	<div class="mx-auto max-w-md px-6 py-12 text-center">
		<h2 class="text-foreground text-xl font-semibold">Not configured yet</h2>
		<p class="text-muted-foreground mt-2 text-sm">
			An admin needs to add at least one question to this Thinking Space before participants
			can take part.
		</p>
	</div>
{:else}
	<div class="relative flex min-h-[600px] flex-col">
		{#if isDev}
			<Button
				variant="outline"
				size="sm"
				onclick={devReset}
				class="absolute top-2 right-2 z-50 gap-1.5 opacity-60 hover:opacity-100"
				title="Dev only: clear local participant state"
			>
				<RotateCcw class="size-3.5" />
				Reset (dev)
			</Button>
		{/if}
		{#if progress.phase === 'questions'}
			<QuestionFlow
				{topic}
				{workflowStepId}
				questions={rootQuestions}
				followUpCount={followUpRoundsCount}
				initialAnswers={progress.answers}
				initialClaims={progress.claims}
				onProgress={handleProgress}
				onComplete={handleQuestionsComplete}
			/>
		{:else if progress.phase === 'review'}
			<ReviewPage
				claims={progress.claims}
				onChange={handleClaimsChange}
				onSubmit={handleSubmit}
			/>
		{:else if progress.phase === 'submitted'}
			<Submitted {claimCount} {onDone} />
		{/if}
	</div>
{/if}
