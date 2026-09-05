<!--
	@component PROTOTYPE variant: Board, decluttered.

	The Board direction as CONTEXT.md defines it (everything on one screen, the room
	interprets) but cut from three competing regions to two. The map is the canvas and
	gets the whole screen; the strip and one line of statement text sit under it.

	What was removed and why:
	- The statement ticker column. Twelve statements of body copy is a wall nobody reads
	  across a room, and it was taking a third of the width to say what the newest line
	  already says. One statement, crossfading, keeps the "the room is still talking"
	  signal at a tenth of the ink.
	- The continuum's card, heading and subtitle. The room does not need the plot to
	  introduce itself.
	- The driver controls. They belong to the prototype harness, not the display.

	The one thing on screen that a facilitator touches is still the strip, so focus has
	a single origin (CONTEXT.md, "Statement ticker").
-->
<script lang="ts">
	import type { RoomDisplayDriver } from '$lib/room-display/driver.svelte';
	import type { Scenario } from '$lib/room-display/types';
	import OpinionMap from '$lib/room-display/OpinionMap.svelte';
	import StatementStrip from './StatementStrip.svelte';

	type Props = {
		driver: RoomDisplayDriver;
		scenario: Scenario;
		question: string;
	};

	let { driver, scenario, question }: Props = $props();

	let focusedTid = $state<number | null>(null);

	const groupIds = $derived(scenario.groups.map((g) => g.group_id));
	const focused = $derived(driver.state.published.find((c) => c.tid === focusedTid) ?? null);
	const newest = $derived(driver.state.published[0] ?? null);
	const shown = $derived(focused ?? newest);
</script>

<div class="flex h-full min-h-0 flex-col gap-4">
	<!-- Question stays on screen all session: people look up mid-argument and need it. -->
	<h1 class="text-foreground max-w-5xl text-2xl leading-tight font-bold text-balance lg:text-3xl">
		{question}
	</h1>

	<div class="min-h-0 flex-1">
		<OpinionMap
			nodes={driver.state.nodes}
			votesByTid={driver.state.votesByTid}
			{focusedTid}
			{groupIds}
			interactive={driver.mode === 'driven'}
		/>
	</div>

	<div class="flex shrink-0 flex-col gap-3">
		<StatementStrip
			comments={driver.state.published}
			{focusedTid}
			interactive={driver.mode === 'driven'}
			height={110}
			onfocusstatement={(tid) => (focusedTid = tid)}
		/>

		{#if shown}
			<!--
				Keyed so a new statement remounts and replays the fade. Fixed height, because
				a line that grows and shrinks shoves the strip around every few seconds.
			-->
			{#key shown.tid}
				<p
					class="text-foreground fade-in flex h-20 items-center text-2xl leading-snug font-medium text-balance lg:text-4xl"
				>
					{shown.text}
				</p>
			{/key}
		{/if}
	</div>
</div>

<style>
	.fade-in {
		animation: fade-in 400ms ease both;
	}

	@keyframes fade-in {
		from {
			opacity: 0;
			transform: translateY(6px);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.fade-in {
			animation: none;
		}
	}
</style>
