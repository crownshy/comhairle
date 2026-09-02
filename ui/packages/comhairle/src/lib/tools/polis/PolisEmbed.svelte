<script lang="ts">
	import { Button, LoadingButton } from '$lib/components/ui/button';
	import { fly, fade } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import {
		ThumbsUp,
		ThumbsDown,
		CheckCircle2,
		ChevronRight,
		MessageSquare,
		AlertTriangle,
		Sprout,
		Info
	} from 'lucide-svelte';
	import PolisApi, { type PolisApiState, type PolisStatement } from './PolisApi';
	import PolisVotingSkeleton from './PolisVotingSkeleton.svelte';
	import PolisAddOpinion from './PolisAddOpinion.svelte';
	import { getVoteData, incrementVotes, resetVoteCount } from './polisVoteStore';
	import { hasSeenOpinionGuidance, markOpinionGuidanceSeen } from './polisGuidance';
	import { opinionCounter } from './polisCounter';
	import * as m from '$lib/paraglide/messages';
	import Separator from '$lib/components/ui/separator/separator.svelte';
	import * as Popover from '$lib/components/ui/popover';
	import { apiClient } from '@crownshy/api-client/client';
	import type { OnSequenceChange } from '$lib/step-brief/toolSequence';

	type Props = {
		polis_id: string;
		polis_url: string;
		user_id: string;
		onDone: () => void | Promise<void>;
		requiredVotes?: number;
		workflowStepId?: string;
		isPreview?: boolean;
		showRemainingStatementCount?: boolean;
		onCanContinueChange?: (canContinue: boolean) => void;
		onSequenceChange?: OnSequenceChange;
	};

	let {
		polis_id,
		polis_url,
		user_id,
		onDone,
		requiredVotes = 10,
		workflowStepId = polis_id,
		isPreview = false,
		onCanContinueChange,
		onSequenceChange,
		showRemainingStatementCount
	}: Props = $props();

	const stepId = workflowStepId;

	// Vote progress is stored per step (and separately for preview vs live) so two
	// Polis steps, even ones sharing a poll, never share their threshold state.
	const voteScopeKey = `${isPreview ? 'preview' : 'live'}-${stepId}`;

	let polisCurrentStatement = $state<PolisStatement | undefined>(undefined);
	let polisLoading = $state(false);
	let polisReady = $state(false);
	let polisError = $state<string | undefined>(undefined);
	let polisRemaining = $state(0);
	let polisTotal = $state(0);
	let polisPid = $state<number | undefined>(undefined);

	function handlePolisChange(s: PolisApiState) {
		polisCurrentStatement = s.currentStatement;
		polisLoading = s.loading;
		polisReady = s.ready;
		polisError = s.error;
		polisRemaining = s.remaining;
		polisTotal = s.total;

		if (s.pid !== undefined && s.pid !== polisPid) {
			polisPid = s.pid;
		}

		const newTxt = s.currentStatement?.txt ?? '';
		if (newTxt !== previousText && !s.loading) {
			previousText = newTxt;
			waitingForNext = false;
		}
	}

	const polis = new PolisApi(user_id, polis_id, handlePolisChange, 'en', polis_url);

	// The composer is an overlay rather than a screen, so whatever the participant was on
	// stays underneath and is still there when they come back.
	type Screen = 'voting' | 'continue-prompt' | 'completed';

	const initialData = getVoteData(user_id, voteScopeKey);
	let totalVotes = $state(initialData.totalVotes);
	// `totalVotes` restarts at every prompt, because it is what the threshold counts against.
	// The prompt wants the other number: everything this participant has voted on so far.
	let votesSoFar = $state(initialData.totalVotes);
	let hasMetThreshold = $state(initialData.hasMetThreshold);
	let screen = $state<Screen>('voting');
	let waitingForNext = $state(false);
	let voteCooldown = $state(false);
	let opinionText = $state('');
	let addOpinionOpen = $state(false);
	let showGuidanceOnOpen = $state(false);
	let opinionSubmitted = $state(false);
	let opinionSubmitting = $state(false);
	let opinionError = $state(false);
	let returningToVoting = $state(false);
	const submitBusy = $derived(opinionSubmitting || returningToVoting);
	let previousText = '';
	let visibleStatementWhenOpened: PolisStatement | undefined = undefined;

	// `required_votes` is optional in the tool config and can arrive as null/0,
	// which would break the threshold and progress maths. Fall back to a sane
	// positive default so "continue" still unlocks correctly.
	const safeRequiredVotes = $derived(
		typeof requiredVotes === 'number' && requiredVotes > 0 ? Math.floor(requiredVotes) : 10
	);

	async function createStatementAux(
		newStatement: { tid: number; pid: number },
		statementText: string,
		visibleTid: number | undefined
	) {
		try {
			await apiClient.PolisCreateStatementAux({
				workflow_step_id: stepId,
				zid: newStatement.pid,
				polis_conversation_id: polis_id,
				polis_statement_id: newStatement.tid,
				statement_text: statementText,
				is_seed: false,
				themes: [],
				visible_statement_when_submitted: visibleTid?.toString() ?? null
			});
		} catch (err) {
			console.error('[PolisEmbed] Failed to create statement aux:', err);
		}
	}

	const disabled = $derived(voteCooldown || waitingForNext);
	const canContinue = $derived(hasMetThreshold);

	$effect(() => {
		onCanContinueChange?.(canContinue);
	});

	let anchoredRemaining = $state<number | null>(null);
	let anchoredTotal = $state<number | null>(null);

	$effect(() => {
		if (!polisReady || polisLoading) return;

		if (anchoredRemaining === null || anchoredTotal === null) {
			anchoredRemaining = polisRemaining;
			anchoredTotal = polisTotal;
		} else if (polisTotal !== anchoredTotal) {
			// The pool changed size. Re-sync to the live counts
			anchoredTotal = polisTotal;
			anchoredRemaining = polisRemaining;
		}
	});

	const opinionPosition = $derived(
		opinionCounter(anchoredTotal ?? polisTotal, anchoredRemaining ?? polisRemaining)
	);

	// What is still unvoted, for the prompt: the same number the "Opinion x of y" counter
	// is built from, said the other way round.
	const opinionsLeft = $derived(Math.max(0, anchoredRemaining ?? polisRemaining));

	const poolExhausted = $derived(
		polisReady && !polisLoading && !polisError && !polisCurrentStatement
	);

	$effect(() => {
		if (screen === 'voting' && poolExhausted) {
			screen = 'completed';
		}
	});

	function doVote(type: 'agree' | 'disagree' | 'pass') {
		if (voteCooldown || !polisCurrentStatement) return;
		waitingForNext = true;
		voteCooldown = true;

		polis.submitVote(type);
		totalVotes++;
		votesSoFar++;

		if (anchoredRemaining !== null && anchoredRemaining > 0) {
			anchoredRemaining--;
		}

		const data = incrementVotes(user_id, voteScopeKey, safeRequiredVotes);
		hasMetThreshold = data.hasMetThreshold;

		if (data.totalVotes === safeRequiredVotes) {
			// Flip now, not after a beat: Polis has already been asked for the next
			// statement, and any delay here lets it render behind the prompt first.
			screen = 'continue-prompt';
			voteCooldown = false;
			waitingForNext = false;
			return;
		}

		setTimeout(() => {
			voteCooldown = false;
		}, 800);
	}

	function resumeVoting() {
		resetVoteCount(user_id, voteScopeKey);
		totalVotes = 0;
		screen = 'voting';
	}

	let continuing = $state(false);

	async function handleContinue() {
		if (continuing) return;
		continuing = true;
		try {
			await onDone();
		} finally {
			// Navigation usually unmounts us first; reset as a safety net if it didn't.
			continuing = false;
		}
	}

	async function submitOpinion(text: string): Promise<boolean> {
		const visibleTid = visibleStatementWhenOpened?.tid;
		opinionSubmitting = true;
		opinionError = false;
		const result = await polis.submitStatement(text);
		opinionSubmitting = false;
		if (!result) {
			opinionError = true;
			return false;
		}
		await createStatementAux(result, text, visibleTid);
		opinionText = '';

		polis.fetchNextStatement();
		return true;
	}

	async function handleSubmitOpinion() {
		const text = opinionText.trim();
		if (!text || opinionSubmitting) return;
		if (!(await submitOpinion(text))) return;
		opinionSubmitted = true;
		returningToVoting = true;
		setTimeout(() => {
			addOpinionOpen = false;
			opinionSubmitted = false;
			returningToVoting = false;
		}, 2000);
	}

	async function handleSubmitAndAddAnother() {
		const text = opinionText.trim();
		if (!text || opinionSubmitting) return;
		if (!(await submitOpinion(text))) return;

		opinionSubmitted = true;
		setTimeout(() => {
			opinionSubmitted = false;
		}, 2000);
	}

	function openAddOpinion() {
		visibleStatementWhenOpened = polisCurrentStatement;
		showGuidanceOnOpen = !hasSeenOpinionGuidance(user_id);
		addOpinionOpen = true;
		opinionSubmitted = false;
		opinionError = false;
		returningToVoting = false;
	}

	function closeAddOpinion() {
		addOpinionOpen = false;
		if (polis.state.remaining === 0) {
			screen = 'completed';
		}
	}

	// Polis has no internal sequence the pager traverses: every vote lands on the same
	// screen. It reports progress and its position only, and the chrome draws both
	// (ADR-0018). `showRemainingStatementCount` now gates the chrome's count.
	$effect(() => {
		onSequenceChange?.({
			progress: totalVotes / safeRequiredVotes,
			count:
				showRemainingStatementCount && polisReady && !polisError && !poolExhausted
					? m.polis_opinion_counter({
							current: opinionPosition.current,
							total: opinionPosition.total
						})
					: undefined
		});
	});
</script>

<div
	class="bg-muted/50 flex min-h-0 w-full flex-1 flex-col items-center gap-8 overflow-y-auto rounded-2xl"
>
	{#if !polisReady}
		<!-- First fetch. `screen` is always 'voting' here, so stand in for that screen
		     whole rather than for the statement alone. -->
		<PolisVotingSkeleton />
	{:else if screen === 'voting'}
		<!-- Voting Screen -->
		<div class="flex w-full flex-1 flex-col" in:fade={{ duration: 300 }}>
			<div
				class="mx-auto flex w-full max-w-[808px] flex-1 flex-col items-start gap-[clamp(0.75rem,2.2vh,1.5rem)] px-6 py-[clamp(1rem,2.5vh,3rem)] md:px-16"
			>
				<!-- Statement text: one opinion at a time, sized to be the thing you read. The
			     statement is the only band that flexes, because it is the only one whose height
			     changes with the content; the vote controls and "add your own" below it hold
			     their place at the bottom of the card. Every vertical measure is a vh clamp, so
			     a short viewport shrinks the screen instead of pushing the buttons off it. -->
				<div class="flex w-full flex-1 items-center">
					{#if polisReady && polisError}
						<div
							class="border-destructive/20 bg-destructive/5 flex w-full flex-col items-center gap-4 rounded-lg border p-6 text-center"
							in:fade={{ duration: 300 }}
						>
							<AlertTriangle class="text-destructive h-8 w-8" />
							<p class="text-foreground text-lg font-medium">
								{m.something_went_wrong()}
							</p>
							<p class="text-muted-foreground text-sm">
								{m.polis_error_description()}
							</p>
						</div>
					{:else if waitingForNext || !polisCurrentStatement}
						<!-- Between statements, or briefly empty before the screen flips to
					     "completed". The vote controls stay put below, so only the statement
					     needs standing in for. -->
						<div in:fade={{ duration: 200 }} class="w-full animate-pulse">
							<div class="space-y-3">
								<div class="bg-foreground/10 h-8 w-full rounded"></div>
								<div class="bg-foreground/10 h-8 w-4/5 rounded"></div>
								<div class="bg-foreground/10 h-8 w-3/5 rounded"></div>
							</div>
						</div>
					{:else if polisCurrentStatement}
						<div
							class="flex w-full flex-col gap-2"
							in:fly={{ y: 20, duration: 500, easing: cubicOut }}
						>
							{#if polisCurrentStatement.is_seed}
								<!-- Tap, not hover: participants are mostly on phones, and "conversation
							     starter" means nothing until someone tells you who wrote it. -->
								<Popover.Root>
									<Popover.Trigger
										class="text-muted-foreground hover:text-foreground flex w-fit items-center gap-1.5 text-sm font-medium underline decoration-dotted underline-offset-4 transition-colors"
										aria-label={m.polis_seed_statement_explainer_label()}
									>
										<Sprout class="size-4 shrink-0" aria-hidden="true" />
										{m.polis_seed_statement()}
										<Info class="size-3.5 shrink-0" aria-hidden="true" />
									</Popover.Trigger>
									<Popover.Content
										align="start"
										class="max-w-[min(20rem,calc(100vw-2rem))]"
									>
										<p class="text-foreground text-base leading-snug">
											{m.polis_seed_statement_explainer()}
										</p>
									</Popover.Content>
								</Popover.Root>
							{/if}
							<p
								class="text-card-foreground text-[clamp(1.25rem,3.2vh,2rem)] leading-tight font-bold sm:text-[clamp(1.5rem,4vh,2.25rem)]"
							>
								{polisCurrentStatement.txt}
							</p>
						</div>
					{/if}
				</div>

				{#if !polisError && polisCurrentStatement}
					<!-- Two thumbs, thumb-reachable, pushed to opposite sides of the column so
					     agree and disagree are never a mis-tap apart. Capped so they stay a pair
					     on a wide viewport instead of drifting to the far edges.

					     No pass control for now. `doVote('pass')` still works and Polis still
					     accepts the vote type: this is the only affordance that sent one. -->
					<div class="flex w-full max-w-[560px] items-center justify-between">
						<button
							type="button"
							class="polis-thumb bg-primary text-primary-foreground flex aspect-square w-[44%] max-w-[clamp(5rem,17vh,148px)] shrink-0 touch-manipulation items-center justify-center rounded-full disabled:opacity-40"
							aria-label={m.polis_agree()}
							disabled={disabled || !polisReady}
							onclick={() => doVote('agree')}
						>
							<ThumbsUp class="size-[55%]" />
						</button>
						<button
							type="button"
							class="polis-thumb bg-primary text-primary-foreground flex aspect-square w-[44%] max-w-[clamp(5rem,17vh,148px)] shrink-0 touch-manipulation items-center justify-center rounded-full disabled:opacity-40"
							aria-label={m.polis_disagree()}
							disabled={disabled || !polisReady}
							onclick={() => doVote('disagree')}
						>
							<ThumbsDown class="size-[55%]" />
						</button>
					</div>
				{/if}
			</div>

			{#if !polisError}
				<!-- Adding your own is a different job from voting, so it gets its own band at
				     the foot of the card rather than a third control in the voting column. -->
				<div
					class="bg-background flex w-full shrink-0 flex-col items-center gap-1 rounded-b-2xl border-t px-6 py-[clamp(0.75rem,2vh,1.25rem)]"
				>
					<!-- The prompt is the first thing to go when there is no room for it: the
					     button below it says the same thing. -->
					<p class="text-muted-foreground text-base [@media(max-height:700px)]:hidden">
						{m.polis_dont_see_your_view()}
					</p>
					<Button
						variant="ghost"
						class="text-primary hover:text-primary flex items-center gap-2 text-base font-semibold"
						disabled={!polisReady}
						onclick={openAddOpinion}
					>
						<MessageSquare class="h-5 w-5" />
						{m.polis_add_opinion()}
					</Button>
				</div>
			{/if}
		</div>
	{:else if screen === 'continue-prompt'}
		<!-- The threshold is met, so this is a fork and not a gate. It says what has been counted
		     and what each way on does, and gives the two of them the same weight: the participant
		     is not being asked to admit they are giving up. -->
		<div
			class="flex w-full max-w-[808px] flex-1 flex-col items-center justify-center gap-[clamp(1.5rem,4vh,2.5rem)] px-6 py-[clamp(1rem,2.5vh,3rem)] text-center md:px-16"
			in:fade={{ duration: 300 }}
		>
			<div class="flex flex-col items-center gap-3">
				<CheckCircle2 class="text-primary size-8" />
				<h2
					class="text-card-foreground max-w-[20ch] text-[clamp(1.5rem,4vh,2rem)] leading-tight font-semibold"
				>
					{votesSoFar === 1
						? m.polis_votes_counted_one({ count: votesSoFar })
						: m.polis_votes_counted({ count: votesSoFar })}
				</h2>
				<p class="text-muted-foreground max-w-[36ch] text-base">
					{opinionsLeft > 0
						? m.polis_continue_prompt_body()
						: m.polis_nothing_left_to_vote_on()}
				</p>
			</div>

			<div class="flex w-full max-w-[360px] flex-col items-stretch gap-3">
				{#if opinionsLeft > 0}
					<Button
						variant="default"
						onclick={resumeVoting}
						class="h-auto w-full flex-col gap-0.5 rounded-2xl px-6 py-3.5 whitespace-normal"
					>
						<span class="text-lg font-semibold">{m.polis_keep_voting()}</span>
						<span class="text-primary-foreground/80 text-base font-normal">
							{opinionsLeft === 1
								? m.polis_keep_voting_hint_one({ count: opinionsLeft })
								: m.polis_keep_voting_hint({ count: opinionsLeft })}
						</span>
					</Button>
				{/if}
				<LoadingButton
					variant={opinionsLeft > 0 ? 'outline' : 'default'}
					loading={continuing}
					class="h-auto w-full flex-col gap-0.5 rounded-2xl px-6 py-3.5 whitespace-normal"
					onclick={handleContinue}
				>
					<span class="text-lg font-semibold">{m.polis_finish_voting()}</span>
					<span class="text-base font-normal opacity-80"
						>{m.polis_finish_voting_hint()}</span
					>
				</LoadingButton>
			</div>
		</div>
	{:else if screen === 'completed'}
		<!-- Voted everything -->
		<div
			class="flex w-full max-w-[808px] flex-col items-start gap-6 px-8 py-8 md:px-24 md:py-12"
			in:fade={{ duration: 300 }}
		>
			<p class="text-card-foreground text-3xl font-normal">
				{m.polis_voted_everything()}
			</p>
			<p class="text-muted-foreground text-lg">
				{m.polis_come_back_later()}
			</p>
			<Separator orientation="horizontal" />

			<!-- Add your own opinion -->
			<Button
				variant="secondary"
				class="text-foreground hover:text-foreground flex items-center gap-2 p-5 text-xl font-bold transition-colors"
				disabled={!polisReady}
				onclick={openAddOpinion}
			>
				<MessageSquare class="h-5 w-5" />
				<span class="hidden md:inline">{m.polis_add_opinion_long()}</span>
				<span class="md:hidden">{m.polis_add_your_own_opinion()}</span>
			</Button>
		</div>

		<LoadingButton
			variant="primaryDark"
			size="lg"
			loading={continuing}
			onclick={handleContinue}
			class="mb-5 gap-2 px-6 py-4 text-lg"
		>
			{m.continue_()}
			{#if !continuing}<ChevronRight class="h-5 w-5" />{/if}
		</LoadingButton>
	{/if}
</div>

{#if addOpinionOpen}
	<PolisAddOpinion
		bind:value={opinionText}
		submitting={submitBusy}
		submitted={opinionSubmitted}
		error={opinionError}
		startOnGuidance={showGuidanceOnOpen}
		onClose={closeAddOpinion}
		onEdit={() => (opinionError = false)}
		onGuidanceRead={() => markOpinionGuidanceSeen(user_id)}
		onSubmit={handleSubmitOpinion}
		onSubmitAndAddAnother={handleSubmitAndAddAnother}
	/>
{/if}

<style>
	.polis-thumb {
		transition:
			transform 140ms ease,
			opacity 140ms ease;
	}

	.polis-thumb:active:not(:disabled) {
		transform: scale(0.9);
	}

	@media (prefers-reduced-motion: reduce) {
		.polis-thumb {
			transition: none;
		}
	}
</style>
