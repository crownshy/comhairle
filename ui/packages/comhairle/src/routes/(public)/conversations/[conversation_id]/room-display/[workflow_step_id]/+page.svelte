<!--
	The Room display for one Polis step, projected in the room while the conversation runs.
	Live by default, or a scripted demo with `?mode=demo`. See NOTES.md.
-->
<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { page } from '$app/state';
	import { dev } from '$app/environment';
	import { replaceState } from '$app/navigation';
	import type { PageProps } from './$types';
	import type { RoomDisplaySource } from './source';
	import { createRoomDisplayDriver } from './demo/driver.svelte';
	import { createLiveRoomDisplaySource } from './live/liveSource.svelte';
	import { buildScenario } from './demo/buildScenario';
	import { PLACEHOLDER_STATEMENTS } from './demo/placeholderStatements';
	import { buildPlaceholderComments } from './demo/placeholderReport';
	import { nextUnlock, describeUnlock } from './revealStage';
	import { participantCount } from './demo/scenario';
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
		type RoomTheme,
		type SlideStyle
	} from './board/blocks';
	import { themeStore } from '$lib/stores/theme.svelte';
	import {
		INITIAL_CONSOLE_STATE,
		openSurfaceLink,
		surfaceHref,
		type ConsoleState,
		type RoomSurface
	} from './board/surfaces';
	import { readStoredBoard, writeStoredBoard } from './board/storedBoard';
	import WarmingScreen from './WarmingScreen.svelte';
	import PrototypeBar from './demo/PrototypeBar.svelte';
	import BoardSettings from './board/BoardSettings.svelte';
	import SizedBlock from './board/SizedBlock.svelte';
	import * as Layout from './Layout';

	let { data }: PageProps = $props();

	const ROOM_SIZE = 28;

	// Built once on purpose: rebuilding the source when `data` changes would restart the run.
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

	let board = $derived<RoomBoard>(data.board);
	let surface = $derived<RoomSurface>(data.surface);

	// Lives here, not in the console layout, because it is shared across windows (ADR-0045).
	let consoleState = $state<ConsoleState>(INITIAL_CONSOLE_STATE);

	// Snapshot before sending: BroadcastChannel cannot clone a `$state` proxy.
	// svelte-ignore state_referenced_locally
	const link = openSurfaceLink(data.workflowStepId, {
		onState: (state) => (consoleState = state),
		onBoard: (next) => applyBoard(next, { share: false }),
		onHello: () => {
			// Only the console answers, so two replies do not race.
			if (surface === 'wall') return;
			link.sendState($state.snapshot(consoleState));
			link.sendBoard($state.snapshot(board));
		}
	});
	onDestroy(() => link.close());

	onMount(() => {
		// localStorage is only readable in the browser. Resolving again keeps the rule
		// that a board set in the URL wins over the stored one.
		board = resolveBoard({ ...data.boardParams, stored: readStoredBoard() });
		applyTheme(board.theme);
	});

	// Rewrites the URL so it always reproduces what is on screen. Pass `share: false` for a
	// board that came from the other window, or the two windows echo it back and forth forever.
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
		url.searchParams.set('slides', next.slides);
		// Only the query string of the current page changes, so there is nothing for `resolve()` to do.
		// eslint-disable-next-line svelte/no-navigation-without-resolve
		replaceState(url, page.state);
	}

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

	function onSetSlideStyle(slides: SlideStyle) {
		applyBoard({ ...board, slides });
	}

	// `auto` leaves the app-wide theme alone, so opening this page never overrides a
	// viewer's own setting. See NOTES.md, "Lighting".
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

	// Written to the URL so a reload keeps this window on the same surface.
	function onSetSurface(next: RoomSurface) {
		surface = next;
		// eslint-disable-next-line svelte/no-navigation-without-resolve
		replaceState(surfaceHref(window.location.href, next), page.state);
	}

	// Needs per-person votes: live data only has vote totals per group, not who cast them.
	const unlock = $derived(
		source.voteMatrix === 'per-participant' ? nextUnlock(source.state, source.stage) : null
	);
	const recruiting = $derived(source.stage === 'empty' || source.stage === 'warming');
</script>

<svelte:head><title>Room display</title></svelte:head>

<!-- Below `lg` the blocks would overlap in a fixed viewport, so the page scrolls instead. -->
<div
	class="room-display bg-background text-foreground min-h-screen p-4 pb-24 lg:h-screen lg:overflow-hidden lg:p-8 lg:pb-20"
>
	{#if recruiting && board.layout !== 'deck'}
		<!-- Nothing to plot before Polis forms groups, so show the join screen. Deck has its own. -->
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
		<Layout.Deck {source} {board} question={data.question} joinUrl={data.joinUrl} />
	{:else if board.layout === 'split'}
		<Layout.Split {source} {board} question={data.question} joinUrl={data.joinUrl} />
	{:else}
		<Layout.Console
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
	{onSetSlideStyle}
	{onReset}
/>

{#if dev && driver}
	<PrototypeBar {driver} />
{/if}

<style>
	/* Unscaled base sizes. SizedBlock multiplies from these so nested blocks do not compound. */
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
