<!--
	@component Two surfaces: a calm wall and a dense facilitator panel. The direction
	the team picked; see NOTES.md for what was folded in from the review.

	The argument: a board is busy because one screen is being asked to do two jobs.
	The room needs a wall it can read from eight metres with no pointer; the
	facilitator needs a dense panel they can scan and click from half a metre. Those
	are opposite requirements and merging them gives you something that is neither.

	So: the wall gets the question, the map with its group labels, one statement and a
	QR code in the corner. Everything the facilitator drives lives on the console,
	including the latest statements, which sit under the strip so what has just come
	in is beside the thing that points the wall at it:

	- Hovering the strip picks one statement. It appears under the strip, and the wall
	  shows it under the map. With per-participant votes the map recolours by it; a
	  live source has none (CONTEXT.md, "Cross-highlight"), so the wall shows the
	  group bars for it instead.
	- Picking an opinion group (or "Consensus statements") swaps the map out for that
	  group's key statements with vote bars. Picking it again brings the map back.

	The console collapses, because on a real big screen it is not there at all: the two
	surfaces are one page here so the pair can be judged in one screenshot. Two real
	windows would need to share focus state, which is a build question, not a design
	one.

	Every region is a block that can be switched off (`blocks.ts`). With every console
	block off the panel goes away by itself, which is the closest this layout gets to
	being the wall on its own.
-->
<script lang="ts">
	import JoinQrCode from '$lib/room-display/JoinQrCode.svelte';
	import type { RoomDisplaySource } from '$lib/room-display/source';
	import SizedBlock from './SizedBlock.svelte';
	import type { ReportComment } from '$lib/tools/polis/reportTypes';
	import { participantCount } from '$lib/room-display/scenario';
	import { nextUnlock, describeUnlock } from '$lib/room-display/revealStage';
	import { presentByGroup, voteBarsFor } from '$lib/room-display/liveVotes';
	import { groupColor } from '$lib/room-display/opinionMap';
	import { hasBlock, type RoomBoard, blockScale } from '$lib/room-display/blocks';
	import { groupLabel } from '$lib/tools/polis/report';
	import OpinionMap from '$lib/room-display/OpinionMap.svelte';
	import { Button } from '$lib/components/ui/button';
	import StatementStrip from './StatementStrip.svelte';
	import WallStatements from './WallStatements.svelte';
	import RoomVoteBar from './RoomVoteBar.svelte';
	import LatestStatements from './LatestStatements.svelte';
	import LatestStatementsMarquee from './LatestStatementsMarquee.svelte';

	type Props = {
		source: RoomDisplaySource;
		question: string;
		joinUrl: string;
		board: RoomBoard;
	};

	let { source, question, joinUrl, board }: Props = $props();

	/** What the wall's main area shows. The map unless the facilitator asks for a list. */
	type WallView = { kind: 'map' } | { kind: 'group'; groupId: number } | { kind: 'consensus' };

	let focusedTid = $state<number | null>(null);
	let wallView = $state<WallView>({ kind: 'map' });
	let consoleOpen = $state(true);

	const groupIds = $derived(source.groups.map((g) => g.group_id));
	const clustered = $derived(source.stage === 'shaped' || source.stage === 'rich');
	const focused = $derived(source.state.published.find((c) => c.tid === focusedTid) ?? null);
	// The countdown counts voters, which an apportioned matrix cannot tell you: it
	// knows how many votes a statement got, not how many people cast them.
	const unlock = $derived(
		source.voteMatrix === 'per-participant' ? nextUnlock(source.state, source.stage) : null
	);
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

	/** Toggles: picking what is already on the wall puts the map back. */
	function showOnWall(view: WallView) {
		wallView = isShowing(view) ? { kind: 'map' } : view;
	}

	// The wall's main area, same rule as the split layout: the map, or whatever the
	// console has put there instead. With neither it is a hole, so it goes.
	const showWallMain = $derived(hasBlock(board, 'map') || wallView.kind !== 'map');
	// Nothing left to drive means no panel: the wall stops sharing the page.
	const hasConsoleBlocks = $derived(
		hasBlock(board, 'counts') ||
			hasBlock(board, 'strip') ||
			hasBlock(board, 'marquee') ||
			hasBlock(board, 'groups')
	);
	const showConsole = $derived(consoleOpen && hasConsoleBlocks);
</script>

<div
	class="grid min-h-0 gap-6 lg:h-full {showConsole ? 'lg:grid-cols-[1.6fr_1fr]' : 'grid-cols-1'}"
>
	<!-- Wall -->
	<section
		class="border-border relative flex min-h-0 flex-col gap-4 rounded-lg border p-6 lg:p-8"
	>
		<p class="text-muted-foreground shrink-0 text-base font-medium tracking-wide uppercase">
			On the wall
		</p>
		{#if hasBlock(board, 'question')}
			<SizedBlock scale={blockScale(board, 'question')}>
				<h1
					class="text-foreground max-w-5xl shrink-0 text-2xl leading-tight font-bold text-balance sm:text-3xl lg:text-5xl"
				>
					{question}
				</h1>
			</SizedBlock>
		{/if}

		<!-- Right padding keeps everything clear of the QR corner, whatever the wall shows. -->
		<div class="min-h-0 flex-1 pb-4 {hasBlock(board, 'qr') ? 'lg:pr-48' : ''}">
			{#if !showWallMain}
				<!-- Deliberately empty: every block that could fill this is switched off. -->
			{:else if wallView.kind === 'map'}
				<div class="flex min-h-0 flex-col gap-4 lg:h-full">
					<SizedBlock scale={board.scale}>
						<div
							class="aspect-square min-h-0 w-full lg:aspect-auto lg:h-auto lg:flex-1"
						>
							<!--
							An apportioned matrix says nothing about how often one person voted,
							so a placed participant counts as settled: Polis only gives someone a
							position once they have voted enough to have one.
						-->
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
					{#if hasBlock(board, 'statement')}
						<SizedBlock scale={blockScale(board, 'statement')}>
							<!-- Fixed minimum height, keyed to replay the fade. -->
							<div class="flex min-h-32 shrink-0 flex-col justify-center gap-3">
								{#if focused}
									{#key focused.tid}
										<p
											class="text-foreground fade-in text-xl leading-snug font-medium text-balance sm:text-3xl lg:text-4xl"
										>
											{focused.text}
										</p>
										<!-- The dots round, so the exact split is spelled out beside them. -->
										{#if source.voteMatrix === 'apportioned'}
											{@const bars = voteBarsFor(source, focused)}
											<div
												class="fade-in grid max-w-5xl gap-8"
												style="grid-template-columns: repeat({1 +
													bars.groups.length}, minmax(0, 1fr));"
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
										class="text-muted-foreground text-base sm:text-2xl lg:text-3xl"
									>
										Every dot is a person. Pick a statement on the console to
										see how the room splits on it.
									</p>
								{/if}
							</div>
						</SizedBlock>
					{/if}
				</div>
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
		</div>

		{#if hasBlock(board, 'qr')}
			<SizedBlock scale={blockScale(board, 'qr')}>
				<!--
				The QR code never leaves the wall. Someone arriving late has to be able to
				join from whatever the screen happens to be showing, not only from the
				recruitment screen the room saw at the start.
			-->
				<div
					class="flex flex-col items-center gap-1 self-end lg:absolute lg:right-8 lg:bottom-8"
				>
					<div class="rounded-xl bg-white p-2">
						<JoinQrCode value={joinUrl} class="size-20 sm:size-24 lg:size-32" />
					</div>
					<span class="text-muted-foreground text-base font-medium">Scan to join</span>
				</div>
			</SizedBlock>
		{/if}
	</section>

	<!-- Console -->
	{#if showConsole}
		<section class="bg-muted flex min-h-0 flex-col gap-5 rounded-lg p-5">
			<div class="flex shrink-0 items-center justify-between">
				<p class="text-muted-foreground text-base font-medium tracking-wide uppercase">
					On your laptop
				</p>
				<Button variant="ghost" size="sm" onclick={() => (consoleOpen = false)}>
					Hide console
				</Button>
			</div>

			{#if hasBlock(board, 'counts')}
				<SizedBlock scale={blockScale(board, 'counts')}>
					<dl class="grid shrink-0 grid-cols-3 gap-3">
						<div>
							<dt class="text-muted-foreground text-base sm:text-lg">Here</dt>
							<dd class="text-foreground text-2xl font-bold tabular-nums sm:text-3xl">
								{participantCount(source.state)}
							</dd>
						</div>
						<div>
							<dt class="text-muted-foreground text-base sm:text-lg">Votes</dt>
							<dd class="text-foreground text-2xl font-bold tabular-nums sm:text-3xl">
								{source.state.totalVotes}
							</dd>
						</div>
						<div>
							<dt class="text-muted-foreground text-base sm:text-lg">Statements</dt>
							<dd class="text-foreground text-2xl font-bold tabular-nums sm:text-3xl">
								{source.state.published.length}
							</dd>
						</div>
					</dl>

					{#if unlock}
						<p class="text-muted-foreground shrink-0 text-lg">
							{describeUnlock(unlock)} until {unlock.stage}
						</p>
					{/if}
				</SizedBlock>
			{/if}

			{#if hasBlock(board, 'strip')}
				<SizedBlock scale={blockScale(board, 'strip')}>
					<div class="h-24 shrink-0 lg:h-28">
						<StatementStrip
							comments={source.state.published}
							dotScale={blockScale(board, 'strip')}
							{focusedTid}
							interactive
							onfocusstatement={(tid) => (focusedTid = tid)}
						/>
					</div>

					{#if hasBlock(board, 'statement')}
						<SizedBlock scale={blockScale(board, 'statement')}>
							<!-- One statement at a time: whatever the strip is pointing at. -->
							<div
								class="bg-background border-border flex min-h-28 shrink-0 items-center rounded-md border px-4 py-3"
							>
								{#if focused}
									{#key focused.tid}
										<p
											class="text-foreground fade-in text-base leading-snug font-medium sm:text-xl lg:text-2xl"
										>
											{focused.text}
										</p>
									{/key}
								{:else}
									<p class="text-muted-foreground text-base sm:text-xl">
										Hover a dot on the strip to see its statement.
									</p>
								{/if}
							</div>
						</SizedBlock>
					{/if}
				</SizedBlock>
			{/if}

			{#if hasBlock(board, 'marquee')}
				<SizedBlock scale={blockScale(board, 'marquee')}>
					<!--
						Under the strip, on the laptop: the facilitator watches what has just
						come in and can point the wall at it. The console is a column, so the
						still list runs down it whatever `?latest=` says.
					-->
					{#if board.latest === 'marquee'}
						<LatestStatementsMarquee comments={source.state.published} />
					{:else}
						<LatestStatements
							comments={source.state.published}
							direction="column"
							compact
						/>
					{/if}
				</SizedBlock>
			{/if}

			{#if hasBlock(board, 'groups')}
				<SizedBlock scale={blockScale(board, 'groups')}>
					<div class="flex min-h-0 flex-col gap-3 overflow-y-auto">
						<p
							class="text-muted-foreground shrink-0 text-base font-medium tracking-wide uppercase"
						>
							Opinion groups
						</p>
						{#if clustered}
							<!-- Each button puts that group's statements on the wall; pressing it again restores the map. -->
							{#each source.groups as group (group.group_id)}
								{@const view: WallView = { kind: 'group', groupId: group.group_id }}
								<button
									type="button"
									class="border-border hover:bg-background flex w-full items-center gap-3 rounded-md border px-4 py-3 text-left text-base transition-colors sm:text-xl"
									class:bg-background={isShowing(view)}
									class:border-primary={isShowing(view)}
									aria-pressed={isShowing(view)}
									onclick={() => showOnWall(view)}
								>
									<span
										class="inline-block size-5 shrink-0 rounded-full"
										style="background: {groupColor(group.group_id)}"
									></span>
									<span class="text-foreground font-semibold"
										>Group {groupLabel(group.group_id)}</span
									>
									<span class="text-muted-foreground ml-auto tabular-nums">
										{present.get(group.group_id) ?? 0} here
									</span>
								</button>
							{/each}
							<button
								type="button"
								class="border-border hover:bg-background flex w-full items-center gap-3 rounded-md border px-4 py-3 text-left text-base transition-colors sm:text-xl"
								class:bg-background={isShowing({ kind: 'consensus' })}
								class:border-primary={isShowing({ kind: 'consensus' })}
								aria-pressed={isShowing({ kind: 'consensus' })}
								onclick={() => showOnWall({ kind: 'consensus' })}
							>
								<span class="text-foreground font-semibold"
									>Consensus statements</span
								>
								<span class="text-muted-foreground ml-auto text-lg"
									>everyone agrees</span
								>
							</button>
						{:else}
							<p class="text-muted-foreground text-lg">
								Groups appear once the room has voted enough for Polis to cluster
								it.
							</p>
						{/if}
					</div>
				</SizedBlock>
			{/if}
		</section>
	{:else if hasConsoleBlocks}
		<Button
			variant="outline"
			size="sm"
			class="fixed top-4 right-4 z-40"
			onclick={() => (consoleOpen = true)}
		>
			Show console
		</Button>
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
