<!--
	@component PROTOTYPE variant: Deck.

	The Deck direction from CONTEXT.md: one idea per screen in sequence, the facilitator
	interprets. Arrow keys, PageUp/PageDown or a click advance, so a presentation
	clicker works without any extra wiring.

	The slides are fixed and the *content* is live. A facilitator learns five screens
	once and can then run any session with them, which is not true of a deck rebuilt per
	event. Each slide resolves against the current state at the moment it is shown, so
	"where the room splits" is whatever splits the room now.

	A slide with nothing to say yet keeps its place and says so, rather than vanishing:
	a deck whose length changes under the facilitator is one they cannot rehearse.
-->
<script lang="ts">
	import type { RoomDisplayDriver } from '$lib/room-display/driver.svelte';
	import type { Scenario } from '$lib/room-display/types';
	import { resolveIntent } from '$lib/room-display/ambientFocus';
	import { participantCount } from '$lib/room-display/scenario';
	import OpinionMap from '$lib/room-display/OpinionMap.svelte';
	import WarmingScreen from '$lib/room-display/WarmingScreen.svelte';
	import VoteBar from '$lib/reports/polis/VoteBar.svelte';
	import { computeMemberVoteBars } from '$lib/tools/polis/report';

	type Props = {
		driver: RoomDisplayDriver;
		scenario: Scenario;
		question: string;
		joinUrl: string;
	};

	let { driver, scenario, question, joinUrl }: Props = $props();

	const SLIDES = [
		{ key: 'join', title: 'Join in' },
		{ key: 'room', title: 'Who is in the room' },
		{ key: 'agree', title: 'What we agree on' },
		{ key: 'split', title: 'Where we split' },
		{ key: 'latest', title: 'Just said' }
	] as const;

	let index = $state(0);
	const slide = $derived(SLIDES[index]);

	const groupIds = $derived(scenario.groups.map((g) => g.group_id));
	const clustered = $derived(driver.stage === 'shaped' || driver.stage === 'rich');

	const consensusTid = $derived(resolveIntent(driver.state, 'strongestConsensus'));
	const divisiveTid = $derived(resolveIntent(driver.state, 'mostDivisive'));
	const consensus = $derived(driver.state.published.find((c) => c.tid === consensusTid) ?? null);
	const divisive = $derived(driver.state.published.find((c) => c.tid === divisiveTid) ?? null);
	const latest = $derived(driver.state.published.slice(0, 4));
	// Bars only. `StatementVoteBlock` repeats the statement text above them, which on a
	// slide whose whole point is that statement is the same sentence twice.
	const consensusBars = $derived(
		consensus ? computeMemberVoteBars(consensus, scenario.groups) : null
	);

	function step(delta: number) {
		index = (index + delta + SLIDES.length) % SLIDES.length;
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
	class="flex h-full min-h-0 flex-col gap-6"
	role="button"
	tabindex="0"
	onclick={() => step(1)}
	onkeydown={(e) => e.key === 'Enter' && step(1)}
>
	<header class="flex shrink-0 items-baseline justify-between gap-6">
		<h2 class="text-muted-foreground text-xl font-medium tracking-wide uppercase lg:text-2xl">
			{slide.title}
		</h2>
		<div class="flex items-center gap-2">
			{#each SLIDES as s, i (s.key)}
				<span
					class="size-2.5 rounded-full transition-colors"
					style="background: {i === index ? 'var(--primary)' : 'var(--border)'}"
				></span>
			{/each}
		</div>
	</header>

	<div class="min-h-0 flex-1">
		{#if slide.key === 'join'}
			<WarmingScreen
				{question}
				{joinUrl}
				participants={participantCount(driver.state)}
				votes={driver.state.totalVotes}
			/>
		{:else if slide.key === 'room'}
			<div class="flex h-full min-h-0 flex-col gap-4">
				<p class="text-foreground shrink-0 text-3xl font-bold text-balance lg:text-5xl">
					{participantCount(driver.state)} people,
					{#if clustered}
						{scenario.groups.length} ways of seeing it
					{:else}
						still finding the shape
					{/if}
				</p>
				<div class="min-h-0 flex-1">
					<OpinionMap
						nodes={driver.state.nodes}
						votesByTid={driver.state.votesByTid}
						focusedTid={null}
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
					<div class="flex max-w-4xl flex-col gap-3">
						<VoteBar {...consensusBars.overall} />
						{#each consensusBars.groups as bar (bar.label)}
							<VoteBar {...bar} />
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
						<p class="text-muted-foreground text-xl lg:text-2xl">
							Every dot is a person, coloured by how they voted on this one.
						</p>
					{:else}
						<p class="text-muted-foreground text-2xl">
							Nothing divides the room enough to show yet.
						</p>
					{/if}
				</div>
				<div class="min-h-0">
					<OpinionMap
						nodes={driver.state.nodes}
						votesByTid={driver.state.votesByTid}
						focusedTid={divisiveTid}
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

	<p class="text-muted-foreground shrink-0 text-base">
		{index + 1} of {SLIDES.length} · click or arrow keys to move
	</p>
</div>
