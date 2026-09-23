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
-->
<script lang="ts">
	import type { RoomDisplaySource } from '$lib/room-display/source';
	import { hasBlock, type RoomBoard, type RoomBlock } from '$lib/room-display/blocks';
	import { resolveIntent } from '$lib/room-display/ambientFocus';
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

	const SLIDES: { key: string; title: string; block: RoomBlock }[] = [
		{ key: 'join', title: 'Join in', block: 'qr' },
		{ key: 'room', title: 'Who is in the room', block: 'map' },
		{ key: 'agree', title: 'What we agree on', block: 'statement' },
		{ key: 'split', title: 'Where we split', block: 'strip' },
		{ key: 'latest', title: 'Just said', block: 'marquee' }
	];

	const slides = $derived(SLIDES.filter((s) => hasBlock(board, s.block)));

	// Wrapped on read rather than clamped on write, so a slide switched off while the
	// deck sits on it lands somewhere valid without an effect mirroring the length.
	let index = $state(0);
	const position = $derived(slides.length > 0 ? index % slides.length : 0);
	const slide = $derived(slides[position] ?? null);

	const groupIds = $derived(source.groups.map((g) => g.group_id));
	const clustered = $derived(source.stage === 'shaped' || source.stage === 'rich');

	const consensusTid = $derived(resolveIntent(source.state, 'strongestConsensus'));
	const divisiveTid = $derived(resolveIntent(source.state, 'mostDivisive'));
	const consensus = $derived(source.state.published.find((c) => c.tid === consensusTid) ?? null);
	const divisive = $derived(source.state.published.find((c) => c.tid === divisiveTid) ?? null);
	const latest = $derived(source.state.published.slice(0, 4));
	// Bars only: the statement is the headline, so a vote block that repeats it above
	// the bars would be the same sentence twice.
	const consensusBars = $derived(consensus ? voteBarsFor(source, consensus) : null);
	const divisiveBars = $derived(divisive ? voteBarsFor(source, divisive) : null);

	function step(delta: number) {
		if (slides.length === 0) return;
		index = (position + delta + slides.length) % slides.length;
	}

	function onkeydown(event: KeyboardEvent) {
		const target = event.target as HTMLElement | null;
		if (target?.closest('input, textarea, [contenteditable]')) return;
		if (event.key === 'ArrowRight' || event.key === 'PageDown') step(1);
		else if (event.key === 'ArrowLeft' || event.key === 'PageUp') step(-1);
	}
</script>

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
			<WarmingScreen
				{question}
				{joinUrl}
				participants={participantCount(source.state)}
				votes={source.state.totalVotes}
			/>
		{:else if slide.key === 'room'}
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
						{groupIds}
					/>
				</div>
			</div>
		{:else if slide.key === 'agree'}
			<div class="flex h-full min-h-0 flex-col justify-center gap-8">
				{#if consensus && consensusBars}
					<p
						class="text-foreground max-w-6xl text-3xl leading-tight font-bold text-balance lg:text-6xl"
					>
						{consensus.text}
					</p>
					<div
						class="grid max-w-5xl gap-8"
						style="grid-template-columns: repeat({1 +
							consensusBars.groups.length}, minmax(0, 1fr));"
					>
						<RoomVoteBar {...consensusBars.overall} />
						{#each consensusBars.groups as bar (bar.label)}
							<RoomVoteBar {...bar} />
						{/each}
					</div>
				{:else}
					<p class="text-muted-foreground text-2xl">Not enough votes to call this yet.</p>
				{/if}
			</div>
		{:else if slide.key === 'split'}
			<div class="grid h-full min-h-0 gap-8 lg:grid-cols-2">
				<div class="flex min-h-0 flex-col justify-center gap-6">
					{#if divisive}
						<p
							class="text-foreground text-3xl leading-tight font-bold text-balance lg:text-5xl"
						>
							{divisive.text}
						</p>
						{#if source.voteMatrix === 'per-participant'}
							<p class="text-muted-foreground text-xl lg:text-2xl">
								Every dot is a person, coloured by how they voted on this one.
							</p>
						{:else if divisiveBars}
							<div
								class="grid gap-6"
								style="grid-template-columns: repeat({1 +
									divisiveBars.groups.length}, minmax(0, 1fr));"
							>
								<RoomVoteBar {...divisiveBars.overall} />
								{#each divisiveBars.groups as bar (bar.label)}
									<RoomVoteBar {...bar} />
								{/each}
							</div>
						{/if}
					{:else}
						<p class="text-muted-foreground text-2xl">
							Nothing divides the room enough to show yet.
						</p>
					{/if}
				</div>
				<div class="min-h-0">
					<OpinionMap
						nodes={source.state.nodes}
						votesByTid={source.state.votesByTid}
						focusedTid={divisiveTid}
						settleVotes={source.voteMatrix === 'per-participant' ? 6 : 0}
						{groupIds}
					/>
				</div>
			</div>
		{:else if slide.key === 'latest'}
			<!--
				The ticker as its own screen. Four statements at headline size is readable
				from the back of a room; twelve at body size is not, which is the whole
				argument against the sidebar it replaces.
			-->
			<ul class="flex h-full min-h-0 flex-col justify-center gap-6">
				{#each latest as statement (statement.tid)}
					<li
						class="text-foreground border-primary border-l-4 pl-5 text-2xl leading-snug font-medium text-balance lg:text-4xl"
					>
						{statement.text}
					</li>
				{:else}
					<li class="text-muted-foreground text-2xl">Nothing said yet.</li>
				{/each}
			</ul>
		{/if}
	</div>

	{#if slides.length > 0}
		<p class="text-muted-foreground shrink-0 text-base">
			{position + 1} of {slides.length} · click or arrow keys to move
		</p>
	{/if}
</div>
