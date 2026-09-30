<!--
	@component Five fixed slides with live content, advanced by hand (arrow keys, a
	presentation clicker, or a click). How the agree and split slides walk or list their
	statements is in ADR-0044.
-->
<script lang="ts">
	import type { RoomDisplaySource } from '../source';
	import type { ReportComment } from '$lib/tools/polis/reportTypes';
	import SizedBlock from '../board/SizedBlock.svelte';
	import { DECK_SLIDES, hasBlock, type RoomBoard, blockScale } from '../board/blocks';
	import { rankIntent, DEFAULT_DWELL_MS } from './ambientFocus';
	import { participantCount } from '../demo/scenario';
	import { voteBarsFor } from '../live/liveVotes';
	import OpinionMap from '../OpinionMap/OpinionMap.svelte';
	import WarmingScreen from '../WarmingScreen.svelte';
	import RoomVoteBar from '../RoomVoteBar.svelte';

	type Props = {
		source: RoomDisplaySource;
		question: string;
		joinUrl: string;
		board: RoomBoard;
	};

	let { source, question, joinUrl, board }: Props = $props();

	/** Five rows with bars is the most that still reads from the back of a room. */
	const STATEMENTS_PER_SLIDE = 5;

	const slides = $derived(DECK_SLIDES.filter((s) => hasBlock(board, s.block)));

	// Wrapped on read, so switching off the current slide still lands on a valid one
	// without an $effect copying the slide count into state.
	let index = $state(0);
	const position = $derived(slides.length > 0 ? index % slides.length : 0);
	const slide = $derived(slides[position] ?? null);

	const groupIds = $derived(source.groups.map((g) => g.group_id));
	const clustered = $derived(source.stage === 'shaped' || source.stage === 'rich');

	// The ranking includes unvoted statements with a zero score; leave those out.
	const agreeCandidates = $derived(
		rankIntent(source.state, 'strongestConsensus', STATEMENTS_PER_SLIDE).filter(
			(c) => (c.group_informed_consensus ?? 0) > 0
		)
	);
	const splitCandidates = $derived(
		rankIntent(source.state, 'mostDivisive', STATEMENTS_PER_SLIDE).filter(
			(c) => (c.divisiveness ?? 0) > 0
		)
	);

	type VoteBars = ReturnType<typeof voteBarsFor>;

	type WalkKey = 'agree' | 'split';

	// The cursor wraps on read, like the slide index, so it survives the list shrinking.
	let walks = $state<Record<WalkKey, { cursor: number; heldTid: number | null }>>({
		agree: { cursor: 0, heldTid: null },
		split: { cursor: 0, heldTid: null }
	});

	const walkKey = $derived<WalkKey | null>(
		slide?.key === 'agree' || slide?.key === 'split' ? slide.key : null
	);
	const style = $derived(walkKey ? board.slides : null);
	const candidates = $derived(
		walkKey === 'agree' ? agreeCandidates : walkKey === 'split' ? splitCandidates : []
	);
	const walk = $derived(walkKey ? walks[walkKey] : null);
	const walkPosition = $derived(
		walk && candidates.length > 0 ? walk.cursor % candidates.length : 0
	);
	const shown = $derived<ReportComment | null>(
		walk === null
			? null
			: walk.heldTid !== null
				? (source.state.published.find((c) => c.tid === walk.heldTid) ?? null)
				: (candidates[walkPosition] ?? null)
	);
	const shownBars = $derived(shown ? voteBarsFor(source, shown) : null);
	/** -1 when a held statement has dropped out of the top five. */
	const shownAt = $derived(candidates.findIndex((c) => c.tid === shown?.tid));
	const walking = $derived(style === 'walk' && walk !== null);
	const cycling = $derived(walking && walk?.heldTid === null && candidates.length > 1);

	const listRows = $derived(
		style === 'list'
			? candidates.map((comment) => ({ comment, bars: voteBarsFor(source, comment) }))
			: []
	);

	// Switching to the list drops both holds, otherwise a walk would come back stuck (ADR-0044).
	$effect(() => {
		if (board.slides !== 'list') return;
		walks.agree.heldTid = null;
		walks.split.heldTid = null;
	});

	$effect(() => {
		if (!cycling || walkKey === null) return;
		const key = walkKey;
		const timer = setInterval(() => {
			walks[key].cursor += 1;
		}, DEFAULT_DWELL_MS);
		return () => clearInterval(timer);
	});

	function step(delta: number) {
		if (slides.length === 0) return;
		index = (position + delta + slides.length) % slides.length;
	}

	function toggleHold() {
		if (walkKey === null || walk === null) return;
		if (walk.heldTid !== null) {
			if (shownAt >= 0) walks[walkKey].cursor = shownAt;
			walks[walkKey].heldTid = null;
		} else if (shown) {
			walks[walkKey].heldTid = shown.tid;
		}
	}

	/** Stepping by hand also holds, so the walk does not move on straight away. */
	function stepWithin(delta: number) {
		if (walkKey === null) return;
		const count = candidates.length;
		if (count === 0) return;
		const from = shownAt >= 0 ? shownAt : walkPosition;
		const next = (from + delta + count) % count;
		walks[walkKey].cursor = next;
		walks[walkKey].heldTid = candidates[next].tid;
	}

	function onkeydown(event: KeyboardEvent) {
		const target = event.target as HTMLElement | null;
		if (target?.closest('input, textarea, [contenteditable]')) return;
		if (event.key === 'ArrowRight' || event.key === 'PageDown') step(1);
		else if (event.key === 'ArrowLeft' || event.key === 'PageUp') step(-1);
		else if (walking) {
			if (event.key === ' ') {
				event.preventDefault();
				toggleHold();
			} else if (event.key === 'ArrowDown') {
				event.preventDefault();
				stepWithin(1);
			} else if (event.key === 'ArrowUp') {
				event.preventDefault();
				stepWithin(-1);
			}
		}
	}
</script>

<!-- The hold button stops the click so the slide does not advance under it. -->
{#snippet walkControls(noun: string)}
	{#if walk && candidates.length > 1}
		<div class="flex flex-wrap items-center gap-4">
			<div class="flex items-center gap-2" aria-hidden="true">
				{#each candidates as c, i (c.tid)}
					<span
						class="size-2 rounded-full transition-colors"
						style="background: {i === shownAt ? 'var(--primary)' : 'var(--border)'}"
					></span>
				{/each}
			</div>
			<button
				type="button"
				class="text-muted-foreground hover:text-foreground border-border rounded-full border px-4 py-1.5 text-base transition-colors"
				aria-pressed={walk.heldTid !== null}
				onclick={(e) => {
					e.stopPropagation();
					toggleHold();
				}}
			>
				{#if walk.heldTid !== null}
					Holding this one · space to move on
				{:else}
					Moving through the {candidates.length} {noun} · space to hold
				{/if}
			</button>
		</div>
	{/if}
{/snippet}

{#snippet bars(set: VoteBars, gap: string)}
	<div
		class="grid {gap}"
		style="grid-template-columns: repeat({1 + set.groups.length}, minmax(0, 1fr));"
	>
		<RoomVoteBar {...set.overall} />
		{#each set.groups as bar (bar.label)}
			<RoomVoteBar {...bar} />
		{/each}
	</div>
{/snippet}

{#snippet list(empty: string)}
	{#if listRows.length > 0}
		<!-- Alternate rows get a background band; rows touch so the bands read as rows. -->
		<ul class="flex h-full min-h-0 flex-col justify-center overflow-hidden">
			{#each listRows as row (row.comment.tid)}
				<!-- Bar columns have a minimum width so their labels never wrap; the statement shrinks instead. -->
				<li
					class="odd:bg-muted/60 flex flex-col gap-3 rounded-xl px-4 py-2 lg:grid lg:items-center lg:gap-x-6 2xl:px-6 2xl:py-4"
					style="grid-template-columns: minmax(0, 4fr) repeat({1 +
						row.bars.groups.length}, minmax(13rem, 1fr));"
				>
					<p
						class="text-foreground text-2xl leading-tight font-bold text-balance lg:text-3xl 2xl:text-4xl"
					>
						{row.comment.text}
					</p>
					<RoomVoteBar {...row.bars.overall} />
					{#each row.bars.groups as bar (bar.label)}
						<RoomVoteBar {...bar} />
					{/each}
				</li>
			{/each}
		</ul>
	{:else}
		<div class="flex h-full min-h-0 flex-col justify-center">
			<p class="text-muted-foreground text-2xl">{empty}</p>
		</div>
	{/if}
{/snippet}

{#snippet walkSlide(noun: string, empty: string)}
	<div class="grid h-full min-h-0 gap-8 lg:grid-cols-2">
		<div class="flex min-h-0 flex-col justify-center gap-6">
			{#if shown}
				<p
					class="text-foreground text-3xl leading-tight font-bold text-balance lg:text-5xl"
				>
					{shown.text}
				</p>
				{#if shownBars}
					{@render bars(shownBars, 'gap-6')}
				{/if}
				{#if source.voteMatrix === 'per-participant'}
					<p class="text-muted-foreground text-xl lg:text-2xl">
						Every dot is a person, coloured by how they voted on this one.
					</p>
				{/if}
				{@render walkControls(noun)}
			{:else}
				<p class="text-muted-foreground text-2xl">{empty}</p>
			{/if}
		</div>
		<div class="min-h-0">
			<OpinionMap
				nodes={source.state.nodes}
				votesByTid={source.state.votesByTid}
				focusedTid={shown?.tid ?? null}
				settleVotes={source.voteMatrix === 'per-participant' ? 6 : 0}
				dotScale={blockScale(board, 'map')}
				{groupIds}
			/>
		</div>
	</div>
{/snippet}

<svelte:window {onkeydown} />

<!-- Clicking anywhere on the slide advances it; the map is not interactive on the deck. -->
<div
	class="flex min-h-[85vh] flex-col gap-6 lg:h-full lg:min-h-0"
	role="button"
	tabindex="0"
	onclick={() => step(1)}
	onkeydown={(e) => e.key === 'Enter' && step(1)}
>
	<header class="flex shrink-0 items-baseline justify-between gap-6">
		<h2 class="text-muted-foreground text-xl font-medium tracking-wide uppercase lg:text-2xl">
			{slide?.title ?? 'Nothing to show'}
		</h2>
		<div class="flex items-center gap-2">
			{#each slides as s, i (s.key)}
				<span
					class="size-2.5 rounded-full transition-colors"
					style="background: {i === position ? 'var(--primary)' : 'var(--border)'}"
				></span>
			{/each}
		</div>
	</header>

	<div class="min-h-0 flex-1">
		{#if slide === null}
			<div class="flex h-full items-center justify-center">
				<p class="text-muted-foreground text-2xl">
					Every slide is switched off. Turn one back on in the board settings.
				</p>
			</div>
		{:else if slide.key === 'join'}
			<!-- The panel's "Join in" row sets this size. -->
			<SizedBlock scale={blockScale(board, 'qr')}>
				<WarmingScreen
					{question}
					{joinUrl}
					participants={participantCount(source.state)}
					votes={source.state.totalVotes}
				/>
			</SizedBlock>
		{:else if slide.key === 'room'}
			<SizedBlock scale={board.scale}>
				<div class="flex h-full min-h-0 flex-col gap-4">
					<p class="text-foreground shrink-0 text-3xl font-bold text-balance lg:text-5xl">
						{participantCount(source.state)} people,
						{#if clustered}
							{source.groups.length} ways of seeing it
						{:else}
							still finding the shape
						{/if}
					</p>
					<div class="min-h-0 flex-1">
						<OpinionMap
							nodes={source.state.nodes}
							votesByTid={source.state.votesByTid}
							focusedTid={null}
							settleVotes={source.voteMatrix === 'per-participant' ? 6 : 0}
							dotScale={blockScale(board, 'map')}
							{groupIds}
						/>
					</div>
				</div>
			</SizedBlock>
		{:else if slide.key === 'agree'}
			<SizedBlock scale={blockScale(board, 'statement')}>
				{#if style === 'list'}
					{@render list('Not enough votes to call this yet.')}
				{:else}
					{@render walkSlide(
						'strongest agreements',
						'Not enough votes to call this yet.'
					)}
				{/if}
			</SizedBlock>
		{:else if slide.key === 'split'}
			<SizedBlock scale={blockScale(board, 'strip')}>
				{#if style === 'list'}
					{@render list('Nothing divides the room enough to show yet.')}
				{:else}
					{@render walkSlide(
						'sharpest splits',
						'Nothing divides the room enough to show yet.'
					)}
				{/if}
			</SizedBlock>
		{:else if slide.key === 'latest'}
			<SizedBlock scale={blockScale(board, 'marquee')}>
				<!-- Four statements at headline size is what reads from the back of a room. -->
				<ul class="flex h-full min-h-0 flex-col justify-center gap-6">
					{#each rankIntent(source.state, 'newest', 4) as statement (statement.tid)}
						<li
							class="text-foreground border-primary border-l-4 pl-5 text-2xl leading-snug font-medium text-balance lg:text-4xl"
						>
							{statement.text}
						</li>
					{:else}
						<li class="text-muted-foreground text-2xl">Nothing said yet.</li>
					{/each}
				</ul>
			</SizedBlock>
		{/if}
	</div>

	{#if slides.length > 0}
		<p class="text-muted-foreground shrink-0 text-base">
			{position + 1} of {slides.length} · click or arrow keys to move{#if cycling || walk?.heldTid !== null}
				· up and down for the next one, space to hold{/if}
		</p>
	{/if}
</div>
