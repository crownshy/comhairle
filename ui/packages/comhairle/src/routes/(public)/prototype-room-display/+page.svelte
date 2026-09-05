<!--
	PROTOTYPE harness for the Room display.

	Question it is answering: the one-screen board is too busy, so what should the Room
	display actually look like? Four variants on this route, switchable with `?variant=`,
	all fed by the same scripted scenario so they are judged on arrangement rather than
	on data.

	They map onto the three directions CONTEXT.md already names, plus a fourth that
	splits the surface in two:

	  board    - Board, cut from three regions to two. The room interprets.
	  deck     - Deck. One idea per screen, the facilitator advances. They interpret.
	  narrator - Narrator. Same discipline, advanced by a timer. The machine interprets.
	  console  - Board split across two surfaces: a calm wall and a dense laptop panel.

	Not a Room display direction itself: this deliberately exposes the driver's controls
	(in the floating bar) rather than hiding them the way a projector view would.

	Configuration is URL parameters (CONTEXT.md, Room display), so each variation is a
	link you can send someone: ?variant=deck&rate=200&question=...&join=...
-->
<script lang="ts">
	import { onDestroy } from 'svelte';
	import { page } from '$app/state';
	import { replaceState } from '$app/navigation';
	import { dev } from '$app/environment';
	import { createRoomDisplayDriver } from '$lib/room-display/driver.svelte';
	import { buildScenario } from '$lib/room-display/buildScenario';
	import { PLACEHOLDER_STATEMENTS } from '$lib/room-display/placeholderStatements';
	import { buildPlaceholderComments } from '$lib/room-display/placeholderReport';
	import { nextUnlock, describeUnlock } from '$lib/room-display/revealStage';
	import { participantCount } from '$lib/room-display/scenario';
	import WarmingScreen from '$lib/room-display/WarmingScreen.svelte';
	import PrototypeBar from './PrototypeBar.svelte';
	import VariantBoard from './VariantBoard.svelte';
	import VariantDeck from './VariantDeck.svelte';
	import VariantNarrator from './VariantNarrator.svelte';
	import VariantConsole from './VariantConsole.svelte';

	const ROOM_SIZE = 28;

	/** `handlesWarming`: the variant has its own pre-clustering screen, so the shell's is skipped. */
	const VARIANTS = [
		{ key: 'board', name: 'Board, decluttered', handlesWarming: false },
		{ key: 'deck', name: 'Deck, facilitator advances', handlesWarming: true },
		{ key: 'narrator', name: 'Narrator, auto-advances', handlesWarming: false },
		{ key: 'console', name: 'Stage and console', handlesWarming: false }
	] as const;

	// Read once at setup: the scenario and the driver's rate are fixed for the life of
	// the page, so re-deriving them would rebuild the run under the viewer.
	const setupParams = page.url.searchParams;
	const params = $derived(page.url.searchParams);
	const question = $derived(
		params.get('question') ?? 'What has to change for the Arctic over the next decade?'
	);
	// Stands in for an open-invite URL. The real display points the QR at an existing
	// open invite so walk-up scanners never hit a signup wall.
	const joinUrl = $derived(params.get('join') ?? `${page.url.origin}/prototype-room-display`);

	const variant = $derived(VARIANTS.find((v) => v.key === params.get('variant')) ?? VARIANTS[0]);

	const scenario = buildScenario({
		statements: PLACEHOLDER_STATEMENTS,
		sourceComments: buildPlaceholderComments(PLACEHOLDER_STATEMENTS),
		participantCount: ROOM_SIZE,
		durationMs: 4.5 * 60 * 60 * 1000,
		seed: 20261007
	});

	// 4.5 hours compressed to about three minutes, so a run is watchable.
	const driver = createRoomDisplayDriver({
		scenario,
		rate: Number(setupParams.get('rate')) || 90,
		autoplay: true
	});
	onDestroy(() => driver.destroy());

	const unlock = $derived(nextUnlock(driver.state, driver.stage));
	const recruiting = $derived(driver.stage === 'empty' || driver.stage === 'warming');

	// Shallow routing: a `goto` would rerun load and rebuild the driver, so switching
	// variants would restart the run and lose your place in it.
	function selectVariant(key: string) {
		const url = new URL(page.url);
		url.searchParams.set('variant', key);
		// Same route, query only, so `resolve` has no path to resolve and the rule's
		// escape hatch (a bare `resolve()` argument) cannot carry a search string.
		// eslint-disable-next-line svelte/no-navigation-without-resolve
		replaceState(url, page.state);
	}
</script>

<svelte:head><title>Room display prototype</title></svelte:head>

<div class="bg-background text-foreground h-screen overflow-hidden p-8 pb-20">
	{#if recruiting && !variant.handlesWarming}
		<!--
			Before Polis clusters there is genuinely nothing to plot, so the whole display
			recruits instead of showing an empty map. Deck opts out: its first slide is
			already the recruitment screen.
		-->
		<WarmingScreen
			{question}
			{joinUrl}
			participants={participantCount(driver.state)}
			votes={driver.state.totalVotes}
			unlockLabel={unlock ? describeUnlock(unlock) : null}
		/>
	{:else if variant.key === 'board'}
		<VariantBoard {driver} {scenario} {question} />
	{:else if variant.key === 'deck'}
		<VariantDeck {driver} {scenario} {question} {joinUrl} />
	{:else if variant.key === 'narrator'}
		<VariantNarrator {driver} {scenario} {question} />
	{:else if variant.key === 'console'}
		<VariantConsole {driver} {scenario} {question} />
	{/if}
</div>

{#if dev}
	<PrototypeBar variants={VARIANTS} current={variant.key} {driver} onselect={selectVariant} />
{/if}
