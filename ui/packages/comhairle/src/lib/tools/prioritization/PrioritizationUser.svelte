<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import { Progress } from '$lib/components/ui/progress';
	import { Badge } from '$lib/components/ui/badge';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import * as Accordion from '$lib/components/ui/accordion';
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
	import Separator from '$lib/components/ui/separator/separator.svelte';

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
	/** proposalId → sectionId → questionId → value */
	let sectionAnswers = $state<Record<string, Record<string, Record<string, number | string>>>>(
		{}
	);
	let submittedIds = $state<Set<string>>(new Set());
	/** Proposals whose review-stage answers diverged from the originally submitted values. Saved on Continue. */
	let dirtyIds = $state<Set<string>>(new Set());
	type LoadState = { kind: 'loading' } | { kind: 'ready' } | { kind: 'error'; message: string };
	let loadState = $state<LoadState>({ kind: 'loading' });
	let currentIndex = $state(0);
	let submitting = $state(false);
	let submitError = $state<string | null>(null);
	let savingReview = $state(false);
	let reviewError = $state<string | null>(null);

	/** Text answers are optional — completeness only requires likert / continuous. */
	const requiredQuestions = $derived<Question[]>(
		toolConfig.questions.filter((q) => q.type.kind !== 'text')
	);
	const requiredSectionQuestions = $derived<Question[]>(
		toolConfig.sectionQuestions.filter((q) => q.type.kind !== 'text')
	);

	function isAnswered(value: number | string | undefined | null): boolean {
		return typeof value === 'number' && value !== null;
	}

	function isSubmittable(value: number | string | undefined): value is number | string {
		if (typeof value === 'number') return true;
		if (typeof value === 'string') return value.trim().length > 0;
		return false;
	}

	/** Flatten a proposal's proposal-level and per-section answers into the wire shape. */
	function buildResponses(proposal: LocalizedProposal): QuestionResponse[] {
		const pa = answers[proposal.id] ?? {};
		const proposalResponses: QuestionResponse[] = toolConfig.questions
			.filter((q) => isSubmittable(pa[q.id]))
			.map((q) => ({ questionId: q.id, value: pa[q.id] as number | string }));
		const sa = sectionAnswers[proposal.id] ?? {};
		const sectionResponses: QuestionResponse[] = proposal.sections.flatMap((section) => {
			const secVals = sa[section.id] ?? {};
			return toolConfig.sectionQuestions
				.filter((q) => isSubmittable(secVals[q.id]))
				.map((q) => ({
					questionId: q.id,
					sectionId: section.id,
					value: secVals[q.id] as number | string
				}));
		});
		return [...proposalResponses, ...sectionResponses];
	}

	function shuffle<T>(arr: T[]): T[] {
		const out = [...arr];
		for (let i = out.length - 1; i > 0; i--) {
			const j = Math.floor(Math.random() * (i + 1));
			[out[i], out[j]] = [out[j], out[i]];
		}
		return out;
	}

	async function loadProposalsAndProgress() {
		loadState = { kind: 'loading' };
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
			const restoredSectionAnswers: Record<
				string,
				Record<string, Record<string, number | string>>
			> = {};
			ordered.forEach((proposal, i) => {
				const mine = responseLists[i].find((r) => r.userId === participantId);
				if (mine) {
					submitted.add(proposal.id);
					const proposalAnswers: Record<string, number | string> = {};
					const secAnswers: Record<string, Record<string, number | string>> = {};
					for (const r of mine.responses) {
						if (r.sectionId) {
							(secAnswers[r.sectionId] ??= {})[r.questionId] = r.value;
						} else {
							proposalAnswers[r.questionId] = r.value;
						}
					}
					restoredAnswers[proposal.id] = proposalAnswers;
					restoredSectionAnswers[proposal.id] = secAnswers;
				}
			});
			submittedIds = submitted;
			answers = restoredAnswers;
			sectionAnswers = restoredSectionAnswers;

			/** Jump to the first un-submitted proposal so returning users don't have to navigate past completed ones manually. */
			const firstUnsubmitted = ordered.findIndex((p) => !submitted.has(p.id));
			currentIndex = firstUnsubmitted === -1 ? ordered.length - 1 : firstUnsubmitted;

			loadState = { kind: 'ready' };
		} catch (e) {
			loadState = {
				kind: 'error',
				message: e instanceof Error ? e.message : 'Failed to load proposals.'
			};
		}
	}

	$effect(() => {
		void loadProposalsAndProgress();
	});

	let current = $derived(proposals[currentIndex] ?? null);
	let currentSubmitted = $derived(current ? submittedIds.has(current.id) : false);
	let currentAnswers = $derived(current ? (answers[current.id] ?? {}) : {});
	let currentSectionAnswers = $derived(current ? (sectionAnswers[current.id] ?? {}) : {});

	let isComplete = $derived.by(() => {
		if (!current) return false;
		const proposalOk = requiredQuestions.every((q) => isAnswered(currentAnswers[q.id]));
		const sectionsOk = current.sections.every((section) => {
			const sa = currentSectionAnswers[section.id] ?? {};
			return requiredSectionQuestions.every((q) => isAnswered(sa[q.id]));
		});
		return proposalOk && sectionsOk;
	});

	let allDone = $derived(proposals.length > 0 && proposals.every((p) => submittedIds.has(p.id)));

	function setAnswer(proposalId: string, questionId: string, value: number | string) {
		const next = { ...(answers[proposalId] ?? {}), [questionId]: value };
		answers = { ...answers, [proposalId]: next };
	}

	function setSectionAnswer(
		proposalId: string,
		sectionId: string,
		questionId: string,
		value: number | string
	) {
		const sections = { ...(sectionAnswers[proposalId] ?? {}) };
		sections[sectionId] = { ...(sections[sectionId] ?? {}), [questionId]: value };
		sectionAnswers = { ...sectionAnswers, [proposalId]: sections };
	}

	function markDirty(proposalId: string) {
		if (!dirtyIds.has(proposalId)) {
			dirtyIds = new Set([...dirtyIds, proposalId]);
		}
	}

	/** Same as setAnswer but also marks the proposal dirty so we know to upsert on Continue. */
	function setReviewAnswer(proposalId: string, questionId: string, value: number | string) {
		setAnswer(proposalId, questionId, value);
		markDirty(proposalId);
	}

	function setSectionReviewAnswer(
		proposalId: string,
		sectionId: string,
		questionId: string,
		value: number | string
	) {
		setSectionAnswer(proposalId, sectionId, questionId, value);
		markDirty(proposalId);
	}

	async function saveReviewEditsAndContinue() {
		if (savingReview) return;
		reviewError = null;
		if (dirtyIds.size === 0) {
			onDone();
			return;
		}
		savingReview = true;
		try {
			for (const proposalId of dirtyIds) {
				const proposal = proposals.find((p) => p.id === proposalId);
				if (!proposal) continue;
				await api.submitResponse(proposalId, buildResponses(proposal));
			}
			dirtyIds = new Set();
			onDone();
		} catch (e) {
			reviewError = e instanceof Error ? e.message : 'Failed to save your changes.';
		} finally {
			savingReview = false;
		}
	}

	async function submitCurrent() {
		if (!current || !isComplete || currentSubmitted) return;
		submitting = true;
		submitError = null;
		try {
			/** Include all answered questions (proposal-level + per-section). Required
			 * (likert/continuous) are blocked by isComplete; text is optional, so blanks are skipped. */
			await api.submitResponse(current.id, buildResponses(current));
			submittedIds = new Set([...submittedIds, current.id]);
		} catch (e) {
			submitError = e instanceof Error ? e.message : 'Failed to submit response.';
		} finally {
			submitting = false;
		}
	}

	async function submitAndAdvance() {
		await submitCurrent();
		/** If anything is left, move on. Otherwise let allDone surface the summary view. */
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

	function formatAnswer(question: Question, value: number | string | undefined): string {
		if (value === undefined || value === null || value === '') return '—';
		if (question.type.kind === 'likert' && typeof value === 'number') {
			const cat = question.type.categories.find((c) => c.value === value);
			return cat?.label ?? String(value);
		}
		if (question.type.kind === 'continuous' && typeof value === 'number') {
			return value.toFixed(2).replace(/\.?0+$/, '');
		}
		return String(value);
	}
</script>

{#if loadState.kind === 'loading'}
	<div class="text-muted-foreground flex items-center justify-center gap-2 py-12">
		<LoaderCircle class="h-5 w-5 animate-spin" /> Loading proposals…
	</div>
{:else if loadState.kind === 'error'}
	<Card.Root>
		<Card.Content class="space-y-3 py-8 text-center">
			<p class="text-destructive">{loadState.message}</p>
			<Button variant="outline" onclick={() => void loadProposalsAndProgress()}>Try again</Button>
		</Card.Content>
	</Card.Root>
{:else if proposals.length === 0}
	<Card.Root>
		<Card.Content class="py-10 text-center">
			<p class="text-muted-foreground">There are no proposals to rate yet.</p>
		</Card.Content>
	</Card.Root>
{:else if allDone}
	<div class="space-y-6">
		<div class="space-y-1 text-center">
			<!-- <CheckCircle2 class="text-primary mx-auto h-10 w-10" /> -->
			<h2 class="text-l mt-5 font-semibold">Your answers</h2>
			<p class="text-muted-foreground text-sm">
				Tap a proposal to review or adjust your answers. Changes are saved when you
				continue.
			</p>
		</div>

		<Accordion.Root type="multiple" class="space-y-3">
			{#each proposals as proposal (proposal.id)}
				{@const proposalAnswers = answers[proposal.id] ?? {}}
				{@const proposalSectionAnswers = sectionAnswers[proposal.id] ?? {}}
				{@const firstRequired = requiredQuestions[0]}
				{@const summary = firstRequired
					? formatAnswer(firstRequired, proposalAnswers[firstRequired.id])
					: ''}
				<Card.Root class="gap-0 overflow-hidden py-0">
					<Accordion.Item value={proposal.id} class="border-b-0">
						<Accordion.Trigger class="px-4 py-3 hover:no-underline">
							<div class="flex w-full items-center justify-between gap-3 text-left">
								<span class="font-medium"
									>{proposal.title || 'Untitled proposal'}</span
								>
								<div class="flex shrink-0 items-center gap-1.5">
									{#if dirtyIds.has(proposal.id)}
										<Badge variant="outline" class="shrink-0">Edited</Badge>
									{/if}
									{#if summary}
										<Badge variant="secondary" class="shrink-0">{summary}</Badge
										>
									{/if}
								</div>
							</div>
						</Accordion.Trigger>
						<Accordion.Content class="bg-primary/10 space-y-8 px-4 py-4">
							{#each proposal.sections as section (section.id)}
								{#if toolConfig.sectionQuestions.length > 0}
									<div class="grid gap-6 lg:grid-cols-2">
										<div class="text-muted-foreground">
											{#if section.body}
												<ContentRenderer content={section.body} />
											{/if}
										</div>
										<div class="space-y-6">
											{#each toolConfig.sectionQuestions as question (question.id)}
												<QuestionField
													{question}
													value={(proposalSectionAnswers[section.id] ??
														{})[question.id] ?? null}
													onChange={(v) =>
														setSectionReviewAnswer(
															proposal.id,
															section.id,
															question.id,
															v
														)}
												/>
											{/each}
										</div>
									</div>
								{/if}
							{/each}
							{#if toolConfig.questions.length > 0}
								<div
									class="space-y-6 {toolConfig.sectionQuestions.length > 0
										? 'border-t pt-6'
										: ''}"
								>
									{#each toolConfig.questions as question (question.id)}
										<QuestionField
											{question}
											value={proposalAnswers[question.id] ?? null}
											onChange={(v) =>
												setReviewAnswer(proposal.id, question.id, v)}
										/>
									{/each}
								</div>
							{/if}
						</Accordion.Content>
					</Accordion.Item>
				</Card.Root>
			{/each}
		</Accordion.Root>

		{#if reviewError}
			<p class="text-destructive text-right text-sm">{reviewError}</p>
		{/if}
		<div class="flex justify-end">
			<Button onclick={() => void saveReviewEditsAndContinue()} disabled={savingReview}>
				{#if savingReview}
					<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
				{/if}
				{dirtyIds.size > 0 ? 'Save & continue' : 'Continue'}
			</Button>
		</div>
	</div>
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
			</Card.Header>
			<Card.Content class="space-y-8">
				{#each current.sections as section (section.id)}
					<div class="grid gap-6 lg:grid-cols-2">
						<div class="text-muted-foreground">
							{#if section.body}
								<ContentRenderer content={section.body} />
							{/if}
						</div>
						{#if toolConfig.sectionQuestions.length > 0}
							<div class="space-y-6">
								{#each toolConfig.sectionQuestions as question (question.id)}
									<QuestionField
										{question}
										value={(currentSectionAnswers[section.id] ?? {})[
											question.id
										] ?? null}
										disabled={currentSubmitted}
										onChange={(v) =>
											setSectionAnswer(current.id, section.id, question.id, v)}
									/>
								{/each}
							</div>
						{/if}
					</div>
				{/each}

				{#if toolConfig.questions.length > 0}
					<div class="space-y-6 border-t pt-6">
						{#each toolConfig.questions as question (question.id)}
							<QuestionField
								{question}
								value={currentAnswers[question.id] ?? null}
								disabled={currentSubmitted}
								onChange={(v) => setAnswer(current.id, question.id, v)}
							/>
						{/each}
					</div>
				{/if}

				{#if toolConfig.questions.length === 0 && toolConfig.sectionQuestions.length === 0}
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
		{#if submitError}
			<p class="text-destructive text-right text-sm">{submitError}</p>
		{/if}
	</div>
{/if}
