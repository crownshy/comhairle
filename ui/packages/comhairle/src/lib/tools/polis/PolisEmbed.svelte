<script lang="ts">
	import { Button, LoadingButton, buttonVariants } from '$lib/components/ui/button';
	import * as Drawer from '$lib/components/ui/drawer';
	import { fly, fade } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import {
		ThumbsUp,
		ThumbsDown,
		CheckCircle2,
		X,
		ChevronRight,
		MessageSquare,
		MessageSquarePlus,
		AlertTriangle,
		Lightbulb,
		Languages
	} from 'lucide-svelte';
	import { onMount } from 'svelte';
	import PolisApi, { type PolisApiState, type PolisStatement } from './PolisApi';
	import {
		getVoteData,
		incrementVotes,
		reconcileServerVotes,
		startNextVotingRound
	} from './polisVoteStore';
	import { m } from '$lib/paraglide/messages';
	import Separator from '$lib/components/ui/separator/separator.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import StatementSourceLabel from './StatementSourceLabel.svelte';
	import ModerationPolicySheet from './ModerationPolicySheet.svelte';
	import { statementSourceOf } from './statementSource';
	import { cn } from '$lib/utils';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { type LocalizedStatement } from '@crownshy/api-client/api';
	import { getLocale } from '$lib/paraglide/runtime';
	import type { OnSequenceChange } from '$lib/tools/toolSequence';
	import type { PolisReportData } from './reportTypes';
	import {
		FREE_VOTES,
		earlySnapshot,
		groupPortrait,
		isRevealDue,
		pickPractice,
		stageAfterSeedResult,
		firstVotingStage,
		toPracticeStatements,
		type PracticePick,
		type PracticeStatement,
		type Stage,
		type Vote
	} from './onboarding/onboarding';
	import { hasSeenOnboarding, markOnboardingSeen } from './onboarding/seen';
	import GuideScreen from './onboarding/GuideScreen.svelte';
	import GroupsReveal from './onboarding/GroupsReveal.svelte';
	import CrowdStatement from './onboarding/CrowdStatement.svelte';

	type Props = {
		polis_id: string;
		polis_url: string;
		user_id: string;
		requiredVotes?: number;
		workflowStepId?: string;
		isPreview?: boolean;
		onCanContinueChange?: (canContinue: boolean) => void;
		/** Completes the step from the threshold prompt's I'm done voting. */
		onDone?: () => void;
		onSequenceChange?: OnSequenceChange;
		topic?: string | null;
		strictModeration?: boolean | null;
	};

	let {
		polis_id,
		polis_url,
		user_id,
		requiredVotes = 10,
		workflowStepId = polis_id,
		isPreview = false,
		onCanContinueChange,
		onDone,
		onSequenceChange,
		topic,
		strictModeration
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
	let waitingForNext = $state(false);

	// First-timer onboarding (ADR-0062): one instruction or one vote on screen at a time.
	const onboardingScope = `${user_id}-${voteScopeKey}`;
	const initialData = getVoteData(user_id, voteScopeKey);
	let stage = $state<Stage>(
		initialData.totalVotes > 0 || hasSeenOnboarding(onboardingScope) ? 'done' : 'deciding'
	);
	let pick = $state<PracticePick>({ seed: undefined, participant: undefined });
	let seedVote = $state<Vote | undefined>(undefined);
	let practisedAny = false;
	// Someone who started with only organisers' statements gets the "someone like you" line on
	// the first participant statement Polis serves them instead.
	let firstHere = $state(false);
	let someoneElseSeen = $state(false);
	let freeVotes = $state(0);
	let votesSinceOnboarding = 0;
	let groupsAfterOpinion = false;
	// After onboarding, the groups screen comes back every few votes.
	let groupsShown = $state(false);
	let report = $state<PolisReportData | undefined>(undefined);
	let reportLoading = $state(false);

	const onboarding = $derived(stage !== 'done');

	const practiceStatement = $derived<PracticeStatement | undefined>(
		stage === 'seed' ? pick.seed : stage === 'participant' ? pick.participant : undefined
	);

	// During practice the card shows the picked statement in place of the one Polis served.
	const shownStatement = $derived<PolisStatement | undefined>(
		practiceStatement
			? {
					txt: practiceStatement.text,
					tid: practiceStatement.tid,
					is_seed: practiceStatement.is_seed,
					is_meta: false,
					is_seen: false,
					lang: undefined,
					remaining: polisRemaining,
					total: polisTotal
				}
			: polisCurrentStatement
	);

	// The current statement resolved into the participant's active locale. When a
	// stored translation exists it carries the translated `text` plus the original
	// (`original_text`/`source_locale`) so we can show a "translated" affordance.
	let localized = $state<LocalizedStatement | undefined>(undefined);
	let showOriginal = $state(false);
	let localizedReqToken = 0;
	let lastLocalizedKey = '';

	// Text to render: the translation when one is available, else the raw Polis text.
	const displayText = $derived(localized?.text ?? shownStatement?.txt ?? '');
	const isTranslated = $derived(localized?.is_translation ?? false);

	// Resolve a locale code (e.g. "gd") to a language name in the viewer's locale,
	// falling back to the raw code if the platform can't name it.
	function languageName(locale: string | null | undefined): string {
		if (!locale) return '';
		try {
			return new Intl.DisplayNames([getLocale()], { type: 'language' }).of(locale) ?? locale;
		} catch {
			return locale;
		}
	}

	// Whenever the statement (or viewer locale) changes, fetch its localized form.
	// A monotonic token drops stale responses so a slow lookup can't overwrite a
	// newer statement. A missing aux row (e.g. just-submitted) falls back to raw text.
	$effect(() => {
		const tid = shownStatement?.tid;
		const locale = getLocale();
		// The shown statement is reassigned on every loading toggle; only act
		// when the statement or viewer locale actually changed.
		const key = `${tid ?? ''}:${locale}`;
		if (key === lastLocalizedKey) return;
		lastLocalizedKey = key;
		showOriginal = false;
		localized = undefined;
		if (tid === undefined) return;
		const token = ++localizedReqToken;
		apiClient
			.PolisGetLocalizedStatement({
				queries: { polis_conversation_id: polis_id, polis_statement_id: tid, locale }
			})
			.then((res) => {
				if (token === localizedReqToken) localized = res;
			})
			.catch((err) => {
				if (token === localizedReqToken) localized = undefined;
				console.debug('[PolisEmbed] localized statement unavailable:', err);
			});
	});

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

	let totalVotes = $state(initialData.totalVotes);
	let hasMetThreshold = $state(initialData.hasMetThreshold);
	let screen = $state<Screen>('voting');

	// Seed progress from the server so the required-votes threshold is correct
	// across devices, not just this browser. Best-effort: on failure we keep the
	// local count seeded above.
	onMount(async () => {
		const result = await tryCatchAsync(() =>
			apiClient.PolisGetUserVoteCount({ queries: { workflow_step_id: stepId } })
		);
		if (result.err !== null) {
			console.error('[PolisEmbed] Failed to load server vote count:', result.err);
		} else {
			const data = reconcileServerVotes(
				user_id,
				voteScopeKey,
				result.ok.vote_count,
				safeRequiredVotes
			);
			totalVotes = data.totalVotes;
			hasMetThreshold = data.hasMetThreshold;
		}
		if (stage === 'deciding') await startOnboarding(totalVotes);
	});

	async function loadReport() {
		reportLoading = true;
		const result = await tryCatchAsync(() =>
			apiClient.PolisGetReportData({ queries: { workflow_step_id: stepId } })
		);
		// No math run yet, or the poll is gone; the groups screen then says there are no groups.
		report = result.err === null ? result.ok : undefined;
		reportLoading = false;
	}

	async function startOnboarding(votesSoFar: number) {
		if (votesSoFar > 0) {
			finishOnboarding();
			return;
		}
		// Polis's own list rather than the report, which leaves out statements its math has
		// not reached yet. Without either, onboarding runs on whatever Polis serves.
		const result = await tryCatchAsync(() => polis.fetchStatements());
		const statements =
			result.err === null ? toPracticeStatements(result.ok, strictModeration ?? false) : [];
		pick = pickPractice(statements);
		firstHere = !pick.participant;
		stage = 'intro';
	}

	function startFreeVoting() {
		// Polis served its first statement before the practice votes; it may be one of them.
		if (practisedAny) polis.fetchNextStatement();
		stage = 'free';
	}

	function goTo(next: Stage) {
		if (next === 'free') startFreeVoting();
		else stage = next;
	}

	function finishOnboarding() {
		stage = 'done';
		groupsShown = false;
		votesSinceOnboarding = 0;
		markOnboardingSeen(onboardingScope);
	}

	function skipOnboarding() {
		if (practisedAny && stage !== 'free') polis.fetchNextStatement();
		finishOnboarding();
		someoneElseSeen = true;
		screen = 'voting';
	}

	function showOnboardingGroups() {
		stage = 'groups';
		loadReport();
	}

	function closeGroups() {
		if (stage === 'groups') finishOnboarding();
		else groupsShown = false;
	}

	async function revealGroupsIfAny() {
		await loadReport();
		if (portrait) groupsShown = true;
	}

	const introTitle = $derived(
		topic ? m.polis_onboarding_intro_topic({ topic }) : m.polis_onboarding_intro()
	);

	const showSomeoneElse = $derived(
		firstHere &&
			!someoneElseSeen &&
			(stage === 'free' || stage === 'done') &&
			!waitingForNext &&
			shownStatement !== undefined &&
			!shownStatement.is_seed
	);

	/** The one line above the statement while onboarding, saying what to do with it. */
	const votingGuide = $derived.by(() => {
		if (stage === 'seed') return m.polis_onboarding_try_seed();
		if (stage === 'participant' || showSomeoneElse) return m.polis_onboarding_try_participant();
		if (stage === 'free')
			return m.polis_onboarding_keep_going({ count: FREE_VOTES - freeVotes });
		return undefined;
	});

	/** An onboarding screen that replaces the voting UI, if one is due. */
	const guideStage = $derived.by(() => {
		if (stage === 'intro' || stage === 'seed-result' || stage === 'missing') return stage;
		if (stage === 'groups' || groupsShown) return 'groups';
		return undefined;
	});

	const portrait = $derived(report ? groupPortrait(report, polisPid) : undefined);
	const snapshot = $derived(earlySnapshot(report));

	const missingText = $derived(
		`${strictModeration ? m.polis_onboarding_missing_body_moderated() : m.polis_onboarding_missing_body()} ${m.polis_onboarding_missing_anytime()}`
	);

	let voteCooldown = $state(false);
	let opinionText = $state('');
	let opinionSubmitted = $state(false);
	let opinionSubmitting = $state(false);
	let opinionError = $state(false);
	let returningToVoting = $state(false);
	const submitBusy = $derived(opinionSubmitting || returningToVoting);
	const MAX_STATEMENT_LENGTH = 200;
	const charactersLeft = $derived(MAX_STATEMENT_LENGTH - opinionText.length);
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

	const disabled = $derived(voteCooldown || waitingForNext || stage === 'deciding');

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

	const opinionsLeft = $derived(Math.max(0, anchoredRemaining ?? polisRemaining));

	const poolExhausted = $derived(
		polisReady && !polisLoading && !polisError && !polisCurrentStatement
	);

	// The pager is the only way out (ADR-0047), so an exhausted pool must unlock it too.
	const canContinue = $derived(hasMetThreshold || poolExhausted);

	$effect(() => {
		onCanContinueChange?.(canContinue);
	});

	$effect(() => {
		if (screen === 'voting' && poolExhausted && !practiceStatement && !guideStage) {
			screen = 'completed';
		}
	});

	/** The vote Polis failed to save, kept so Try again can resend it. */
	let failedVote = $state<Vote | undefined>(undefined);

	/** Counts a vote towards progress. True when it is the one that meets the threshold. */
	function countVote(): boolean {
		totalVotes++;
		if (anchoredRemaining !== null && anchoredRemaining > 0) {
			anchoredRemaining--;
		}
		const data = incrementVotes(user_id, voteScopeKey, safeRequiredVotes);
		hasMetThreshold = data.hasMetThreshold;
		return data.totalVotes - data.votesAtLastPrompt === safeRequiredVotes;
	}

	function afterLiveVote() {
		if (stage === 'free') {
			freeVotes++;
			if (freeVotes >= FREE_VOTES) stage = 'missing';
		} else if (stage === 'done') {
			votesSinceOnboarding++;
			if (isRevealDue(votesSinceOnboarding)) revealGroupsIfAny();
		}
	}

	async function practiceVote(statement: PracticeStatement, vote: Vote) {
		voteCooldown = true;
		const ok = await polis.voteOn(statement.tid, vote);
		voteCooldown = false;
		if (!ok) {
			skipOnboarding();
			return;
		}
		practisedAny = true;
		countVote();
		if (stage === 'seed') {
			seedVote = vote;
			stage = 'seed-result';
		} else {
			startFreeVoting();
		}
	}

	async function doVote(type: Vote) {
		if (voteCooldown || !shownStatement) return;
		if (practiceStatement) {
			practiceVote(practiceStatement, type);
			return;
		}
		if (showSomeoneElse) someoneElseSeen = true;
		waitingForNext = true;
		voteCooldown = true;
		failedVote = undefined;

		// Count the vote only once Polis has saved it, so Next never unlocks on lost votes.
		const saved = await polis.submitVote(type);
		if (!saved) {
			failedVote = type;
			waitingForNext = false;
			voteCooldown = false;
			return;
		}
		const metThreshold = countVote();
		afterLiveVote();

		// Flip now: a delay shows the between-statements skeleton first, then swaps it out.
		if (metThreshold) {
			screen = 'continue-prompt';
			voteCooldown = false;
			waitingForNext = false;
			return;
		}

		setTimeout(() => {
			voteCooldown = false;
		}, 800);
	}

	/** Back from writing a statement: on to the groups if the missing screen sent them there. */
	function returnToVoting() {
		if (groupsAfterOpinion) {
			groupsAfterOpinion = false;
			showOnboardingGroups();
		}
		if (polis.state.remaining === 0 && !guideStage) {
			screen = 'completed';
		} else {
			screen = 'voting';
		}
	}

	function resumeVoting() {
		startNextVotingRound(user_id, voteScopeKey);
		screen = 'voting';
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
			returnToVoting();
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
		if (stage === 'missing') groupsAfterOpinion = true;
		visibleStatementWhenOpened = shownStatement;
		screen = 'add-opinion';
		opinionSubmitted = false;
		opinionError = false;
		returningToVoting = false;
	}

	// Polis reports to the chrome's bar instead of drawing its own (ADR-0047).
	const progress = $derived(
		canContinue ? 1 : Math.min(1, Math.max(0, totalVotes / safeRequiredVotes))
	);

	const votesRemaining = $derived(Math.max(safeRequiredVotes - totalVotes, 1));
	const blockedReason = $derived.by(() => {
		if (canContinue) return undefined;
		if (votesRemaining === 1) return m.polis_vote_one_more_to_continue();
		return m.polis_vote_more_to_continue({ count: votesRemaining });
	});

	$effect(() => {
		onSequenceChange?.({ progress, blockedReason });
	});
</script>

{#snippet opinionTips()}
	<ul class="list-outside list-disc space-y-2 ps-5">
		<li>{m.polis_tip_agreeable()}</li>
		<li>{m.polis_tip_one_idea()}</li>
		<li>{m.polis_tip_max_length({ max: MAX_STATEMENT_LENGTH })}</li>
		<li>{m.polis_tip_no_jargon()}</li>
		<li>{m.polis_tip_many_statements()}</li>
		<li>{m.polis_tip_come_back()}</li>
	</ul>
{/snippet}

{#snippet skipIntro()}
	<Button
		variant="link"
		class="text-muted-foreground hover:text-foreground h-auto px-0 text-base"
		onclick={skipOnboarding}
	>
		{m.polis_onboarding_skip()}
	</Button>
{/snippet}

<div class="flex w-full flex-col items-center gap-8 py-4 md:py-0">
	{#if screen === 'voting' && guideStage}
		<div
			class="flex w-full max-w-[808px] flex-col items-start gap-6 px-4 py-8 sm:px-8 md:px-24 md:py-12"
		>
			{#if guideStage === 'intro'}
				<GuideScreen
					title={introTitle}
					next={m.polis_onboarding_next()}
					onNext={() => goTo(firstVotingStage(pick))}
				>
					<p class="text-card-foreground text-lg">{m.polis_onboarding_intro_body()}</p>
				</GuideScreen>
			{:else if guideStage === 'seed-result' && pick.seed && seedVote}
				<GuideScreen
					title={m.polis_onboarding_seed_result()}
					next={m.polis_onboarding_next()}
					onNext={() => goTo(stageAfterSeedResult(pick))}
				>
					<CrowdStatement statement={pick.seed} ownVote={seedVote} />
					<p class="text-card-foreground text-lg">
						{m.polis_onboarding_seed_result_changes()}
					</p>
				</GuideScreen>
			{:else if guideStage === 'missing'}
				<GuideScreen
					title={m.polis_onboarding_missing_title()}
					next={m.polis_onboarding_keep_voting()}
					onNext={showOnboardingGroups}
				>
					<p class="text-card-foreground text-lg">{missingText}</p>
					{#snippet actions()}
						<Button
							variant="ghost"
							size="lg"
							class="h-12 gap-2 px-6 text-lg"
							onclick={openAddOpinion}
						>
							<MessageSquarePlus class="size-5" />
							{m.polis_add_your_own_opinion()}
						</Button>
					{/snippet}
				</GuideScreen>
			{:else if reportLoading}
				<Skeleton class="h-9 w-3/4 rounded-md" />
				<Skeleton class="h-24 w-full rounded-xl" />
			{:else}
				<GuideScreen
					title={portrait
						? m.polis_onboarding_groups_title()
						: m.polis_onboarding_no_groups_title()}
					next={m.polis_onboarding_keep_voting()}
					onNext={closeGroups}
				>
					<GroupsReveal {portrait} {snapshot} />
				</GuideScreen>
			{/if}
			{#if onboarding}
				{@render skipIntro()}
			{/if}
		</div>
	{:else if screen === 'voting'}
		<!-- Voting Screen -->
		<div
			class="flex w-full max-w-[808px] flex-col items-start gap-1 px-4 sm:px-8 md:gap-6 md:px-24 md:py-12"
			in:fade={{ duration: 300 }}
		>
			{#if votingGuide && !polisError}
				{#key votingGuide}
					<p
						class="text-card-foreground text-xl leading-snug font-medium sm:text-2xl"
						in:fade={{ duration: 300 }}
					>
						{votingGuide}
					</p>
				{/key}
			{/if}

			<div class="flex w-full flex-col gap-1 md:gap-6">
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
							<p class="text-muted-foreground text-base">
								{m.polis_error_description()}
							</p>
						</div>
					{:else if !polisReady || waitingForNext || polisLoading || !shownStatement || stage === 'deciding'}
						<!-- Loading, between statements, or briefly empty before the screen
					     flips to "completed". Rows match a one-line source label, the statement
					     box and its line heights so nothing below moves when the statement arrives. -->
						<div in:fade={{ duration: 200 }} class="w-full">
							<div class="border-foreground/10 rounded-2xl border p-3 sm:p-6">
								<Skeleton
									class="mb-3 h-5 w-64 max-w-full rounded-md sm:mb-4 sm:h-6"
								/>
								<div class="flex h-7 items-center sm:h-9">
									<Skeleton class="h-6 w-full rounded sm:h-7" />
								</div>
								<div class="flex h-7 items-center sm:h-9">
									<Skeleton class="h-6 w-3/5 rounded sm:h-7" />
								</div>
							</div>
						</div>
					{:else}
						{@const source = statementSourceOf(shownStatement.is_seed)}
						<!-- Keyed so back-to-back practice statements animate in like served ones. -->
						{#key shownStatement.tid}
							<div
								class={cn(
									'rounded-2xl border p-3 transition-colors sm:p-6',
									source.cardClass
								)}
								in:fly={{ y: 20, duration: 500, easing: cubicOut }}
							>
								<StatementSourceLabel {source} />
								<p
									class="text-card-foreground mt-3 text-xl leading-snug font-normal sm:mt-4 sm:text-3xl sm:leading-9"
								>
									{displayText}
								</p>
								{#if isTranslated}
									<button
										type="button"
										onclick={() => (showOriginal = !showOriginal)}
										class="text-muted-foreground hover:text-foreground mt-3 inline-flex items-center gap-1.5 text-sm font-medium transition-colors"
										aria-expanded={showOriginal}
									>
										<Languages class="h-4 w-4" />
										{localized?.source_locale
											? m.polis_translated_from({
													language: languageName(localized.source_locale)
												})
											: m.polis_translated_label()}
									</button>
									{#if showOriginal}
										<div
											class="border-border bg-muted/40 text-card-foreground mt-2 rounded-lg border p-4 text-left"
											in:fade={{ duration: 150 }}
										>
											<p
												class="text-muted-foreground mb-1 text-xs font-semibold tracking-wide uppercase"
											>
												{m.polis_original_statement()}
											</p>
											<p class="text-base leading-7">
												{localized?.original_text}
											</p>
											<p class="text-muted-foreground mt-2 text-xs">
												{m.polis_ai_translation_note()}
											</p>
										</div>
									{/if}
								{/if}
							</div>
						{/key}
					{/if}
				</div>

				{#if failedVote && !waitingForNext}
					<div
						class="border-destructive/20 bg-destructive/5 flex w-full flex-wrap items-center justify-between gap-3 rounded-lg border px-4 py-3"
						role="alert"
					>
						<p class="text-foreground text-base">{m.polis_vote_not_saved()}</p>
						<Button variant="outline" onclick={() => failedVote && doVote(failedVote)}>
							{m.try_again()}
						</Button>
					</div>
				{/if}

				<!-- Rendered (disabled) while loading so the layout doesn't shift once Polis is ready. -->
				{#if !polisError && (!polisReady || shownStatement)}
					{@const VOTE_BUTTON_CLASS =
						'h-12 flex-1 px-6 text-lg has-[>svg]:px-6 sm:flex-none'}
					<!-- Vote buttons -->
					<div class="flex w-full flex-wrap items-center gap-3 md:gap-5">
						<Button
							size="lg"
							disabled={disabled || !polisReady}
							onclick={() => doVote('agree')}
							class={VOTE_BUTTON_CLASS}
						>
							<ThumbsUp class="size-6" />
							{m.polis_agree()}
						</Button>
						<Button
							size="lg"
							disabled={disabled || !polisReady}
							onclick={() => doVote('disagree')}
							class={VOTE_BUTTON_CLASS}
						>
							<ThumbsDown class="size-6" />
							{m.polis_disagree()}
						</Button>
						<Button
							variant="ghost"
							size="lg"
							class="text-foreground/80 hover:text-foreground h-12 w-full px-0 text-lg hover:bg-transparent has-[>svg]:px-0 sm:w-auto sm:justify-start"
							disabled={disabled || !polisReady}
							onclick={() => doVote('pass')}
						>
							{m.polis_neutral()}
						</Button>
					</div>
				{/if}
			</div>

			{#if onboarding}
				{@render skipIntro()}
			{:else if !polisError && (!polisReady || shownStatement)}
				<Button
					variant="ghost"
					class="text-muted-foreground hover:text-foreground mt-3 h-auto w-full px-0 py-1 text-center text-lg font-normal whitespace-normal hover:bg-transparent has-[>svg]:px-0 sm:mt-0 sm:w-auto sm:justify-start sm:text-start"
					disabled={!polisReady}
					onclick={openAddOpinion}
				>
					<MessageSquarePlus class="size-6" />
					{m.polis_add_your_own_opinion()}
				</Button>
			{/if}
		</div>
	{:else if screen === 'add-opinion'}
		<!-- Add Opinion Screen -->
		<div
			class="flex w-full max-w-[808px] flex-col items-start gap-6 px-4 py-8 sm:px-8 md:px-24 md:py-12"
			in:fade={{ duration: 300 }}
		>
			<div class="flex w-full items-start justify-between gap-2 sm:items-center">
				<div class="flex items-center gap-3 sm:gap-4">
					<MessageSquare
						fill="currentColor"
						class="text-card-foreground size-6 shrink-0 sm:size-8"
					/>
					<h2 class="text-card-foreground text-xl font-semibold sm:text-3xl">
						{m.polis_add_your_own_opinion()}
					</h2>
				</div>
				<Button
					variant="link"
					class="text-foreground/80 hover:text-foreground/60 shrink-0 text-xl transition-colors"
					onclick={returnToVoting}
					aria-label={m.polis_close()}
				>
					<X class="h-5 w-5" />
				</Button>
			</div>

			<!-- Phones open the tips in a sheet so the text box stays above the fold. -->
			<div class="text-card-foreground hidden text-base sm:block sm:px-4">
				{@render opinionTips()}
			</div>
			<Drawer.Root>
				<Drawer.Trigger
					class="text-primary inline-flex items-center gap-2 text-base font-medium underline-offset-4 hover:underline sm:hidden"
				>
					<Lightbulb class="size-5" aria-hidden="true" />
					{m.polis_tips_for_your_opinion()}
				</Drawer.Trigger>
				<Drawer.Content>
					<div class="flex flex-col gap-4 overflow-y-auto px-6 pt-4 pb-8">
						<Drawer.Title class="text-card-foreground text-xl font-semibold">
							{m.polis_tips_for_your_opinion()}
						</Drawer.Title>
						<div class="text-card-foreground text-base">
							{@render opinionTips()}
						</div>
						<Drawer.Close
							class={cn(buttonVariants({ size: 'lg' }), 'mt-2 w-full text-lg')}
						>
							{m.polis_close()}
						</Drawer.Close>
					</div>
				</Drawer.Content>
			</Drawer.Root>
			<ModerationPolicySheet workflowStepId={stepId} {isPreview} />

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
					{m.polis_opinion_not_submitted()}
				</div>
			{/if}

			<div class="w-full sm:pb-6">
				<textarea
					bind:value={opinionText}
					oninput={() => (opinionError = false)}
					placeholder={m.polis_opinion_placeholder()}
					maxlength={MAX_STATEMENT_LENGTH}
					aria-describedby="polis-opinion-characters-left"
					class="bg-background text-foreground placeholder:text-muted-foreground border-input focus:ring-primary/30 h-28 w-full resize-none rounded-lg border p-4 text-base shadow-sm outline-none focus:ring-2"
				></textarea>
				<p
					id="polis-opinion-characters-left"
					class="mt-2 text-end text-base {charactersLeft === 0
						? 'text-destructive'
						: 'text-muted-foreground'}"
				>
					{m.polis_characters_left({ count: charactersLeft })}
				</p>
			</div>

			<div
				class="flex w-full flex-col items-center gap-4 sm:w-auto sm:flex-row sm:flex-wrap sm:items-start sm:gap-6"
			>
				<LoadingButton
					variant="default"
					size="lg"
					loading={submitBusy}
					disabled={!opinionText.trim()}
					onclick={handleSubmitOpinion}
					class="w-full gap-2 px-6 py-4 text-lg sm:w-auto"
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
					{#if !submitBusy}<ChevronRight class="h-5 w-5 rtl:-scale-x-100" />{/if}
				</LoadingButton>
			</div>

			<button
				class="text-muted-foreground hover:text-foreground self-center text-base font-medium transition-colors sm:mt-2 sm:self-start"
				onclick={returnToVoting}
			>
				&larr; {m.polis_back_to_voting()}
			</button>
		</div>
	{:else if screen === 'continue-prompt'}
		<!-- The threshold is met, so this is a fork and not a gate. Both ways on get the same weight,
		     so finishing doesn't read as giving up. -->
		<div
			class="flex w-full max-w-[808px] flex-1 flex-col items-center justify-center gap-8 px-6 py-8 text-center md:px-16"
			in:fade={{ duration: 300 }}
		>
			<div class="flex flex-col items-center gap-3">
				<CheckCircle2 class="text-primary size-8" />
				<h2 class="text-card-foreground text-2xl leading-tight font-semibold">
					{totalVotes === 1
						? m.polis_votes_counted_one({ count: totalVotes })
						: m.polis_votes_counted({ count: totalVotes })}
				</h2>
				<p class="text-muted-foreground max-w-[44ch] text-base">
					{opinionsLeft > 0
						? m.polis_continue_prompt_body()
						: m.polis_nothing_left_to_vote_on()}
				</p>
			</div>

			<div class="flex w-full max-w-[360px] flex-col items-stretch gap-3">
				{#if opinionsLeft > 0}
					<Button
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
				{#if onDone}
					<Button
						variant={opinionsLeft > 0 ? 'outline' : 'default'}
						onclick={onDone}
						class="h-auto w-full flex-col gap-0.5 rounded-2xl px-6 py-3.5 whitespace-normal"
					>
						<span class="text-lg font-semibold">{m.polis_finish_voting()}</span>
						<span class="text-base font-normal opacity-80"
							>{m.polis_finish_voting_hint()}</span
						>
					</Button>
				{/if}
			</div>
		</div>
	{:else if screen === 'completed'}
		<!-- Voted everything -->
		<div
			class="flex w-full max-w-[808px] flex-col items-center gap-6 px-4 py-8 text-center sm:items-start sm:px-8 sm:text-start md:px-24 md:py-12"
			in:fade={{ duration: 300 }}
		>
			<p class="text-card-foreground text-3xl font-normal">
				{totalVotes === 0 ? m.polis_no_opinions_yet() : m.polis_voted_everything()}
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
	{/if}
</div>
