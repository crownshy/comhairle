<!--
	@component The Room display's Deck direction. Kept beside the console as the
	alternative for a facilitator who would rather present than drive.

	One idea per screen in sequence, the facilitator interprets. Arrow keys,
	PageUp/PageDown or a click advance, so a presentation clicker works without any
	extra wiring.

	The slides are fixed and the *content* is live. A facilitator learns five screens
	once and can then run any session with them, which is not true of a deck rebuilt per
	event. Each slide resolves against the current state at the moment it is shown, so
	"where the room splits" is whatever splits the room now.

	A slide with nothing to say yet keeps its place and says so, rather than vanishing:
	a deck whose length changes under the facilitator is one they cannot rehearse.

	Each slide is one of the blocks in `blocks.ts`, so switching a block off anywhere
	removes that slide here. That is a deliberate exception to the paragraph above:
	turning a block off is the facilitator changing the deck on purpose, not the data
	changing it under them.

	Two slides draw from several statements, the top five by consensus and the top
	five by divisiveness, and both are shown one way or the other (`slides` on the
	board, ADR-0044). A *list* puts all five on screen. A *walk* shows one at a time, moving
	on after the ambient dwell, with its bars beside it and the map coloured by it on
	the right; the facilitator can hold the one they are talking about (space, or the
	pill under the statement) and step through by hand (up and down arrows). The hold is by statement, not by position, so a held statement stays
	put while the ranking under it keeps moving with the votes. Each walking slide
	keeps its own place and hold, so holding on one does not freeze the other.
-->
<script lang="ts">
	import type { RoomDisplaySource } from '$lib/room-display/source';
	import type { ReportComment } from '$lib/tools/polis/reportTypes';
	import SizedBlock from './SizedBlock.svelte';
	import { DECK_SLIDES, hasBlock, type RoomBoard, blockScale } from '$lib/room-display/blocks';
	import { rankIntent, DEFAULT_DWELL_MS } from '$lib/room-display/ambientFocus';
	import { participantCount } from '$lib/room-display/scenario';
	import { voteBarsFor } from '$lib/room-display/liveVotes';
	import OpinionMap from '$lib/room-display/OpinionMap.svelte';
	import WarmingScreen from '$lib/room-display/WarmingScreen.svelte';
	import RoomVoteBar from './RoomVoteBar.svelte';

	type Props = {
		source: RoomDisplaySource;
		question: string;
		joinUrl: string;
		board: RoomBoard;
	};

	let { source, question, joinUrl, board }: Props = $props();

	/**
	 * How many statements the agree and split slides each draw from. Five rows of
	 * headline text with a bar beside each is the most that still reads from the back
	 * of a room; a walk goes through the same number one at a time.
	 */
	const STATEMENTS_PER_SLIDE = 5;

	const slides = $derived(DECK_SLIDES.filter((s) => hasBlock(board, s.block)));

	// Wrapped on read rather than clamped on write, so a slide switched off while the
	// deck sits on it lands somewhere valid without an effect mirroring the length.
	let index = $state(0);
	const position = $derived(slides.length > 0 ? index % slides.length : 0);
	const slide = $derived(slides[position] ?? null);

	const groupIds = $derived(source.groups.map((g) => g.group_id));
	const clustered = $derived(source.stage === 'shaped' || source.stage === 'rich');

	// A zero score is "nothing to say", not a statement worth a headline: the ranking
	// pads its list with unvoted statements, and those are left out here.
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

	// One place and hold per walking slide. Cursor wraps on read, like the slide
	// index, so the walk survives its candidate list shrinking or reordering.
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
	/** The statement a walking slide shows: the held one, or where the walk has got to. */
	const shown = $derived<ReportComment | null>(
		walk === null
			? null
			: walk.heldTid !== null
				? (source.state.published.find((c) => c.tid === walk.heldTid) ?? null)
				: (candidates[walkPosition] ?? null)
	);
	// Bars only: the statement is the headline, so a vote block that repeats it above
	// the bars would be the same sentence twice.
	const shownBars = $derived(shown ? voteBarsFor(source, shown) : null);
	/** Where the shown statement sits in the walk, or -1 for a held one that fell out. */
	const shownAt = $derived(candidates.findIndex((c) => c.tid === shown?.tid));
	const walking = $derived(style === 'walk' && walk !== null);
	const cycling = $derived(walking && walk?.heldTid === null && candidates.length > 1);

	const listRows = $derived(
		style === 'list'
			? candidates.map((comment) => ({ comment, bars: voteBarsFor(source, comment) }))
			: []
	);

	// Switching to the list drops both holds. A hold is "stay on this one while I
	// talk", and once the slides have been changed out from under it, that sentence is
	// over; a walk that came back still held would look like a walk that had stopped.
	$effect(() => {
		if (board.slides !== 'list') return;
		walks.agree.heldTid = null;
		walks.split.heldTid = null;
	});

	// The one thing on the deck that moves without a hand on it. Statements swap on
	// arrival and sit still in between, which is the motion the room display allows.
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

	/** Holds the statement on screen, or lets the walk go on from where it is. */
	function toggleHold() {
		if (walkKey === null || walk === null) return;
		if (walk.heldTid !== null) {
			if (shownAt >= 0) walks[walkKey].cursor = shownAt;
			walks[walkKey].heldTid = null;
		} else if (shown) {
			walks[walkKey].heldTid = shown.tid;
		}
	}

	/** Moves to the next or previous statement by hand, and holds there. */
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

<!--
	The walk's own position and its hold toggle. A button rather than the footer hint
	alone so a facilitator with only a mouse can hold one too; it stops the click so
	the slide does not advance under it.
-->
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

<!-- The bars for one statement in a row: overall first, then each group. -->
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

<!--
	Strongest first, top to bottom. Every row gets the same size: the order already says
	which is strongest, and a headline row over four small ones would be the
	single-statement slide with footnotes.
-->
{#snippet list(empty: string)}
	{#if listRows.length > 0}
		<!--
			Every other row sits on a band, so five statements read as five things rather
			than one block of text with bars beside it. Rows touch, which is what makes the
			bands read as rows; the padding inside them is the spacing the gap used to be.
		-->
		<ul class="flex h-full min-h-0 flex-col justify-center overflow-hidden">
			{#each listRows as row (row.comment.tid)}
				<!--
					The text takes what the bars leave. Each bar column has a floor wide
					enough for "Overall 33% agree" on one line, so a 768px projector
					shortens the statement column rather than truncating the labels. The rows
					sit closer together below 1536px wide for the same reason: five rows
					with a three-line statement or two have to fit 768px high.
				-->
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

<!--
	One statement at a time, the same shape on both slides: the statement and its bars
	on the left, the map coloured by it on the right. The bars say how the room split
	on it; the map says who. Together they are the whole picture of one statement,
	which is what a walk is for.
-->
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

<!--
	The whole slide is the advance target, which is how a facilitator standing at a
	laptop actually drives one. The map inside is not interactive on the deck: a click
	means "next", nothing else.
-->
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
			<!-- Sized by its own slide's block, which is what the panel's "Join in" row sets. -->
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
					<!--
						No map here: it can only be coloured by one statement, and five rows of
						per-group bars already show where each one splits.
					-->
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
				<!--
				The ticker as its own screen. Four statements at headline size is readable
				from the back of a room; twelve at body size is not, which is the whole
				argument against the sidebar it replaces.
			-->
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
