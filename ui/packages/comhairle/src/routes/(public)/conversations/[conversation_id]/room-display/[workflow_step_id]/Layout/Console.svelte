<!--
	@component Two surfaces: a calm wall for the room and a dense panel for the facilitator.
	They share one page by default, or open as separate windows kept in sync. See NOTES.md
	and ADR-0045.
-->
<script lang="ts">
	import JoinQrCode from '../JoinQrCode.svelte';
	import type { RoomDisplaySource } from '../source';
	import SizedBlock from '../board/SizedBlock.svelte';
	import type { ReportComment } from '$lib/tools/polis/reportTypes';
	import { participantCount } from '../demo/scenario';
	import { nextUnlock, describeUnlock } from '../revealStage';
	import { presentByGroup, voteBarsFor } from '../live/liveVotes';
	import { groupColor } from '../OpinionMap/opinionMap';
	import { hasBlock, type RoomBoard, blockScale, stillLatestDirection } from '../board/blocks';
	import {
		SURFACE_WINDOW_NAMES,
		surfaceHref,
		type ConsoleState,
		type RoomSurface,
		type WallView
	} from '../board/surfaces';
	import { PanelRight } from '@lucide/svelte';
	import { groupLabel } from '$lib/tools/polis/report';
	import OpinionMap from '../OpinionMap/OpinionMap.svelte';
	import { Button } from '$lib/components/ui/button';
	import StatementStrip from '../StatementStrip.svelte';
	import WallStatements from '../WallStatements.svelte';
	import RoomVoteBar from '../RoomVoteBar.svelte';
	import * as LatestStatements from '../LatestStatements';

	type Props = {
		source: RoomDisplaySource;
		question: string;
		joinUrl: string;
		board: RoomBoard;
		/** Which half this window is. `both` is the one-page prototype. */
		surface: RoomSurface;
		console: ConsoleState;
		onSetConsole: (next: ConsoleState) => void;
		onSetSurface: (next: RoomSurface) => void;
	};

	let { source, question, joinUrl, board, surface, console, onSetConsole, onSetSurface }: Props =
		$props();

	const focusedTid = $derived(console.focusedTid);
	const wallView = $derived(console.wallView);
	let consoleOpen = $state(true);

	function focusStatement(tid: number | null) {
		onSetConsole({ ...console, focusedTid: tid });
	}

	/** The window is named, so asking twice brings the open one forward instead of opening another. */
	function openSurface(target: 'wall' | 'console') {
		window.open(surfaceHref(window.location.href, target), SURFACE_WINDOW_NAMES[target]);
	}

	function detachConsole() {
		openSurface('console');
		onSetSurface('wall');
	}

	const groupIds = $derived(source.groups.map((g) => g.group_id));
	const clustered = $derived(source.stage === 'shaped' || source.stage === 'rich');
	const focused = $derived(source.state.published.find((c) => c.tid === focusedTid) ?? null);
	// The countdown counts voters. Live data only knows vote totals, not who cast them.
	const unlock = $derived(
		source.voteMatrix === 'per-participant' ? nextUnlock(source.state, source.stage) : null
	);
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

	/** Picking what is already on the wall puts the map back. */
	function showOnWall(view: WallView) {
		onSetConsole({ ...console, wallView: isShowing(view) ? { kind: 'map' } : view });
	}

	const showWallMain = $derived(hasBlock(board, 'map') || wallView.kind !== 'map');
	const hasConsoleBlocks = $derived(
		hasBlock(board, 'counts') ||
			hasBlock(board, 'strip') ||
			hasBlock(board, 'marquee') ||
			hasBlock(board, 'groups')
	);
	const showWall = $derived(surface !== 'console');
	const showConsole = $derived(surface !== 'wall' && consoleOpen && hasConsoleBlocks);
</script>

<div
	class="grid min-h-0 gap-6 lg:h-full {showWall && showConsole
		? 'lg:grid-cols-[1.6fr_1fr]'
		: 'grid-cols-1'}"
>
	{#if showWall}
		<!-- In its own window the wall is the whole screen, so it drops the frame and caption. -->
		<section
			class="relative flex min-h-0 flex-col gap-4 {surface === 'both'
				? 'border-border rounded-lg border p-6 lg:p-8'
				: 'p-2 lg:p-4'}"
		>
			{#if surface === 'both'}
				<p
					class="text-muted-foreground shrink-0 text-base font-medium tracking-wide uppercase"
				>
					On the wall
				</p>
			{/if}
			{#if hasBlock(board, 'question')}
				<SizedBlock scale={blockScale(board, 'question')}>
					<h1
						class="text-foreground max-w-5xl shrink-0 text-2xl leading-tight font-bold text-balance sm:text-3xl lg:text-5xl"
					>
						{question}
					</h1>
				</SizedBlock>
			{/if}

			<!-- Right padding keeps content clear of the QR code in the corner. -->
			<div class="min-h-0 flex-1 pb-4 {hasBlock(board, 'qr') ? 'lg:pr-48' : ''}">
				{#if !showWallMain}
					<!-- Empty on purpose: every block that could fill this is switched off. -->
				{:else if wallView.kind === 'map'}
					<div class="flex min-h-0 flex-col gap-4 lg:h-full">
						<SizedBlock scale={board.scale}>
							<div
								class="aspect-square min-h-0 w-full lg:aspect-auto lg:h-auto lg:flex-1"
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
						{#if hasBlock(board, 'statement')}
							<SizedBlock scale={blockScale(board, 'statement')}>
								<!-- Minimum height stops the layout jumping; the key replays the fade on each new statement. -->
								<div class="flex min-h-32 shrink-0 flex-col justify-center gap-3">
									{#if focused}
										{#key focused.tid}
											<p
												class="text-foreground animate-in fade-in-0 text-xl leading-snug font-medium text-balance duration-400 motion-reduce:animate-none sm:text-3xl lg:text-4xl"
											>
												{focused.text}
											</p>
											<!-- Live dots only approximate the split, so the exact numbers are shown as bars. -->
											{#if source.voteMatrix === 'apportioned'}
												{@const bars = voteBarsFor(source, focused)}
												<div
													class="animate-in fade-in-0 grid max-w-5xl gap-8 duration-400 motion-reduce:animate-none"
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
											Every dot is a person. Pick a statement on the console
											to see how the room splits on it.
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

			{#if hasBlock(board, 'marquee')}
				<SizedBlock scale={blockScale(board, 'marquee')}>
					<div
						class="min-h-0 shrink-0 overflow-hidden {hasBlock(board, 'qr')
							? 'lg:pr-48'
							: ''}"
					>
						{#if board.latest === 'marquee'}
							<LatestStatements.Marquee comments={source.state.published} />
						{:else}
							<LatestStatements.Default
								comments={source.state.published}
								direction={stillLatestDirection(board)}
							/>
						{/if}
					</div>
				</SizedBlock>
			{/if}

			{#if hasBlock(board, 'qr')}
				<SizedBlock scale={blockScale(board, 'qr')}>
					<!-- The QR code stays on the wall all session so latecomers can join. -->
					<div
						class="flex flex-col items-center gap-1 self-end lg:absolute lg:right-8 lg:bottom-8"
					>
						<div class="rounded-xl bg-white p-2">
							<JoinQrCode value={joinUrl} class="size-20 sm:size-24 lg:size-32" />
						</div>
						<span class="text-muted-foreground text-base font-medium">Scan to join</span
						>
					</div>
				</SizedBlock>
			{/if}
		</section>
	{/if}

	{#if showConsole}
		<section class="bg-muted flex min-h-0 flex-col gap-5 rounded-lg p-5">
			<div class="flex shrink-0 flex-wrap items-center justify-between gap-2">
				<p class="text-muted-foreground text-base font-medium tracking-wide uppercase">
					On your laptop
				</p>
				<div class="flex gap-1">
					{#if surface === 'both'}
						<Button variant="ghost" size="sm" onclick={() => (consoleOpen = false)}>
							Hide console
						</Button>
						<Button variant="ghost" size="sm" onclick={detachConsole}>
							Open in new window
						</Button>
					{:else}
						<Button variant="ghost" size="sm" onclick={() => openSurface('wall')}>
							Open wall in new window
						</Button>
					{/if}
				</div>
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
							onfocusstatement={focusStatement}
						/>
					</div>

					{#if hasBlock(board, 'statement')}
						<SizedBlock scale={blockScale(board, 'statement')}>
							<div
								class="bg-background border-border flex min-h-28 shrink-0 items-center rounded-md border px-4 py-3"
							>
								{#if focused}
									{#key focused.tid}
										<p
											class="text-foreground animate-in fade-in-0 text-base leading-snug font-medium duration-400 motion-reduce:animate-none sm:text-xl lg:text-2xl"
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
					<!-- The console is a narrow column, so the list always runs vertically here. -->
					{#if board.latest === 'marquee'}
						<LatestStatements.Marquee comments={source.state.published} />
					{:else}
						<LatestStatements.Default
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
	{:else if surface === 'wall'}
		<!-- Faint because it is not part of what the room reads, like the settings gear. -->
		<button
			type="button"
			class="text-muted-foreground hover:text-foreground focus-visible:text-foreground fixed top-4 right-4 z-40 rounded-full p-2 opacity-30 transition-opacity hover:opacity-100 focus-visible:opacity-100"
			aria-label="Open the console"
			title="Open the console in its own window"
			onclick={() => openSurface('console')}
		>
			<PanelRight class="size-6" />
		</button>
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
