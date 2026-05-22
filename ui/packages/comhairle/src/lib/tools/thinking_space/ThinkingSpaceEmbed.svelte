<script lang="ts">
	import { onMount } from 'svelte';
	import { Loader2 } from 'lucide-svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import QuestionFlow from './QuestionFlow.svelte';
	import Overview from './Overview.svelte';
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

	let canContinue = $derived(phase === 'overview');

	$effect(() => {
		onCanContinueChange?.(canContinue);
	});

	let configIncomplete = $derived(
		rootQuestions.length === 0 || rootQuestions.every((q) => q.text.trim().length === 0)
	);

	onMount(async () => {
		try {
			answers = await hydrateAnswers();
			// A returning participant who already finished lands on the overview.
			if (allComplete(answers)) phase = 'overview';
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
		const mains = saved.filter((a) => !a.isFollowUp);
		const followUps = saved.filter((a) => a.isFollowUp);

		const result: QuestionAnswers[] = [];
		for (const q of rootQuestions) {
			// Answers store the question text, not the config id, so match on text.
			const main = mains.find((m) => m.question === q.text);
			if (!main) continue;
			result.push({
				questionId: q.id,
				mainAnswer: main.answer,
				mainAnswerId: main.id,
				followUps: followUps
					.filter((f) => f.rootQuestionId === main.id)
					.map((f) => ({ id: f.id, question: f.question, answer: f.answer }))
			});
		}
		return result;
	}

	function allComplete(list: QuestionAnswers[]): boolean {
		if (rootQuestions.length === 0) return false;
		return rootQuestions.every((q) => {
			const a = list.find((x) => x.questionId === q.id);
			return !!a && a.followUps.length >= followUpRoundsCount;
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
					phase = 'overview';
				}}
			/>
		{:else if phase === 'overview'}
			<Overview {topic} questions={rootQuestions} {answers} {onDone} />
		{/if}
	</div>
{/if}
