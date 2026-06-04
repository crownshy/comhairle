<script lang="ts">
	import { onMount } from 'svelte';
	import { Loader2 } from 'lucide-svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import QuestionFlow from './QuestionFlow.svelte';
	import Summary from './Summary.svelte';
	import { hydrateSummary } from './summary';
	import type { QuestionConfig, QuestionAnswers, ThinkingSpacePhase } from './types';

	type Props = {
		workflowStepId: string;
		userId: string;
		topic?: string;
		rootQuestions?: QuestionConfig[];
		followUpRoundsCount?: number;
		onDone?: () => void;
		onCanContinueChange?: (canContinue: boolean) => void;
	};

	let {
		workflowStepId,
		userId,
		topic = '',
		rootQuestions = [],
		followUpRoundsCount = 2,
		onDone,
		onCanContinueChange
	}: Props = $props();

	let loaded = $state(false);
	let loadError = $state(false);
	let phase = $state<ThinkingSpacePhase>('questions');
	let answers = $state<QuestionAnswers[]>([]);
	let savedSummary = $state<string | null>(null);

	let canContinue = $derived(phase === 'summary');

	$effect(() => {
		onCanContinueChange?.(canContinue);
	});

	let configIncomplete = $derived(
		rootQuestions.length === 0 ||
			rootQuestions.every((question) => question.text.trim().length === 0)
	);

	onMount(async () => {
		try {
			answers = await hydrateAnswers();
			// A returning participant who already finished lands on the summary
			// step — the agreed final screen for revisits. If they had also
			// submitted a summary, we render it directly with no AI re-call.
			if (allComplete(answers)) {
				phase = 'summary';
				savedSummary = await hydrateSummary({ workflowStepId });
			}
		} catch (e) {
			console.error('thinking_space: failed to load saved answers', e);
			loadError = true;
		} finally {
			loaded = true;
		}
	});

	// Rebuild the participant's progress from answers already saved on the backend.
	async function hydrateAnswers(): Promise<QuestionAnswers[]> {
		const saved = await apiClient.ListThinkingSpaceAnswers({
			queries: { workflow_step_id: workflowStepId, user_id: userId }
		});
		const savedRoots = saved.filter((answer) => !answer.isFollowUp);
		const savedFollowUps = saved.filter((answer) => answer.isFollowUp);

		const result: QuestionAnswers[] = [];
		for (const question of rootQuestions) {
			// Answers store the question text, not the config id, so match on text.
			const savedRoot = savedRoots.find((answer) => answer.question === question.text);
			if (!savedRoot) continue;
			result.push({
				questionId: question.id,
				rootAnswer: savedRoot.answer,
				rootAnswerId: savedRoot.id,
				followUps: savedFollowUps
					.filter((followUp) => followUp.rootQuestionId === savedRoot.id)
					.map((followUp) => ({
						id: followUp.id,
						question: followUp.question,
						answer: followUp.answer
					}))
			});
		}
		return result;
	}

	function allComplete(list: QuestionAnswers[]): boolean {
		if (rootQuestions.length === 0) return false;
		return rootQuestions.every((question) => {
			const answer = list.find((questionAnswer) => questionAnswer.questionId === question.id);
			return !!answer && answer.followUps.length >= followUpRoundsCount;
		});
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
{:else if loadError}
	<div class="mx-auto max-w-md px-6 py-12 text-center">
		<h2 class="text-foreground text-xl font-semibold">Couldn't load your progress</h2>
		<p class="text-muted-foreground mt-2 text-sm">
			Something went wrong loading this Thinking Space. Please refresh and try again.
		</p>
	</div>
{:else}
	<div class="relative flex min-h-[600px] flex-col">
		{#if phase === 'questions'}
			<QuestionFlow
				{topic}
				{workflowStepId}
				questions={rootQuestions}
				followUpCount={followUpRoundsCount}
				initialAnswers={answers}
				onComplete={(final) => {
					answers = final;
					phase = 'summary';
				}}
			/>
		{:else if phase === 'summary'}
			<Summary
				{topic}
				{workflowStepId}
				questions={rootQuestions}
				{answers}
				initialSummary={savedSummary}
				{onDone}
			/>
		{/if}
	</div>
{/if}
