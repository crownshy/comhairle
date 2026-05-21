<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import { Progress } from '$lib/components/ui/progress';
	import { Badge } from '$lib/components/ui/badge';
	import { ArrowLeft, ArrowRight, CheckCircle2, LoaderCircle, RotateCcw } from 'lucide-svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import { getAdapter, getStepContext } from '../context';
	import QuestionField from '../components/QuestionField.svelte';
	import type { LocalizedProposal, Question, QuestionResponse } from '../types';

	const adapter = getAdapter();
	const ctx = getStepContext();
	const onDone = ctx.onDone;

	let proposals = $state<LocalizedProposal[]>([]);
	let answers = $state<Record<string, Record<string, number>>>({}); // proposalId → questionId → value
	let submittedIds = $state<Set<string>>(new Set());
	let loadState = $state<'loading' | 'ready' | 'error'>('loading');
	let loadError = $state<string | null>(null);
	let currentIndex = $state(0);
	let submitting = $state(false);
	let resetting = $state(false);

	/** Only expose the dev reset affordance when Vite is in dev. Production builds tree-shake the `import.meta.env.DEV` check. */
	const showDevReset = import.meta.env.DEV;

	/** Filter out text questions from required-completeness checks because the backend payload doesn't accept them yet. */
	const ratableQuestions = $derived<Question[]>(
		ctx.toolConfig.questions.filter((q) => q.type.kind !== 'text')
	);

	function shuffle<T>(arr: T[]): T[] {
		const out = [...arr];
		for (let i = out.length - 1; i > 0; i--) {
			const j = Math.floor(Math.random() * (i + 1));
			[out[i], out[j]] = [out[j], out[i]];
		}
		return out;
	}

	async function bootstrap() {
		loadState = 'loading';
		try {
			const raw = await adapter.listLocalizedProposals();
			const ordered = ctx.toolConfig.randomizeOrder ? shuffle(raw) : raw;
			proposals = ordered;

			/** Fetch existing responses per proposal in parallel. Filter to the current participant so we can lock proposals they've already answered. */
			const responseLists = await Promise.all(
				ordered.map((p) => adapter.listResponses(p.id).catch(() => []))
			);
			const submitted = new Set<string>();
			const restoredAnswers: Record<string, Record<string, number>> = {};
			ordered.forEach((proposal, i) => {
				const mine = responseLists[i].find((r) => r.userId === ctx.participantId);
				if (mine) {
					submitted.add(proposal.id);
					restoredAnswers[proposal.id] = Object.fromEntries(
						mine.responses.map((r) => [r.questionId, r.value])
					);
				}
			});
			submittedIds = submitted;

			/** Layer drafts from localStorage on top, but never overwrite already-submitted answers. */
			const draft = adapter.loadDraft(ctx.participantId);
			const draftAnswers = draft?.answers ?? {};
			for (const [proposalId, qs] of Object.entries(draftAnswers)) {
				if (submitted.has(proposalId)) continue;
				restoredAnswers[proposalId] = { ...qs };
			}
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
		void bootstrap();
	});

	let current = $derived(proposals[currentIndex] ?? null);
	let currentSubmitted = $derived(current ? submittedIds.has(current.id) : false);
	let currentAnswers = $derived(current ? (answers[current.id] ?? {}) : {});

	let isComplete = $derived(
		current
			? ratableQuestions.every(
					(q) => typeof currentAnswers[q.id] === 'number' && currentAnswers[q.id] !== null
				)
			: false
	);

	let allDone = $derived(proposals.length > 0 && proposals.every((p) => submittedIds.has(p.id)));

	/** Persist drafts (debounced via microtask batching — the value only settles after Svelte's reactive update, so any quick succession of changes collapses into one write). */
	let saveTimer: ReturnType<typeof setTimeout> | undefined;
	function saveDraftSoon() {
		clearTimeout(saveTimer);
		saveTimer = setTimeout(() => {
			adapter.saveDraft({
				stepId: ctx.stepId,
				participantId: ctx.participantId,
				answers,
				updatedAt: Date.now()
			});
		}, 400);
	}

	function setAnswer(proposalId: string, questionId: string, value: number) {
		const next = { ...(answers[proposalId] ?? {}), [questionId]: value };
		answers = { ...answers, [proposalId]: next };
		saveDraftSoon();
	}

	async function submitCurrent() {
		if (!current || !isComplete || currentSubmitted) return;
		submitting = true;
		try {
			const responses: QuestionResponse[] = ratableQuestions
				.map((q) => ({ questionId: q.id, value: currentAnswers[q.id] }))
				.filter((r): r is QuestionResponse => typeof r.value === 'number');
			await adapter.submitResponse(current.id, responses);
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

	function finish() {
		adapter.clearDraft(ctx.participantId);
		onDone?.();
	}

	async function devReset() {
		if (!showDevReset) return;

		resetting = true;
		try {
			await Promise.all(proposals.map((p) => adapter.clearMyResponses(p.id)));
			adapter.clearDraft(ctx.participantId);
			submittedIds = new Set();
			answers = {};
			currentIndex = 0;
			await bootstrap();
		} catch (e) {
			loadError = e instanceof Error ? e.message : 'Reset failed.';
		} finally {
			resetting = false;
		}
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
{:else if allDone}
	<Card.Root>
		<Card.Content class="space-y-4 py-12 text-center">
			<CheckCircle2 class="text-primary mx-auto h-12 w-12" />
			<h2 class="text-2xl font-semibold">Thank you!</h2>
			<p class="text-muted-foreground">
				Your ratings for all {proposals.length} proposals have been recorded.
			</p>

			<div class="flex items-center justify-center gap-2">
				{#if onDone}
					<Button onclick={finish}>Continue</Button>
				{/if}
				{#if showDevReset}
					<Button variant="ghost" onclick={devReset} disabled={resetting}>
						{#if resetting}
							<LoaderCircle class="mr-2 h-3 w-3 animate-spin" />
						{:else}
							<RotateCcw class="mr-2 h-3 w-3" />
						{/if}
						Dev reset
					</Button>
				{/if}
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
				<div class="flex items-center gap-3">
					{#if showDevReset}
						<Button
							variant="ghost"
							size="sm"
							class="text-muted-foreground hover:text-foreground gap-1 text-xs"
							onclick={devReset}
							disabled={resetting || submitting}
							title="Dev only — deletes all of your submitted responses for this step"
						>
							{#if resetting}
								<LoaderCircle class="h-3 w-3 animate-spin" />
							{:else}
								<RotateCcw class="h-3 w-3" />
							{/if}
							Dev reset
						</Button>
					{/if}
					<span class="text-muted-foreground">
						{submittedIds.size} of {proposals.length} done
					</span>
				</div>
			</div>
			<Progress value={progressPercent} />
		</div>

		<Card.Root>
			<Card.Header>
				<div class="flex items-start justify-between gap-3">
					<Card.Title class="text-xl">{current.title || 'Untitled proposal'}</Card.Title>
					{#if currentSubmitted}
						<Badge variant="secondary" class="shrink-0">
							<CheckCircle2 class="mr-1 h-3 w-3" /> Submitted
						</Badge>
					{/if}
				</div>
				{#if current.body}
					<div class="text-muted-foreground pt-2">
						<ContentRenderer content={current.body} />
					</div>
				{/if}
			</Card.Header>
			<Card.Content class="space-y-6">
				{#each ctx.toolConfig.questions as question (question.id)}
					<QuestionField
						{question}
						value={currentAnswers[question.id] ?? null}
						disabled={currentSubmitted}
						onChange={(v) => setAnswer(current.id, question.id, v)}
					/>
				{/each}
				{#if ctx.toolConfig.questions.length === 0}
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
					<Button onclick={finish}>Finish</Button>
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
