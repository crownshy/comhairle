<script lang="ts">
	import { Button, LoadingButton } from '$lib/components/ui/button';
	import { fly, fade } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import {
		ThumbsUp,
		ThumbsDown,
		SkipForward,
		PenLine,
		X,
		ChevronRight,
		MessageSquare,
		AlertTriangle
	} from 'lucide-svelte';
	import PolisApi, { type PolisApiState, type PolisStatement } from './PolisApi';
	import { getVoteData, incrementVotes, resetVoteCount } from './polisVoteStore';
	import { opinionCounter } from './polisCounter';
	import * as m from '$lib/paraglide/messages';
	import Separator from '$lib/components/ui/separator/separator.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { getLocale } from '$lib/paraglide/runtime';

	type Props = {
		polis_id: string;
		polis_url: string;
		user_id: string;
		onDone: () => void | Promise<void>;
		requiredVotes?: number;
		workflowStepId?: string;
		showRemainingStatementCount?: boolean;
		onCanContinueChange?: (canContinue: boolean) => void;
	};

	let {
		polis_id,
		polis_url,
		user_id,
		onDone,
		requiredVotes = 10,
		workflowStepId = polis_id,
		onCanContinueChange,
		showRemainingStatementCount
	}: Props = $props();

	const stepId = workflowStepId;

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

	type Screen = 'voting' | 'add-opinion' | 'continue-prompt' | 'completed';

	const initialData = getVoteData(user_id, polis_id);
	let totalVotes = $state(initialData.totalVotes);
	let hasMetThreshold = $state(initialData.hasMetThreshold);
	let screen = $state<Screen>('voting');
	let waitingForNext = $state(false);
	let voteCooldown = $state(false);
	let opinionText = $state('');
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
				// Hint the source language from the participant's active UI locale.
				// The backend falls back to auto-detection when this is absent.
				source_locale: getLocale(),
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

		if (anchoredRemaining !== null && anchoredRemaining > 0) {
			anchoredRemaining--;
		}

		const data = incrementVotes(user_id, polis_id, safeRequiredVotes);
		hasMetThreshold = data.hasMetThreshold;

		if (data.totalVotes === safeRequiredVotes) {
			setTimeout(() => {
				screen = 'continue-prompt';
				voteCooldown = false;
				waitingForNext = false;
			}, 600);
			return;
		}

		setTimeout(() => {
			voteCooldown = false;
		}, 800);
	}

	function resumeVoting() {
		resetVoteCount(user_id, polis_id);
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
			screen = 'voting';
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
		screen = 'add-opinion';
		opinionSubmitted = false;
		opinionError = false;
		returningToVoting = false;
	}

	function closeAddOpinion() {
		if (polis.state.remaining === 0) {
			screen = 'completed';
		} else {
			screen = 'voting';
		}
	}

	const remainingBeforeContinue = $derived(safeRequiredVotes - totalVotes);
	const progress = $derived(Math.min(100, Math.max(0, (totalVotes / safeRequiredVotes) * 100)));
</script>

<div
	class="bg-primary/5 relative left-1/2 flex w-screen -translate-x-1/2 flex-col items-center gap-8 overflow-visible py-4 md:py-0"
>
	{#if screen === 'voting'}
		<!-- Voting Screen -->
		<div
			class="flex w-full max-w-[808px] flex-col items-start gap-1 px-8 md:gap-6 md:px-24 md:py-12"
			in:fade={{ duration: 300 }}
		>
			<!-- Opinion counter -->
			{#if !polisReady}
				<div class="bg-foreground/10 h-5 w-32 animate-pulse rounded md:h-6"></div>
			{:else if !polisError && !poolExhausted && showRemainingStatementCount}
				<p class="text-muted-foreground tex-base font-semibold md:text-lg">
					{m.polis_opinion_counter({
						current: opinionPosition.current,
						total: opinionPosition.total
					})}
				</p>
				<div class="bg-secondary/30 relative h-1.5 w-full">
					<div
						class="bg-secondary absolute top-0 left-0 h-full transition-all duration-300"
						style="width: {progress}%"
					></div>
				</div>
			{/if}

			<!-- Statement text -->
			<div class="w-full pt-2 pb-6">
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
				{:else if !polisReady || waitingForNext || !polisCurrentStatement}
					<!-- Loading, between statements, or briefly empty before the screen
					     flips to "completed" — show a skeleton, never a blank card. -->
					<div in:fade={{ duration: 200 }} class="w-full animate-pulse">
						<div class="space-y-3">
							<div class="bg-foreground/10 h-8 w-full rounded"></div>
							<div class="bg-foreground/10 h-8 w-4/5 rounded"></div>
							<div class="bg-foreground/10 h-8 w-3/5 rounded"></div>
						</div>
					</div>
				{:else if polisCurrentStatement}
					{#if polisCurrentStatement.is_seed}
						<p class="text-seed-highlight mb-1 text-right text-xs font-medium">
							{m.polis_seed_statement()}
						</p>
					{/if}
					<div
						class="border-seed-highlight rounded-lg transition-colors {polisCurrentStatement.is_seed
							? 'bg-seed-highlight-bg border-seed-highlight border px-4 py-3'
							: ''}"
						in:fly={{ y: 20, duration: 500, easing: cubicOut }}
					>
						<p class="text-card-foreground text-xl leading-9 font-normal sm:text-3xl">
							{polisCurrentStatement.txt}
						</p>
					</div>
				{/if}
			</div>

			{#if !polisError && polisCurrentStatement}
				<!-- Vote buttons -->
				<div class="flex flex-wrap items-start gap-4 md:gap-6">
					<Button
						variant="default"
						size="lg"
						disabled={disabled || !polisReady}
						onclick={() => doVote('agree')}
						class="text-lg"
					>
						<ThumbsUp class="h-5 w-5" />
						{m.polis_agree()}
					</Button>
					<Button
						variant="default"
						size="lg"
						disabled={disabled || !polisReady}
						onclick={() => doVote('disagree')}
						class="gap-2 px-6 py-4 text-lg"
					>
						<ThumbsDown class="h-5 w-5" />
						{m.polis_disagree()}
					</Button>
					<Button
						variant="ghost"
						size="lg"
						class="text-lg"
						disabled={disabled || !polisReady}
						onclick={() => doVote('pass')}
					>
						{m.polis_pass_unsure()}
						<SkipForward class="h-5 w-5" />
					</Button>
				</div>

				<Separator orientation="horizontal" />

				<!-- Add your own opinion -->
				<p>{m.polis_dont_see_your_view()}</p>

				<Button
					variant="secondary"
					class="text-foreground hover:text-foreground flex items-center gap-2 p-5 text-xl font-bold transition-colors"
					disabled={!polisReady}
					onclick={openAddOpinion}
				>
					<MessageSquare fill="currentColor" class="h-5 w-5" />
					{m.polis_add_opinion()}
				</Button>
			{/if}

			<!-- Continue to next step (only after threshold) -->
			{#if canContinue}
				<div class="mt-4 w-full border-t pt-6" in:fade={{ duration: 300 }}>
					<LoadingButton
						variant="primaryDark"
						size="lg"
						loading={continuing}
						onclick={handleContinue}
						class="gap-2 px-6 py-4 text-lg"
					>
						{m.polis_continue_to_next_step()}
						{#if !continuing}<ChevronRight class="h-5 w-5" />{/if}
					</LoadingButton>
				</div>
			{/if}
		</div>
	{:else if screen === 'add-opinion'}
		<!-- Add Opinion Screen -->
		<div
			class="flex w-full max-w-[808px] flex-col items-start gap-6 px-8 py-8 md:px-24 md:py-12"
			in:fade={{ duration: 300 }}
		>
			<div class="flex w-full items-center justify-between">
				<div class="flex items-center gap-4">
					<MessageSquare fill="currentColor" class="text-card-foreground h-8 w-8" />
					<h2 class="text-card-foreground text-3xl font-semibold">
						{m.polis_add_your_own_opinion()}
					</h2>
				</div>
				<Button
					variant="link"
					class="text-foreground/80 hover:text-foreground/60 text-xl transition-colors"
					onclick={closeAddOpinion}
					aria-label={m.polis_close()}
				>
					<X class="h-5 w-5" />
				</Button>
			</div>

			<div class="text-card-foreground flex flex-col px-4 text-base">
				<ul class="list-inside list-disc space-y-2">
					<li>{m.polis_tip_agreeable()}</li>
					<li>{m.polis_tip_one_idea()}</li>
					<li>{m.polis_tip_no_jargon()}</li>
					<li>{m.polis_tip_many_statements()}</li>
					<li>{m.polis_tip_come_back()}</li>
				</ul>
			</div>

			{#if opinionSubmitted}
				<div
					class="bg-primary/10 text-primary w-full rounded-lg p-4 text-center font-medium"
				>
					{m.polis_opinion_submitted()}
				</div>
			{:else if opinionError}
				<div
					class="bg-destructive/10 text-destructive w-full rounded-lg p-4 text-center font-medium"
				>
					{m.something_went_wrong()}
				</div>
			{/if}

			<div class="w-full pb-6">
				<textarea
					bind:value={opinionText}
					oninput={() => (opinionError = false)}
					placeholder={m.polis_opinion_placeholder()}
					class="bg-background text-foreground placeholder:text-muted-foreground border-input focus:ring-primary/30 h-28 w-full resize-none rounded-lg border p-4 text-base shadow-sm outline-none focus:ring-2"
				></textarea>
			</div>

			<div class="flex flex-wrap items-start gap-6">
				<LoadingButton
					variant="default"
					size="lg"
					loading={submitBusy}
					disabled={!opinionText.trim()}
					onclick={handleSubmitOpinion}
					class="gap-2 px-6 py-4 text-lg"
				>
					{m.submit()}
				</LoadingButton>
				<LoadingButton
					variant="ghost"
					size="lg"
					class="text-lg"
					loading={submitBusy}
					disabled={!opinionText.trim()}
					onclick={handleSubmitAndAddAnother}
				>
					{m.polis_submit_and_add_another()}
					{#if !submitBusy}<ChevronRight class="h-5 w-5" />{/if}
				</LoadingButton>
			</div>

			<button
				class="text-muted-foreground hover:text-foreground mt-2 text-base font-medium transition-colors"
				onclick={closeAddOpinion}
			>
				&larr; {m.polis_back_to_voting()}
			</button>
		</div>
	{:else if screen === 'continue-prompt'}
		<!-- Do you want to continue? -->
		<div
			class="flex w-full max-w-[808px] flex-col items-start gap-6 px-8 py-8 md:px-24 md:py-12"
			in:fade={{ duration: 300 }}
		>
			<div class="flex items-center gap-4">
				<PenLine class="text-card-foreground h-8 w-8" />
				<h2 class="text-card-foreground text-3xl font-semibold">
					{m.polis_do_you_want_to_continue()}
				</h2>
			</div>

			<div class="flex flex-wrap items-start gap-6">
				<Button
					variant="default"
					size="lg"
					onclick={resumeVoting}
					class="w-72 gap-2 px-6 py-4 text-lg"
				>
					{m.polis_continue_voting()}
				</Button>
				<LoadingButton
					variant="ghost"
					size="lg"
					loading={continuing}
					class="text-muted-foreground hover:text-foreground flex items-center gap-2 px-6 py-4 text-lg font-medium transition-colors"
					onclick={handleContinue}
				>
					{m.polis_continue_to_next_step()}
					{#if !continuing}<ChevronRight class="h-5 w-5" />{/if}
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
				<MessageSquare fill="currentColor" class="h-5 w-5" />
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
