<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import { Progress } from '$lib/components/ui/progress';
	import { Badge } from '$lib/components/ui/badge';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { ArrowLeft, ArrowRight, CheckCircle2, Info, LoaderCircle } from 'lucide-svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import QuestionField from './components/QuestionField.svelte';
	import * as api from './prioritizationApi';
	import type {
		ConversationInput,
		LocalizedProposal,
		Question,
		QuestionResponse,
		WorkflowStepInput
	} from './types';

	let {
		workflowStep,
		conversation,
		participantId = '',
		onDone
	}: {
		workflowStep: WorkflowStepInput;
		conversation: ConversationInput;
		participantId?: string;
		onDone: () => void;
	} = $props();

	const stepId = $derived(workflowStep.id);
	const toolConfig = $derived(api.resolveToolConfig(workflowStep, conversation.isLive ?? false));

	let proposals = $state<LocalizedProposal[]>([]);
	let answers = $state<Record<string, Record<string, number | string>>>({}); // proposalId → questionId → value
	let submittedIds = $state<Set<string>>(new Set());
	let loadState = $state<'loading' | 'ready' | 'error'>('loading');
	let loadError = $state<string | null>(null);
	let currentIndex = $state(0);
	let submitting = $state(false);

	/** Text answers are optional — completeness only requires likert / continuous. */
	const requiredQuestions = $derived<Question[]>(
		toolConfig.questions.filter((q) => q.type.kind !== 'text')
	);

	function shuffle<T>(arr: T[]): T[] {
		const out = [...arr];
		for (let i = out.length - 1; i > 0; i--) {
			const j = Math.floor(Math.random() * (i + 1));
			[out[i], out[j]] = [out[j], out[i]];
		}
		return out;
	}

	async function loadProposalsAndProgress() {
		loadState = 'loading';
		try {
			const raw = await api.listLocalizedProposals(stepId);
			const ordered = toolConfig.randomizeOrder ? shuffle(raw) : raw;
			proposals = ordered;

			/** Fetch existing responses per proposal in parallel. Filter to the current participant so we can lock proposals they've already answered. */
			const responseLists = await Promise.all(
				ordered.map((p) => api.listResponses(p.id).catch(() => []))
			);
			const submitted = new Set<string>();
			const restoredAnswers: Record<string, Record<string, number | string>> = {};
			ordered.forEach((proposal, i) => {
				const mine = responseLists[i].find((r) => r.userId === participantId);
				if (mine) {
					submitted.add(proposal.id);
					restoredAnswers[proposal.id] = Object.fromEntries(
						mine.responses.map((r) => [r.questionId, r.value])
					);
				}
			});
			submittedIds = submitted;
			answers = restoredAnswers;

			/** Jump to the first un-submitted proposal so returning users don't have to navigate past completed ones manually. */
			const firstUnsubmitted = ordered.findIndex((p) => !submitted.has(p.id));
			currentIndex = firstUnsubmitted === -1 ? ordered.length - 1 : firstUnsubmitted;

			loadState = 'ready';
		} catch (e) {
			loadError = e instanceof Error ? e.message : 'Failed to load proposals.';
			loadState = 'error';
		}
	}

	$effect(() => {
		void loadProposalsAndProgress();
	});

	let current = $derived(proposals[currentIndex] ?? null);
	let currentSubmitted = $derived(current ? submittedIds.has(current.id) : false);
	let currentAnswers = $derived(current ? (answers[current.id] ?? {}) : {});

	let isComplete = $derived(
		current
			? requiredQuestions.every(
					(q) => typeof currentAnswers[q.id] === 'number' && currentAnswers[q.id] !== null
				)
			: false
	);

	let allDone = $derived(proposals.length > 0 && proposals.every((p) => submittedIds.has(p.id)));

	let reviewing = $state(false);

	function startReview() {
		reviewing = true;
		currentIndex = 0;
	}

	function endReview() {
		reviewing = false;
	}

	function setAnswer(proposalId: string, questionId: string, value: number | string) {
		const next = { ...(answers[proposalId] ?? {}), [questionId]: value };
		answers = { ...answers, [proposalId]: next };
	}

	async function submitCurrent() {
		if (!current || !isComplete || currentSubmitted) return;
		submitting = true;
		try {
			/** Include all answered questions. Required (likert/continuous) are blocked
			 * by isComplete; text is optional, so we just skip blanks. */
			const responses: QuestionResponse[] = toolConfig.questions
				.map((q) => ({ questionId: q.id, value: currentAnswers[q.id] }))
				.filter((r): r is QuestionResponse => {
					if (typeof r.value === 'number') return true;
					if (typeof r.value === 'string') return r.value.trim().length > 0;
					return false;
				});
			await api.submitResponse(current.id, responses);
			submittedIds = new Set([...submittedIds, current.id]);
		} catch (e) {
			loadError = e instanceof Error ? e.message : 'Failed to submit response.';
		} finally {
			submitting = false;
		}
	}

	async function submitAndAdvance() {
		await submitCurrent();
		/** If anything is left, move on. Otherwise let allDone surface the thank-you screen. */
		if (currentIndex < proposals.length - 1) {
			currentIndex += 1;
		}
	}

	function goBack() {
		if (currentIndex > 0) currentIndex -= 1;
	}

	let progressPercent = $derived(
		proposals.length === 0 ? 0 : Math.round((submittedIds.size / proposals.length) * 100)
	);
</script>

{#if loadState === 'loading'}
	<div class="text-muted-foreground flex items-center justify-center gap-2 py-12">
		<LoaderCircle class="h-5 w-5 animate-spin" /> Loading proposals…
	</div>
{:else if loadState === 'error'}
	<Card.Root>
		<Card.Content class="space-y-3 py-8 text-center">
			<p class="text-destructive">{loadError}</p>
			<Button variant="outline" onclick={() => void bootstrap()}>Try again</Button>
		</Card.Content>
	</Card.Root>
{:else if proposals.length === 0}
	<Card.Root>
		<Card.Content class="py-10 text-center">
			<p class="text-muted-foreground">There are no proposals to rate yet.</p>
		</Card.Content>
	</Card.Root>
{:else if allDone && !reviewing}
	<Card.Root>
		<Card.Content class="space-y-4 py-12 text-center">
			<CheckCircle2 class="text-primary mx-auto h-12 w-12" />
			<h2 class="text-2xl font-semibold">Thank you!</h2>
			<p class="text-muted-foreground">
				Your ratings for all {proposals.length} proposals have been recorded.
			</p>

			<div class="flex flex-wrap items-center justify-center gap-2 pt-2">
				<Button variant="outline" onclick={startReview}>Review your answers</Button>
				<Button onclick={onDone}>Continue</Button>
			</div>
		</Card.Content>
	</Card.Root>
{:else if current}
	<div class="space-y-6">
		<div class="space-y-2">
			<div class="flex items-center justify-between gap-3 text-sm">
				<span class="text-muted-foreground">
					Proposal {currentIndex + 1} of {proposals.length}
				</span>
				<span class="text-muted-foreground">
					{submittedIds.size} of {proposals.length} done
				</span>
			</div>
			<Progress value={progressPercent} />
		</div>

		<Card.Root>
			<Card.Header>
				<div class="flex items-start justify-between gap-3">
					<Card.Title class="text-xl">{current.title || 'Untitled proposal'}</Card.Title>
					{#if currentSubmitted}
						<div class="flex shrink-0 items-center gap-1.5">
							<Badge variant="secondary">
								<CheckCircle2 class="mr-1 h-3 w-3" /> Submitted
							</Badge>
							<Tooltip.Provider delayDuration={150}>
								<Tooltip.Root>
									<Tooltip.Trigger
										class="text-muted-foreground hover:text-foreground"
										aria-label="What does Submitted mean?"
									>
										<Info class="size-4" />
									</Tooltip.Trigger>
									<Tooltip.Content class="max-w-xs text-sm">
										You've already submitted your answers for this proposal.
										They can't be changed, but you can review them here.
									</Tooltip.Content>
								</Tooltip.Root>
							</Tooltip.Provider>
						</div>
					{/if}
				</div>
				{#if current.body}
					<div class="text-muted-foreground pt-2">
						<ContentRenderer content={current.body} />
					</div>
				{/if}
			</Card.Header>
			<Card.Content class="space-y-6">
				{#each toolConfig.questions as question (question.id)}
					<QuestionField
						{question}
						value={currentAnswers[question.id] ?? null}
						disabled={currentSubmitted}
						onChange={(v) => setAnswer(current.id, question.id, v)}
					/>
				{/each}
				{#if toolConfig.questions.length === 0}
					<p class="text-muted-foreground text-sm">
						No questions configured for this step yet.
					</p>
				{/if}
			</Card.Content>
		</Card.Root>

		<div class="flex items-center justify-between">
			<Button variant="ghost" onclick={goBack} disabled={currentIndex === 0 || submitting}>
				<ArrowLeft class="mr-2 h-4 w-4" /> Previous
			</Button>

			{#if currentSubmitted}
				{#if currentIndex < proposals.length - 1}
					<Button onclick={() => (currentIndex += 1)}>
						Next <ArrowRight class="ml-2 h-4 w-4" />
					</Button>
				{:else if reviewing}
					<Button onclick={endReview}>Back to summary</Button>
				{:else}
					<Button onclick={onDone}>Finish</Button>
				{/if}
			{:else}
				<Button onclick={submitAndAdvance} disabled={!isComplete || submitting}>
					{#if submitting}
						<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
					{/if}
					{currentIndex < proposals.length - 1 ? 'Submit & continue' : 'Submit'}
					{#if !submitting}<ArrowRight class="ml-2 h-4 w-4" />{/if}
				</Button>
			{/if}
		</div>
	</div>
{/if}
