<!--
	@component PROTOTYPE variant: Narrator.

	The Narrator direction from CONTEXT.md: a templated rolling commentary, the machine
	interprets. Nobody drives it. It cycles the ambient focus intents already built in
	`ambientFocus.ts` ("where the room splits hardest", "what the room agrees on most",
	"just added", "most voted on"), resolving each against live data every time it comes
	round, and gives each one the whole screen for a dwell.

	This is the Deck with the facilitator replaced by a timer, so the two are worth
	judging against each other: same one-idea-per-screen discipline, opposite answer to
	"who decides when it moves".

	The dwell runs on wall-clock time, not scenario time. The prototype plays 4.5 hours
	at 90x, and a rotation tied to scenario time would strobe.
-->
<script lang="ts">
	import { onDestroy } from 'svelte';
	import type { RoomDisplayDriver } from '$lib/room-display/driver.svelte';
	import type { Scenario } from '$lib/room-display/types';
	import { ambientFocusAt, describeIntent } from '$lib/room-display/ambientFocus';
	import OpinionMap from '$lib/room-display/OpinionMap.svelte';

	type Props = {
		driver: RoomDisplayDriver;
		scenario: Scenario;
		question: string;
	};

	let { driver, scenario, question }: Props = $props();

	const DWELL_MS = 9000;

	let wallMs = $state(0);
	const startedAt = Date.now();
	// 100ms is fine for a progress bar nobody is measuring, and cheaper than a frame loop.
	const timer = setInterval(() => (wallMs = Date.now() - startedAt), 100);
	onDestroy(() => clearInterval(timer));

	const focus = $derived(ambientFocusAt(driver.state, wallMs, DWELL_MS));
	const statement = $derived(driver.state.published.find((c) => c.tid === focus.tid) ?? null);
	const groupIds = $derived(scenario.groups.map((g) => g.group_id));
	const dwellProgress = $derived((wallMs % DWELL_MS) / DWELL_MS);
</script>

<div class="flex h-full min-h-0 flex-col gap-6">
	<!--
		The bar is the only thing telling the room the screen is about to move. Without
		it a slide change reads as the display glitching rather than turning a page.
	-->
	<div class="bg-border h-1 w-full shrink-0 overflow-hidden rounded-full">
		<div
			class="bg-primary h-full origin-left"
			style="transform: scaleX({dwellProgress}); transition: transform 100ms linear"
		></div>
	</div>

	<div class="flex shrink-0 flex-col gap-2">
		<p class="text-muted-foreground text-base font-medium tracking-wide uppercase">
			{question}
		</p>
		<!-- Keyed on the intent so the headline and statement change together, visibly. -->
		{#key focus.intent}
			<h1
				class="text-foreground fade-in text-3xl leading-tight font-bold text-balance lg:text-5xl"
			>
				{describeIntent(focus.intent)}
			</h1>
		{/key}
	</div>

	{#if statement}
		{#key statement.tid}
			<p
				class="text-foreground fade-in shrink-0 text-2xl leading-snug font-medium text-balance lg:text-4xl"
			>
				{statement.text}
			</p>
		{/key}
	{/if}

	<div class="min-h-0 flex-1">
		<OpinionMap
			nodes={driver.state.nodes}
			votesByTid={driver.state.votesByTid}
			focusedTid={focus.tid}
			{groupIds}
		/>
	</div>
</div>

<style>
	.fade-in {
		animation: fade-in 500ms ease both;
	}

	@keyframes fade-in {
		from {
			opacity: 0;
			transform: translateY(10px);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.fade-in {
			animation: none;
		}
	}
</style>
