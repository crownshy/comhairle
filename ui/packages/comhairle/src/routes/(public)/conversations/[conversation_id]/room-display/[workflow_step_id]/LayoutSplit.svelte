<!--
	@component One full-width wall, no second surface. Reached as `?variant=marquee`,
	or as `layout=split` with any set of blocks.

	Console splits the display across two surfaces because the wall and the facilitator
	want opposite things. This layout takes the other bet: there is only one surface,
	and the facilitator's controls are a bar along the bottom of it, close enough to a
	presenter's hand and far enough from the map to stay out of the room's way.

	What that buys is width. The map and the selected statement sit side by side at
	full wall width rather than squeezed into two thirds of it, and the space the
	console used to occupy becomes the marquee: the newest statements scrolling along
	the bottom, which is the room seeing its own words go past.

	Every region is a block that can be switched off (`blocks.ts`), so the columns
	collapse rather than leaving a hole: with the map off the statement takes the whole
	wall, and with the statement and strip both off the map does.
-->
<script lang="ts">
	import JoinQrCode from '$lib/room-display/JoinQrCode.svelte';
	import type { RoomDisplaySource } from '$lib/room-display/source';
	import type { ReportComment } from '$lib/tools/polis/reportTypes';
	import { participantCount } from '$lib/room-display/scenario';
	import { presentByGroup, voteBarsFor } from '$lib/room-display/liveVotes';
	import { groupColor } from '$lib/room-display/opinionMap';
	import {
		stillLatestDirection,
		hasBlock,
		latestIsBeside,
		type RoomBoard
	} from '$lib/room-display/blocks';
	import { groupLabel } from '$lib/tools/polis/report';
	import OpinionMap from '$lib/room-display/OpinionMap.svelte';
	import StatementStrip from './StatementStrip.svelte';
	import WallStatements from './WallStatements.svelte';
	import RoomVoteBar from './RoomVoteBar.svelte';
	import LatestStatements from './LatestStatements.svelte';
	import LatestStatementsMarquee from './LatestStatementsMarquee.svelte';

	/** What the focus column shows. The map unless the bottom bar asks for a list. */
	type WallView = { kind: 'map' } | { kind: 'group'; groupId: number } | { kind: 'consensus' };

	type Props = {
		source: RoomDisplaySource;
		question: string;
		joinUrl: string;
		board: RoomBoard;
	};

	let { source, question, joinUrl, board }: Props = $props();

	let focusedTid = $state<number | null>(null);
	let wallView = $state<WallView>({ kind: 'map' });
	let viewportHeight = $state(0);
	let viewportWidth = $state(0);
	/** Measured height of the map's slot, which is what decides how wide the map can be. */
	let mapSlotHeight = $state(0);

	const groupIds = $derived(source.groups.map((g) => g.group_id));
	const clustered = $derived(source.stage === 'shaped' || source.stage === 'rich');
	const focused = $derived(source.state.published.find((c) => c.tid === focusedTid) ?? null);
	const present = $derived(presentByGroup(source.state, source.groups));

	const publishedByTid = $derived(new Map(source.state.published.map((c) => [c.tid, c])));

	/** A group's representative statements, limited to what has been published so far. */
	function groupStatements(groupId: number): ReportComment[] {
		const group = source.groups.find((g) => g.group_id === groupId);
		if (!group) return [];
		return group.representative_comments
			.map((r) => publishedByTid.get(r.tid))
			.filter((c): c is ReportComment => c !== undefined);
	}

	const consensusStatements = $derived(
		[...source.state.published].sort(
			(a, b) => (b.group_informed_consensus ?? 0) - (a.group_informed_consensus ?? 0)
		)
	);

	function isShowing(view: WallView): boolean {
		if (view.kind !== wallView.kind) return false;
		return (
			view.kind !== 'group' || wallView.kind !== 'group' || view.groupId === wallView.groupId
		);
	}

	/** Toggles: picking what is already up puts the map back. */
	function showOnWall(view: WallView) {
		wallView = isShowing(view) ? { kind: 'map' } : view;
	}

	// The strip is a fixed-height plot, so in a full-height column it leaves dead space
	// underneath. `aside` spends that space on the latest statements instead.
	const showLatest = $derived(hasBlock(board, 'marquee'));
	const latestBeside = $derived(showLatest && latestIsBeside(board));
	const latestBelow = $derived(showLatest && !latestBeside);
	/**
	 * What fits under the strip. A projector has room for two; a laptop window is short
	 * enough that the second would be cut off halfway down its box, which reads as
	 * broken rather than as "there is more".
	 */
	const asideLatestMax = $derived(viewportHeight >= 1000 ? 2 : 1);

	/**
	 * A wall short enough that room-scale type and spacing stop fitting. A projector at
	 * 1080p is not this; a laptop window with every block switched on is. Without the
	 * step down the header and the latest row eat the height the map and the strip need,
	 * and the blocks that lose are clipped rather than shrunk.
	 */
	const compactWall = $derived(viewportHeight > 0 && viewportHeight < 900);

	// The focus column earns its space either because the map is on, or because the
	// bottom bar has put a list of statements in it. With neither, it is a hole.
	const showFocus = $derived(hasBlock(board, 'map') || wallView.kind !== 'map');
	const showAside = $derived(
		hasBlock(board, 'statement') || hasBlock(board, 'strip') || latestBeside
	);
	const columns = $derived(showFocus && showAside ? 'lg:grid-cols-[1.1fr_1fr]' : 'grid-cols-1');

	/**
	 * The opinion map is a square plot: its SVG keeps a 1:1 viewBox, so in a column wider
	 * than it is tall it draws a small square in the middle and leaves the rest of the
	 * column empty. That dead space is most of what makes a short wall look broken.
	 *
	 * So the map's column is sized to the map rather than to a fraction of the wall: as
	 * wide as its slot is tall, which is exactly what the square can use. A CSS `auto`
	 * column cannot do this (it resolves to the legend's intrinsic width, not the
	 * square's), and `aspect-square` on the slot only moves the empty space inside it.
	 * The width the map was wasting goes to the statement and the strip.
	 */
	const wallColumns = $derived(
		viewportWidth >= 1024 &&
			showFocus &&
			showAside &&
			hasBlock(board, 'map') &&
			mapSlotHeight > 0
			? `grid-template-columns: ${Math.round(
					Math.max(240, Math.min(mapSlotHeight, viewportWidth * 0.45))
				)}px minmax(0, 1fr);`
			: ''
	);
	const showHeader = $derived(
		hasBlock(board, 'question') || hasBlock(board, 'counts') || hasBlock(board, 'qr')
	);
</script>

<svelte:window bind:innerHeight={viewportHeight} bind:innerWidth={viewportWidth} />

<div class="flex min-h-0 flex-col gap-6 lg:h-full lg:overflow-hidden">
	{#if showHeader}
		<header class="flex shrink-0 items-start justify-between gap-4 sm:gap-8">
			<div class="flex min-w-0 flex-col gap-3">
				{#if hasBlock(board, 'question')}
					<h1
						class="text-foreground max-w-5xl text-2xl leading-tight font-bold text-balance sm:text-3xl {compactWall
							? 'lg:text-4xl'
							: 'lg:text-5xl'}"
					>
						{question}
					</h1>
				{/if}
				{#if hasBlock(board, 'counts')}
					<!--
						The console carried these; without it the room would have no running
						count of itself at all, so they come along under the question.
					-->
					<dl
						class="text-muted-foreground flex flex-wrap items-baseline gap-x-6 text-base sm:gap-x-8 sm:text-xl"
					>
						<div class="flex items-baseline gap-2">
							<dd class="text-foreground text-xl font-bold tabular-nums sm:text-2xl">
								{participantCount(source.state)}
							</dd>
							<dt>here</dt>
						</div>
						<div class="flex items-baseline gap-2">
							<dd class="text-foreground text-xl font-bold tabular-nums sm:text-2xl">
								{source.state.totalVotes}
							</dd>
							<dt>votes</dt>
						</div>
						<div class="flex items-baseline gap-2">
							<dd class="text-foreground text-xl font-bold tabular-nums sm:text-2xl">
								{source.state.published.length}
							</dd>
							<dt>statements</dt>
						</div>
					</dl>
				{/if}
			</div>

			{#if hasBlock(board, 'qr')}
				<!-- Latecomers have to be able to join from whatever the wall happens to show. -->
				<div class="flex shrink-0 flex-col items-center gap-1">
					<div class="rounded-xl bg-white p-2">
						<JoinQrCode value={joinUrl} class="size-20 sm:size-24 lg:size-28" />
					</div>
					<span class="text-muted-foreground text-base font-medium">Scan to join</span>
				</div>
			{/if}
		</header>
	{/if}

	{#if showFocus || showAside}
		<div
			class="grid min-h-0 flex-1 gap-6 lg:gap-8 lg:overflow-hidden {columns}"
			style={wallColumns}
		>
			{#if showFocus}
				<section class="flex min-h-0 flex-col gap-4 lg:overflow-hidden">
					{#if wallView.kind === 'map'}
						<p
							class="text-muted-foreground shrink-0 text-base font-medium tracking-wide uppercase"
						>
							Opinion groups
						</p>
						<div
							class="aspect-square min-h-0 w-full self-center lg:aspect-auto lg:h-auto lg:w-full lg:flex-1 lg:self-stretch"
							bind:clientHeight={mapSlotHeight}
						>
							<!--
								An apportioned matrix says nothing about how often one person
								voted, so a placed participant counts as settled: Polis only
								gives someone a position once they have voted enough to have
								one.
							-->
							<OpinionMap
								nodes={source.state.nodes}
								votesByTid={source.state.votesByTid}
								{focusedTid}
								settleVotes={source.voteMatrix === 'per-participant' ? 6 : 0}
								{groupIds}
							/>
						</div>
					{:else if wallView.kind === 'group'}
						<WallStatements
							title="What Group {groupLabel(wallView.groupId)} thinks"
							statements={groupStatements(wallView.groupId)}
							{source}
							empty="Nothing sets this group apart yet."
						/>
					{:else}
						<WallStatements
							title="What the room agrees on"
							statements={consensusStatements}
							{source}
							empty="Not enough votes to call this yet."
						/>
					{/if}
				</section>
			{/if}

			{#if showAside}
				<section class="flex min-h-0 flex-col gap-4 lg:overflow-hidden">
					{#if hasBlock(board, 'statement')}
						<p
							class="text-muted-foreground shrink-0 text-base font-medium tracking-wide uppercase"
						>
							Selected statement
						</p>
						<div
							class="border-border bg-card flex min-h-32 shrink-0 flex-col justify-center gap-3 rounded-lg border px-5 py-4 lg:px-6 lg:py-5 {compactWall
								? ''
								: 'lg:min-h-40 lg:gap-4'}"
						>
							{#if focused}
								{#key focused.tid}
									<p
										class="text-card-foreground fade-in text-xl leading-snug font-medium text-balance sm:text-2xl lg:text-3xl"
									>
										{focused.text}
									</p>
									<!-- The dots round, so the exact split is spelled out beside them. -->
									{#if source.voteMatrix === 'apportioned'}
										{@const bars = voteBarsFor(source, focused)}
										<div
											class="fade-in grid gap-x-6 gap-y-3"
											style="grid-template-columns: repeat(auto-fit, minmax(9rem, 1fr));"
										>
											<RoomVoteBar {...bars.overall} />
											{#each bars.groups as bar (bar.label)}
												<RoomVoteBar {...bar} />
											{/each}
										</div>
									{/if}
								{/key}
							{:else}
								<p class="text-muted-foreground text-base sm:text-xl lg:text-2xl">
									{#if hasBlock(board, 'strip')}
										Every dot below is a statement. Pick one to see how the room
										splits on it.
									{:else}
										Turn the statement strip on to pick one.
									{/if}
								</p>
							{/if}
						</div>
					{/if}

					{#if hasBlock(board, 'strip')}
						<div
							class="flex-1 {compactWall ? 'min-h-24' : 'min-h-28'} {latestBeside
								? 'lg:max-h-40'
								: ''}"
						>
							<StatementStrip
								comments={source.state.published}
								{focusedTid}
								interactive
								onfocusstatement={(tid) => (focusedTid = tid)}
							/>
						</div>
					{/if}

					{#if latestBeside}
						<div class="min-h-0 flex-1 overflow-hidden">
							<LatestStatements
								comments={source.state.published}
								direction="column"
								max={asideLatestMax}
							/>
						</div>
					{/if}
				</section>
			{/if}
		</div>
	{/if}

	{#if latestBelow}
		<div class="min-h-0 shrink-0 overflow-hidden">
			{#if board.latest === 'marquee'}
				<LatestStatementsMarquee comments={source.state.published} />
			{:else}
				<LatestStatements
					comments={source.state.published}
					direction={stillLatestDirection(board)}
					compact={compactWall}
				/>
			{/if}
		</div>
	{/if}

	{#if hasBlock(board, 'groups')}
		<!--
			The facilitator's controls, along the bottom. Each button puts that group's
			statements in the focus column; pressing it again restores the map.
		-->
		<nav class="flex shrink-0 flex-wrap items-center gap-3" aria-label="Room display controls">
			{#if clustered}
				{#each source.groups as group (group.group_id)}
					{@const view: WallView = { kind: 'group', groupId: group.group_id }}
					<button
						type="button"
						class="border-border hover:bg-muted flex items-center gap-3 rounded-md border px-4 py-2.5 text-base transition-colors sm:px-5 sm:py-3 sm:text-xl"
						class:bg-muted={isShowing(view)}
						class:border-primary={isShowing(view)}
						aria-pressed={isShowing(view)}
						onclick={() => showOnWall(view)}
					>
						<span
							class="inline-block size-5 shrink-0 rounded-full"
							style="background: {groupColor(group.group_id)}"
						></span>
						<span class="text-foreground font-semibold">
							Group {groupLabel(group.group_id)}
						</span>
						<span class="text-muted-foreground tabular-nums">
							{present.get(group.group_id) ?? 0} here
						</span>
					</button>
				{/each}
				<button
					type="button"
					class="border-border hover:bg-muted flex items-center gap-3 rounded-md border px-4 py-2.5 text-base transition-colors sm:px-5 sm:py-3 sm:text-xl"
					class:bg-muted={isShowing({ kind: 'consensus' })}
					class:border-primary={isShowing({ kind: 'consensus' })}
					aria-pressed={isShowing({ kind: 'consensus' })}
					onclick={() => showOnWall({ kind: 'consensus' })}
				>
					<span class="text-foreground font-semibold">Consensus statements</span>
				</button>
			{:else}
				<p class="text-muted-foreground text-base sm:text-xl">
					Groups appear once the room has voted enough for Polis to cluster it.
				</p>
			{/if}
		</nav>
	{/if}
</div>

<style>
	.fade-in {
		animation: fade-in 400ms ease both;
	}

	@keyframes fade-in {
		from {
			opacity: 0;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.fade-in {
			animation: none;
		}
	}
</style>
