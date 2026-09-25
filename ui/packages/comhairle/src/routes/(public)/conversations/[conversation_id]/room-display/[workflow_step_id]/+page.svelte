<!--
	The Room display for one Polis step: a large-format surface projected in a room
	while the conversation runs (CONTEXT.md, "Room display").

	Two sources behind one display, chosen by `?mode=` (see +page.ts):

	  live  - polls the step's real report data. What a room actually sees.
	  demo  - a scripted 4.5 hour run compressed to a few minutes, with animated joins
	          and votes, so the thing can be shown off without a room. Only the demo
	          carries per-participant votes, so only the demo colours dots by a vote.

	What the display shows is a board: a layout plus a set of blocks (`blocks.ts`).
	`?variant=` names a familiar one, `?layout=` and `?blocks=` say it exactly, and the
	settings panel on the display edits it live. The demo's transport controls sit in a
	floating bar in dev builds only.
-->
<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { page } from '$app/state';
	import { dev } from '$app/environment';
	import { replaceState } from '$app/navigation';
	import type { PageProps } from './$types';
	import type { RoomDisplaySource } from '$lib/room-display/source';
	import { createRoomDisplayDriver } from '$lib/room-display/driver.svelte';
	import { createLiveRoomDisplaySource } from '$lib/room-display/liveSource.svelte';
	import { buildScenario } from '$lib/room-display/buildScenario';
	import { PLACEHOLDER_STATEMENTS } from '$lib/room-display/placeholderStatements';
	import { buildPlaceholderComments } from '$lib/room-display/placeholderReport';
	import { nextUnlock, describeUnlock } from '$lib/room-display/revealStage';
	import { participantCount } from '$lib/room-display/scenario';
	import {
		applyPreset,
		matchingPreset,
		resolveBoard,
		serializeBlocks,
		serializeSizes,
		setBlockSize,
		toggleBlock,
		type BoardPreset,
		type LatestStyle,
		type RoomBlock,
		type RoomBoard,
		type RoomLayout,
		type RoomTheme
	} from '$lib/room-display/blocks';
	import { themeStore } from '$lib/stores/theme.svelte';
	import {
		INITIAL_CONSOLE_STATE,
		openSurfaceLink,
		surfaceHref,
		type ConsoleState,
		type RoomSurface
	} from '$lib/room-display/surfaces';
	import { readStoredBoard, writeStoredBoard } from '$lib/room-display/storedBoard';
	import WarmingScreen from '$lib/room-display/WarmingScreen.svelte';
	import PrototypeBar from './PrototypeBar.svelte';
	import BoardSettings from './BoardSettings.svelte';
	import SizedBlock from './SizedBlock.svelte';
	import LayoutDeck from './LayoutDeck.svelte';
	import LayoutConsole from './LayoutConsole.svelte';
	import LayoutSplit from './LayoutSplit.svelte';

	let { data }: PageProps = $props();

	const ROOM_SIZE = 28;

	// Built once: the mode and rate are fixed for the life of the page, so re-deriving
	// the source would rebuild the run under the viewer. Reading `data` here captures
	// its initial value, which is the point.
	// svelte-ignore state_referenced_locally
	const driver =
		data.mode === 'demo'
			? createRoomDisplayDriver({
					scenario: buildScenario({
						statements: PLACEHOLDER_STATEMENTS,
						sourceComments: buildPlaceholderComments(PLACEHOLDER_STATEMENTS),
						participantCount: ROOM_SIZE,
						durationMs: 4.5 * 60 * 60 * 1000,
						seed: 20261007
					}),
					rate: data.rate,
					autoplay: true
				})
			: null;
	// svelte-ignore state_referenced_locally
	const source: RoomDisplaySource =
		driver ??
		createLiveRoomDisplaySource({ api: page.data.api, workflowStepId: data.workflowStepId });
	onDestroy(() => source.destroy());

	// Starts at what the server could work out, which is the URL and nothing else.
	// svelte-ignore state_referenced_locally
	let board = $state<RoomBoard>(data.board);
	// svelte-ignore state_referenced_locally
	let surface = $state<RoomSurface>(data.surface);

	// What the console has decided and the wall shows: the focused statement and the
	// wall's main view. Held here rather than in the console layout because it has to
	// cross windows (surfaces.ts), and the wire is opened once per page.
	let consoleState = $state<ConsoleState>(INITIAL_CONSOLE_STATE);

	// Everything sent is snapshotted first: what goes over the channel is structured
	// cloned, and a `$state` proxy cannot be.
	// svelte-ignore state_referenced_locally
	const link = openSurfaceLink(data.workflowStepId, {
		onState: (state) => (consoleState = state),
		onBoard: (next) => applyBoard(next, { share: false }),
		onHello: () => {
			// A wall has nothing to tell a newcomer, and if it answered alongside the
			// console the two replies would race.
			if (surface === 'wall') return;
			link.sendState($state.snapshot(consoleState));
			link.sendBoard($state.snapshot(board));
		}
	});
	onDestroy(() => link.close());

	onMount(() => {
		// localStorage is not readable during SSR, so what this display remembered can
		// only be folded in here. Resolving again rather than assigning the stored board
		// keeps the precedence rule in one place: an explicit URL still wins.
		board = resolveBoard({ ...data.boardParams, stored: readStoredBoard() });
		applyTheme(board.theme);
	});

	/**
	 * The URL is rewritten to spell the board out, so the address bar is always a link
	 * that reproduces what is on screen. `variant` stays only while the board still is
	 * that template: once a block has been touched by hand the name is no longer true,
	 * and leaving it would make the link mean something different from the screen that
	 * produced it.
	 *
	 * A board that arrived from the other window is applied but not sent back, or the
	 * two would echo it at each other forever.
	 */
	function applyBoard(next: RoomBoard, { share } = { share: true }) {
		board = next;
		writeStoredBoard(next);
		applyTheme(next.theme);
		if (share) link.sendBoard($state.snapshot(next));

		const url = new URL(window.location.href);
		const preset = matchingPreset(next);
		if (preset) url.searchParams.set('variant', preset);
		else url.searchParams.delete('variant');
		url.searchParams.set('layout', next.layout);
		url.searchParams.set('blocks', serializeBlocks(next.blocks));
		url.searchParams.set('latest', next.latest);
		url.searchParams.set('theme', next.theme);
		url.searchParams.set('scale', String(next.scale));
		url.searchParams.set('sizes', serializeSizes(next.sizes));
		// This rewrites the query string of the page we are already on rather than
		// navigating anywhere, so there is no route for `resolve()` to resolve.
		// eslint-disable-next-line svelte/no-navigation-without-resolve
		replaceState(url, page.state);
	}

	/** A template replaces the arrangement; the room keeps its lighting and scale. */
	function onSetPreset(preset: BoardPreset) {
		applyBoard(applyPreset(board, preset));
	}

	function onToggleBlock(block: RoomBlock) {
		applyBoard(toggleBlock(board, block));
	}

	function onSetLayout(layout: RoomLayout) {
		applyBoard({ ...board, layout });
	}

	function onSetLatest(latest: LatestStyle) {
		applyBoard({ ...board, latest });
	}

	function onSetTheme(theme: RoomTheme) {
		applyBoard({ ...board, theme });
	}

	function onSetScale(scale: number) {
		applyBoard({ ...board, scale });
	}

	function onSetBlockSize(block: RoomBlock, size: number) {
		applyBoard(setBlockSize(board, block, size));
	}

	/**
	 * `auto` deliberately does nothing rather than restoring a previous mode: dark is a
	 * class on `<html>` and there is no per-page scope for it, so the display drives the
	 * app-wide store. Forcing a mode on every load would override the viewer's own
	 * preference just for opening this page, so only an explicit pick touches it. That
	 * does mean going back to `auto` leaves the last pick in place until something else
	 * changes it, which on a projector is what you want anyway.
	 */
	function applyTheme(theme: RoomTheme) {
		if (theme === 'auto') return;
		themeStore.setMode(theme);
	}

	function onReset() {
		applyBoard(resolveBoard({ preset: data.boardParams.preset }));
	}

	function onSetConsole(next: ConsoleState) {
		consoleState = next;
		link.sendState($state.snapshot(next));
	}

	/** This window becomes one half of the pair; the URL says which, so a reload agrees. */
	function onSetSurface(next: RoomSurface) {
		surface = next;
		// eslint-disable-next-line svelte/no-navigation-without-resolve
		replaceState(surfaceHref(window.location.href, next), page.state);
	}

	// Counts voters, which an apportioned matrix cannot tell you: it knows how many
	// votes a statement got, not how many people cast them.
	const unlock = $derived(
		source.voteMatrix === 'per-participant' ? nextUnlock(source.state, source.stage) : null
	);
	const recruiting = $derived(source.stage === 'empty' || source.stage === 'warming');
</script>

<svelte:head><title>Room display</title></svelte:head>

<!--
	The wall is a fixed viewport with nothing to scroll: that is the whole point of a
	projected surface. A phone or a narrow window cannot honour that without painting
	the blocks on top of each other, so below `lg` the display stops pretending to be a
	wall and becomes an ordinary scrolling page.
-->
<div
	class="room-display bg-background text-foreground min-h-screen p-4 pb-24 lg:h-screen lg:overflow-hidden lg:p-8 lg:pb-20"
>
	{#if recruiting && board.layout !== 'deck'}
		<!--
			Before Polis clusters there is genuinely nothing to plot, so the whole display
			recruits instead of showing an empty map. It is the recruitment screen rather
			than the board, so the block set does not apply to it. Deck opts out: its
			first slide is already this screen.
		-->
		<SizedBlock scale={board.scale}>
			<WarmingScreen
				question={data.question}
				joinUrl={data.joinUrl}
				participants={participantCount(source.state)}
				votes={source.state.totalVotes}
				unlockLabel={unlock ? describeUnlock(unlock) : null}
			/>
		</SizedBlock>
	{:else if board.layout === 'deck'}
		<LayoutDeck {source} {board} question={data.question} joinUrl={data.joinUrl} />
	{:else if board.layout === 'split'}
		<LayoutSplit {source} {board} question={data.question} joinUrl={data.joinUrl} />
	{:else}
		<LayoutConsole
			{source}
			{board}
			{surface}
			console={consoleState}
			{onSetConsole}
			{onSetSurface}
			question={data.question}
			joinUrl={data.joinUrl}
		/>
	{/if}
</div>

<BoardSettings
	{board}
	{onSetPreset}
	{onToggleBlock}
	{onSetLayout}
	{onSetLatest}
	{onSetTheme}
	{onSetScale}
	{onSetBlockSize}
	{onReset}
/>

{#if dev && driver}
	<PrototypeBar {driver} />
{/if}

<style>
	/*
	 * The theme's text and spacing sizes, captured once where nothing has scaled them
	 * yet. Every block (SizedBlock.svelte) multiplies from these rather than from the
	 * live variables, so a block inside a block does not compound its parent's size.
	 */
	.room-display {
		--room-base-spacing: var(--spacing);
		--room-base-text-xs: var(--text-xs);
		--room-base-text-sm: var(--text-sm);
		--room-base-text-base: var(--text-base);
		--room-base-text-lg: var(--text-lg);
		--room-base-text-xl: var(--text-xl);
		--room-base-text-2xl: var(--text-2xl);
		--room-base-text-3xl: var(--text-3xl);
		--room-base-text-4xl: var(--text-4xl);
		--room-base-text-5xl: var(--text-5xl);
		--room-base-text-6xl: var(--text-6xl);
	}
</style>
