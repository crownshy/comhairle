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
		resolveBoard,
		serializeBlocks,
		toggleBlock,
		type RoomBlock,
		type RoomBoard,
		type RoomLayout
	} from '$lib/room-display/blocks';
	import { readStoredBoard, writeStoredBoard } from '$lib/room-display/storedBoard';
	import WarmingScreen from '$lib/room-display/WarmingScreen.svelte';
	import PrototypeBar from './PrototypeBar.svelte';
	import BoardSettings from './BoardSettings.svelte';
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

	onMount(() => {
		// localStorage is not readable during SSR, so what this display remembered can
		// only be folded in here. Resolving again rather than assigning the stored board
		// keeps the precedence rule in one place: an explicit URL still wins.
		board = resolveBoard({ ...data.boardParams, stored: readStoredBoard() });
	});

	/**
	 * The URL is rewritten to spell the board out, so the address bar is always a link
	 * that reproduces what is on screen. `variant` goes: once a block has been touched
	 * by hand the preset name is no longer true, and leaving it would make the link
	 * mean something different from the screen that produced it.
	 */
	function applyBoard(next: RoomBoard) {
		board = next;
		writeStoredBoard(next);

		const url = new URL(window.location.href);
		url.searchParams.delete('variant');
		url.searchParams.set('layout', next.layout);
		url.searchParams.set('blocks', serializeBlocks(next.blocks));
		// This rewrites the query string of the page we are already on rather than
		// navigating anywhere, so there is no route for `resolve()` to resolve.
		// eslint-disable-next-line svelte/no-navigation-without-resolve
		replaceState(url, page.state);
	}

	function onToggleBlock(block: RoomBlock) {
		applyBoard(toggleBlock(board, block));
	}

	function onSetLayout(layout: RoomLayout) {
		applyBoard({ layout, blocks: board.blocks });
	}

	function onReset() {
		applyBoard(resolveBoard({ preset: data.boardParams.preset }));
	}

	const unlock = $derived(
		source.perParticipantVotes ? nextUnlock(source.state, source.stage) : null
	);
	const recruiting = $derived(source.stage === 'empty' || source.stage === 'warming');
</script>

<svelte:head><title>Room display</title></svelte:head>

<div class="bg-background text-foreground h-screen overflow-hidden p-8 pb-20">
	{#if recruiting && board.layout !== 'deck'}
		<!--
			Before Polis clusters there is genuinely nothing to plot, so the whole display
			recruits instead of showing an empty map. It is the recruitment screen rather
			than the board, so the block set does not apply to it. Deck opts out: its
			first slide is already this screen.
		-->
		<WarmingScreen
			question={data.question}
			joinUrl={data.joinUrl}
			participants={participantCount(source.state)}
			votes={source.state.totalVotes}
			unlockLabel={unlock ? describeUnlock(unlock) : null}
		/>
	{:else if board.layout === 'deck'}
		<LayoutDeck {source} {board} question={data.question} joinUrl={data.joinUrl} />
	{:else if board.layout === 'split'}
		<LayoutSplit {source} {board} question={data.question} joinUrl={data.joinUrl} />
	{:else}
		<LayoutConsole {source} {board} question={data.question} joinUrl={data.joinUrl} />
	{/if}
</div>

<BoardSettings {board} {onToggleBlock} {onSetLayout} {onReset} />

{#if dev && driver}
	<PrototypeBar {driver} />
{/if}
