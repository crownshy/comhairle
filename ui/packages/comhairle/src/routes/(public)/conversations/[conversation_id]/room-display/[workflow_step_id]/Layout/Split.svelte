<!--
	@component One full-width wall with the facilitator's controls along the bottom, no
	second surface. See NOTES.md, "Which direction?".
-->
<script lang="ts">
	import JoinQrCode from '../JoinQrCode.svelte';
	import type { RoomDisplaySource } from '../source';
	import SizedBlock from '../board/SizedBlock.svelte';
	import type { ReportComment } from '$lib/tools/polis/reportTypes';
	import { participantCount } from '../demo/scenario';
	import { presentByGroup, voteBarsFor } from '../live/liveVotes';
	import { groupColor } from '../OpinionMap/opinionMap';
	import {
		stillLatestDirection,
		hasBlock,
		latestIsBeside,
		type RoomBoard,
		blockScale
	} from '../board/blocks';
	import { groupLabel } from '$lib/tools/polis/report';
	import OpinionMap from '../OpinionMap/OpinionMap.svelte';
	import StatementStrip from '../StatementStrip.svelte';
	import WallStatements from '../WallStatements.svelte';
	import RoomVoteBar from '../RoomVoteBar.svelte';
	import * as LatestStatements from '../LatestStatements';

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
	let mapSlotHeight = $state(0);

	const groupIds = $derived(source.groups.map((g) => g.group_id));
	const clustered = $derived(source.stage === 'shaped' || source.stage === 'rich');
	const focused = $derived(source.state.published.find((c) => c.tid === focusedTid) ?? null);
	const present = $derived(presentByGroup(source.state, source.groups));

	const publishedByTid = $derived(new Map(source.state.published.map((c) => [c.tid, c])));

	/** Only statements already published, so the wall never shows one ahead of the room. */
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

	/** Picking what is already up puts the map back. */
	function showOnWall(view: WallView) {
		wallView = isShowing(view) ? { kind: 'map' } : view;
	}

	const showLatest = $derived(hasBlock(board, 'marquee'));
	const latestBeside = $derived(showLatest && latestIsBeside(board));
	const latestBelow = $derived(showLatest && !latestBeside);
	/** On a short laptop window a second statement would be cut off halfway, so show one. */
	const asideLatestMax = $derived(viewportHeight >= 1000 ? 2 : 1);

	/** A laptop window rather than a projector: type and spacing step down so nothing is clipped. */
	const compactWall = $derived(viewportHeight > 0 && viewportHeight < 900);

	const showFocus = $derived(hasBlock(board, 'map') || wallView.kind !== 'map');
	const showAside = $derived(
		hasBlock(board, 'statement') || hasBlock(board, 'strip') || latestBeside
	);
	const columns = $derived(showFocus && showAside ? 'lg:grid-cols-[1.1fr_1fr]' : 'grid-cols-1');

	/**
	 * The map is square, so its column is sized to the slot's height and the spare width
	 * goes to the other column. CSS `auto` would size to the legend instead of the map.
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
					<SizedBlock scale={blockScale(board, 'question')}>
						<h1
							class="text-foreground max-w-5xl text-2xl leading-tight font-bold text-balance sm:text-3xl {compactWall
								? 'lg:text-4xl'
								: 'lg:text-5xl'}"
						>
							{question}
						</h1>
					</SizedBlock>
				{/if}
				{#if hasBlock(board, 'counts')}
					<SizedBlock scale={blockScale(board, 'counts')}>
						<dl
							class="text-muted-foreground flex flex-wrap items-baseline gap-x-6 text-base sm:gap-x-8 sm:text-xl"
						>
							<div class="flex items-baseline gap-2">
								<dd
									class="text-foreground text-xl font-bold tabular-nums sm:text-2xl"
								>
									{participantCount(source.state)}
								</dd>
								<dt>here</dt>
							</div>
							<div class="flex items-baseline gap-2">
								<dd
									class="text-foreground text-xl font-bold tabular-nums sm:text-2xl"
								>
									{source.state.totalVotes}
								</dd>
								<dt>votes</dt>
							</div>
							<div class="flex items-baseline gap-2">
								<dd
									class="text-foreground text-xl font-bold tabular-nums sm:text-2xl"
								>
									{source.state.published.length}
								</dd>
								<dt>statements</dt>
							</div>
						</dl>
					</SizedBlock>
				{/if}
			</div>

			{#if hasBlock(board, 'qr')}
				<SizedBlock scale={blockScale(board, 'qr')}>
					<div class="flex shrink-0 flex-col items-center gap-1">
						<div class="rounded-xl bg-white p-2">
							<JoinQrCode value={joinUrl} class="size-20 sm:size-24 lg:size-28" />
						</div>
						<span class="text-muted-foreground text-base font-medium">Scan to join</span
						>
					</div>
				</SizedBlock>
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
						<SizedBlock scale={board.scale}>
							<p
								class="text-muted-foreground shrink-0 text-base font-medium tracking-wide uppercase"
							>
								Opinion groups
							</p>
							<div
								class="aspect-square min-h-0 w-full self-center lg:aspect-auto lg:h-auto lg:w-full lg:flex-1 lg:self-stretch"
								bind:clientHeight={mapSlotHeight}
							>
								<!-- Live data has no per-person vote counts, and Polis only places someone who has voted enough, so every dot counts as settled. -->
								<OpinionMap
									nodes={source.state.nodes}
									votesByTid={source.state.votesByTid}
									{focusedTid}
									settleVotes={source.voteMatrix === 'per-participant' ? 6 : 0}
									dotScale={blockScale(board, 'map')}
									{groupIds}
								/>
							</div>
						</SizedBlock>
					{:else if wallView.kind === 'group'}
						<SizedBlock scale={blockScale(board, 'statement')}>
							<WallStatements
								title="What Group {groupLabel(wallView.groupId)} thinks"
								statements={groupStatements(wallView.groupId)}
								{source}
								empty="Nothing sets this group apart yet."
							/>
						</SizedBlock>
					{:else}
						<SizedBlock scale={blockScale(board, 'statement')}>
							<WallStatements
								title="What the room agrees on"
								statements={consensusStatements}
								{source}
								empty="Not enough votes to call this yet."
							/>
						</SizedBlock>
					{/if}
				</section>
			{/if}

			{#if showAside}
				<section class="flex min-h-0 flex-col gap-4 lg:overflow-hidden">
					{#if hasBlock(board, 'statement')}
						<SizedBlock scale={blockScale(board, 'statement')}>
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
											class="text-card-foreground animate-in fade-in-0 text-xl leading-snug font-medium text-balance duration-400 motion-reduce:animate-none sm:text-2xl lg:text-3xl"
										>
											{focused.text}
										</p>
										<!-- Live dots only approximate the split, so the exact numbers are shown as bars. -->
										{#if source.voteMatrix === 'apportioned'}
											{@const bars = voteBarsFor(source, focused)}
											<div
												class="animate-in fade-in-0 grid gap-x-6 gap-y-3 duration-400 motion-reduce:animate-none"
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
									<p
										class="text-muted-foreground text-base sm:text-xl lg:text-2xl"
									>
										{#if hasBlock(board, 'strip')}
											Every dot below is a statement. Pick one to see how the
											room splits on it.
										{:else}
											Turn the statement strip on to pick one.
										{/if}
									</p>
								{/if}
							</div>
						</SizedBlock>
					{/if}

					{#if hasBlock(board, 'strip')}
						<SizedBlock scale={blockScale(board, 'strip')}>
							<div
								class="flex-1 {compactWall ? 'min-h-24' : 'min-h-28'} {latestBeside
									? 'lg:max-h-40'
									: ''}"
							>
								<StatementStrip
									comments={source.state.published}
									dotScale={blockScale(board, 'strip')}
									{focusedTid}
									interactive
									onfocusstatement={(tid) => (focusedTid = tid)}
								/>
							</div>
						</SizedBlock>
					{/if}

					{#if latestBeside}
						<SizedBlock scale={blockScale(board, 'marquee')}>
							<div class="min-h-0 flex-1 overflow-hidden">
								<LatestStatements.Default
									comments={source.state.published}
									direction="column"
									max={asideLatestMax}
								/>
							</div>
						</SizedBlock>
					{/if}
				</section>
			{/if}
		</div>
	{/if}

	{#if latestBelow}
		<SizedBlock scale={blockScale(board, 'marquee')}>
			<div class="min-h-0 shrink-0 overflow-hidden">
				{#if board.latest === 'marquee'}
					<LatestStatements.Marquee comments={source.state.published} />
				{:else}
					<LatestStatements.Default
						comments={source.state.published}
						direction={stillLatestDirection(board)}
						compact={compactWall}
					/>
				{/if}
			</div>
		</SizedBlock>
	{/if}

	{#if hasBlock(board, 'groups')}
		<SizedBlock scale={blockScale(board, 'groups')}>
			<nav
				class="flex shrink-0 flex-wrap items-center gap-3"
				aria-label="Room display controls"
			>
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
		</SizedBlock>
	{/if}
</div>
