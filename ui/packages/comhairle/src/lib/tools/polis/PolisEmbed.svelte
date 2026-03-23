<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { fly, fade } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { ThumbsUp, ThumbsDown, SkipForward, PenLine, X, ChevronRight } from 'lucide-svelte';
	import PolisApi, { type PolisApiState, type PolisStatement } from './PolisApi';
	import { getVoteData, incrementVotes, savePid, getSavedPid } from './polisVoteStore';

	type Props = {
		polis_id: string;
		polis_url: string;
		user_id: string;
		onDone: () => void;
		requiredVotes?: number;
		workflowStepId?: string;
	};

	let {
		polis_id,
		polis_url,
		user_id,
		onDone,
		requiredVotes = 5,
		workflowStepId = polis_id
	}: Props = $props();

	const stepId = workflowStepId;

	let polisCurrentStatement = $state<PolisStatement | undefined>(undefined);
	let polisLoading = $state(false);
	let polisReady = $state(false);
	let polisRemaining = $state(0);
	let polisTotal = $state(0);
	let polisPid = $state<number | undefined>(getSavedPid(stepId));

	function handlePolisChange(s: PolisApiState) {
		polisCurrentStatement = s.currentStatement;
		polisLoading = s.loading;
		polisReady = s.ready;
		polisRemaining = s.remaining;
		polisTotal = s.total;

		if (s.pid !== undefined && s.pid !== polisPid) {
			polisPid = s.pid;
			savePid(stepId, s.pid);
		}

		const newTxt = s.currentStatement?.txt ?? '';
		if (newTxt !== previousText && !s.loading) {
			previousText = newTxt;
			waitingForNext = false;
		}

		if (screen === 'voting' && s.ready && !s.loading && !s.currentStatement) {
			screen = 'completed';
		}
	}

	const polis = new PolisApi(
		user_id,
		polis_id,
		handlePolisChange,
		'en',
		polis_url,
		getSavedPid(stepId)
	);

	type Screen = 'voting' | 'add-opinion' | 'continue-prompt' | 'completed';

	const initialData = getVoteData(stepId);
	let totalVotes = $state(initialData.totalVotes);
	let hasMetThreshold = $state(initialData.hasMetThreshold);
	let screen = $state<Screen>('voting');
	let waitingForNext = $state(false);
	let voteCooldown = $state(false);
	let opinionText = $state('');
	let opinionSubmitted = $state(false);
	let previousText = '';

	const disabled = $derived(voteCooldown || waitingForNext);
	const canContinue = $derived(hasMetThreshold);

	let anchoredRemaining = $state<number | null>(null);
	let anchoredTotal = $state<number | null>(null);

	$effect(() => {
		if (polisReady && !polisLoading && anchoredRemaining === null) {
			anchoredRemaining = polisRemaining;
			anchoredTotal = polisTotal;
		}
	});

	const displayedRemaining = $derived(Math.max(0, anchoredRemaining ?? polisRemaining));
	const displayedTotal = $derived(anchoredTotal ?? polisTotal);
	const currentOpinionNumber = $derived(
		displayedTotal > 0 ? displayedTotal - displayedRemaining : 0
	);

	function doVote(type: 'agree' | 'disagree' | 'pass') {
		if (voteCooldown) return;
		waitingForNext = true;
		voteCooldown = true;

		polis.submitVote(type);
		totalVotes++;

		if (anchoredRemaining !== null && anchoredRemaining > 0) {
			anchoredRemaining--;
		}

		const data = incrementVotes(stepId, requiredVotes);
		hasMetThreshold = data.hasMetThreshold;

		if (data.totalVotes === requiredVotes) {
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
		screen = 'voting';
	}

	function handleSubmitOpinion() {
		if (!opinionText.trim()) return;
		polis.submitStatement(opinionText.trim());
		opinionText = '';
		opinionSubmitted = true;
	}

	function handleSubmitAndAddAnother() {
		if (!opinionText.trim()) return;
		polis.submitStatement(opinionText.trim());
		opinionText = '';
		opinionSubmitted = false;
	}

	function openAddOpinion() {
		screen = 'add-opinion';
		opinionSubmitted = false;
	}

	function closeAddOpinion() {
		screen = 'voting';
	}
</script>

<div class="bg-primary/5 mx-auto flex w-full max-w-4xl flex-col items-center gap-8 py-12">
	{#if screen === 'voting'}
		<!-- Voting Screen -->
		<div
			class="flex w-full max-w-[808px] flex-col items-start gap-6 px-8 py-8 md:px-24 md:py-12"
			in:fade={{ duration: 300 }}
		>
			<!-- Opinion counter -->
			<p class="text-muted-foreground text-xl font-semibold">
				Opinion {currentOpinionNumber + 1} of {displayedTotal}
			</p>

			<!-- Statement text -->
			<div class="w-full pt-2 pb-6">
				{#if waitingForNext}
					<div in:fade={{ duration: 200 }} class="w-full animate-pulse">
						<div class="space-y-3">
							<div class="bg-foreground/10 h-8 w-full rounded"></div>
							<div class="bg-foreground/10 h-8 w-4/5 rounded"></div>
							<div class="bg-foreground/10 h-8 w-3/5 rounded"></div>
						</div>
					</div>
				{:else if polisCurrentStatement}
					<p
						class="text-card-foreground text-3xl leading-9 font-normal"
						in:fly={{ y: 20, duration: 500, easing: cubicOut }}
					>
						{polisCurrentStatement.txt}
					</p>
				{/if}
			</div>

			<!-- Vote buttons -->
			<div class="flex flex-wrap items-start gap-4 md:gap-6">
				<Button
					variant="default"
					size="lg"
					{disabled}
					onclick={() => doVote('agree')}
					class="gap-2 px-6 py-4 text-lg"
				>
					<ThumbsUp class="h-5 w-5" />
					Agree
				</Button>
				<Button
					variant="default"
					size="lg"
					{disabled}
					onclick={() => doVote('disagree')}
					class="gap-2 px-6 py-4 text-lg"
				>
					<ThumbsDown class="h-5 w-5" />
					Disagree
				</Button>
				<button
					class="text-muted-foreground hover:text-foreground flex items-center gap-2 px-6 py-4 text-lg font-medium transition-colors disabled:opacity-50"
					{disabled}
					onclick={() => doVote('pass')}
				>
					Skip
					<SkipForward class="h-5 w-5" />
				</button>
			</div>

			<!-- Add your own opinion -->
			<button
				class="text-muted-foreground hover:text-foreground flex items-center gap-2 pt-2 text-xl font-normal transition-colors"
				onclick={openAddOpinion}
			>
				<PenLine class="h-5 w-5" />
				Add your own opinion
			</button>

			<!-- Continue to next step (only after threshold) -->
			{#if canContinue}
				<div class="mt-4 w-full border-t pt-6" in:fade={{ duration: 300 }}>
					<Button
						variant="primaryDark"
						size="lg"
						onclick={onDone}
						class="gap-2 px-6 py-4 text-lg"
					>
						Continue to next step
						<ChevronRight class="h-5 w-5" />
					</Button>
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
					<PenLine class="text-card-foreground h-8 w-8" />
					<h2 class="text-card-foreground text-3xl font-semibold">
						Add your own opinion
					</h2>
				</div>
				<button
					class="text-foreground hover:text-foreground/70 transition-colors"
					onclick={closeAddOpinion}
					aria-label="Close"
				>
					<X class="h-5 w-5" />
				</button>
			</div>

			<div class="text-card-foreground flex flex-col text-base">
				<p>An opinion should be answerable with "agree" or "disagree"</p>
				<p>Include only one idea at a time in one opinion</p>
				<p>Avoid use jargon (if necessary, try unpack with plain language)</p>
			</div>

			{#if opinionSubmitted}
				<div
					class="bg-primary/10 text-primary w-full rounded-lg p-4 text-center font-medium"
				>
					Your opinion has been submitted. Thank you!
				</div>
			{/if}

			<div class="w-full pb-6">
				<textarea
					bind:value={opinionText}
					placeholder="Enter your own opinion here"
					class="bg-background text-foreground placeholder:text-muted-foreground border-input focus:ring-primary/30 h-28 w-full resize-none rounded-lg border p-4 text-base shadow-sm outline-none focus:ring-2"
				></textarea>
			</div>

			<div class="flex flex-wrap items-start gap-6">
				<Button
					variant="default"
					size="lg"
					disabled={!opinionText.trim()}
					onclick={handleSubmitOpinion}
					class="gap-2 px-6 py-4 text-lg"
				>
					Submit
				</Button>
				<button
					class="text-muted-foreground hover:text-foreground flex items-center gap-2 px-6 py-4 text-lg font-medium transition-colors disabled:opacity-50"
					disabled={!opinionText.trim()}
					onclick={handleSubmitAndAddAnother}
				>
					Submit and add another one
					<ChevronRight class="h-5 w-5" />
				</button>
			</div>

			<button
				class="text-muted-foreground hover:text-foreground mt-2 text-base font-medium transition-colors"
				onclick={closeAddOpinion}
			>
				&larr; Back to voting
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
					Do you want to continue?
				</h2>
			</div>

			<div class="flex flex-wrap items-start gap-6">
				<Button
					variant="default"
					size="lg"
					onclick={resumeVoting}
					class="w-72 gap-2 px-6 py-4 text-lg"
				>
					Continue voting
				</Button>
				<button
					class="text-muted-foreground hover:text-foreground flex items-center gap-2 px-6 py-4 text-lg font-medium transition-colors"
					onclick={onDone}
				>
					continue to next step
					<ChevronRight class="h-5 w-5" />
				</button>
			</div>
		</div>
	{:else if screen === 'completed'}
		<!-- Voted everything -->
		<div
			class="flex w-full max-w-[808px] flex-col items-start gap-6 px-8 py-8 md:px-24 md:py-12"
			in:fade={{ duration: 300 }}
		>
			<p class="text-card-foreground text-3xl font-normal">
				You have voted on everything. Thank you!
			</p>

			<!-- Add your own opinion -->
			<button
				class="text-muted-foreground hover:text-foreground flex items-center gap-2 pt-2 text-xl font-normal transition-colors"
				onclick={openAddOpinion}
			>
				<PenLine class="h-5 w-5" />
				Add your own opinion
			</button>
		</div>

		<Button variant="primaryDark" size="lg" onclick={onDone} class="gap-2 px-6 py-4 text-lg">
			Continue
			<ChevronRight class="h-5 w-5" />
		</Button>
	{/if}
</div>
