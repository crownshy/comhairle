<script lang="ts">
	import { onMount } from 'svelte';
	import { Loader2 } from 'lucide-svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import QuestionFlow from './QuestionFlow.svelte';
	import Summary from './Summary.svelte';
	import { generateNextRound, hydrateRounds } from './summary';
	import { notifications } from '$lib/notifications.svelte';
	import type {
		QuestionConfig,
		QuestionAnswers,
		SummaryRound,
		ThinkingSpacePhase
	} from './types';
	import type { FlowMode } from './questionFlowState.svelte';

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
	let savedRounds = $state<SummaryRound[]>([]);
	let flowMode = $state<FlowMode>('initial');
	// Snapshot of answer count taken when entering extension mode. If the
	// participant adds no new follow-ups before completing, we skip the new
	// summary generation — nothing to summarise.
	let answerCountAtExtensionStart = $state(0);
	let generatingNextRound = $state(false);
	let generationError = $state(false);

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
			// step — the agreed final screen for revisits. If they already have
			// generated summary rounds, we render them directly with no AI
			// re-call. If they completed Q&A but no summary exists yet (first
			// visit after finishing, or backend wasn't reachable last time),
			// generate the first round now.
			if (allComplete(answers)) {
				phase = 'summary';
				savedRounds = await hydrateRounds({ workflowStepId });
				if (savedRounds.length === 0) {
					void generateRound(answers);
				}
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
						answer: followUp.answer,
						otherQuestions: followUp.otherQuestions ?? []
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

	function totalAnswerCount(list: QuestionAnswers[]): number {
		return list.reduce((acc, q) => acc + (q.rootAnswer ? 1 : 0) + q.followUps.length, 0);
	}

	function handleAnswerMore() {
		flowMode = 'extension';
		answerCountAtExtensionStart = totalAnswerCount(answers);
		phase = 'questions';
	}

	function handleBackFromExtension() {
		// Participant changed their mind mid-extension. Any follow-ups they
		// did answer are already persisted; if any new ones were added, mint
		// a new summary round, otherwise just return to the existing stack.
		void finishExtension(answers);
	}

	async function finishExtension(final: QuestionAnswers[]) {
		const before = answerCountAtExtensionStart;
		const after = totalAnswerCount(final);
		answers = final;
		flowMode = 'initial';
		// Flip back to summary immediately so the participant sees their stack
		// of existing rounds while the new one is generating. Summary renders
		// a loading skeleton below the stack while pendingNextRound is true.
		phase = 'summary';
		if (after <= before) return;
		await generateRound(final);
	}

	async function generateRound(forAnswers: QuestionAnswers[]) {
		generatingNextRound = true;
		generationError = false;
		try {
			const round = await generateNextRound({
				workflowStepId,
				topic,
				questions: rootQuestions,
				answers: forAnswers
			});
			savedRounds = [...savedRounds, round];
		} catch (e) {
			console.error('thinking_space: failed to generate summary round', e);
			generationError = true;
			notifications.send({
				message: 'Could not generate your summary. Please try again.',
				priority: 'ERROR'
			});
		} finally {
			generatingNextRound = false;
		}
	}

	function handleRetryGenerate() {
		void generateRound(answers);
	}

	function handleQuestionFlowComplete(final: QuestionAnswers[]) {
		if (flowMode === 'extension') {
			void finishExtension(final);
			return;
		}
		answers = final;
		// Initial-mode completion: mint the first summary round, then show it.
		phase = 'summary';
		void generateRound(final);
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
				mode={flowMode}
				onComplete={handleQuestionFlowComplete}
				onBack={flowMode === 'extension' ? handleBackFromExtension : undefined}
			/>
		{:else if phase === 'summary'}
			<Summary
				{topic}
				{workflowStepId}
				questions={rootQuestions}
				{answers}
				rounds={savedRounds}
				pendingNextRound={generatingNextRound}
				loadError={generationError}
				onRetryGenerate={handleRetryGenerate}
				{onDone}
				onAnswerMore={handleAnswerMore}
			/>
		{/if}
	</div>
{/if}
