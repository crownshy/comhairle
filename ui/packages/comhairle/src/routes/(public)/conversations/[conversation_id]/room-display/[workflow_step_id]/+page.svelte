<!--
	The Room display for one Polis step: a large-format surface projected in a room
	while the conversation runs (CONTEXT.md, "Room display").

	Two sources behind one display, chosen by `?mode=` (see +page.ts):

	  live  - polls the step's real report data. What a room actually sees.
	  demo  - a scripted 4.5 hour run compressed to a few minutes, with animated joins
	          and votes, so the thing can be shown off without a room. Only the demo
	          carries per-participant votes, so only the demo colours dots by a vote.

	Two directions, chosen by `?variant=`: console (the pick) and deck. The demo's
	transport controls sit in a floating bar in dev builds only.
-->
<script lang="ts">
	import { onDestroy } from 'svelte';
	import { page } from '$app/state';
	import { dev } from '$app/environment';
	import type { PageProps } from './$types';
	import type { RoomDisplaySource } from '$lib/room-display/source';
	import { createRoomDisplayDriver } from '$lib/room-display/driver.svelte';
	import { createLiveRoomDisplaySource } from '$lib/room-display/liveSource.svelte';
	import { buildScenario } from '$lib/room-display/buildScenario';
	import { PLACEHOLDER_STATEMENTS } from '$lib/room-display/placeholderStatements';
	import { buildPlaceholderComments } from '$lib/room-display/placeholderReport';
	import { nextUnlock, describeUnlock } from '$lib/room-display/revealStage';
	import { participantCount } from '$lib/room-display/scenario';
	import WarmingScreen from '$lib/room-display/WarmingScreen.svelte';
	import PrototypeBar from './PrototypeBar.svelte';
	import VariantDeck from './VariantDeck.svelte';
	import VariantConsole from './VariantConsole.svelte';

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

	const unlock = $derived(
		source.perParticipantVotes ? nextUnlock(source.state, source.stage) : null
	);
	const recruiting = $derived(source.stage === 'empty' || source.stage === 'warming');
</script>

<svelte:head><title>Room display</title></svelte:head>

<div class="bg-background text-foreground h-screen overflow-hidden p-8 pb-20">
	{#if recruiting && data.variant !== 'deck'}
		<!--
			Before Polis clusters there is genuinely nothing to plot, so the whole display
			recruits instead of showing an empty map. Deck opts out: its first slide is
			already the recruitment screen.
		-->
		<WarmingScreen
			question={data.question}
			joinUrl={data.joinUrl}
			participants={participantCount(source.state)}
			votes={source.state.totalVotes}
			unlockLabel={unlock ? describeUnlock(unlock) : null}
		/>
	{:else if data.variant === 'deck'}
		<VariantDeck {source} question={data.question} joinUrl={data.joinUrl} />
	{:else}
		<VariantConsole {source} question={data.question} joinUrl={data.joinUrl} />
	{/if}
</div>

{#if dev && driver}
	<PrototypeBar {driver} />
{/if}
